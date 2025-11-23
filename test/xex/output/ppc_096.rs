pub fn sub_82673088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673088 size=112
    let mut pc: u32 = 0x82673088;
    'dispatch: loop {
        match pc {
            0x82673088 => {
    //   block [0x82673088..0x826730F8)
	// 82673088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267308C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673094: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673098: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267309C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 826730A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826730A4: 390B1F30  addi r8, r11, 0x1f30
	ctx.r[8].s64 = ctx.r[11].s64 + 7984;
	// 826730A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826730AC: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 826730B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826730B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826730B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826730BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826730C0: 386A2CB8  addi r3, r10, 0x2cb8
	ctx.r[3].s64 = ctx.r[10].s64 + 11448;
	// 826730C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826730C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826730CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826730D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826730D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826730D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826730DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826730E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826730E4: 4BDF3D3D  bl 0x82466e20
	ctx.lr = 0x826730E8;
	sub_82466E20(ctx, base);
	// 826730E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826730EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826730F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826730F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826730F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826730F8 size=112
    let mut pc: u32 = 0x826730F8;
    'dispatch: loop {
        match pc {
            0x826730F8 => {
    //   block [0x826730F8..0x82673168)
	// 826730F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826730FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673104: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673108: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267310C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82673110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673114: 390B1F48  addi r8, r11, 0x1f48
	ctx.r[8].s64 = ctx.r[11].s64 + 8008;
	// 82673118: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267311C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 82673120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673124: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673128: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267312C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673130: 386A2CE8  addi r3, r10, 0x2ce8
	ctx.r[3].s64 = ctx.r[10].s64 + 11496;
	// 82673134: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82673138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267313C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673144: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82673148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267314C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673154: 4BDF3CCD  bl 0x82466e20
	ctx.lr = 0x82673158;
	sub_82466E20(ctx, base);
	// 82673158: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267315C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673168 size=112
    let mut pc: u32 = 0x82673168;
    'dispatch: loop {
        match pc {
            0x82673168 => {
    //   block [0x82673168..0x826731D8)
	// 82673168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267316C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673174: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673178: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267317C: 38AA2CB8  addi r5, r10, 0x2cb8
	ctx.r[5].s64 = ctx.r[10].s64 + 11448;
	// 82673180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673184: 390B1F60  addi r8, r11, 0x1f60
	ctx.r[8].s64 = ctx.r[11].s64 + 8032;
	// 82673188: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267318C: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 82673190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673194: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673198: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267319C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826731A0: 386A2D18  addi r3, r10, 0x2d18
	ctx.r[3].s64 = ctx.r[10].s64 + 11544;
	// 826731A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826731A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826731AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826731B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826731B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826731B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826731BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826731C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826731C4: 4BDF3C5D  bl 0x82466e20
	ctx.lr = 0x826731C8;
	sub_82466E20(ctx, base);
	// 826731C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826731CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826731D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826731D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826731D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826731D8 size=72
    let mut pc: u32 = 0x826731D8;
    'dispatch: loop {
        match pc {
            0x826731D8 => {
    //   block [0x826731D8..0x82673220)
	// 826731D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826731DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826731E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826731E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826731E8: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 826731EC: 38CB1250  addi r6, r11, 0x1250
	ctx.r[6].s64 = ctx.r[11].s64 + 4688;
	// 826731F0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826731F4: 388B1C08  addi r4, r11, 0x1c08
	ctx.r[4].s64 = ctx.r[11].s64 + 7176;
	// 826731F8: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826731FC: 386B2D48  addi r3, r11, 0x2d48
	ctx.r[3].s64 = ctx.r[11].s64 + 11592;
	// 82673200: 4BE08889  bl 0x8247ba88
	ctx.lr = 0x82673204;
	sub_8247BA88(ctx, base);
	// 82673204: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82673208: 386BCE10  addi r3, r11, -0x31f0
	ctx.r[3].s64 = ctx.r[11].s64 + -12784;
	// 8267320C: 4BEBF92D  bl 0x82532b38
	ctx.lr = 0x82673210;
	sub_82532B38(ctx, base);
	// 82673210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82673214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267321C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673220 size=108
    let mut pc: u32 = 0x82673220;
    'dispatch: loop {
        match pc {
            0x82673220 => {
    //   block [0x82673220..0x8267328C)
	// 82673220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267322C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673234: 38EB2890  addi r7, r11, 0x2890
	ctx.r[7].s64 = ctx.r[11].s64 + 10384;
	// 82673238: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8267323C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 82673240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673244: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673248: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267324C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673250: 386A2D60  addi r3, r10, 0x2d60
	ctx.r[3].s64 = ctx.r[10].s64 + 11616;
	// 82673254: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267325C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267326C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673278: 4BDF3BA9  bl 0x82466e20
	ctx.lr = 0x8267327C;
	sub_82466E20(ctx, base);
	// 8267327C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673290 size=108
    let mut pc: u32 = 0x82673290;
    'dispatch: loop {
        match pc {
            0x82673290 => {
    //   block [0x82673290..0x826732FC)
	// 82673290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267329C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826732A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826732A4: 38EB2908  addi r7, r11, 0x2908
	ctx.r[7].s64 = ctx.r[11].s64 + 10504;
	// 826732A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826732AC: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 826732B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826732B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826732B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826732BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826732C0: 386A2D90  addi r3, r10, 0x2d90
	ctx.r[3].s64 = ctx.r[10].s64 + 11664;
	// 826732C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826732C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826732CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826732D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826732D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826732D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826732DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826732E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826732E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826732E8: 4BDF3B39  bl 0x82466e20
	ctx.lr = 0x826732EC;
	sub_82466E20(ctx, base);
	// 826732EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826732F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826732F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826732F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673300 size=108
    let mut pc: u32 = 0x82673300;
    'dispatch: loop {
        match pc {
            0x82673300 => {
    //   block [0x82673300..0x8267336C)
	// 82673300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267330C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673310: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82673314: 38EB2938  addi r7, r11, 0x2938
	ctx.r[7].s64 = ctx.r[11].s64 + 10552;
	// 82673318: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267331C: 388AAEA0  addi r4, r10, -0x5160
	ctx.r[4].s64 = ctx.r[10].s64 + -20832;
	// 82673320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673324: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673328: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267332C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673330: 386A2DC0  addi r3, r10, 0x2dc0
	ctx.r[3].s64 = ctx.r[10].s64 + 11712;
	// 82673334: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267333C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267334C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673358: 4BDF3AC9  bl 0x82466e20
	ctx.lr = 0x8267335C;
	sub_82466E20(ctx, base);
	// 8267335C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673370 size=108
    let mut pc: u32 = 0x82673370;
    'dispatch: loop {
        match pc {
            0x82673370 => {
    //   block [0x82673370..0x826733DC)
	// 82673370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267337C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673380: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82673384: 38EB2968  addi r7, r11, 0x2968
	ctx.r[7].s64 = ctx.r[11].s64 + 10600;
	// 82673388: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267338C: 388AAE2C  addi r4, r10, -0x51d4
	ctx.r[4].s64 = ctx.r[10].s64 + -20948;
	// 82673390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673394: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673398: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267339C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826733A0: 386A2DF0  addi r3, r10, 0x2df0
	ctx.r[3].s64 = ctx.r[10].s64 + 11760;
	// 826733A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826733A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826733AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826733B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826733B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826733B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826733BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826733C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826733C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826733C8: 4BDF3A59  bl 0x82466e20
	ctx.lr = 0x826733CC;
	sub_82466E20(ctx, base);
	// 826733CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826733D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826733D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826733D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826733E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826733E0 size=112
    let mut pc: u32 = 0x826733E0;
    'dispatch: loop {
        match pc {
            0x826733E0 => {
    //   block [0x826733E0..0x82673450)
	// 826733E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826733E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826733E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826733EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826733F0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826733F4: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 826733F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826733FC: 390B2998  addi r8, r11, 0x2998
	ctx.r[8].s64 = ctx.r[11].s64 + 10648;
	// 82673400: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82673404: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 82673408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267340C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673410: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82673414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673418: 386A2E20  addi r3, r10, 0x2e20
	ctx.r[3].s64 = ctx.r[10].s64 + 11808;
	// 8267341C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82673420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267342C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267343C: 4BDF39E5  bl 0x82466e20
	ctx.lr = 0x82673440;
	sub_82466E20(ctx, base);
	// 82673440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267344C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673450 size=108
    let mut pc: u32 = 0x82673450;
    'dispatch: loop {
        match pc {
            0x82673450 => {
    //   block [0x82673450..0x826734BC)
	// 82673450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267345C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673460: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82673464: 38EB29F8  addi r7, r11, 0x29f8
	ctx.r[7].s64 = ctx.r[11].s64 + 10744;
	// 82673468: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267346C: 388AAE70  addi r4, r10, -0x5190
	ctx.r[4].s64 = ctx.r[10].s64 + -20880;
	// 82673470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673474: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673478: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267347C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673480: 386A2E50  addi r3, r10, 0x2e50
	ctx.r[3].s64 = ctx.r[10].s64 + 11856;
	// 82673484: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267348C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267349C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826734A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826734A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826734A8: 4BDF3979  bl 0x82466e20
	ctx.lr = 0x826734AC;
	sub_82466E20(ctx, base);
	// 826734AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826734B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826734B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826734B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826734C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826734C0 size=112
    let mut pc: u32 = 0x826734C0;
    'dispatch: loop {
        match pc {
            0x826734C0 => {
    //   block [0x826734C0..0x82673530)
	// 826734C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826734C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826734C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826734CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826734D0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826734D4: 38AA2E20  addi r5, r10, 0x2e20
	ctx.r[5].s64 = ctx.r[10].s64 + 11808;
	// 826734D8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826734DC: 390B2A58  addi r8, r11, 0x2a58
	ctx.r[8].s64 = ctx.r[11].s64 + 10840;
	// 826734E0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826734E4: 388AAF30  addi r4, r10, -0x50d0
	ctx.r[4].s64 = ctx.r[10].s64 + -20688;
	// 826734E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826734EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826734F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826734F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826734F8: 386A2E80  addi r3, r10, 0x2e80
	ctx.r[3].s64 = ctx.r[10].s64 + 11904;
	// 826734FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82673500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267350C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267351C: 4BDF3905  bl 0x82466e20
	ctx.lr = 0x82673520;
	sub_82466E20(ctx, base);
	// 82673520: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267352C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673530 size=112
    let mut pc: u32 = 0x82673530;
    'dispatch: loop {
        match pc {
            0x82673530 => {
    //   block [0x82673530..0x826735A0)
	// 82673530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267353C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673540: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673544: 38AA2E20  addi r5, r10, 0x2e20
	ctx.r[5].s64 = ctx.r[10].s64 + 11808;
	// 82673548: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267354C: 390B2AE8  addi r8, r11, 0x2ae8
	ctx.r[8].s64 = ctx.r[11].s64 + 10984;
	// 82673550: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82673554: 388AAF18  addi r4, r10, -0x50e8
	ctx.r[4].s64 = ctx.r[10].s64 + -20712;
	// 82673558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267355C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673560: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82673564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673568: 386A2EB0  addi r3, r10, 0x2eb0
	ctx.r[3].s64 = ctx.r[10].s64 + 11952;
	// 8267356C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82673570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267357C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267358C: 4BDF3895  bl 0x82466e20
	ctx.lr = 0x82673590;
	sub_82466E20(ctx, base);
	// 82673590: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267359C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826735A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826735A0 size=108
    let mut pc: u32 = 0x826735A0;
    'dispatch: loop {
        match pc {
            0x826735A0 => {
    //   block [0x826735A0..0x8267360C)
	// 826735A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826735A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826735A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826735AC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826735B0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826735B4: 38EB2B00  addi r7, r11, 0x2b00
	ctx.r[7].s64 = ctx.r[11].s64 + 11008;
	// 826735B8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826735BC: 388AAE40  addi r4, r10, -0x51c0
	ctx.r[4].s64 = ctx.r[10].s64 + -20928;
	// 826735C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826735C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826735C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826735CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826735D0: 386A2EE0  addi r3, r10, 0x2ee0
	ctx.r[3].s64 = ctx.r[10].s64 + 12000;
	// 826735D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826735D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826735DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826735E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826735E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826735E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826735EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826735F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826735F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826735F8: 4BDF3829  bl 0x82466e20
	ctx.lr = 0x826735FC;
	sub_82466E20(ctx, base);
	// 826735FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673610 size=112
    let mut pc: u32 = 0x82673610;
    'dispatch: loop {
        match pc {
            0x82673610 => {
    //   block [0x82673610..0x82673680)
	// 82673610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267361C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673620: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673624: 38AA2E20  addi r5, r10, 0x2e20
	ctx.r[5].s64 = ctx.r[10].s64 + 11808;
	// 82673628: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267362C: 390B2B60  addi r8, r11, 0x2b60
	ctx.r[8].s64 = ctx.r[11].s64 + 11104;
	// 82673630: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82673634: 388AAEF8  addi r4, r10, -0x5108
	ctx.r[4].s64 = ctx.r[10].s64 + -20744;
	// 82673638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267363C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82673644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673648: 386A2F10  addi r3, r10, 0x2f10
	ctx.r[3].s64 = ctx.r[10].s64 + 12048;
	// 8267364C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82673650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267365C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267366C: 4BDF37B5  bl 0x82466e20
	ctx.lr = 0x82673670;
	sub_82466E20(ctx, base);
	// 82673670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267367C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673680 size=108
    let mut pc: u32 = 0x82673680;
    'dispatch: loop {
        match pc {
            0x82673680 => {
    //   block [0x82673680..0x826736EC)
	// 82673680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267368C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673694: 38EB2C08  addi r7, r11, 0x2c08
	ctx.r[7].s64 = ctx.r[11].s64 + 11272;
	// 82673698: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267369C: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 826736A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826736A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826736A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826736AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826736B0: 386A2F40  addi r3, r10, 0x2f40
	ctx.r[3].s64 = ctx.r[10].s64 + 12096;
	// 826736B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826736B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826736BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826736C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826736C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826736C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826736CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826736D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826736D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826736D8: 4BDF3749  bl 0x82466e20
	ctx.lr = 0x826736DC;
	sub_82466E20(ctx, base);
	// 826736DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826736E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826736E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826736E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826736F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826736F0 size=100
    let mut pc: u32 = 0x826736F0;
    'dispatch: loop {
        match pc {
            0x826736F0 => {
    //   block [0x826736F0..0x82673754)
	// 826736F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826736F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826736F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826736FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673704: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82673708: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267370C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673710: 388AAE18  addi r4, r10, -0x51e8
	ctx.r[4].s64 = ctx.r[10].s64 + -20968;
	// 82673714: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267371C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673724: 386A2F70  addi r3, r10, 0x2f70
	ctx.r[3].s64 = ctx.r[10].s64 + 12144;
	// 82673728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267372C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673730: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82673734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673738: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267373C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673740: 4BDF36E1  bl 0x82466e20
	ctx.lr = 0x82673744;
	sub_82466E20(ctx, base);
	// 82673744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267374C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673758 size=112
    let mut pc: u32 = 0x82673758;
    'dispatch: loop {
        match pc {
            0x82673758 => {
    //   block [0x82673758..0x826737C8)
	// 82673758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267375C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673764: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673768: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267376C: 38AA2F70  addi r5, r10, 0x2f70
	ctx.r[5].s64 = ctx.r[10].s64 + 12144;
	// 82673770: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82673774: 390B2C68  addi r8, r11, 0x2c68
	ctx.r[8].s64 = ctx.r[11].s64 + 11368;
	// 82673778: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8267377C: 388AAF4C  addi r4, r10, -0x50b4
	ctx.r[4].s64 = ctx.r[10].s64 + -20660;
	// 82673780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673784: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267378C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673790: 386A2FA0  addi r3, r10, 0x2fa0
	ctx.r[3].s64 = ctx.r[10].s64 + 12192;
	// 82673794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82673798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267379C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826737A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826737A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826737A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826737AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826737B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826737B4: 4BDF366D  bl 0x82466e20
	ctx.lr = 0x826737B8;
	sub_82466E20(ctx, base);
	// 826737B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826737BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826737C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826737C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826737C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826737C8 size=96
    let mut pc: u32 = 0x826737C8;
    'dispatch: loop {
        match pc {
            0x826737C8 => {
    //   block [0x826737C8..0x82673828)
	// 826737C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826737CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826737D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826737D4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826737D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826737DC: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826737E0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826737E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826737E8: 386A2FD0  addi r3, r10, 0x2fd0
	ctx.r[3].s64 = ctx.r[10].s64 + 12240;
	// 826737EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826737F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826737F4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826737F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826737FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673808: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267380C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673810: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82673814: 4BDF360D  bl 0x82466e20
	ctx.lr = 0x82673818;
	sub_82466E20(ctx, base);
	// 82673818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267381C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673828 size=112
    let mut pc: u32 = 0x82673828;
    'dispatch: loop {
        match pc {
            0x82673828 => {
    //   block [0x82673828..0x82673898)
	// 82673828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267382C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673834: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673838: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267383C: 38AA2FD0  addi r5, r10, 0x2fd0
	ctx.r[5].s64 = ctx.r[10].s64 + 12240;
	// 82673840: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82673844: 390B2CE0  addi r8, r11, 0x2ce0
	ctx.r[8].s64 = ctx.r[11].s64 + 11488;
	// 82673848: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267384C: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 82673850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673854: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673858: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267385C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673860: 386A3000  addi r3, r10, 0x3000
	ctx.r[3].s64 = ctx.r[10].s64 + 12288;
	// 82673864: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82673868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267386C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267387C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673884: 4BDF359D  bl 0x82466e20
	ctx.lr = 0x82673888;
	sub_82466E20(ctx, base);
	// 82673888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267388C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673898 size=108
    let mut pc: u32 = 0x82673898;
    'dispatch: loop {
        match pc {
            0x82673898 => {
    //   block [0x82673898..0x82673904)
	// 82673898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267389C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826738A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826738A4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826738A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826738AC: 38EB2D10  addi r7, r11, 0x2d10
	ctx.r[7].s64 = ctx.r[11].s64 + 11536;
	// 826738B0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826738B4: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 826738B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826738BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826738C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826738C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826738C8: 386A3030  addi r3, r10, 0x3030
	ctx.r[3].s64 = ctx.r[10].s64 + 12336;
	// 826738CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826738D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826738D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826738D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826738DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826738E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826738E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826738E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826738EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826738F0: 4BDF3531  bl 0x82466e20
	ctx.lr = 0x826738F4;
	sub_82466E20(ctx, base);
	// 826738F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826738F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826738FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673908 size=108
    let mut pc: u32 = 0x82673908;
    'dispatch: loop {
        match pc {
            0x82673908 => {
    //   block [0x82673908..0x82673974)
	// 82673908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267390C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673914: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673918: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267391C: 38EB2DB8  addi r7, r11, 0x2db8
	ctx.r[7].s64 = ctx.r[11].s64 + 11704;
	// 82673920: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82673924: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 82673928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267392C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673930: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673938: 386A3060  addi r3, r10, 0x3060
	ctx.r[3].s64 = ctx.r[10].s64 + 12384;
	// 8267393C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267394C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267395C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673960: 4BDF34C1  bl 0x82466e20
	ctx.lr = 0x82673964;
	sub_82466E20(ctx, base);
	// 82673964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267396C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673978 size=108
    let mut pc: u32 = 0x82673978;
    'dispatch: loop {
        match pc {
            0x82673978 => {
    //   block [0x82673978..0x826739E4)
	// 82673978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267397C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673984: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673988: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267398C: 38EB2DE8  addi r7, r11, 0x2de8
	ctx.r[7].s64 = ctx.r[11].s64 + 11752;
	// 82673990: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82673994: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 82673998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267399C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826739A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826739A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826739A8: 386A3090  addi r3, r10, 0x3090
	ctx.r[3].s64 = ctx.r[10].s64 + 12432;
	// 826739AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826739B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826739B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826739B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826739BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826739C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826739C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826739C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826739CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826739D0: 4BDF3451  bl 0x82466e20
	ctx.lr = 0x826739D4;
	sub_82466E20(ctx, base);
	// 826739D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826739D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826739DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826739E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826739E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826739E8 size=28
    let mut pc: u32 = 0x826739E8;
    'dispatch: loop {
        match pc {
            0x826739E8 => {
    //   block [0x826739E8..0x82673A04)
	// 826739E8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826739EC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826739F0: 394A6BA0  addi r10, r10, 0x6ba0
	ctx.r[10].s64 = ctx.r[10].s64 + 27552;
	// 826739F4: 816B2E18  lwz r11, 0x2e18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11800 as u32) ) } as u64;
	// 826739F8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826739FC: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82673A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673A08 size=112
    let mut pc: u32 = 0x82673A08;
    'dispatch: loop {
        match pc {
            0x82673A08 => {
    //   block [0x82673A08..0x82673A78)
	// 82673A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673A14: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82673A18: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673A1C: 392A30F0  addi r9, r10, 0x30f0
	ctx.r[9].s64 = ctx.r[10].s64 + 12528;
	// 82673A20: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82673A24: 390B6BA0  addi r8, r11, 0x6ba0
	ctx.r[8].s64 = ctx.r[11].s64 + 27552;
	// 82673A28: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82673A2C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 82673A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673A34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673A38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82673A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673A40: 386A30C0  addi r3, r10, 0x30c0
	ctx.r[3].s64 = ctx.r[10].s64 + 12480;
	// 82673A44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82673A48: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82673A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673A5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673A64: 4BDF33BD  bl 0x82466e20
	ctx.lr = 0x82673A68;
	sub_82466E20(ctx, base);
	// 82673A68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673A78 size=108
    let mut pc: u32 = 0x82673A78;
    'dispatch: loop {
        match pc {
            0x82673A78 => {
    //   block [0x82673A78..0x82673AE4)
	// 82673A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673A84: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673A88: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82673A8C: 38EB2E24  addi r7, r11, 0x2e24
	ctx.r[7].s64 = ctx.r[11].s64 + 11812;
	// 82673A90: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82673A94: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 82673A98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673A9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673AA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673AA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673AA8: 386A30F0  addi r3, r10, 0x30f0
	ctx.r[3].s64 = ctx.r[10].s64 + 12528;
	// 82673AAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673AB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673AB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673AC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673ACC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673AD0: 4BDF3351  bl 0x82466e20
	ctx.lr = 0x82673AD4;
	sub_82466E20(ctx, base);
	// 82673AD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673AE8 size=108
    let mut pc: u32 = 0x82673AE8;
    'dispatch: loop {
        match pc {
            0x82673AE8 => {
    //   block [0x82673AE8..0x82673B54)
	// 82673AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673AF4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673AF8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82673AFC: 38EB2E54  addi r7, r11, 0x2e54
	ctx.r[7].s64 = ctx.r[11].s64 + 11860;
	// 82673B00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82673B04: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 82673B08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673B0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673B10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673B14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673B18: 386A3120  addi r3, r10, 0x3120
	ctx.r[3].s64 = ctx.r[10].s64 + 12576;
	// 82673B1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673B28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673B30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673B38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673B3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673B40: 4BDF32E1  bl 0x82466e20
	ctx.lr = 0x82673B44;
	sub_82466E20(ctx, base);
	// 82673B44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673B58 size=108
    let mut pc: u32 = 0x82673B58;
    'dispatch: loop {
        match pc {
            0x82673B58 => {
    //   block [0x82673B58..0x82673BC4)
	// 82673B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673B64: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673B68: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82673B6C: 38EB2E88  addi r7, r11, 0x2e88
	ctx.r[7].s64 = ctx.r[11].s64 + 11912;
	// 82673B70: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82673B74: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 82673B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673B7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673B80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673B88: 386A3150  addi r3, r10, 0x3150
	ctx.r[3].s64 = ctx.r[10].s64 + 12624;
	// 82673B8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673BA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673BAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673BB0: 4BDF3271  bl 0x82466e20
	ctx.lr = 0x82673BB4;
	sub_82466E20(ctx, base);
	// 82673BB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673BC8 size=108
    let mut pc: u32 = 0x82673BC8;
    'dispatch: loop {
        match pc {
            0x82673BC8 => {
    //   block [0x82673BC8..0x82673C34)
	// 82673BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673BD4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673BD8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82673BDC: 38EB2F48  addi r7, r11, 0x2f48
	ctx.r[7].s64 = ctx.r[11].s64 + 12104;
	// 82673BE0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82673BE4: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 82673BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673BEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673BF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673BF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673BF8: 386A3180  addi r3, r10, 0x3180
	ctx.r[3].s64 = ctx.r[10].s64 + 12672;
	// 82673BFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673C00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673C08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673C10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673C18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673C1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673C20: 4BDF3201  bl 0x82466e20
	ctx.lr = 0x82673C24;
	sub_82466E20(ctx, base);
	// 82673C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673C38 size=108
    let mut pc: u32 = 0x82673C38;
    'dispatch: loop {
        match pc {
            0x82673C38 => {
    //   block [0x82673C38..0x82673CA4)
	// 82673C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673C44: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673C48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673C4C: 38EB2F60  addi r7, r11, 0x2f60
	ctx.r[7].s64 = ctx.r[11].s64 + 12128;
	// 82673C50: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82673C54: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 82673C58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673C5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673C60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673C64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673C68: 386A31B0  addi r3, r10, 0x31b0
	ctx.r[3].s64 = ctx.r[10].s64 + 12720;
	// 82673C6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673C70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673C74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673C78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673C7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673C80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673C84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673C88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673C8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673C90: 4BDF3191  bl 0x82466e20
	ctx.lr = 0x82673C94;
	sub_82466E20(ctx, base);
	// 82673C94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673C98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673C9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673CA8 size=108
    let mut pc: u32 = 0x82673CA8;
    'dispatch: loop {
        match pc {
            0x82673CA8 => {
    //   block [0x82673CA8..0x82673D14)
	// 82673CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673CB4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673CB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673CBC: 38EB2FD8  addi r7, r11, 0x2fd8
	ctx.r[7].s64 = ctx.r[11].s64 + 12248;
	// 82673CC0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82673CC4: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 82673CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673CCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673CD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673CD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673CD8: 386A31E0  addi r3, r10, 0x31e0
	ctx.r[3].s64 = ctx.r[10].s64 + 12768;
	// 82673CDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673CE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673CFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673D00: 4BDF3121  bl 0x82466e20
	ctx.lr = 0x82673D04;
	sub_82466E20(ctx, base);
	// 82673D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673D18 size=108
    let mut pc: u32 = 0x82673D18;
    'dispatch: loop {
        match pc {
            0x82673D18 => {
    //   block [0x82673D18..0x82673D84)
	// 82673D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673D24: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673D28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673D2C: 38EB3068  addi r7, r11, 0x3068
	ctx.r[7].s64 = ctx.r[11].s64 + 12392;
	// 82673D30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82673D34: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 82673D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673D3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673D40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673D48: 386A3210  addi r3, r10, 0x3210
	ctx.r[3].s64 = ctx.r[10].s64 + 12816;
	// 82673D4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673D6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673D70: 4BDF30B1  bl 0x82466e20
	ctx.lr = 0x82673D74;
	sub_82466E20(ctx, base);
	// 82673D74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673D88 size=108
    let mut pc: u32 = 0x82673D88;
    'dispatch: loop {
        match pc {
            0x82673D88 => {
    //   block [0x82673D88..0x82673DF4)
	// 82673D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673D94: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673D98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673D9C: 38EB3098  addi r7, r11, 0x3098
	ctx.r[7].s64 = ctx.r[11].s64 + 12440;
	// 82673DA0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82673DA4: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 82673DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673DAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673DB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673DB8: 386A3240  addi r3, r10, 0x3240
	ctx.r[3].s64 = ctx.r[10].s64 + 12864;
	// 82673DBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673DDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673DE0: 4BDF3041  bl 0x82466e20
	ctx.lr = 0x82673DE4;
	sub_82466E20(ctx, base);
	// 82673DE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82673DF8 size=24
    let mut pc: u32 = 0x82673DF8;
    'dispatch: loop {
        match pc {
            0x82673DF8 => {
    //   block [0x82673DF8..0x82673E10)
	// 82673DF8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673DFC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82673E00: 394A6C60  addi r10, r10, 0x6c60
	ctx.r[10].s64 = ctx.r[10].s64 + 27744;
	// 82673E04: 816B2E84  lwz r11, 0x2e84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11908 as u32) ) } as u64;
	// 82673E08: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82673E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673E10 size=112
    let mut pc: u32 = 0x82673E10;
    'dispatch: loop {
        match pc {
            0x82673E10 => {
    //   block [0x82673E10..0x82673E80)
	// 82673E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673E1C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82673E20: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673E24: 392A3144  addi r9, r10, 0x3144
	ctx.r[9].s64 = ctx.r[10].s64 + 12612;
	// 82673E28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673E2C: 390B6C60  addi r8, r11, 0x6c60
	ctx.r[8].s64 = ctx.r[11].s64 + 27744;
	// 82673E30: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82673E34: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 82673E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673E3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673E40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82673E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673E48: 386A3270  addi r3, r10, 0x3270
	ctx.r[3].s64 = ctx.r[10].s64 + 12912;
	// 82673E4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82673E50: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82673E54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673E64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673E6C: 4BDF2FB5  bl 0x82466e20
	ctx.lr = 0x82673E70;
	sub_82466E20(ctx, base);
	// 82673E70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673E80 size=108
    let mut pc: u32 = 0x82673E80;
    'dispatch: loop {
        match pc {
            0x82673E80 => {
    //   block [0x82673E80..0x82673EEC)
	// 82673E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673E8C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673E94: 38EB3144  addi r7, r11, 0x3144
	ctx.r[7].s64 = ctx.r[11].s64 + 12612;
	// 82673E98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82673E9C: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 82673EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673EA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673EA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673EB0: 386A32A0  addi r3, r10, 0x32a0
	ctx.r[3].s64 = ctx.r[10].s64 + 12960;
	// 82673EB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673ED4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673ED8: 4BDF2F49  bl 0x82466e20
	ctx.lr = 0x82673EDC;
	sub_82466E20(ctx, base);
	// 82673EDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673EF0 size=112
    let mut pc: u32 = 0x82673EF0;
    'dispatch: loop {
        match pc {
            0x82673EF0 => {
    //   block [0x82673EF0..0x82673F60)
	// 82673EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673EFC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82673F00: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673F04: 392A3188  addi r9, r10, 0x3188
	ctx.r[9].s64 = ctx.r[10].s64 + 12680;
	// 82673F08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673F0C: 390B3178  addi r8, r11, 0x3178
	ctx.r[8].s64 = ctx.r[11].s64 + 12664;
	// 82673F10: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82673F14: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 82673F18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673F1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673F20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82673F24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673F28: 386A32D0  addi r3, r10, 0x32d0
	ctx.r[3].s64 = ctx.r[10].s64 + 13008;
	// 82673F2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82673F30: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82673F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673F44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673F48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673F4C: 4BDF2ED5  bl 0x82466e20
	ctx.lr = 0x82673F50;
	sub_82466E20(ctx, base);
	// 82673F50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673F60 size=108
    let mut pc: u32 = 0x82673F60;
    'dispatch: loop {
        match pc {
            0x82673F60 => {
    //   block [0x82673F60..0x82673FCC)
	// 82673F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673F6C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673F74: 38EB3208  addi r7, r11, 0x3208
	ctx.r[7].s64 = ctx.r[11].s64 + 12808;
	// 82673F78: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82673F7C: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 82673F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673F84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673F88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673F90: 386A3300  addi r3, r10, 0x3300
	ctx.r[3].s64 = ctx.r[10].s64 + 13056;
	// 82673F94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673FA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673FB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673FB8: 4BDF2E69  bl 0x82466e20
	ctx.lr = 0x82673FBC;
	sub_82466E20(ctx, base);
	// 82673FBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673FD0 size=108
    let mut pc: u32 = 0x82673FD0;
    'dispatch: loop {
        match pc {
            0x82673FD0 => {
    //   block [0x82673FD0..0x8267403C)
	// 82673FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673FDC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673FE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673FE4: 38EB3220  addi r7, r11, 0x3220
	ctx.r[7].s64 = ctx.r[11].s64 + 12832;
	// 82673FE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82673FEC: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 82673FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673FF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673FF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674000: 386A3330  addi r3, r10, 0x3330
	ctx.r[3].s64 = ctx.r[10].s64 + 13104;
	// 82674004: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267400C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674014: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267401C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674024: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674028: 4BDF2DF9  bl 0x82466e20
	ctx.lr = 0x8267402C;
	sub_82466E20(ctx, base);
	// 8267402C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82674040 size=24
    let mut pc: u32 = 0x82674040;
    'dispatch: loop {
        match pc {
            0x82674040 => {
    //   block [0x82674040..0x82674058)
	// 82674040: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674044: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82674048: 394A6CD8  addi r10, r10, 0x6cd8
	ctx.r[10].s64 = ctx.r[10].s64 + 27864;
	// 8267404C: 816B3174  lwz r11, 0x3174(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12660 as u32) ) } as u64;
	// 82674050: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82674054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674058 size=112
    let mut pc: u32 = 0x82674058;
    'dispatch: loop {
        match pc {
            0x82674058 => {
    //   block [0x82674058..0x826740C8)
	// 82674058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267405C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674064: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82674068: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267406C: 392A31C4  addi r9, r10, 0x31c4
	ctx.r[9].s64 = ctx.r[10].s64 + 12740;
	// 82674070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674074: 390B6CD8  addi r8, r11, 0x6cd8
	ctx.r[8].s64 = ctx.r[11].s64 + 27864;
	// 82674078: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8267407C: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 82674080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674084: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674088: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267408C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674090: 386A3360  addi r3, r10, 0x3360
	ctx.r[3].s64 = ctx.r[10].s64 + 13152;
	// 82674094: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82674098: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267409C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826740A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826740A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826740A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826740AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826740B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826740B4: 4BDF2D6D  bl 0x82466e20
	ctx.lr = 0x826740B8;
	sub_82466E20(ctx, base);
	// 826740B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826740BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826740C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826740C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826740C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826740C8 size=108
    let mut pc: u32 = 0x826740C8;
    'dispatch: loop {
        match pc {
            0x826740C8 => {
    //   block [0x826740C8..0x82674134)
	// 826740C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826740CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826740D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826740D4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826740D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826740DC: 38EB3250  addi r7, r11, 0x3250
	ctx.r[7].s64 = ctx.r[11].s64 + 12880;
	// 826740E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826740E4: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826740E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826740EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826740F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826740F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826740F8: 386A3390  addi r3, r10, 0x3390
	ctx.r[3].s64 = ctx.r[10].s64 + 13200;
	// 826740FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267410C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267411C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674120: 4BDF2D01  bl 0x82466e20
	ctx.lr = 0x82674124;
	sub_82466E20(ctx, base);
	// 82674124: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267412C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674138 size=108
    let mut pc: u32 = 0x82674138;
    'dispatch: loop {
        match pc {
            0x82674138 => {
    //   block [0x82674138..0x826741A4)
	// 82674138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267413C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674144: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267414C: 38EB3268  addi r7, r11, 0x3268
	ctx.r[7].s64 = ctx.r[11].s64 + 12904;
	// 82674150: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82674154: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 82674158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267415C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674160: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674168: 386A33C0  addi r3, r10, 0x33c0
	ctx.r[3].s64 = ctx.r[10].s64 + 13248;
	// 8267416C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267417C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267418C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674190: 4BDF2C91  bl 0x82466e20
	ctx.lr = 0x82674194;
	sub_82466E20(ctx, base);
	// 82674194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267419C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826741A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826741A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826741A8 size=108
    let mut pc: u32 = 0x826741A8;
    'dispatch: loop {
        match pc {
            0x826741A8 => {
    //   block [0x826741A8..0x82674214)
	// 826741A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826741AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826741B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826741B4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826741B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826741BC: 38EB32B0  addi r7, r11, 0x32b0
	ctx.r[7].s64 = ctx.r[11].s64 + 12976;
	// 826741C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826741C4: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826741C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826741CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826741D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826741D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826741D8: 386A33F0  addi r3, r10, 0x33f0
	ctx.r[3].s64 = ctx.r[10].s64 + 13296;
	// 826741DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826741E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826741E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826741E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826741EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826741F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826741F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826741F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826741FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674200: 4BDF2C21  bl 0x82466e20
	ctx.lr = 0x82674204;
	sub_82466E20(ctx, base);
	// 82674204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267420C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674218 size=108
    let mut pc: u32 = 0x82674218;
    'dispatch: loop {
        match pc {
            0x82674218 => {
    //   block [0x82674218..0x82674284)
	// 82674218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267421C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674224: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267422C: 38EB32E0  addi r7, r11, 0x32e0
	ctx.r[7].s64 = ctx.r[11].s64 + 13024;
	// 82674230: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82674234: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 82674238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267423C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674240: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674248: 386A3420  addi r3, r10, 0x3420
	ctx.r[3].s64 = ctx.r[10].s64 + 13344;
	// 8267424C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267425C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267426C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674270: 4BDF2BB1  bl 0x82466e20
	ctx.lr = 0x82674274;
	sub_82466E20(ctx, base);
	// 82674274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267427C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674288 size=108
    let mut pc: u32 = 0x82674288;
    'dispatch: loop {
        match pc {
            0x82674288 => {
    //   block [0x82674288..0x826742F4)
	// 82674288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267428C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674294: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674298: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267429C: 38EB3400  addi r7, r11, 0x3400
	ctx.r[7].s64 = ctx.r[11].s64 + 13312;
	// 826742A0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826742A4: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 826742A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826742AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826742B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826742B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826742B8: 386A3450  addi r3, r10, 0x3450
	ctx.r[3].s64 = ctx.r[10].s64 + 13392;
	// 826742BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826742C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826742C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826742C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826742CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826742D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826742D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826742D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826742DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826742E0: 4BDF2B41  bl 0x82466e20
	ctx.lr = 0x826742E4;
	sub_82466E20(ctx, base);
	// 826742E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826742E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826742EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826742F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826742F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826742F8 size=108
    let mut pc: u32 = 0x826742F8;
    'dispatch: loop {
        match pc {
            0x826742F8 => {
    //   block [0x826742F8..0x82674364)
	// 826742F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826742FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674304: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267430C: 38EB3490  addi r7, r11, 0x3490
	ctx.r[7].s64 = ctx.r[11].s64 + 13456;
	// 82674310: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82674314: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 82674318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267431C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674320: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674328: 386A3480  addi r3, r10, 0x3480
	ctx.r[3].s64 = ctx.r[10].s64 + 13440;
	// 8267432C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267433C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267434C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674350: 4BDF2AD1  bl 0x82466e20
	ctx.lr = 0x82674354;
	sub_82466E20(ctx, base);
	// 82674354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267435C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674368 size=108
    let mut pc: u32 = 0x82674368;
    'dispatch: loop {
        match pc {
            0x82674368 => {
    //   block [0x82674368..0x826743D4)
	// 82674368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267436C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674374: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674378: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267437C: 38EB3550  addi r7, r11, 0x3550
	ctx.r[7].s64 = ctx.r[11].s64 + 13648;
	// 82674380: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82674384: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 82674388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267438C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674390: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674398: 386A34B0  addi r3, r10, 0x34b0
	ctx.r[3].s64 = ctx.r[10].s64 + 13488;
	// 8267439C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826743A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826743A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826743A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826743AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826743B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826743B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826743B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826743BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826743C0: 4BDF2A61  bl 0x82466e20
	ctx.lr = 0x826743C4;
	sub_82466E20(ctx, base);
	// 826743C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826743C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826743CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826743D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826743D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826743D8 size=108
    let mut pc: u32 = 0x826743D8;
    'dispatch: loop {
        match pc {
            0x826743D8 => {
    //   block [0x826743D8..0x82674444)
	// 826743D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826743DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826743E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826743E4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826743E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826743EC: 38EB3628  addi r7, r11, 0x3628
	ctx.r[7].s64 = ctx.r[11].s64 + 13864;
	// 826743F0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826743F4: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826743F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826743FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674400: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674408: 386A34E0  addi r3, r10, 0x34e0
	ctx.r[3].s64 = ctx.r[10].s64 + 13536;
	// 8267440C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267441C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267442C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674430: 4BDF29F1  bl 0x82466e20
	ctx.lr = 0x82674434;
	sub_82466E20(ctx, base);
	// 82674434: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267443C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674448 size=108
    let mut pc: u32 = 0x82674448;
    'dispatch: loop {
        match pc {
            0x82674448 => {
    //   block [0x82674448..0x826744B4)
	// 82674448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267444C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674454: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267445C: 38EB36E8  addi r7, r11, 0x36e8
	ctx.r[7].s64 = ctx.r[11].s64 + 14056;
	// 82674460: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82674464: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 82674468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267446C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674478: 386A3510  addi r3, r10, 0x3510
	ctx.r[3].s64 = ctx.r[10].s64 + 13584;
	// 8267447C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267448C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267449C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826744A0: 4BDF2981  bl 0x82466e20
	ctx.lr = 0x826744A4;
	sub_82466E20(ctx, base);
	// 826744A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826744A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826744AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826744B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826744B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826744B8 size=108
    let mut pc: u32 = 0x826744B8;
    'dispatch: loop {
        match pc {
            0x826744B8 => {
    //   block [0x826744B8..0x82674524)
	// 826744B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826744BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826744C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826744C4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826744C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826744CC: 38EB3790  addi r7, r11, 0x3790
	ctx.r[7].s64 = ctx.r[11].s64 + 14224;
	// 826744D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826744D4: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 826744D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826744DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826744E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826744E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826744E8: 386A3540  addi r3, r10, 0x3540
	ctx.r[3].s64 = ctx.r[10].s64 + 13632;
	// 826744EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826744F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826744F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826744F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826744FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267450C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674510: 4BDF2911  bl 0x82466e20
	ctx.lr = 0x82674514;
	sub_82466E20(ctx, base);
	// 82674514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267451C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674528 size=108
    let mut pc: u32 = 0x82674528;
    'dispatch: loop {
        match pc {
            0x82674528 => {
    //   block [0x82674528..0x82674594)
	// 82674528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267452C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674534: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267453C: 38EB37D8  addi r7, r11, 0x37d8
	ctx.r[7].s64 = ctx.r[11].s64 + 14296;
	// 82674540: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82674544: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 82674548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267454C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674550: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674558: 386A3570  addi r3, r10, 0x3570
	ctx.r[3].s64 = ctx.r[10].s64 + 13680;
	// 8267455C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267456C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267457C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674580: 4BDF28A1  bl 0x82466e20
	ctx.lr = 0x82674584;
	sub_82466E20(ctx, base);
	// 82674584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267458C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674598 size=108
    let mut pc: u32 = 0x82674598;
    'dispatch: loop {
        match pc {
            0x82674598 => {
    //   block [0x82674598..0x82674604)
	// 82674598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267459C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826745A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826745A4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826745A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826745AC: 38EB3838  addi r7, r11, 0x3838
	ctx.r[7].s64 = ctx.r[11].s64 + 14392;
	// 826745B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826745B4: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 826745B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826745BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826745C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826745C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826745C8: 386A35A0  addi r3, r10, 0x35a0
	ctx.r[3].s64 = ctx.r[10].s64 + 13728;
	// 826745CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826745D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826745D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826745D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826745DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826745E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826745E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826745E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826745EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826745F0: 4BDF2831  bl 0x82466e20
	ctx.lr = 0x826745F4;
	sub_82466E20(ctx, base);
	// 826745F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826745F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826745FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674608 size=108
    let mut pc: u32 = 0x82674608;
    'dispatch: loop {
        match pc {
            0x82674608 => {
    //   block [0x82674608..0x82674674)
	// 82674608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267460C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674614: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267461C: 38EB3868  addi r7, r11, 0x3868
	ctx.r[7].s64 = ctx.r[11].s64 + 14440;
	// 82674620: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82674624: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 82674628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267462C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674638: 386A35D0  addi r3, r10, 0x35d0
	ctx.r[3].s64 = ctx.r[10].s64 + 13776;
	// 8267463C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267464C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267465C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674660: 4BDF27C1  bl 0x82466e20
	ctx.lr = 0x82674664;
	sub_82466E20(ctx, base);
	// 82674664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267466C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674678 size=108
    let mut pc: u32 = 0x82674678;
    'dispatch: loop {
        match pc {
            0x82674678 => {
    //   block [0x82674678..0x826746E4)
	// 82674678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267467C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674684: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267468C: 38EB3970  addi r7, r11, 0x3970
	ctx.r[7].s64 = ctx.r[11].s64 + 14704;
	// 82674690: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82674694: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 82674698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267469C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826746A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826746A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826746A8: 386A3600  addi r3, r10, 0x3600
	ctx.r[3].s64 = ctx.r[10].s64 + 13824;
	// 826746AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826746B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826746B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826746B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826746BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826746C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826746C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826746C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826746CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826746D0: 4BDF2751  bl 0x82466e20
	ctx.lr = 0x826746D4;
	sub_82466E20(ctx, base);
	// 826746D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826746D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826746DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826746E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826746E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826746E8 size=108
    let mut pc: u32 = 0x826746E8;
    'dispatch: loop {
        match pc {
            0x826746E8 => {
    //   block [0x826746E8..0x82674754)
	// 826746E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826746EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826746F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826746F4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826746F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826746FC: 38EB39D0  addi r7, r11, 0x39d0
	ctx.r[7].s64 = ctx.r[11].s64 + 14800;
	// 82674700: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82674704: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 82674708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267470C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674710: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674718: 386A3630  addi r3, r10, 0x3630
	ctx.r[3].s64 = ctx.r[10].s64 + 13872;
	// 8267471C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267472C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267473C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674740: 4BDF26E1  bl 0x82466e20
	ctx.lr = 0x82674744;
	sub_82466E20(ctx, base);
	// 82674744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267474C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674758 size=108
    let mut pc: u32 = 0x82674758;
    'dispatch: loop {
        match pc {
            0x82674758 => {
    //   block [0x82674758..0x826747C4)
	// 82674758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267475C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674764: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267476C: 38EB3AC0  addi r7, r11, 0x3ac0
	ctx.r[7].s64 = ctx.r[11].s64 + 15040;
	// 82674770: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82674774: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 82674778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267477C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674780: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674788: 386A3660  addi r3, r10, 0x3660
	ctx.r[3].s64 = ctx.r[10].s64 + 13920;
	// 8267478C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267479C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826747A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826747A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826747A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826747AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826747B0: 4BDF2671  bl 0x82466e20
	ctx.lr = 0x826747B4;
	sub_82466E20(ctx, base);
	// 826747B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826747B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826747BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826747C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826747C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826747C8 size=108
    let mut pc: u32 = 0x826747C8;
    'dispatch: loop {
        match pc {
            0x826747C8 => {
    //   block [0x826747C8..0x82674834)
	// 826747C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826747CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826747D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826747D4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826747D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826747DC: 38EB3B98  addi r7, r11, 0x3b98
	ctx.r[7].s64 = ctx.r[11].s64 + 15256;
	// 826747E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826747E4: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 826747E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826747EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826747F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826747F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826747F8: 386A3690  addi r3, r10, 0x3690
	ctx.r[3].s64 = ctx.r[10].s64 + 13968;
	// 826747FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267480C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267481C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674820: 4BDF2601  bl 0x82466e20
	ctx.lr = 0x82674824;
	sub_82466E20(ctx, base);
	// 82674824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267482C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674838 size=108
    let mut pc: u32 = 0x82674838;
    'dispatch: loop {
        match pc {
            0x82674838 => {
    //   block [0x82674838..0x826748A4)
	// 82674838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267483C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674844: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267484C: 38EB3BC8  addi r7, r11, 0x3bc8
	ctx.r[7].s64 = ctx.r[11].s64 + 15304;
	// 82674850: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82674854: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 82674858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267485C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674860: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674868: 386A36C0  addi r3, r10, 0x36c0
	ctx.r[3].s64 = ctx.r[10].s64 + 14016;
	// 8267486C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267487C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267488C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674890: 4BDF2591  bl 0x82466e20
	ctx.lr = 0x82674894;
	sub_82466E20(ctx, base);
	// 82674894: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267489C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826748A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826748A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826748A8 size=108
    let mut pc: u32 = 0x826748A8;
    'dispatch: loop {
        match pc {
            0x826748A8 => {
    //   block [0x826748A8..0x82674914)
	// 826748A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826748AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826748B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826748B4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826748B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826748BC: 38EB3BE0  addi r7, r11, 0x3be0
	ctx.r[7].s64 = ctx.r[11].s64 + 15328;
	// 826748C0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826748C4: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 826748C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826748CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826748D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826748D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826748D8: 386A36F0  addi r3, r10, 0x36f0
	ctx.r[3].s64 = ctx.r[10].s64 + 14064;
	// 826748DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826748E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826748E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826748E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826748EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826748F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826748F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826748F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826748FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674900: 4BDF2521  bl 0x82466e20
	ctx.lr = 0x82674904;
	sub_82466E20(ctx, base);
	// 82674904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267490C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674918 size=112
    let mut pc: u32 = 0x82674918;
    'dispatch: loop {
        match pc {
            0x82674918 => {
    //   block [0x82674918..0x82674988)
	// 82674918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267491C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674924: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674928: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267492C: 38AA36F0  addi r5, r10, 0x36f0
	ctx.r[5].s64 = ctx.r[10].s64 + 14064;
	// 82674930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674934: 390B3C40  addi r8, r11, 0x3c40
	ctx.r[8].s64 = ctx.r[11].s64 + 15424;
	// 82674938: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267493C: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 82674940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674944: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267494C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674950: 386A3720  addi r3, r10, 0x3720
	ctx.r[3].s64 = ctx.r[10].s64 + 14112;
	// 82674954: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82674958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267495C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267496C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674974: 4BDF24AD  bl 0x82466e20
	ctx.lr = 0x82674978;
	sub_82466E20(ctx, base);
	// 82674978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267497C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674988 size=96
    let mut pc: u32 = 0x82674988;
    'dispatch: loop {
        match pc {
            0x82674988 => {
    //   block [0x82674988..0x826749E8)
	// 82674988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267498C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674994: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267499C: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 826749A0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826749A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826749A8: 386A3750  addi r3, r10, 0x3750
	ctx.r[3].s64 = ctx.r[10].s64 + 14160;
	// 826749AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826749B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826749B4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826749B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826749BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826749C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826749C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826749C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826749CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826749D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826749D4: 4BDF244D  bl 0x82466e20
	ctx.lr = 0x826749D8;
	sub_82466E20(ctx, base);
	// 826749D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826749DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826749E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826749E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826749E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826749E8 size=112
    let mut pc: u32 = 0x826749E8;
    'dispatch: loop {
        match pc {
            0x826749E8 => {
    //   block [0x826749E8..0x82674A58)
	// 826749E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826749EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826749F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826749F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826749F8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826749FC: 38AA4E60  addi r5, r10, 0x4e60
	ctx.r[5].s64 = ctx.r[10].s64 + 20064;
	// 82674A00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674A04: 390B3C88  addi r8, r11, 0x3c88
	ctx.r[8].s64 = ctx.r[11].s64 + 15496;
	// 82674A08: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82674A0C: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 82674A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674A14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674A18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82674A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674A20: 386A3780  addi r3, r10, 0x3780
	ctx.r[3].s64 = ctx.r[10].s64 + 14208;
	// 82674A24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82674A28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674A30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674A38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674A40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674A44: 4BDF23DD  bl 0x82466e20
	ctx.lr = 0x82674A48;
	sub_82466E20(ctx, base);
	// 82674A48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674A58 size=96
    let mut pc: u32 = 0x82674A58;
    'dispatch: loop {
        match pc {
            0x82674A58 => {
    //   block [0x82674A58..0x82674AB8)
	// 82674A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674A64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674A6C: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 82674A70: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674A78: 386A37B0  addi r3, r10, 0x37b0
	ctx.r[3].s64 = ctx.r[10].s64 + 14256;
	// 82674A7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674A80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674A84: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82674A88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674A90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674A98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82674A9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674AA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82674AA4: 4BDF237D  bl 0x82466e20
	ctx.lr = 0x82674AA8;
	sub_82466E20(ctx, base);
	// 82674AA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674AB8 size=100
    let mut pc: u32 = 0x82674AB8;
    'dispatch: loop {
        match pc {
            0x82674AB8 => {
    //   block [0x82674AB8..0x82674B1C)
	// 82674AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674AC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674ACC: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82674AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674AD8: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 82674ADC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674AE4: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82674AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674AEC: 386A37E0  addi r3, r10, 0x37e0
	ctx.r[3].s64 = ctx.r[10].s64 + 14304;
	// 82674AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674AF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674AF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82674AFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674B00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82674B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674B08: 4BDF2319  bl 0x82466e20
	ctx.lr = 0x82674B0C;
	sub_82466E20(ctx, base);
	// 82674B0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674B10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674B14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674B20 size=96
    let mut pc: u32 = 0x82674B20;
    'dispatch: loop {
        match pc {
            0x82674B20 => {
    //   block [0x82674B20..0x82674B80)
	// 82674B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674B2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674B34: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 82674B38: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674B40: 386A3810  addi r3, r10, 0x3810
	ctx.r[3].s64 = ctx.r[10].s64 + 14352;
	// 82674B44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674B4C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82674B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674B60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82674B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674B68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82674B6C: 4BDF22B5  bl 0x82466e20
	ctx.lr = 0x82674B70;
	sub_82466E20(ctx, base);
	// 82674B70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674B80 size=112
    let mut pc: u32 = 0x82674B80;
    'dispatch: loop {
        match pc {
            0x82674B80 => {
    //   block [0x82674B80..0x82674BF0)
	// 82674B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674B8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674B90: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674B94: 38AA37E0  addi r5, r10, 0x37e0
	ctx.r[5].s64 = ctx.r[10].s64 + 14304;
	// 82674B98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674B9C: 390B3CE8  addi r8, r11, 0x3ce8
	ctx.r[8].s64 = ctx.r[11].s64 + 15592;
	// 82674BA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82674BA4: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 82674BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674BAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674BB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82674BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674BB8: 386A3840  addi r3, r10, 0x3840
	ctx.r[3].s64 = ctx.r[10].s64 + 14400;
	// 82674BBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82674BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674BDC: 4BDF2245  bl 0x82466e20
	ctx.lr = 0x82674BE0;
	sub_82466E20(ctx, base);
	// 82674BE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674BF0 size=112
    let mut pc: u32 = 0x82674BF0;
    'dispatch: loop {
        match pc {
            0x82674BF0 => {
    //   block [0x82674BF0..0x82674C60)
	// 82674BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674BFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674C00: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674C04: 38AA37E0  addi r5, r10, 0x37e0
	ctx.r[5].s64 = ctx.r[10].s64 + 14304;
	// 82674C08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674C0C: 390B3D18  addi r8, r11, 0x3d18
	ctx.r[8].s64 = ctx.r[11].s64 + 15640;
	// 82674C10: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82674C14: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 82674C18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674C1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674C20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82674C24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674C28: 386A3870  addi r3, r10, 0x3870
	ctx.r[3].s64 = ctx.r[10].s64 + 14448;
	// 82674C2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82674C30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674C34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674C3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674C44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674C4C: 4BDF21D5  bl 0x82466e20
	ctx.lr = 0x82674C50;
	sub_82466E20(ctx, base);
	// 82674C50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674C60 size=100
    let mut pc: u32 = 0x82674C60;
    'dispatch: loop {
        match pc {
            0x82674C60 => {
    //   block [0x82674C60..0x82674CC4)
	// 82674C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674C68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674C6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674C74: 38AA37E0  addi r5, r10, 0x37e0
	ctx.r[5].s64 = ctx.r[10].s64 + 14304;
	// 82674C78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674C80: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 82674C84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674C8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674C94: 386A38A0  addi r3, r10, 0x38a0
	ctx.r[3].s64 = ctx.r[10].s64 + 14496;
	// 82674C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674C9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674CA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82674CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674CA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82674CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674CB0: 4BDF2171  bl 0x82466e20
	ctx.lr = 0x82674CB4;
	sub_82466E20(ctx, base);
	// 82674CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674CC8 size=96
    let mut pc: u32 = 0x82674CC8;
    'dispatch: loop {
        match pc {
            0x82674CC8 => {
    //   block [0x82674CC8..0x82674D28)
	// 82674CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674CD4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674CDC: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 82674CE0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674CE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674CE8: 386A38D0  addi r3, r10, 0x38d0
	ctx.r[3].s64 = ctx.r[10].s64 + 14544;
	// 82674CEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674CF4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82674CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674D04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674D08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82674D0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674D10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82674D14: 4BDF210D  bl 0x82466e20
	ctx.lr = 0x82674D18;
	sub_82466E20(ctx, base);
	// 82674D18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674D28 size=112
    let mut pc: u32 = 0x82674D28;
    'dispatch: loop {
        match pc {
            0x82674D28 => {
    //   block [0x82674D28..0x82674D98)
	// 82674D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674D30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674D34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674D38: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674D3C: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82674D40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674D44: 390B3D30  addi r8, r11, 0x3d30
	ctx.r[8].s64 = ctx.r[11].s64 + 15664;
	// 82674D48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82674D4C: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 82674D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674D54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674D58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82674D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674D60: 386A3900  addi r3, r10, 0x3900
	ctx.r[3].s64 = ctx.r[10].s64 + 14592;
	// 82674D64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82674D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674D84: 4BDF209D  bl 0x82466e20
	ctx.lr = 0x82674D88;
	sub_82466E20(ctx, base);
	// 82674D88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674D98 size=108
    let mut pc: u32 = 0x82674D98;
    'dispatch: loop {
        match pc {
            0x82674D98 => {
    //   block [0x82674D98..0x82674E04)
	// 82674D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674DA4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674DA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674DAC: 38EB3D48  addi r7, r11, 0x3d48
	ctx.r[7].s64 = ctx.r[11].s64 + 15688;
	// 82674DB0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82674DB4: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 82674DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674DBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674DC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674DC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674DC8: 386A3930  addi r3, r10, 0x3930
	ctx.r[3].s64 = ctx.r[10].s64 + 14640;
	// 82674DCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674DD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674DD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674DE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674DE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674DEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674DF0: 4BDF2031  bl 0x82466e20
	ctx.lr = 0x82674DF4;
	sub_82466E20(ctx, base);
	// 82674DF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674DF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674DFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674E08 size=112
    let mut pc: u32 = 0x82674E08;
    'dispatch: loop {
        match pc {
            0x82674E08 => {
    //   block [0x82674E08..0x82674E78)
	// 82674E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674E14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674E18: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674E1C: 38AA3A50  addi r5, r10, 0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + 14928;
	// 82674E20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674E24: 390B3DA8  addi r8, r11, 0x3da8
	ctx.r[8].s64 = ctx.r[11].s64 + 15784;
	// 82674E28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82674E2C: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 82674E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674E34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674E38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82674E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674E40: 386A3960  addi r3, r10, 0x3960
	ctx.r[3].s64 = ctx.r[10].s64 + 14688;
	// 82674E44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82674E48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674E4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674E50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674E54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674E58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674E64: 4BDF1FBD  bl 0x82466e20
	ctx.lr = 0x82674E68;
	sub_82466E20(ctx, base);
	// 82674E68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674E78 size=112
    let mut pc: u32 = 0x82674E78;
    'dispatch: loop {
        match pc {
            0x82674E78 => {
    //   block [0x82674E78..0x82674EE8)
	// 82674E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674E84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674E88: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674E8C: 38AA3900  addi r5, r10, 0x3900
	ctx.r[5].s64 = ctx.r[10].s64 + 14592;
	// 82674E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674E94: 390B3DC0  addi r8, r11, 0x3dc0
	ctx.r[8].s64 = ctx.r[11].s64 + 15808;
	// 82674E98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82674E9C: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 82674EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674EA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674EA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82674EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674EB0: 386A3990  addi r3, r10, 0x3990
	ctx.r[3].s64 = ctx.r[10].s64 + 14736;
	// 82674EB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82674EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674ED4: 4BDF1F4D  bl 0x82466e20
	ctx.lr = 0x82674ED8;
	sub_82466E20(ctx, base);
	// 82674ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674EE8 size=112
    let mut pc: u32 = 0x82674EE8;
    'dispatch: loop {
        match pc {
            0x82674EE8 => {
    //   block [0x82674EE8..0x82674F58)
	// 82674EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674EF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674EF8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674EFC: 38AA3900  addi r5, r10, 0x3900
	ctx.r[5].s64 = ctx.r[10].s64 + 14592;
	// 82674F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674F04: 390B3DF0  addi r8, r11, 0x3df0
	ctx.r[8].s64 = ctx.r[11].s64 + 15856;
	// 82674F08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82674F0C: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 82674F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674F14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674F18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82674F1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674F20: 386A39C0  addi r3, r10, 0x39c0
	ctx.r[3].s64 = ctx.r[10].s64 + 14784;
	// 82674F24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82674F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674F44: 4BDF1EDD  bl 0x82466e20
	ctx.lr = 0x82674F48;
	sub_82466E20(ctx, base);
	// 82674F48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674F58 size=112
    let mut pc: u32 = 0x82674F58;
    'dispatch: loop {
        match pc {
            0x82674F58 => {
    //   block [0x82674F58..0x82674FC8)
	// 82674F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674F64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674F68: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674F6C: 38AA3A50  addi r5, r10, 0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + 14928;
	// 82674F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674F74: 390B3E08  addi r8, r11, 0x3e08
	ctx.r[8].s64 = ctx.r[11].s64 + 15880;
	// 82674F78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82674F7C: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 82674F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674F84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674F88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82674F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674F90: 386A39F0  addi r3, r10, 0x39f0
	ctx.r[3].s64 = ctx.r[10].s64 + 14832;
	// 82674F94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82674F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674FA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674FB4: 4BDF1E6D  bl 0x82466e20
	ctx.lr = 0x82674FB8;
	sub_82466E20(ctx, base);
	// 82674FB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674FC8 size=112
    let mut pc: u32 = 0x82674FC8;
    'dispatch: loop {
        match pc {
            0x82674FC8 => {
    //   block [0x82674FC8..0x82675038)
	// 82674FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674FD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674FD8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674FDC: 38AA3900  addi r5, r10, 0x3900
	ctx.r[5].s64 = ctx.r[10].s64 + 14592;
	// 82674FE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674FE4: 390B3E38  addi r8, r11, 0x3e38
	ctx.r[8].s64 = ctx.r[11].s64 + 15928;
	// 82674FE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82674FEC: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 82674FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674FF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674FF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82674FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675000: 386A3A20  addi r3, r10, 0x3a20
	ctx.r[3].s64 = ctx.r[10].s64 + 14880;
	// 82675004: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267500C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267501C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675024: 4BDF1DFD  bl 0x82466e20
	ctx.lr = 0x82675028;
	sub_82466E20(ctx, base);
	// 82675028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267502C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675038 size=112
    let mut pc: u32 = 0x82675038;
    'dispatch: loop {
        match pc {
            0x82675038 => {
    //   block [0x82675038..0x826750A8)
	// 82675038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267503C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675044: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675048: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267504C: 38AA3F00  addi r5, r10, 0x3f00
	ctx.r[5].s64 = ctx.r[10].s64 + 16128;
	// 82675050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675054: 390B3E50  addi r8, r11, 0x3e50
	ctx.r[8].s64 = ctx.r[11].s64 + 15952;
	// 82675058: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267505C: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 82675060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675064: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267506C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675070: 386A3A50  addi r3, r10, 0x3a50
	ctx.r[3].s64 = ctx.r[10].s64 + 14928;
	// 82675074: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267507C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267508C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675094: 4BDF1D8D  bl 0x82466e20
	ctx.lr = 0x82675098;
	sub_82466E20(ctx, base);
	// 82675098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267509C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826750A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826750A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826750A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826750A8 size=112
    let mut pc: u32 = 0x826750A8;
    'dispatch: loop {
        match pc {
            0x826750A8 => {
    //   block [0x826750A8..0x82675118)
	// 826750A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826750AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826750B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826750B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826750B8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826750BC: 38AA3C60  addi r5, r10, 0x3c60
	ctx.r[5].s64 = ctx.r[10].s64 + 15456;
	// 826750C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826750C4: 390B3E68  addi r8, r11, 0x3e68
	ctx.r[8].s64 = ctx.r[11].s64 + 15976;
	// 826750C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826750CC: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 826750D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826750D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826750D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826750DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826750E0: 386A3A80  addi r3, r10, 0x3a80
	ctx.r[3].s64 = ctx.r[10].s64 + 14976;
	// 826750E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826750E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826750EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826750F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826750F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826750F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826750FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675104: 4BDF1D1D  bl 0x82466e20
	ctx.lr = 0x82675108;
	sub_82466E20(ctx, base);
	// 82675108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267510C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675118 size=112
    let mut pc: u32 = 0x82675118;
    'dispatch: loop {
        match pc {
            0x82675118 => {
    //   block [0x82675118..0x82675188)
	// 82675118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267511C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675124: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675128: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267512C: 38AA3A20  addi r5, r10, 0x3a20
	ctx.r[5].s64 = ctx.r[10].s64 + 14880;
	// 82675130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675134: 390B3E80  addi r8, r11, 0x3e80
	ctx.r[8].s64 = ctx.r[11].s64 + 16000;
	// 82675138: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267513C: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 82675140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675144: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675148: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267514C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675150: 386A3AB0  addi r3, r10, 0x3ab0
	ctx.r[3].s64 = ctx.r[10].s64 + 15024;
	// 82675154: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267515C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267516C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675174: 4BDF1CAD  bl 0x82466e20
	ctx.lr = 0x82675178;
	sub_82466E20(ctx, base);
	// 82675178: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267517C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675188 size=112
    let mut pc: u32 = 0x82675188;
    'dispatch: loop {
        match pc {
            0x82675188 => {
    //   block [0x82675188..0x826751F8)
	// 82675188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267518C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675194: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675198: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267519C: 38AA3A50  addi r5, r10, 0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + 14928;
	// 826751A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826751A4: 390B3EC8  addi r8, r11, 0x3ec8
	ctx.r[8].s64 = ctx.r[11].s64 + 16072;
	// 826751A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826751AC: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 826751B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826751B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826751B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826751BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826751C0: 386A3AE0  addi r3, r10, 0x3ae0
	ctx.r[3].s64 = ctx.r[10].s64 + 15072;
	// 826751C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826751C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826751CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826751D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826751D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826751D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826751DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826751E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826751E4: 4BDF1C3D  bl 0x82466e20
	ctx.lr = 0x826751E8;
	sub_82466E20(ctx, base);
	// 826751E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826751EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826751F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826751F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826751F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826751F8 size=112
    let mut pc: u32 = 0x826751F8;
    'dispatch: loop {
        match pc {
            0x826751F8 => {
    //   block [0x826751F8..0x82675268)
	// 826751F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826751FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675204: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675208: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267520C: 38AA3A50  addi r5, r10, 0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + 14928;
	// 82675210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675214: 390B3EF8  addi r8, r11, 0x3ef8
	ctx.r[8].s64 = ctx.r[11].s64 + 16120;
	// 82675218: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267521C: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 82675220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675224: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267522C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675230: 386A3B10  addi r3, r10, 0x3b10
	ctx.r[3].s64 = ctx.r[10].s64 + 15120;
	// 82675234: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267523C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267524C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675254: 4BDF1BCD  bl 0x82466e20
	ctx.lr = 0x82675258;
	sub_82466E20(ctx, base);
	// 82675258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267525C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675268 size=108
    let mut pc: u32 = 0x82675268;
    'dispatch: loop {
        match pc {
            0x82675268 => {
    //   block [0x82675268..0x826752D4)
	// 82675268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267526C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675274: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267527C: 38EB3F28  addi r7, r11, 0x3f28
	ctx.r[7].s64 = ctx.r[11].s64 + 16168;
	// 82675280: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82675284: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 82675288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267528C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82675294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675298: 386A3B40  addi r3, r10, 0x3b40
	ctx.r[3].s64 = ctx.r[10].s64 + 15168;
	// 8267529C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826752A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826752A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826752A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826752AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826752B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826752B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826752B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826752BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826752C0: 4BDF1B61  bl 0x82466e20
	ctx.lr = 0x826752C4;
	sub_82466E20(ctx, base);
	// 826752C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826752C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826752CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826752D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826752D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826752D8 size=112
    let mut pc: u32 = 0x826752D8;
    'dispatch: loop {
        match pc {
            0x826752D8 => {
    //   block [0x826752D8..0x82675348)
	// 826752D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826752DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826752E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826752E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826752E8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826752EC: 38AA3A50  addi r5, r10, 0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + 14928;
	// 826752F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826752F4: 390B3F70  addi r8, r11, 0x3f70
	ctx.r[8].s64 = ctx.r[11].s64 + 16240;
	// 826752F8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826752FC: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 82675300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675304: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675308: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267530C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675310: 386A3B70  addi r3, r10, 0x3b70
	ctx.r[3].s64 = ctx.r[10].s64 + 15216;
	// 82675314: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267531C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267532C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675334: 4BDF1AED  bl 0x82466e20
	ctx.lr = 0x82675338;
	sub_82466E20(ctx, base);
	// 82675338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267533C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675348 size=112
    let mut pc: u32 = 0x82675348;
    'dispatch: loop {
        match pc {
            0x82675348 => {
    //   block [0x82675348..0x826753B8)
	// 82675348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267534C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675354: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675358: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267535C: 38AA3A50  addi r5, r10, 0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + 14928;
	// 82675360: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675364: 390B3FE8  addi r8, r11, 0x3fe8
	ctx.r[8].s64 = ctx.r[11].s64 + 16360;
	// 82675368: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8267536C: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 82675370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675374: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267537C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675380: 386A3BA0  addi r3, r10, 0x3ba0
	ctx.r[3].s64 = ctx.r[10].s64 + 15264;
	// 82675384: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675388: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267538C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267539C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826753A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826753A4: 4BDF1A7D  bl 0x82466e20
	ctx.lr = 0x826753A8;
	sub_82466E20(ctx, base);
	// 826753A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826753AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826753B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826753B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826753B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826753B8 size=100
    let mut pc: u32 = 0x826753B8;
    'dispatch: loop {
        match pc {
            0x826753B8 => {
    //   block [0x826753B8..0x8267541C)
	// 826753B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826753BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826753C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826753C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826753C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826753CC: 38AA3CF0  addi r5, r10, 0x3cf0
	ctx.r[5].s64 = ctx.r[10].s64 + 15600;
	// 826753D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826753D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826753D8: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 826753DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826753E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826753E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826753E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826753EC: 386A3BD0  addi r3, r10, 0x3bd0
	ctx.r[3].s64 = ctx.r[10].s64 + 15312;
	// 826753F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826753F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826753F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826753FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675400: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82675404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675408: 4BDF1A19  bl 0x82466e20
	ctx.lr = 0x8267540C;
	sub_82466E20(ctx, base);
	// 8267540C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675420 size=100
    let mut pc: u32 = 0x82675420;
    'dispatch: loop {
        match pc {
            0x82675420 => {
    //   block [0x82675420..0x82675484)
	// 82675420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267542C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675434: 38AA3900  addi r5, r10, 0x3900
	ctx.r[5].s64 = ctx.r[10].s64 + 14592;
	// 82675438: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267543C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675440: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 82675444: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267544C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675454: 386A3C00  addi r3, r10, 0x3c00
	ctx.r[3].s64 = ctx.r[10].s64 + 15360;
	// 82675458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267545C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675460: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82675464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675468: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267546C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675470: 4BDF19B1  bl 0x82466e20
	ctx.lr = 0x82675474;
	sub_82466E20(ctx, base);
	// 82675474: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267547C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675488 size=108
    let mut pc: u32 = 0x82675488;
    'dispatch: loop {
        match pc {
            0x82675488 => {
    //   block [0x82675488..0x826754F4)
	// 82675488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267548C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675494: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267549C: 38EB4060  addi r7, r11, 0x4060
	ctx.r[7].s64 = ctx.r[11].s64 + 16480;
	// 826754A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826754A4: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 826754A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826754AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826754B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826754B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826754B8: 386A3C30  addi r3, r10, 0x3c30
	ctx.r[3].s64 = ctx.r[10].s64 + 15408;
	// 826754BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826754C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826754C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826754C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826754CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826754D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826754D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826754D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826754DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826754E0: 4BDF1941  bl 0x82466e20
	ctx.lr = 0x826754E4;
	sub_82466E20(ctx, base);
	// 826754E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826754E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826754EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826754F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826754F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826754F8 size=112
    let mut pc: u32 = 0x826754F8;
    'dispatch: loop {
        match pc {
            0x826754F8 => {
    //   block [0x826754F8..0x82675568)
	// 826754F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826754FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675504: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675508: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267550C: 38AA3A20  addi r5, r10, 0x3a20
	ctx.r[5].s64 = ctx.r[10].s64 + 14880;
	// 82675510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675514: 390B4090  addi r8, r11, 0x4090
	ctx.r[8].s64 = ctx.r[11].s64 + 16528;
	// 82675518: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267551C: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 82675520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675524: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267552C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675530: 386A3C60  addi r3, r10, 0x3c60
	ctx.r[3].s64 = ctx.r[10].s64 + 15456;
	// 82675534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267553C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267554C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675554: 4BDF18CD  bl 0x82466e20
	ctx.lr = 0x82675558;
	sub_82466E20(ctx, base);
	// 82675558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267555C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675568 size=108
    let mut pc: u32 = 0x82675568;
    'dispatch: loop {
        match pc {
            0x82675568 => {
    //   block [0x82675568..0x826755D4)
	// 82675568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267556C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675574: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267557C: 38EB40A8  addi r7, r11, 0x40a8
	ctx.r[7].s64 = ctx.r[11].s64 + 16552;
	// 82675580: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82675584: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 82675588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267558C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675590: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82675594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675598: 386A3C90  addi r3, r10, 0x3c90
	ctx.r[3].s64 = ctx.r[10].s64 + 15504;
	// 8267559C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826755A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826755A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826755A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826755AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826755B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826755B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826755B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826755BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826755C0: 4BDF1861  bl 0x82466e20
	ctx.lr = 0x826755C4;
	sub_82466E20(ctx, base);
	// 826755C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826755C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826755CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826755D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826755D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826755D8 size=24
    let mut pc: u32 = 0x826755D8;
    'dispatch: loop {
        match pc {
            0x826755D8 => {
    //   block [0x826755D8..0x826755F0)
	// 826755D8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826755DC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826755E0: 394A6D50  addi r10, r10, 0x6d50
	ctx.r[10].s64 = ctx.r[10].s64 + 27984;
	// 826755E4: 816B40C0  lwz r11, 0x40c0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16576 as u32) ) } as u64;
	// 826755E8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826755EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826755F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826755F0 size=108
    let mut pc: u32 = 0x826755F0;
    'dispatch: loop {
        match pc {
            0x826755F0 => {
    //   block [0x826755F0..0x8267565C)
	// 826755F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826755F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826755F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826755FC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675604: 38EB6D50  addi r7, r11, 0x6d50
	ctx.r[7].s64 = ctx.r[11].s64 + 27984;
	// 82675608: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8267560C: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 82675610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675614: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675618: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267561C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675620: 386A3CC0  addi r3, r10, 0x3cc0
	ctx.r[3].s64 = ctx.r[10].s64 + 15552;
	// 82675624: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82675628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267562C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267563C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82675648: 4BDF17D9  bl 0x82466e20
	ctx.lr = 0x8267564C;
	sub_82466E20(ctx, base);
	// 8267564C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675660 size=116
    let mut pc: u32 = 0x82675660;
    'dispatch: loop {
        match pc {
            0x82675660 => {
    //   block [0x82675660..0x826756D4)
	// 82675660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267566C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675670: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82675674: 390B40C8  addi r8, r11, 0x40c8
	ctx.r[8].s64 = ctx.r[11].s64 + 16584;
	// 82675678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267567C: 392A31F8  addi r9, r10, 0x31f8
	ctx.r[9].s64 = ctx.r[10].s64 + 12792;
	// 82675680: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675684: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82675688: 38AA3A20  addi r5, r10, 0x3a20
	ctx.r[5].s64 = ctx.r[10].s64 + 14880;
	// 8267568C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675694: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267569C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826756A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826756A4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826756A8: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 826756AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826756B0: 386B3CF0  addi r3, r11, 0x3cf0
	ctx.r[3].s64 = ctx.r[11].s64 + 15600;
	// 826756B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826756B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826756BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826756C0: 4BDF1761  bl 0x82466e20
	ctx.lr = 0x826756C4;
	sub_82466E20(ctx, base);
	// 826756C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826756C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826756CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826756D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826756D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826756D8 size=112
    let mut pc: u32 = 0x826756D8;
    'dispatch: loop {
        match pc {
            0x826756D8 => {
    //   block [0x826756D8..0x82675748)
	// 826756D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826756DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826756E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826756E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826756E8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826756EC: 38AA39C0  addi r5, r10, 0x39c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14784;
	// 826756F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826756F4: 390B4128  addi r8, r11, 0x4128
	ctx.r[8].s64 = ctx.r[11].s64 + 16680;
	// 826756F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826756FC: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 82675700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675704: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267570C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675710: 386A3D20  addi r3, r10, 0x3d20
	ctx.r[3].s64 = ctx.r[10].s64 + 15648;
	// 82675714: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267571C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267572C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675734: 4BDF16ED  bl 0x82466e20
	ctx.lr = 0x82675738;
	sub_82466E20(ctx, base);
	// 82675738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267573C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675748 size=108
    let mut pc: u32 = 0x82675748;
    'dispatch: loop {
        match pc {
            0x82675748 => {
    //   block [0x82675748..0x826757B4)
	// 82675748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267574C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675754: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267575C: 38EB4140  addi r7, r11, 0x4140
	ctx.r[7].s64 = ctx.r[11].s64 + 16704;
	// 82675760: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82675764: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 82675768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267576C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82675774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675778: 386A3D50  addi r3, r10, 0x3d50
	ctx.r[3].s64 = ctx.r[10].s64 + 15696;
	// 8267577C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82675780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267578C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267579C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826757A0: 4BDF1681  bl 0x82466e20
	ctx.lr = 0x826757A4;
	sub_82466E20(ctx, base);
	// 826757A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826757A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826757AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826757B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826757B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826757B8 size=112
    let mut pc: u32 = 0x826757B8;
    'dispatch: loop {
        match pc {
            0x826757B8 => {
    //   block [0x826757B8..0x82675828)
	// 826757B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826757BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826757C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826757C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826757C8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826757CC: 38AA3900  addi r5, r10, 0x3900
	ctx.r[5].s64 = ctx.r[10].s64 + 14592;
	// 826757D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826757D4: 390B4170  addi r8, r11, 0x4170
	ctx.r[8].s64 = ctx.r[11].s64 + 16752;
	// 826757D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826757DC: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 826757E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826757E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826757E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826757EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826757F0: 386A3D80  addi r3, r10, 0x3d80
	ctx.r[3].s64 = ctx.r[10].s64 + 15744;
	// 826757F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826757F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826757FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267580C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675814: 4BDF160D  bl 0x82466e20
	ctx.lr = 0x82675818;
	sub_82466E20(ctx, base);
	// 82675818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267581C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675828 size=116
    let mut pc: u32 = 0x82675828;
    'dispatch: loop {
        match pc {
            0x82675828 => {
    //   block [0x82675828..0x8267589C)
	// 82675828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267582C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675834: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675838: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267583C: 390B41A0  addi r8, r11, 0x41a0
	ctx.r[8].s64 = ctx.r[11].s64 + 16800;
	// 82675840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675844: 392A3214  addi r9, r10, 0x3214
	ctx.r[9].s64 = ctx.r[10].s64 + 12820;
	// 82675848: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267584C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82675850: 38AA3F00  addi r5, r10, 0x3f00
	ctx.r[5].s64 = ctx.r[10].s64 + 16128;
	// 82675854: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267585C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267586C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82675870: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 82675874: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82675878: 386B3DB0  addi r3, r11, 0x3db0
	ctx.r[3].s64 = ctx.r[11].s64 + 15792;
	// 8267587C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82675880: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675888: 4BDF1599  bl 0x82466e20
	ctx.lr = 0x8267588C;
	sub_82466E20(ctx, base);
	// 8267588C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826758A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826758A0 size=100
    let mut pc: u32 = 0x826758A0;
    'dispatch: loop {
        match pc {
            0x826758A0 => {
    //   block [0x826758A0..0x82675904)
	// 826758A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826758A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826758A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826758AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826758B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826758B4: 38AA3900  addi r5, r10, 0x3900
	ctx.r[5].s64 = ctx.r[10].s64 + 14592;
	// 826758B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826758BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826758C0: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 826758C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826758C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826758CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826758D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826758D4: 386A3DE0  addi r3, r10, 0x3de0
	ctx.r[3].s64 = ctx.r[10].s64 + 15840;
	// 826758D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826758DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826758E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826758E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826758E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826758EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826758F0: 4BDF1531  bl 0x82466e20
	ctx.lr = 0x826758F4;
	sub_82466E20(ctx, base);
	// 826758F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826758F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826758FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675908 size=112
    let mut pc: u32 = 0x82675908;
    'dispatch: loop {
        match pc {
            0x82675908 => {
    //   block [0x82675908..0x82675978)
	// 82675908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267590C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675914: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675918: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267591C: 38AA3C00  addi r5, r10, 0x3c00
	ctx.r[5].s64 = ctx.r[10].s64 + 15360;
	// 82675920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675924: 390B41D0  addi r8, r11, 0x41d0
	ctx.r[8].s64 = ctx.r[11].s64 + 16848;
	// 82675928: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267592C: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 82675930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675934: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267593C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675940: 386A3E10  addi r3, r10, 0x3e10
	ctx.r[3].s64 = ctx.r[10].s64 + 15888;
	// 82675944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267594C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267595C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675964: 4BDF14BD  bl 0x82466e20
	ctx.lr = 0x82675968;
	sub_82466E20(ctx, base);
	// 82675968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267596C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675978 size=112
    let mut pc: u32 = 0x82675978;
    'dispatch: loop {
        match pc {
            0x82675978 => {
    //   block [0x82675978..0x826759E8)
	// 82675978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267597C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675984: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675988: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267598C: 38AA3C00  addi r5, r10, 0x3c00
	ctx.r[5].s64 = ctx.r[10].s64 + 15360;
	// 82675990: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82675994: 390B4218  addi r8, r11, 0x4218
	ctx.r[8].s64 = ctx.r[11].s64 + 16920;
	// 82675998: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8267599C: 388AAF68  addi r4, r10, -0x5098
	ctx.r[4].s64 = ctx.r[10].s64 + -20632;
	// 826759A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826759A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826759A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826759AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826759B0: 386A3E40  addi r3, r10, 0x3e40
	ctx.r[3].s64 = ctx.r[10].s64 + 15936;
	// 826759B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826759B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826759BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826759C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826759C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826759C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826759CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826759D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826759D4: 4BDF144D  bl 0x82466e20
	ctx.lr = 0x826759D8;
	sub_82466E20(ctx, base);
	// 826759D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826759DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826759E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826759E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826759E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826759E8 size=108
    let mut pc: u32 = 0x826759E8;
    'dispatch: loop {
        match pc {
            0x826759E8 => {
    //   block [0x826759E8..0x82675A54)
	// 826759E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826759EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826759F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826759F4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826759F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826759FC: 38EB42D8  addi r7, r11, 0x42d8
	ctx.r[7].s64 = ctx.r[11].s64 + 17112;
	// 82675A00: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82675A04: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 82675A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675A0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675A10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82675A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675A18: 386A3E70  addi r3, r10, 0x3e70
	ctx.r[3].s64 = ctx.r[10].s64 + 15984;
	// 82675A1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82675A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675A3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82675A40: 4BDF13E1  bl 0x82466e20
	ctx.lr = 0x82675A44;
	sub_82466E20(ctx, base);
	// 82675A44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675A58 size=112
    let mut pc: u32 = 0x82675A58;
    'dispatch: loop {
        match pc {
            0x82675A58 => {
    //   block [0x82675A58..0x82675AC8)
	// 82675A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675A64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675A68: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675A6C: 38AA3A20  addi r5, r10, 0x3a20
	ctx.r[5].s64 = ctx.r[10].s64 + 14880;
	// 82675A70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675A74: 390B4320  addi r8, r11, 0x4320
	ctx.r[8].s64 = ctx.r[11].s64 + 17184;
	// 82675A78: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82675A7C: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 82675A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675A84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675A88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675A90: 386A3EA0  addi r3, r10, 0x3ea0
	ctx.r[3].s64 = ctx.r[10].s64 + 16032;
	// 82675A94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675AAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675AB4: 4BDF136D  bl 0x82466e20
	ctx.lr = 0x82675AB8;
	sub_82466E20(ctx, base);
	// 82675AB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675AC8 size=100
    let mut pc: u32 = 0x82675AC8;
    'dispatch: loop {
        match pc {
            0x82675AC8 => {
    //   block [0x82675AC8..0x82675B2C)
	// 82675AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675AD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675ADC: 38AA3A50  addi r5, r10, 0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + 14928;
	// 82675AE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675AE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675AE8: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 82675AEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675AFC: 386A3ED0  addi r3, r10, 0x3ed0
	ctx.r[3].s64 = ctx.r[10].s64 + 16080;
	// 82675B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675B04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675B08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82675B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675B10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82675B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675B18: 4BDF1309  bl 0x82466e20
	ctx.lr = 0x82675B1C;
	sub_82466E20(ctx, base);
	// 82675B1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675B30 size=100
    let mut pc: u32 = 0x82675B30;
    'dispatch: loop {
        match pc {
            0x82675B30 => {
    //   block [0x82675B30..0x82675B94)
	// 82675B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675B3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675B44: 38AA3900  addi r5, r10, 0x3900
	ctx.r[5].s64 = ctx.r[10].s64 + 14592;
	// 82675B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675B50: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 82675B54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675B58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675B5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675B60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675B64: 386A3F00  addi r3, r10, 0x3f00
	ctx.r[3].s64 = ctx.r[10].s64 + 16128;
	// 82675B68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675B6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675B70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82675B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675B78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82675B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675B80: 4BDF12A1  bl 0x82466e20
	ctx.lr = 0x82675B84;
	sub_82466E20(ctx, base);
	// 82675B84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675B98 size=108
    let mut pc: u32 = 0x82675B98;
    'dispatch: loop {
        match pc {
            0x82675B98 => {
    //   block [0x82675B98..0x82675C04)
	// 82675B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675BA4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675BA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675BAC: 38EB4380  addi r7, r11, 0x4380
	ctx.r[7].s64 = ctx.r[11].s64 + 17280;
	// 82675BB0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82675BB4: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 82675BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675BBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675BC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82675BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675BC8: 386A3F30  addi r3, r10, 0x3f30
	ctx.r[3].s64 = ctx.r[10].s64 + 16176;
	// 82675BCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82675BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675BDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675BEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82675BF0: 4BDF1231  bl 0x82466e20
	ctx.lr = 0x82675BF4;
	sub_82466E20(ctx, base);
	// 82675BF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675BF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675BFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675C08 size=112
    let mut pc: u32 = 0x82675C08;
    'dispatch: loop {
        match pc {
            0x82675C08 => {
    //   block [0x82675C08..0x82675C78)
	// 82675C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675C14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675C18: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675C1C: 38AA3CF0  addi r5, r10, 0x3cf0
	ctx.r[5].s64 = ctx.r[10].s64 + 15600;
	// 82675C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675C24: 390B43F8  addi r8, r11, 0x43f8
	ctx.r[8].s64 = ctx.r[11].s64 + 17400;
	// 82675C28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82675C2C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 82675C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675C34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675C38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675C40: 386A3F60  addi r3, r10, 0x3f60
	ctx.r[3].s64 = ctx.r[10].s64 + 16224;
	// 82675C44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675C4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675C54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675C64: 4BDF11BD  bl 0x82466e20
	ctx.lr = 0x82675C68;
	sub_82466E20(ctx, base);
	// 82675C68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675C78 size=112
    let mut pc: u32 = 0x82675C78;
    'dispatch: loop {
        match pc {
            0x82675C78 => {
    //   block [0x82675C78..0x82675CE8)
	// 82675C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675C84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675C88: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675C8C: 38AA3900  addi r5, r10, 0x3900
	ctx.r[5].s64 = ctx.r[10].s64 + 14592;
	// 82675C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675C94: 390B4410  addi r8, r11, 0x4410
	ctx.r[8].s64 = ctx.r[11].s64 + 17424;
	// 82675C98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82675C9C: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 82675CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675CA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675CA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675CB0: 386A3F90  addi r3, r10, 0x3f90
	ctx.r[3].s64 = ctx.r[10].s64 + 16272;
	// 82675CB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675CD4: 4BDF114D  bl 0x82466e20
	ctx.lr = 0x82675CD8;
	sub_82466E20(ctx, base);
	// 82675CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675CE8 size=112
    let mut pc: u32 = 0x82675CE8;
    'dispatch: loop {
        match pc {
            0x82675CE8 => {
    //   block [0x82675CE8..0x82675D58)
	// 82675CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675CF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675CF8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675CFC: 38AA3A50  addi r5, r10, 0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + 14928;
	// 82675D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675D04: 390B4458  addi r8, r11, 0x4458
	ctx.r[8].s64 = ctx.r[11].s64 + 17496;
	// 82675D08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82675D0C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 82675D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675D14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675D18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675D20: 386A3FC0  addi r3, r10, 0x3fc0
	ctx.r[3].s64 = ctx.r[10].s64 + 16320;
	// 82675D24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675D44: 4BDF10DD  bl 0x82466e20
	ctx.lr = 0x82675D48;
	sub_82466E20(ctx, base);
	// 82675D48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675D58 size=108
    let mut pc: u32 = 0x82675D58;
    'dispatch: loop {
        match pc {
            0x82675D58 => {
    //   block [0x82675D58..0x82675DC4)
	// 82675D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675D64: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675D68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82675D6C: 38EB44A0  addi r7, r11, 0x44a0
	ctx.r[7].s64 = ctx.r[11].s64 + 17568;
	// 82675D70: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82675D74: 388A21D8  addi r4, r10, 0x21d8
	ctx.r[4].s64 = ctx.r[10].s64 + 8664;
	// 82675D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675D7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675D80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82675D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675D88: 386A3FF0  addi r3, r10, 0x3ff0
	ctx.r[3].s64 = ctx.r[10].s64 + 16368;
	// 82675D8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82675D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675DAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82675DB0: 4BDF1071  bl 0x82466e20
	ctx.lr = 0x82675DB4;
	sub_82466E20(ctx, base);
	// 82675DB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675DC8 size=112
    let mut pc: u32 = 0x82675DC8;
    'dispatch: loop {
        match pc {
            0x82675DC8 => {
    //   block [0x82675DC8..0x82675E38)
	// 82675DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675DD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675DD8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675DDC: 38AA39C0  addi r5, r10, 0x39c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14784;
	// 82675DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675DE4: 390B44E8  addi r8, r11, 0x44e8
	ctx.r[8].s64 = ctx.r[11].s64 + 17640;
	// 82675DE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82675DEC: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 82675DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675DF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675E00: 386A4020  addi r3, r10, 0x4020
	ctx.r[3].s64 = ctx.r[10].s64 + 16416;
	// 82675E04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675E24: 4BDF0FFD  bl 0x82466e20
	ctx.lr = 0x82675E28;
	sub_82466E20(ctx, base);
	// 82675E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675E38 size=112
    let mut pc: u32 = 0x82675E38;
    'dispatch: loop {
        match pc {
            0x82675E38 => {
    //   block [0x82675E38..0x82675EA8)
	// 82675E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675E44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675E48: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675E4C: 38AA3A20  addi r5, r10, 0x3a20
	ctx.r[5].s64 = ctx.r[10].s64 + 14880;
	// 82675E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675E54: 390B4500  addi r8, r11, 0x4500
	ctx.r[8].s64 = ctx.r[11].s64 + 17664;
	// 82675E58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82675E5C: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 82675E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675E64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675E68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675E70: 386A4050  addi r3, r10, 0x4050
	ctx.r[3].s64 = ctx.r[10].s64 + 16464;
	// 82675E74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675E78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675E80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675E88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675E90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675E94: 4BDF0F8D  bl 0x82466e20
	ctx.lr = 0x82675E98;
	sub_82466E20(ctx, base);
	// 82675E98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675EA8 size=112
    let mut pc: u32 = 0x82675EA8;
    'dispatch: loop {
        match pc {
            0x82675EA8 => {
    //   block [0x82675EA8..0x82675F18)
	// 82675EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675EB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675EB8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675EBC: 38AA40B0  addi r5, r10, 0x40b0
	ctx.r[5].s64 = ctx.r[10].s64 + 16560;
	// 82675EC0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82675EC4: 390B4530  addi r8, r11, 0x4530
	ctx.r[8].s64 = ctx.r[11].s64 + 17712;
	// 82675EC8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82675ECC: 388AAEE4  addi r4, r10, -0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + -20764;
	// 82675ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675ED4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675ED8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675EE0: 386A4080  addi r3, r10, 0x4080
	ctx.r[3].s64 = ctx.r[10].s64 + 16512;
	// 82675EE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675EE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675EEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675EF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675EF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675EFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675F00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675F04: 4BDF0F1D  bl 0x82466e20
	ctx.lr = 0x82675F08;
	sub_82466E20(ctx, base);
	// 82675F08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82675F18 size=24
    let mut pc: u32 = 0x82675F18;
    'dispatch: loop {
        match pc {
            0x82675F18 => {
    //   block [0x82675F18..0x82675F30)
	// 82675F18: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675F1C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82675F20: 394A6E70  addi r10, r10, 0x6e70
	ctx.r[10].s64 = ctx.r[10].s64 + 28272;
	// 82675F24: 816B4620  lwz r11, 0x4620(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17952 as u32) ) } as u64;
	// 82675F28: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82675F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675F30 size=112
    let mut pc: u32 = 0x82675F30;
    'dispatch: loop {
        match pc {
            0x82675F30 => {
    //   block [0x82675F30..0x82675FA0)
	// 82675F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675F3C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82675F40: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675F44: 392A3240  addi r9, r10, 0x3240
	ctx.r[9].s64 = ctx.r[10].s64 + 12864;
	// 82675F48: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82675F4C: 390B6E70  addi r8, r11, 0x6e70
	ctx.r[8].s64 = ctx.r[11].s64 + 28272;
	// 82675F50: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82675F54: 388AAEBC  addi r4, r10, -0x5144
	ctx.r[4].s64 = ctx.r[10].s64 + -20804;
	// 82675F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675F5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675F60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675F68: 386A40B0  addi r3, r10, 0x40b0
	ctx.r[3].s64 = ctx.r[10].s64 + 16560;
	// 82675F6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82675F70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82675F74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675F84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82675F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675F8C: 4BDF0E95  bl 0x82466e20
	ctx.lr = 0x82675F90;
	sub_82466E20(ctx, base);
	// 82675F90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675FA0 size=108
    let mut pc: u32 = 0x82675FA0;
    'dispatch: loop {
        match pc {
            0x82675FA0 => {
    //   block [0x82675FA0..0x8267600C)
	// 82675FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675FAC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675FB0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82675FB4: 38EB4624  addi r7, r11, 0x4624
	ctx.r[7].s64 = ctx.r[11].s64 + 17956;
	// 82675FB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82675FBC: 388AAED4  addi r4, r10, -0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + -20780;
	// 82675FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675FC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675FC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82675FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675FD0: 386A40E0  addi r3, r10, 0x40e0
	ctx.r[3].s64 = ctx.r[10].s64 + 16608;
	// 82675FD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82675FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675FF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82675FF8: 4BDF0E29  bl 0x82466e20
	ctx.lr = 0x82675FFC;
	sub_82466E20(ctx, base);
	// 82675FFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676010 size=108
    let mut pc: u32 = 0x82676010;
    'dispatch: loop {
        match pc {
            0x82676010 => {
    //   block [0x82676010..0x8267607C)
	// 82676010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267601C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676024: 38EB4658  addi r7, r11, 0x4658
	ctx.r[7].s64 = ctx.r[11].s64 + 18008;
	// 82676028: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8267602C: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 82676030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676034: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676038: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267603C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676040: 386A4110  addi r3, r10, 0x4110
	ctx.r[3].s64 = ctx.r[10].s64 + 16656;
	// 82676044: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82676048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267604C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267605C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676064: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82676068: 4BDF0DB9  bl 0x82466e20
	ctx.lr = 0x8267606C;
	sub_82466E20(ctx, base);
	// 8267606C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676080 size=108
    let mut pc: u32 = 0x82676080;
    'dispatch: loop {
        match pc {
            0x82676080 => {
    //   block [0x82676080..0x826760EC)
	// 82676080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267608C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676094: 38EB4748  addi r7, r11, 0x4748
	ctx.r[7].s64 = ctx.r[11].s64 + 18248;
	// 82676098: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267609C: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 826760A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826760A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826760A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826760AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826760B0: 386A4140  addi r3, r10, 0x4140
	ctx.r[3].s64 = ctx.r[10].s64 + 16704;
	// 826760B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826760B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826760BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826760C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826760C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826760C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826760CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826760D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826760D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826760D8: 4BDF0D49  bl 0x82466e20
	ctx.lr = 0x826760DC;
	sub_82466E20(ctx, base);
	// 826760DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826760E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826760E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826760E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826760F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826760F0 size=108
    let mut pc: u32 = 0x826760F0;
    'dispatch: loop {
        match pc {
            0x826760F0 => {
    //   block [0x826760F0..0x8267615C)
	// 826760F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826760F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826760F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826760FC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676104: 38EB4790  addi r7, r11, 0x4790
	ctx.r[7].s64 = ctx.r[11].s64 + 18320;
	// 82676108: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8267610C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 82676110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676114: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676118: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267611C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676120: 386A4170  addi r3, r10, 0x4170
	ctx.r[3].s64 = ctx.r[10].s64 + 16752;
	// 82676124: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82676128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267612C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267613C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676144: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82676148: 4BDF0CD9  bl 0x82466e20
	ctx.lr = 0x8267614C;
	sub_82466E20(ctx, base);
	// 8267614C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676160 size=108
    let mut pc: u32 = 0x82676160;
    'dispatch: loop {
        match pc {
            0x82676160 => {
    //   block [0x82676160..0x826761CC)
	// 82676160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267616C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676174: 38EB4838  addi r7, r11, 0x4838
	ctx.r[7].s64 = ctx.r[11].s64 + 18488;
	// 82676178: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267617C: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 82676180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676184: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267618C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676190: 386A41A0  addi r3, r10, 0x41a0
	ctx.r[3].s64 = ctx.r[10].s64 + 16800;
	// 82676194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82676198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267619C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826761A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826761A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826761A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826761AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826761B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826761B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826761B8: 4BDF0C69  bl 0x82466e20
	ctx.lr = 0x826761BC;
	sub_82466E20(ctx, base);
	// 826761BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826761C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826761C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826761C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826761D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826761D0 size=112
    let mut pc: u32 = 0x826761D0;
    'dispatch: loop {
        match pc {
            0x826761D0 => {
    //   block [0x826761D0..0x82676240)
	// 826761D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826761D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826761D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826761DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826761E0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826761E4: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 826761E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826761EC: 390B4850  addi r8, r11, 0x4850
	ctx.r[8].s64 = ctx.r[11].s64 + 18512;
	// 826761F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826761F4: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 826761F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826761FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676200: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676208: 386A41D0  addi r3, r10, 0x41d0
	ctx.r[3].s64 = ctx.r[10].s64 + 16848;
	// 8267620C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267621C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267622C: 4BDF0BF5  bl 0x82466e20
	ctx.lr = 0x82676230;
	sub_82466E20(ctx, base);
	// 82676230: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267623C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676240 size=112
    let mut pc: u32 = 0x82676240;
    'dispatch: loop {
        match pc {
            0x82676240 => {
    //   block [0x82676240..0x826762B0)
	// 82676240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267624C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676250: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676254: 38AA41D0  addi r5, r10, 0x41d0
	ctx.r[5].s64 = ctx.r[10].s64 + 16848;
	// 82676258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267625C: 390B48B0  addi r8, r11, 0x48b0
	ctx.r[8].s64 = ctx.r[11].s64 + 18608;
	// 82676260: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82676264: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 82676268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267626C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676278: 386A4200  addi r3, r10, 0x4200
	ctx.r[3].s64 = ctx.r[10].s64 + 16896;
	// 8267627C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267628C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267629C: 4BDF0B85  bl 0x82466e20
	ctx.lr = 0x826762A0;
	sub_82466E20(ctx, base);
	// 826762A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826762A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826762A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826762AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826762B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826762B0 size=112
    let mut pc: u32 = 0x826762B0;
    'dispatch: loop {
        match pc {
            0x826762B0 => {
    //   block [0x826762B0..0x82676320)
	// 826762B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826762B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826762B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826762BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826762C0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826762C4: 38AA41D0  addi r5, r10, 0x41d0
	ctx.r[5].s64 = ctx.r[10].s64 + 16848;
	// 826762C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826762CC: 390B48C8  addi r8, r11, 0x48c8
	ctx.r[8].s64 = ctx.r[11].s64 + 18632;
	// 826762D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826762D4: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 826762D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826762DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826762E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826762E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826762E8: 386A4230  addi r3, r10, 0x4230
	ctx.r[3].s64 = ctx.r[10].s64 + 16944;
	// 826762EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826762F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826762F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826762F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826762FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267630C: 4BDF0B15  bl 0x82466e20
	ctx.lr = 0x82676310;
	sub_82466E20(ctx, base);
	// 82676310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267631C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676320 size=112
    let mut pc: u32 = 0x82676320;
    'dispatch: loop {
        match pc {
            0x82676320 => {
    //   block [0x82676320..0x82676390)
	// 82676320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267632C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676330: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676334: 38AA41D0  addi r5, r10, 0x41d0
	ctx.r[5].s64 = ctx.r[10].s64 + 16848;
	// 82676338: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267633C: 390B48F8  addi r8, r11, 0x48f8
	ctx.r[8].s64 = ctx.r[11].s64 + 18680;
	// 82676340: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82676344: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 82676348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267634C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676350: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676358: 386A4260  addi r3, r10, 0x4260
	ctx.r[3].s64 = ctx.r[10].s64 + 16992;
	// 8267635C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267636C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267637C: 4BDF0AA5  bl 0x82466e20
	ctx.lr = 0x82676380;
	sub_82466E20(ctx, base);
	// 82676380: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267638C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82676390 size=24
    let mut pc: u32 = 0x82676390;
    'dispatch: loop {
        match pc {
            0x82676390 => {
    //   block [0x82676390..0x826763A8)
	// 82676390: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676394: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82676398: 394A6E88  addi r10, r10, 0x6e88
	ctx.r[10].s64 = ctx.r[10].s64 + 28296;
	// 8267639C: 816B4654  lwz r11, 0x4654(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18004 as u32) ) } as u64;
	// 826763A0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826763A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826763A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826763A8 size=112
    let mut pc: u32 = 0x826763A8;
    'dispatch: loop {
        match pc {
            0x826763A8 => {
    //   block [0x826763A8..0x82676418)
	// 826763A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826763AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826763B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826763B4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826763B8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826763BC: 392A327C  addi r9, r10, 0x327c
	ctx.r[9].s64 = ctx.r[10].s64 + 12924;
	// 826763C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826763C4: 390B6E88  addi r8, r11, 0x6e88
	ctx.r[8].s64 = ctx.r[11].s64 + 28296;
	// 826763C8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826763CC: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 826763D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826763D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826763D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826763DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826763E0: 386A4290  addi r3, r10, 0x4290
	ctx.r[3].s64 = ctx.r[10].s64 + 17040;
	// 826763E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826763E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826763EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826763F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826763F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826763F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826763FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82676400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676404: 4BDF0A1D  bl 0x82466e20
	ctx.lr = 0x82676408;
	sub_82466E20(ctx, base);
	// 82676408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267640C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676418 size=108
    let mut pc: u32 = 0x82676418;
    'dispatch: loop {
        match pc {
            0x82676418 => {
    //   block [0x82676418..0x82676484)
	// 82676418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267641C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676424: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267642C: 38EB4910  addi r7, r11, 0x4910
	ctx.r[7].s64 = ctx.r[11].s64 + 18704;
	// 82676430: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82676434: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 82676438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267643C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676440: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82676444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676448: 386A42C0  addi r3, r10, 0x42c0
	ctx.r[3].s64 = ctx.r[10].s64 + 17088;
	// 8267644C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82676450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267645C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267646C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82676470: 4BDF09B1  bl 0x82466e20
	ctx.lr = 0x82676474;
	sub_82466E20(ctx, base);
	// 82676474: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267647C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676488 size=108
    let mut pc: u32 = 0x82676488;
    'dispatch: loop {
        match pc {
            0x82676488 => {
    //   block [0x82676488..0x826764F4)
	// 82676488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267648C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676494: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267649C: 38EB4928  addi r7, r11, 0x4928
	ctx.r[7].s64 = ctx.r[11].s64 + 18728;
	// 826764A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826764A4: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 826764A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826764AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826764B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826764B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826764B8: 386A42F0  addi r3, r10, 0x42f0
	ctx.r[3].s64 = ctx.r[10].s64 + 17136;
	// 826764BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826764C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826764C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826764C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826764CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826764D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826764D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826764D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826764DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826764E0: 4BDF0941  bl 0x82466e20
	ctx.lr = 0x826764E4;
	sub_82466E20(ctx, base);
	// 826764E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826764E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826764EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826764F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826764F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826764F8 size=112
    let mut pc: u32 = 0x826764F8;
    'dispatch: loop {
        match pc {
            0x826764F8 => {
    //   block [0x826764F8..0x82676568)
	// 826764F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826764FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676504: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676508: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267650C: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82676510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676514: 390B495C  addi r8, r11, 0x495c
	ctx.r[8].s64 = ctx.r[11].s64 + 18780;
	// 82676518: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267651C: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 82676520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676524: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267652C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676530: 386A4320  addi r3, r10, 0x4320
	ctx.r[3].s64 = ctx.r[10].s64 + 17184;
	// 82676534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267653C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267654C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676554: 4BDF08CD  bl 0x82466e20
	ctx.lr = 0x82676558;
	sub_82466E20(ctx, base);
	// 82676558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267655C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676568 size=108
    let mut pc: u32 = 0x82676568;
    'dispatch: loop {
        match pc {
            0x82676568 => {
    //   block [0x82676568..0x826765D4)
	// 82676568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267656C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676574: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676578: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8267657C: 38EB4978  addi r7, r11, 0x4978
	ctx.r[7].s64 = ctx.r[11].s64 + 18808;
	// 82676580: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82676584: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 82676588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267658C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676590: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82676594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676598: 386A4350  addi r3, r10, 0x4350
	ctx.r[3].s64 = ctx.r[10].s64 + 17232;
	// 8267659C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826765A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826765A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826765A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826765AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826765B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826765B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826765B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826765BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826765C0: 4BDF0861  bl 0x82466e20
	ctx.lr = 0x826765C4;
	sub_82466E20(ctx, base);
	// 826765C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826765C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826765CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826765D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826765D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826765D8 size=24
    let mut pc: u32 = 0x826765D8;
    'dispatch: loop {
        match pc {
            0x826765D8 => {
    //   block [0x826765D8..0x826765F0)
	// 826765D8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826765DC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826765E0: 394A6ED0  addi r10, r10, 0x6ed0
	ctx.r[10].s64 = ctx.r[10].s64 + 28368;
	// 826765E4: 816B4974  lwz r11, 0x4974(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18804 as u32) ) } as u64;
	// 826765E8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826765EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826765F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826765F0 size=116
    let mut pc: u32 = 0x826765F0;
    'dispatch: loop {
        match pc {
            0x826765F0 => {
    //   block [0x826765F0..0x82676664)
	// 826765F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826765F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826765F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826765FC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676600: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82676604: 390B6ED0  addi r8, r11, 0x6ed0
	ctx.r[8].s64 = ctx.r[11].s64 + 28368;
	// 82676608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267660C: 392A333C  addi r9, r10, 0x333c
	ctx.r[9].s64 = ctx.r[10].s64 + 13116;
	// 82676610: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676614: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82676618: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 8267661C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676624: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267662C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676634: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82676638: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 8267663C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82676640: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 82676644: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82676648: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267664C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676650: 4BDF07D1  bl 0x82466e20
	ctx.lr = 0x82676654;
	sub_82466E20(ctx, base);
	// 82676654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267665C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676668 size=112
    let mut pc: u32 = 0x82676668;
    'dispatch: loop {
        match pc {
            0x82676668 => {
    //   block [0x82676668..0x826766D8)
	// 82676668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267666C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676674: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676678: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267667C: 38AA4320  addi r5, r10, 0x4320
	ctx.r[5].s64 = ctx.r[10].s64 + 17184;
	// 82676680: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676684: 390B49D8  addi r8, r11, 0x49d8
	ctx.r[8].s64 = ctx.r[11].s64 + 18904;
	// 82676688: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267668C: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 82676690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676694: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676698: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267669C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826766A0: 386A43B0  addi r3, r10, 0x43b0
	ctx.r[3].s64 = ctx.r[10].s64 + 17328;
	// 826766A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826766A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826766AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826766B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826766B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826766B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826766BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826766C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826766C4: 4BDF075D  bl 0x82466e20
	ctx.lr = 0x826766C8;
	sub_82466E20(ctx, base);
	// 826766C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826766CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826766D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826766D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826766D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826766D8 size=108
    let mut pc: u32 = 0x826766D8;
    'dispatch: loop {
        match pc {
            0x826766D8 => {
    //   block [0x826766D8..0x82676744)
	// 826766D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826766DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826766E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826766E4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826766E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826766EC: 38EB4A08  addi r7, r11, 0x4a08
	ctx.r[7].s64 = ctx.r[11].s64 + 18952;
	// 826766F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826766F4: 388A2224  addi r4, r10, 0x2224
	ctx.r[4].s64 = ctx.r[10].s64 + 8740;
	// 826766F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826766FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676700: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82676704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676708: 386A43E0  addi r3, r10, 0x43e0
	ctx.r[3].s64 = ctx.r[10].s64 + 17376;
	// 8267670C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82676710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267671C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267672C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82676730: 4BDF06F1  bl 0x82466e20
	ctx.lr = 0x82676734;
	sub_82466E20(ctx, base);
	// 82676734: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267673C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676748 size=108
    let mut pc: u32 = 0x82676748;
    'dispatch: loop {
        match pc {
            0x82676748 => {
    //   block [0x82676748..0x826767B4)
	// 82676748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267674C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676754: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676758: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8267675C: 38EB4A50  addi r7, r11, 0x4a50
	ctx.r[7].s64 = ctx.r[11].s64 + 19024;
	// 82676760: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82676764: 388A224C  addi r4, r10, 0x224c
	ctx.r[4].s64 = ctx.r[10].s64 + 8780;
	// 82676768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267676C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82676774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676778: 386A4410  addi r3, r10, 0x4410
	ctx.r[3].s64 = ctx.r[10].s64 + 17424;
	// 8267677C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82676780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267678C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267679C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826767A0: 4BDF0681  bl 0x82466e20
	ctx.lr = 0x826767A4;
	sub_82466E20(ctx, base);
	// 826767A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826767A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826767AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826767B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826767B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826767B8 size=112
    let mut pc: u32 = 0x826767B8;
    'dispatch: loop {
        match pc {
            0x826767B8 => {
    //   block [0x826767B8..0x82676828)
	// 826767B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826767BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826767C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826767C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826767C8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826767CC: 38AA4320  addi r5, r10, 0x4320
	ctx.r[5].s64 = ctx.r[10].s64 + 17184;
	// 826767D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826767D4: 390B4A80  addi r8, r11, 0x4a80
	ctx.r[8].s64 = ctx.r[11].s64 + 19072;
	// 826767D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826767DC: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 826767E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826767E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826767E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826767EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826767F0: 386A4440  addi r3, r10, 0x4440
	ctx.r[3].s64 = ctx.r[10].s64 + 17472;
	// 826767F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826767F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826767FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267680C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676814: 4BDF060D  bl 0x82466e20
	ctx.lr = 0x82676818;
	sub_82466E20(ctx, base);
	// 82676818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267681C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676828 size=108
    let mut pc: u32 = 0x82676828;
    'dispatch: loop {
        match pc {
            0x82676828 => {
    //   block [0x82676828..0x82676894)
	// 82676828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267682C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676834: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676838: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8267683C: 38EB4AB0  addi r7, r11, 0x4ab0
	ctx.r[7].s64 = ctx.r[11].s64 + 19120;
	// 82676840: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82676844: 388A2274  addi r4, r10, 0x2274
	ctx.r[4].s64 = ctx.r[10].s64 + 8820;
	// 82676848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267684C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82676854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676858: 386A4470  addi r3, r10, 0x4470
	ctx.r[3].s64 = ctx.r[10].s64 + 17520;
	// 8267685C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82676860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267686C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267687C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82676880: 4BDF05A1  bl 0x82466e20
	ctx.lr = 0x82676884;
	sub_82466E20(ctx, base);
	// 82676884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267688C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676898 size=108
    let mut pc: u32 = 0x82676898;
    'dispatch: loop {
        match pc {
            0x82676898 => {
    //   block [0x82676898..0x82676904)
	// 82676898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267689C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826768A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826768A4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826768A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826768AC: 38EB4B10  addi r7, r11, 0x4b10
	ctx.r[7].s64 = ctx.r[11].s64 + 19216;
	// 826768B0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826768B4: 388A22A4  addi r4, r10, 0x22a4
	ctx.r[4].s64 = ctx.r[10].s64 + 8868;
	// 826768B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826768BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826768C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826768C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826768C8: 386A44A0  addi r3, r10, 0x44a0
	ctx.r[3].s64 = ctx.r[10].s64 + 17568;
	// 826768CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826768D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826768D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826768D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826768DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826768E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826768E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826768E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826768EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826768F0: 4BDF0531  bl 0x82466e20
	ctx.lr = 0x826768F4;
	sub_82466E20(ctx, base);
	// 826768F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826768F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826768FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676908 size=112
    let mut pc: u32 = 0x82676908;
    'dispatch: loop {
        match pc {
            0x82676908 => {
    //   block [0x82676908..0x82676978)
	// 82676908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267690C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676914: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676918: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267691C: 38AA4320  addi r5, r10, 0x4320
	ctx.r[5].s64 = ctx.r[10].s64 + 17184;
	// 82676920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676924: 390B4B58  addi r8, r11, 0x4b58
	ctx.r[8].s64 = ctx.r[11].s64 + 19288;
	// 82676928: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8267692C: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 82676930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676934: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267693C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676940: 386A44D0  addi r3, r10, 0x44d0
	ctx.r[3].s64 = ctx.r[10].s64 + 17616;
	// 82676944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267694C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267695C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676964: 4BDF04BD  bl 0x82466e20
	ctx.lr = 0x82676968;
	sub_82466E20(ctx, base);
	// 82676968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267696C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676978 size=112
    let mut pc: u32 = 0x82676978;
    'dispatch: loop {
        match pc {
            0x82676978 => {
    //   block [0x82676978..0x826769E8)
	// 82676978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267697C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676984: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676988: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267698C: 38AA4530  addi r5, r10, 0x4530
	ctx.r[5].s64 = ctx.r[10].s64 + 17712;
	// 82676990: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82676994: 390B4BD0  addi r8, r11, 0x4bd0
	ctx.r[8].s64 = ctx.r[11].s64 + 19408;
	// 82676998: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267699C: 388AAFD0  addi r4, r10, -0x5030
	ctx.r[4].s64 = ctx.r[10].s64 + -20528;
	// 826769A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826769A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826769A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826769AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826769B0: 386A4500  addi r3, r10, 0x4500
	ctx.r[3].s64 = ctx.r[10].s64 + 17664;
	// 826769B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826769B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826769BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826769C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826769C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826769C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826769CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826769D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826769D4: 4BDF044D  bl 0x82466e20
	ctx.lr = 0x826769D8;
	sub_82466E20(ctx, base);
	// 826769D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826769DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826769E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826769E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826769E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826769E8 size=112
    let mut pc: u32 = 0x826769E8;
    'dispatch: loop {
        match pc {
            0x826769E8 => {
    //   block [0x826769E8..0x82676A58)
	// 826769E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826769EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826769F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826769F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826769F8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826769FC: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82676A00: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82676A04: 390B4C30  addi r8, r11, 0x4c30
	ctx.r[8].s64 = ctx.r[11].s64 + 19504;
	// 82676A08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82676A0C: 388AAF84  addi r4, r10, -0x507c
	ctx.r[4].s64 = ctx.r[10].s64 + -20604;
	// 82676A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676A14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676A18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676A20: 386A4530  addi r3, r10, 0x4530
	ctx.r[3].s64 = ctx.r[10].s64 + 17712;
	// 82676A24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676A28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676A30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676A38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676A40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676A44: 4BDF03DD  bl 0x82466e20
	ctx.lr = 0x82676A48;
	sub_82466E20(ctx, base);
	// 82676A48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676A58 size=112
    let mut pc: u32 = 0x82676A58;
    'dispatch: loop {
        match pc {
            0x82676A58 => {
    //   block [0x82676A58..0x82676AC8)
	// 82676A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676A64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676A68: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676A6C: 38AA4530  addi r5, r10, 0x4530
	ctx.r[5].s64 = ctx.r[10].s64 + 17712;
	// 82676A70: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82676A74: 390B4C48  addi r8, r11, 0x4c48
	ctx.r[8].s64 = ctx.r[11].s64 + 19528;
	// 82676A78: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82676A7C: 388AAFBC  addi r4, r10, -0x5044
	ctx.r[4].s64 = ctx.r[10].s64 + -20548;
	// 82676A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676A84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676A88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676A90: 386A4560  addi r3, r10, 0x4560
	ctx.r[3].s64 = ctx.r[10].s64 + 17760;
	// 82676A94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676AAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676AB4: 4BDF036D  bl 0x82466e20
	ctx.lr = 0x82676AB8;
	sub_82466E20(ctx, base);
	// 82676AB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676AC8 size=112
    let mut pc: u32 = 0x82676AC8;
    'dispatch: loop {
        match pc {
            0x82676AC8 => {
    //   block [0x82676AC8..0x82676B38)
	// 82676AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676AD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676AD8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676ADC: 38AA4530  addi r5, r10, 0x4530
	ctx.r[5].s64 = ctx.r[10].s64 + 17712;
	// 82676AE0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82676AE4: 390B4CA8  addi r8, r11, 0x4ca8
	ctx.r[8].s64 = ctx.r[11].s64 + 19624;
	// 82676AE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82676AEC: 388AAFA8  addi r4, r10, -0x5058
	ctx.r[4].s64 = ctx.r[10].s64 + -20568;
	// 82676AF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676AF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676AF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676AFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676B00: 386A4590  addi r3, r10, 0x4590
	ctx.r[3].s64 = ctx.r[10].s64 + 17808;
	// 82676B04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676B08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676B10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676B18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676B1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676B20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676B24: 4BDF02FD  bl 0x82466e20
	ctx.lr = 0x82676B28;
	sub_82466E20(ctx, base);
	// 82676B28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676B38 size=112
    let mut pc: u32 = 0x82676B38;
    'dispatch: loop {
        match pc {
            0x82676B38 => {
    //   block [0x82676B38..0x82676BA8)
	// 82676B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676B44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676B48: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676B4C: 38AA4530  addi r5, r10, 0x4530
	ctx.r[5].s64 = ctx.r[10].s64 + 17712;
	// 82676B50: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82676B54: 390B4CD8  addi r8, r11, 0x4cd8
	ctx.r[8].s64 = ctx.r[11].s64 + 19672;
	// 82676B58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82676B5C: 388AAF98  addi r4, r10, -0x5068
	ctx.r[4].s64 = ctx.r[10].s64 + -20584;
	// 82676B60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676B64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676B68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676B70: 386A45C0  addi r3, r10, 0x45c0
	ctx.r[3].s64 = ctx.r[10].s64 + 17856;
	// 82676B74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676B78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676B80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676B84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676B88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676B8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676B90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676B94: 4BDF028D  bl 0x82466e20
	ctx.lr = 0x82676B98;
	sub_82466E20(ctx, base);
	// 82676B98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676BA8 size=112
    let mut pc: u32 = 0x82676BA8;
    'dispatch: loop {
        match pc {
            0x82676BA8 => {
    //   block [0x82676BA8..0x82676C18)
	// 82676BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676BB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676BB8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676BBC: 38AA4620  addi r5, r10, 0x4620
	ctx.r[5].s64 = ctx.r[10].s64 + 17952;
	// 82676BC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676BC4: 390B4D08  addi r8, r11, 0x4d08
	ctx.r[8].s64 = ctx.r[11].s64 + 19720;
	// 82676BC8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82676BCC: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 82676BD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676BD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676BD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676BDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676BE0: 386A45F0  addi r3, r10, 0x45f0
	ctx.r[3].s64 = ctx.r[10].s64 + 17904;
	// 82676BE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676BE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676BF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676BF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676BF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676BFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676C00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676C04: 4BDF021D  bl 0x82466e20
	ctx.lr = 0x82676C08;
	sub_82466E20(ctx, base);
	// 82676C08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676C18 size=100
    let mut pc: u32 = 0x82676C18;
    'dispatch: loop {
        match pc {
            0x82676C18 => {
    //   block [0x82676C18..0x82676C7C)
	// 82676C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676C24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676C2C: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82676C30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676C34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676C38: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 82676C3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676C4C: 386A4620  addi r3, r10, 0x4620
	ctx.r[3].s64 = ctx.r[10].s64 + 17952;
	// 82676C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676C54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676C58: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82676C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676C60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82676C64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676C68: 4BDF01B9  bl 0x82466e20
	ctx.lr = 0x82676C6C;
	sub_82466E20(ctx, base);
	// 82676C6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82676C80 size=24
    let mut pc: u32 = 0x82676C80;
    'dispatch: loop {
        match pc {
            0x82676C80 => {
    //   block [0x82676C80..0x82676C98)
	// 82676C80: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676C84: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82676C88: 394A6F90  addi r10, r10, 0x6f90
	ctx.r[10].s64 = ctx.r[10].s64 + 28560;
	// 82676C8C: 816B4D80  lwz r11, 0x4d80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19840 as u32) ) } as u64;
	// 82676C90: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82676C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676C98 size=116
    let mut pc: u32 = 0x82676C98;
    'dispatch: loop {
        match pc {
            0x82676C98 => {
    //   block [0x82676C98..0x82676D0C)
	// 82676C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676CA4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676CA8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82676CAC: 390B6F90  addi r8, r11, 0x6f90
	ctx.r[8].s64 = ctx.r[11].s64 + 28560;
	// 82676CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676CB4: 392A3378  addi r9, r10, 0x3378
	ctx.r[9].s64 = ctx.r[10].s64 + 13176;
	// 82676CB8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676CBC: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82676CC0: 38AA4320  addi r5, r10, 0x4320
	ctx.r[5].s64 = ctx.r[10].s64 + 17184;
	// 82676CC4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676CC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676CCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676CD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676CD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676CDC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82676CE0: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 82676CE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82676CE8: 386B4650  addi r3, r11, 0x4650
	ctx.r[3].s64 = ctx.r[11].s64 + 18000;
	// 82676CEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82676CF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676CF8: 4BDF0129  bl 0x82466e20
	ctx.lr = 0x82676CFC;
	sub_82466E20(ctx, base);
	// 82676CFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676D00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676D04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676D10 size=112
    let mut pc: u32 = 0x82676D10;
    'dispatch: loop {
        match pc {
            0x82676D10 => {
    //   block [0x82676D10..0x82676D80)
	// 82676D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676D1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676D20: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676D24: 38AA4320  addi r5, r10, 0x4320
	ctx.r[5].s64 = ctx.r[10].s64 + 17184;
	// 82676D28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676D2C: 390B4D88  addi r8, r11, 0x4d88
	ctx.r[8].s64 = ctx.r[11].s64 + 19848;
	// 82676D30: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82676D34: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 82676D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676D3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676D40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676D44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676D48: 386A4680  addi r3, r10, 0x4680
	ctx.r[3].s64 = ctx.r[10].s64 + 18048;
	// 82676D4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676D54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676D6C: 4BDF00B5  bl 0x82466e20
	ctx.lr = 0x82676D70;
	sub_82466E20(ctx, base);
	// 82676D70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676D80 size=112
    let mut pc: u32 = 0x82676D80;
    'dispatch: loop {
        match pc {
            0x82676D80 => {
    //   block [0x82676D80..0x82676DF0)
	// 82676D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676D8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676D90: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676D94: 38AA44D0  addi r5, r10, 0x44d0
	ctx.r[5].s64 = ctx.r[10].s64 + 17616;
	// 82676D98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82676D9C: 390B4DD0  addi r8, r11, 0x4dd0
	ctx.r[8].s64 = ctx.r[11].s64 + 19920;
	// 82676DA0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82676DA4: 388A22D4  addi r4, r10, 0x22d4
	ctx.r[4].s64 = ctx.r[10].s64 + 8916;
	// 82676DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676DAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676DB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676DB8: 386A46B0  addi r3, r10, 0x46b0
	ctx.r[3].s64 = ctx.r[10].s64 + 18096;
	// 82676DBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676DC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676DDC: 4BDF0045  bl 0x82466e20
	ctx.lr = 0x82676DE0;
	sub_82466E20(ctx, base);
	// 82676DE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676DF0 size=112
    let mut pc: u32 = 0x82676DF0;
    'dispatch: loop {
        match pc {
            0x82676DF0 => {
    //   block [0x82676DF0..0x82676E60)
	// 82676DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676DFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676E00: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676E04: 38AA4800  addi r5, r10, 0x4800
	ctx.r[5].s64 = ctx.r[10].s64 + 18432;
	// 82676E08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82676E0C: 390B4E18  addi r8, r11, 0x4e18
	ctx.r[8].s64 = ctx.r[11].s64 + 19992;
	// 82676E10: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82676E14: 388A2464  addi r4, r10, 0x2464
	ctx.r[4].s64 = ctx.r[10].s64 + 9316;
	// 82676E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676E1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676E20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676E28: 386A46E0  addi r3, r10, 0x46e0
	ctx.r[3].s64 = ctx.r[10].s64 + 18144;
	// 82676E2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676E34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676E4C: 4BDEFFD5  bl 0x82466e20
	ctx.lr = 0x82676E50;
	sub_82466E20(ctx, base);
	// 82676E50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676E60 size=108
    let mut pc: u32 = 0x82676E60;
    'dispatch: loop {
        match pc {
            0x82676E60 => {
    //   block [0x82676E60..0x82676ECC)
	// 82676E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676E6C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676E70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82676E74: 38EB4E90  addi r7, r11, 0x4e90
	ctx.r[7].s64 = ctx.r[11].s64 + 20112;
	// 82676E78: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82676E7C: 388A22F4  addi r4, r10, 0x22f4
	ctx.r[4].s64 = ctx.r[10].s64 + 8948;
	// 82676E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676E84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676E88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82676E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676E90: 386A4710  addi r3, r10, 0x4710
	ctx.r[3].s64 = ctx.r[10].s64 + 18192;
	// 82676E94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82676E98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676E9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676EA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676EA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676EB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676EB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82676EB8: 4BDEFF69  bl 0x82466e20
	ctx.lr = 0x82676EBC;
	sub_82466E20(ctx, base);
	// 82676EBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676ED0 size=108
    let mut pc: u32 = 0x82676ED0;
    'dispatch: loop {
        match pc {
            0x82676ED0 => {
    //   block [0x82676ED0..0x82676F3C)
	// 82676ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676ED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676EDC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676EE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82676EE4: 38EB4ED8  addi r7, r11, 0x4ed8
	ctx.r[7].s64 = ctx.r[11].s64 + 20184;
	// 82676EE8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82676EEC: 388A2320  addi r4, r10, 0x2320
	ctx.r[4].s64 = ctx.r[10].s64 + 8992;
	// 82676EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676EF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676EF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82676EFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676F00: 386A4740  addi r3, r10, 0x4740
	ctx.r[3].s64 = ctx.r[10].s64 + 18240;
	// 82676F04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82676F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676F14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676F1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676F24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82676F28: 4BDEFEF9  bl 0x82466e20
	ctx.lr = 0x82676F2C;
	sub_82466E20(ctx, base);
	// 82676F2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676F30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676F34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676F38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676F40 size=112
    let mut pc: u32 = 0x82676F40;
    'dispatch: loop {
        match pc {
            0x82676F40 => {
    //   block [0x82676F40..0x82676FB0)
	// 82676F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676F48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676F4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676F50: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676F54: 38AA4320  addi r5, r10, 0x4320
	ctx.r[5].s64 = ctx.r[10].s64 + 17184;
	// 82676F58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676F5C: 390B4F20  addi r8, r11, 0x4f20
	ctx.r[8].s64 = ctx.r[11].s64 + 20256;
	// 82676F60: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82676F64: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 82676F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676F6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676F70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676F78: 386A4770  addi r3, r10, 0x4770
	ctx.r[3].s64 = ctx.r[10].s64 + 18288;
	// 82676F7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676F80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676F84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676F88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676F8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676F90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676F94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676F98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676F9C: 4BDEFE85  bl 0x82466e20
	ctx.lr = 0x82676FA0;
	sub_82466E20(ctx, base);
	// 82676FA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676FB0 size=108
    let mut pc: u32 = 0x82676FB0;
    'dispatch: loop {
        match pc {
            0x82676FB0 => {
    //   block [0x82676FB0..0x8267701C)
	// 82676FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676FBC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676FC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82676FC4: 38EB4F98  addi r7, r11, 0x4f98
	ctx.r[7].s64 = ctx.r[11].s64 + 20376;
	// 82676FC8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82676FCC: 388A2484  addi r4, r10, 0x2484
	ctx.r[4].s64 = ctx.r[10].s64 + 9348;
	// 82676FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676FD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676FD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82676FDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676FE0: 386A47A0  addi r3, r10, 0x47a0
	ctx.r[3].s64 = ctx.r[10].s64 + 18336;
	// 82676FE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82676FE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676FEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676FF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676FF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676FFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677004: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82677008: 4BDEFE19  bl 0x82466e20
	ctx.lr = 0x8267700C;
	sub_82466E20(ctx, base);
	// 8267700C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677020 size=108
    let mut pc: u32 = 0x82677020;
    'dispatch: loop {
        match pc {
            0x82677020 => {
    //   block [0x82677020..0x8267708C)
	// 82677020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267702C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677030: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82677034: 38EB4FE0  addi r7, r11, 0x4fe0
	ctx.r[7].s64 = ctx.r[11].s64 + 20448;
	// 82677038: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267703C: 388A24AC  addi r4, r10, 0x24ac
	ctx.r[4].s64 = ctx.r[10].s64 + 9388;
	// 82677040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82677044: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677048: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267704C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677050: 386A47D0  addi r3, r10, 0x47d0
	ctx.r[3].s64 = ctx.r[10].s64 + 18384;
	// 82677054: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82677058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267705C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82677064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267706C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82677078: 4BDEFDA9  bl 0x82466e20
	ctx.lr = 0x8267707C;
	sub_82466E20(ctx, base);
	// 8267707C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677090 size=112
    let mut pc: u32 = 0x82677090;
    'dispatch: loop {
        match pc {
            0x82677090 => {
    //   block [0x82677090..0x82677100)
	// 82677090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267709C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826770A0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826770A4: 38AA4320  addi r5, r10, 0x4320
	ctx.r[5].s64 = ctx.r[10].s64 + 17184;
	// 826770A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826770AC: 390B5028  addi r8, r11, 0x5028
	ctx.r[8].s64 = ctx.r[11].s64 + 20520;
	// 826770B0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826770B4: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 826770B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826770BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826770C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826770C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826770C8: 386A4800  addi r3, r10, 0x4800
	ctx.r[3].s64 = ctx.r[10].s64 + 18432;
	// 826770CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826770D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826770D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826770D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826770DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826770E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826770E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826770E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826770EC: 4BDEFD35  bl 0x82466e20
	ctx.lr = 0x826770F0;
	sub_82466E20(ctx, base);
	// 826770F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826770F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826770F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826770FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677100 size=112
    let mut pc: u32 = 0x82677100;
    'dispatch: loop {
        match pc {
            0x82677100 => {
    //   block [0x82677100..0x82677170)
	// 82677100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267710C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677110: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677114: 38AA4320  addi r5, r10, 0x4320
	ctx.r[5].s64 = ctx.r[10].s64 + 17184;
	// 82677118: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267711C: 390B50E8  addi r8, r11, 0x50e8
	ctx.r[8].s64 = ctx.r[11].s64 + 20712;
	// 82677120: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82677124: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 82677128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267712C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677130: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82677134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677138: 386A4830  addi r3, r10, 0x4830
	ctx.r[3].s64 = ctx.r[10].s64 + 18480;
	// 8267713C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82677140: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677148: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267714C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677150: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267715C: 4BDEFCC5  bl 0x82466e20
	ctx.lr = 0x82677160;
	sub_82466E20(ctx, base);
	// 82677160: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267716C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677170 size=108
    let mut pc: u32 = 0x82677170;
    'dispatch: loop {
        match pc {
            0x82677170 => {
    //   block [0x82677170..0x826771DC)
	// 82677170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267717C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677180: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82677184: 38EB5130  addi r7, r11, 0x5130
	ctx.r[7].s64 = ctx.r[11].s64 + 20784;
	// 82677188: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267718C: 388A234C  addi r4, r10, 0x234c
	ctx.r[4].s64 = ctx.r[10].s64 + 9036;
	// 82677190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82677194: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677198: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267719C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826771A0: 386A4860  addi r3, r10, 0x4860
	ctx.r[3].s64 = ctx.r[10].s64 + 18528;
	// 826771A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826771A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826771AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826771B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826771B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826771B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826771BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826771C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826771C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826771C8: 4BDEFC59  bl 0x82466e20
	ctx.lr = 0x826771CC;
	sub_82466E20(ctx, base);
	// 826771CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826771D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826771D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826771D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826771E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826771E0 size=108
    let mut pc: u32 = 0x826771E0;
    'dispatch: loop {
        match pc {
            0x826771E0 => {
    //   block [0x826771E0..0x8267724C)
	// 826771E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826771E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826771E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826771EC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826771F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826771F4: 38EB5160  addi r7, r11, 0x5160
	ctx.r[7].s64 = ctx.r[11].s64 + 20832;
	// 826771F8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826771FC: 388A2374  addi r4, r10, 0x2374
	ctx.r[4].s64 = ctx.r[10].s64 + 9076;
	// 82677200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82677204: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677208: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267720C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677210: 386A4890  addi r3, r10, 0x4890
	ctx.r[3].s64 = ctx.r[10].s64 + 18576;
	// 82677214: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82677218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267721C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82677224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267722C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677234: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82677238: 4BDEFBE9  bl 0x82466e20
	ctx.lr = 0x8267723C;
	sub_82466E20(ctx, base);
	// 8267723C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677250 size=112
    let mut pc: u32 = 0x82677250;
    'dispatch: loop {
        match pc {
            0x82677250 => {
    //   block [0x82677250..0x826772C0)
	// 82677250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267725C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677260: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677264: 38AA4320  addi r5, r10, 0x4320
	ctx.r[5].s64 = ctx.r[10].s64 + 17184;
	// 82677268: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267726C: 390B51F0  addi r8, r11, 0x51f0
	ctx.r[8].s64 = ctx.r[11].s64 + 20976;
	// 82677270: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82677274: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 82677278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267727C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677280: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82677284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677288: 386A48C0  addi r3, r10, 0x48c0
	ctx.r[3].s64 = ctx.r[10].s64 + 18624;
	// 8267728C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82677290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267729C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826772A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826772A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826772A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826772AC: 4BDEFB75  bl 0x82466e20
	ctx.lr = 0x826772B0;
	sub_82466E20(ctx, base);
	// 826772B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826772B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826772B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826772BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826772C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826772C0 size=112
    let mut pc: u32 = 0x826772C0;
    'dispatch: loop {
        match pc {
            0x826772C0 => {
    //   block [0x826772C0..0x82677330)
	// 826772C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826772C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826772C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826772CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826772D0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826772D4: 38AA4320  addi r5, r10, 0x4320
	ctx.r[5].s64 = ctx.r[10].s64 + 17184;
	// 826772D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826772DC: 390B5280  addi r8, r11, 0x5280
	ctx.r[8].s64 = ctx.r[11].s64 + 21120;
	// 826772E0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826772E4: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 826772E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826772EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826772F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826772F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826772F8: 386A48F0  addi r3, r10, 0x48f0
	ctx.r[3].s64 = ctx.r[10].s64 + 18672;
	// 826772FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82677300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267730C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267731C: 4BDEFB05  bl 0x82466e20
	ctx.lr = 0x82677320;
	sub_82466E20(ctx, base);
	// 82677320: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267732C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677330 size=112
    let mut pc: u32 = 0x82677330;
    'dispatch: loop {
        match pc {
            0x82677330 => {
    //   block [0x82677330..0x826773A0)
	// 82677330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267733C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677340: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677344: 38AA4320  addi r5, r10, 0x4320
	ctx.r[5].s64 = ctx.r[10].s64 + 17184;
	// 82677348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267734C: 390B5328  addi r8, r11, 0x5328
	ctx.r[8].s64 = ctx.r[11].s64 + 21288;
	// 82677350: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82677354: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 82677358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267735C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677360: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82677364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677368: 386A4920  addi r3, r10, 0x4920
	ctx.r[3].s64 = ctx.r[10].s64 + 18720;
	// 8267736C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82677370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267737C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267738C: 4BDEFA95  bl 0x82466e20
	ctx.lr = 0x82677390;
	sub_82466E20(ctx, base);
	// 82677390: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267739C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826773A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826773A0 size=108
    let mut pc: u32 = 0x826773A0;
    'dispatch: loop {
        match pc {
            0x826773A0 => {
    //   block [0x826773A0..0x8267740C)
	// 826773A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826773A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826773A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826773AC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826773B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826773B4: 38EB5340  addi r7, r11, 0x5340
	ctx.r[7].s64 = ctx.r[11].s64 + 21312;
	// 826773B8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826773BC: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 826773C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826773C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826773C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826773CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826773D0: 386A4950  addi r3, r10, 0x4950
	ctx.r[3].s64 = ctx.r[10].s64 + 18768;
	// 826773D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826773D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826773DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826773E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826773E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826773E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826773EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826773F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826773F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826773F8: 4BDEFA29  bl 0x82466e20
	ctx.lr = 0x826773FC;
	sub_82466E20(ctx, base);
	// 826773FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677410 size=112
    let mut pc: u32 = 0x82677410;
    'dispatch: loop {
        match pc {
            0x82677410 => {
    //   block [0x82677410..0x82677480)
	// 82677410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267741C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677420: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677424: 38AA4320  addi r5, r10, 0x4320
	ctx.r[5].s64 = ctx.r[10].s64 + 17184;
	// 82677428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267742C: 390B53B8  addi r8, r11, 0x53b8
	ctx.r[8].s64 = ctx.r[11].s64 + 21432;
	// 82677430: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82677434: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 82677438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267743C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82677444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677448: 386A4980  addi r3, r10, 0x4980
	ctx.r[3].s64 = ctx.r[10].s64 + 18816;
	// 8267744C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82677450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267745C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267746C: 4BDEF9B5  bl 0x82466e20
	ctx.lr = 0x82677470;
	sub_82466E20(ctx, base);
	// 82677470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267747C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677480 size=112
    let mut pc: u32 = 0x82677480;
    'dispatch: loop {
        match pc {
            0x82677480 => {
    //   block [0x82677480..0x826774F0)
	// 82677480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267748C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677490: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677494: 38AA4DD0  addi r5, r10, 0x4dd0
	ctx.r[5].s64 = ctx.r[10].s64 + 19920;
	// 82677498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267749C: 390B5400  addi r8, r11, 0x5400
	ctx.r[8].s64 = ctx.r[11].s64 + 21504;
	// 826774A0: 39200011  li r9, 0x11
	ctx.r[9].s64 = 17;
	// 826774A4: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 826774A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826774AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826774B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826774B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826774B8: 386A49B0  addi r3, r10, 0x49b0
	ctx.r[3].s64 = ctx.r[10].s64 + 18864;
	// 826774BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826774C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826774C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826774C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826774CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826774D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826774D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826774D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826774DC: 4BDEF945  bl 0x82466e20
	ctx.lr = 0x826774E0;
	sub_82466E20(ctx, base);
	// 826774E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826774E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826774E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826774EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826774F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826774F0 size=100
    let mut pc: u32 = 0x826774F0;
    'dispatch: loop {
        match pc {
            0x826774F0 => {
    //   block [0x826774F0..0x82677554)
	// 826774F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826774F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826774F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826774FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82677504: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82677508: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267750C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677510: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 82677514: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267751C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677524: 386A49E0  addi r3, r10, 0x49e0
	ctx.r[3].s64 = ctx.r[10].s64 + 18912;
	// 82677528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267752C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677530: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82677534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677538: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267753C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677540: 4BDEF8E1  bl 0x82466e20
	ctx.lr = 0x82677544;
	sub_82466E20(ctx, base);
	// 82677544: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267754C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677558 size=100
    let mut pc: u32 = 0x82677558;
    'dispatch: loop {
        match pc {
            0x82677558 => {
    //   block [0x82677558..0x826775BC)
	// 82677558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267755C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82677564: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267756C: 38AA4A70  addi r5, r10, 0x4a70
	ctx.r[5].s64 = ctx.r[10].s64 + 19056;
	// 82677570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82677574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677578: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 8267757C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82677584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267758C: 386A4A10  addi r3, r10, 0x4a10
	ctx.r[3].s64 = ctx.r[10].s64 + 18960;
	// 82677590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677594: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677598: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267759C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826775A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826775A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826775A8: 4BDEF879  bl 0x82466e20
	ctx.lr = 0x826775AC;
	sub_82466E20(ctx, base);
	// 826775AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826775B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826775B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826775B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826775C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826775C0 size=100
    let mut pc: u32 = 0x826775C0;
    'dispatch: loop {
        match pc {
            0x826775C0 => {
    //   block [0x826775C0..0x82677624)
	// 826775C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826775C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826775C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826775CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826775D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826775D4: 38AA49B0  addi r5, r10, 0x49b0
	ctx.r[5].s64 = ctx.r[10].s64 + 18864;
	// 826775D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826775DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826775E0: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 826775E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826775E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826775EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826775F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826775F4: 386A4A40  addi r3, r10, 0x4a40
	ctx.r[3].s64 = ctx.r[10].s64 + 19008;
	// 826775F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826775FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677600: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82677604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677608: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267760C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677610: 4BDEF811  bl 0x82466e20
	ctx.lr = 0x82677614;
	sub_82466E20(ctx, base);
	// 82677614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267761C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677628 size=104
    let mut pc: u32 = 0x82677628;
    'dispatch: loop {
        match pc {
            0x82677628 => {
    //   block [0x82677628..0x82677690)
	// 82677628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267762C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82677634: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82677638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267763C: 392A33AC  addi r9, r10, 0x33ac
	ctx.r[9].s64 = ctx.r[10].s64 + 13228;
	// 82677640: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677648: 38AA49E0  addi r5, r10, 0x49e0
	ctx.r[5].s64 = ctx.r[10].s64 + 18912;
	// 8267764C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82677650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82677654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267765C: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 82677660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677664: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677668: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267766C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677670: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82677674: 386A4A70  addi r3, r10, 0x4a70
	ctx.r[3].s64 = ctx.r[10].s64 + 19056;
	// 82677678: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267767C: 4BDEF7A5  bl 0x82466e20
	ctx.lr = 0x82677680;
	sub_82466E20(ctx, base);
	// 82677680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267768C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677690 size=108
    let mut pc: u32 = 0x82677690;
    'dispatch: loop {
        match pc {
            0x82677690 => {
    //   block [0x82677690..0x826776FC)
	// 82677690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267769C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826776A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826776A4: 38EB5598  addi r7, r11, 0x5598
	ctx.r[7].s64 = ctx.r[11].s64 + 21912;
	// 826776A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826776AC: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 826776B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826776B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826776B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826776BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826776C0: 386A4AA0  addi r3, r10, 0x4aa0
	ctx.r[3].s64 = ctx.r[10].s64 + 19104;
	// 826776C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826776C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826776CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826776D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826776D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826776D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826776DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826776E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826776E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826776E8: 4BDEF739  bl 0x82466e20
	ctx.lr = 0x826776EC;
	sub_82466E20(ctx, base);
	// 826776EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826776F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826776F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826776F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677700 size=112
    let mut pc: u32 = 0x82677700;
    'dispatch: loop {
        match pc {
            0x82677700 => {
    //   block [0x82677700..0x82677770)
	// 82677700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267770C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677710: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677714: 38AA4A70  addi r5, r10, 0x4a70
	ctx.r[5].s64 = ctx.r[10].s64 + 19056;
	// 82677718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267771C: 390B55C8  addi r8, r11, 0x55c8
	ctx.r[8].s64 = ctx.r[11].s64 + 21960;
	// 82677720: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82677724: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 82677728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267772C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677730: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82677734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677738: 386A4AD0  addi r3, r10, 0x4ad0
	ctx.r[3].s64 = ctx.r[10].s64 + 19152;
	// 8267773C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82677740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267774C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267775C: 4BDEF6C5  bl 0x82466e20
	ctx.lr = 0x82677760;
	sub_82466E20(ctx, base);
	// 82677760: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267776C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677770 size=116
    let mut pc: u32 = 0x82677770;
    'dispatch: loop {
        match pc {
            0x82677770 => {
    //   block [0x82677770..0x826777E4)
	// 82677770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267777C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677780: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82677784: 390B5674  addi r8, r11, 0x5674
	ctx.r[8].s64 = ctx.r[11].s64 + 22132;
	// 82677788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267778C: 392A3408  addi r9, r10, 0x3408
	ctx.r[9].s64 = ctx.r[10].s64 + 13320;
	// 82677790: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677794: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82677798: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 8267779C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826777A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826777A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826777A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826777AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826777B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826777B4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826777B8: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 826777BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826777C0: 386B4B00  addi r3, r11, 0x4b00
	ctx.r[3].s64 = ctx.r[11].s64 + 19200;
	// 826777C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826777C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826777CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826777D0: 4BDEF651  bl 0x82466e20
	ctx.lr = 0x826777D4;
	sub_82466E20(ctx, base);
	// 826777D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826777D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826777DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826777E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826777E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826777E8 size=112
    let mut pc: u32 = 0x826777E8;
    'dispatch: loop {
        match pc {
            0x826777E8 => {
    //   block [0x826777E8..0x82677858)
	// 826777E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826777EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826777F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826777F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826777F8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826777FC: 38AA4BC0  addi r5, r10, 0x4bc0
	ctx.r[5].s64 = ctx.r[10].s64 + 19392;
	// 82677800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82677804: 390B568C  addi r8, r11, 0x568c
	ctx.r[8].s64 = ctx.r[11].s64 + 22156;
	// 82677808: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267780C: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 82677810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82677814: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267781C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677820: 386A4B30  addi r3, r10, 0x4b30
	ctx.r[3].s64 = ctx.r[10].s64 + 19248;
	// 82677824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82677828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267782C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82677834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267783C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677844: 4BDEF5DD  bl 0x82466e20
	ctx.lr = 0x82677848;
	sub_82466E20(ctx, base);
	// 82677848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267784C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677858 size=100
    let mut pc: u32 = 0x82677858;
    'dispatch: loop {
        match pc {
            0x82677858 => {
    //   block [0x82677858..0x826778BC)
	// 82677858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267785C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82677864: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267786C: 38AA4B90  addi r5, r10, 0x4b90
	ctx.r[5].s64 = ctx.r[10].s64 + 19344;
	// 82677870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82677874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677878: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 8267787C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82677884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267788C: 386A4B60  addi r3, r10, 0x4b60
	ctx.r[3].s64 = ctx.r[10].s64 + 19296;
	// 82677890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677894: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677898: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267789C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826778A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826778A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826778A8: 4BDEF579  bl 0x82466e20
	ctx.lr = 0x826778AC;
	sub_82466E20(ctx, base);
	// 826778AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826778B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826778B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826778B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826778C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826778C0 size=112
    let mut pc: u32 = 0x826778C0;
    'dispatch: loop {
        match pc {
            0x826778C0 => {
    //   block [0x826778C0..0x82677930)
	// 826778C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826778C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826778C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826778CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826778D0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826778D4: 38AA4BC0  addi r5, r10, 0x4bc0
	ctx.r[5].s64 = ctx.r[10].s64 + 19392;
	// 826778D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826778DC: 390B56A4  addi r8, r11, 0x56a4
	ctx.r[8].s64 = ctx.r[11].s64 + 22180;
	// 826778E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826778E4: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 826778E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826778EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826778F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826778F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826778F8: 386A4B90  addi r3, r10, 0x4b90
	ctx.r[3].s64 = ctx.r[10].s64 + 19344;
	// 826778FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82677900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267790C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267791C: 4BDEF505  bl 0x82466e20
	ctx.lr = 0x82677920;
	sub_82466E20(ctx, base);
	// 82677920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267792C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677930 size=112
    let mut pc: u32 = 0x82677930;
    'dispatch: loop {
        match pc {
            0x82677930 => {
    //   block [0x82677930..0x826779A0)
	// 82677930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267793C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677940: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677944: 38AA4B00  addi r5, r10, 0x4b00
	ctx.r[5].s64 = ctx.r[10].s64 + 19200;
	// 82677948: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8267794C: 390B56C0  addi r8, r11, 0x56c0
	ctx.r[8].s64 = ctx.r[11].s64 + 22208;
	// 82677950: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82677954: 388A2424  addi r4, r10, 0x2424
	ctx.r[4].s64 = ctx.r[10].s64 + 9252;
	// 82677958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267795C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82677964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677968: 386A4BC0  addi r3, r10, 0x4bc0
	ctx.r[3].s64 = ctx.r[10].s64 + 19392;
	// 8267796C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82677970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267797C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267798C: 4BDEF495  bl 0x82466e20
	ctx.lr = 0x82677990;
	sub_82466E20(ctx, base);
	// 82677990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267799C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826779A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826779A0 size=100
    let mut pc: u32 = 0x826779A0;
    'dispatch: loop {
        match pc {
            0x826779A0 => {
    //   block [0x826779A0..0x82677A04)
	// 826779A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826779A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826779A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826779AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826779B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826779B4: 38AA4BC0  addi r5, r10, 0x4bc0
	ctx.r[5].s64 = ctx.r[10].s64 + 19392;
	// 826779B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826779BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826779C0: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 826779C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826779C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826779CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826779D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826779D4: 386A4BF0  addi r3, r10, 0x4bf0
	ctx.r[3].s64 = ctx.r[10].s64 + 19440;
	// 826779D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826779DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826779E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826779E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826779E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826779EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826779F0: 4BDEF431  bl 0x82466e20
	ctx.lr = 0x826779F4;
	sub_82466E20(ctx, base);
	// 826779F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826779F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826779FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677A08 size=100
    let mut pc: u32 = 0x82677A08;
    'dispatch: loop {
        match pc {
            0x82677A08 => {
    //   block [0x82677A08..0x82677A6C)
	// 82677A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82677A14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82677A1C: 38AA4B30  addi r5, r10, 0x4b30
	ctx.r[5].s64 = ctx.r[10].s64 + 19248;
	// 82677A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82677A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677A28: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 82677A2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677A30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82677A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677A38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677A3C: 386A4C20  addi r3, r10, 0x4c20
	ctx.r[3].s64 = ctx.r[10].s64 + 19488;
	// 82677A40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677A44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677A48: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82677A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677A50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82677A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677A58: 4BDEF3C9  bl 0x82466e20
	ctx.lr = 0x82677A5C;
	sub_82466E20(ctx, base);
	// 82677A5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677A70 size=100
    let mut pc: u32 = 0x82677A70;
    'dispatch: loop {
        match pc {
            0x82677A70 => {
    //   block [0x82677A70..0x82677AD4)
	// 82677A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82677A7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82677A84: 38AA4BF0  addi r5, r10, 0x4bf0
	ctx.r[5].s64 = ctx.r[10].s64 + 19440;
	// 82677A88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82677A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677A90: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 82677A94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82677A9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677AA4: 386A4C50  addi r3, r10, 0x4c50
	ctx.r[3].s64 = ctx.r[10].s64 + 19536;
	// 82677AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677AAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677AB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82677AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677AB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82677ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677AC0: 4BDEF361  bl 0x82466e20
	ctx.lr = 0x82677AC4;
	sub_82466E20(ctx, base);
	// 82677AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677AD8 size=112
    let mut pc: u32 = 0x82677AD8;
    'dispatch: loop {
        match pc {
            0x82677AD8 => {
    //   block [0x82677AD8..0x82677B48)
	// 82677AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82677AE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677AE8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677AEC: 38AA4CE0  addi r5, r10, 0x4ce0
	ctx.r[5].s64 = ctx.r[10].s64 + 19680;
	// 82677AF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82677AF4: 390B5768  addi r8, r11, 0x5768
	ctx.r[8].s64 = ctx.r[11].s64 + 22376;
	// 82677AF8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82677AFC: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 82677B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82677B04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677B08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82677B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677B10: 386A4C80  addi r3, r10, 0x4c80
	ctx.r[3].s64 = ctx.r[10].s64 + 19584;
	// 82677B14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82677B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82677B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677B34: 4BDEF2ED  bl 0x82466e20
	ctx.lr = 0x82677B38;
	sub_82466E20(ctx, base);
	// 82677B38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677B48 size=112
    let mut pc: u32 = 0x82677B48;
    'dispatch: loop {
        match pc {
            0x82677B48 => {
    //   block [0x82677B48..0x82677BB8)
	// 82677B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82677B54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677B58: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677B5C: 38AA4D10  addi r5, r10, 0x4d10
	ctx.r[5].s64 = ctx.r[10].s64 + 19728;
	// 82677B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82677B64: 390B5798  addi r8, r11, 0x5798
	ctx.r[8].s64 = ctx.r[11].s64 + 22424;
	// 82677B68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82677B6C: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 82677B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82677B74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677B78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82677B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677B80: 386A4CB0  addi r3, r10, 0x4cb0
	ctx.r[3].s64 = ctx.r[10].s64 + 19632;
	// 82677B84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82677B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82677B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677BA4: 4BDEF27D  bl 0x82466e20
	ctx.lr = 0x82677BA8;
	sub_82466E20(ctx, base);
	// 82677BA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677BB8 size=112
    let mut pc: u32 = 0x82677BB8;
    'dispatch: loop {
        match pc {
            0x82677BB8 => {
    //   block [0x82677BB8..0x82677C28)
	// 82677BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82677BC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677BC8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677BCC: 38AA4DD0  addi r5, r10, 0x4dd0
	ctx.r[5].s64 = ctx.r[10].s64 + 19920;
	// 82677BD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82677BD4: 390B57B0  addi r8, r11, 0x57b0
	ctx.r[8].s64 = ctx.r[11].s64 + 22448;
	// 82677BD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82677BDC: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 82677BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82677BE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677BE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82677BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677BF0: 386A4CE0  addi r3, r10, 0x4ce0
	ctx.r[3].s64 = ctx.r[10].s64 + 19680;
	// 82677BF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82677BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82677C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677C14: 4BDEF20D  bl 0x82466e20
	ctx.lr = 0x82677C18;
	sub_82466E20(ctx, base);
	// 82677C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677C28 size=112
    let mut pc: u32 = 0x82677C28;
    'dispatch: loop {
        match pc {
            0x82677C28 => {
    //   block [0x82677C28..0x82677C98)
	// 82677C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82677C34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677C38: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677C3C: 38AA4CE0  addi r5, r10, 0x4ce0
	ctx.r[5].s64 = ctx.r[10].s64 + 19680;
	// 82677C40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82677C44: 390B57E0  addi r8, r11, 0x57e0
	ctx.r[8].s64 = ctx.r[11].s64 + 22496;
	// 82677C48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82677C4C: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 82677C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82677C54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677C58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82677C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677C60: 386A4D10  addi r3, r10, 0x4d10
	ctx.r[3].s64 = ctx.r[10].s64 + 19728;
	// 82677C64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82677C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677C70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82677C74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677C78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677C7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677C80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677C84: 4BDEF19D  bl 0x82466e20
	ctx.lr = 0x82677C88;
	sub_82466E20(ctx, base);
	// 82677C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677C98 size=112
    let mut pc: u32 = 0x82677C98;
    'dispatch: loop {
        match pc {
            0x82677C98 => {
    //   block [0x82677C98..0x82677D08)
	// 82677C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82677CA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677CA8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677CAC: 38AA4D10  addi r5, r10, 0x4d10
	ctx.r[5].s64 = ctx.r[10].s64 + 19728;
	// 82677CB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82677CB4: 390B57F8  addi r8, r11, 0x57f8
	ctx.r[8].s64 = ctx.r[11].s64 + 22520;
	// 82677CB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82677CBC: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 82677CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82677CC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677CC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82677CCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677CD0: 386A4D40  addi r3, r10, 0x4d40
	ctx.r[3].s64 = ctx.r[10].s64 + 19776;
	// 82677CD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82677CD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677CE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82677CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677CE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677CEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677CF4: 4BDEF12D  bl 0x82466e20
	ctx.lr = 0x82677CF8;
	sub_82466E20(ctx, base);
	// 82677CF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677D08 size=112
    let mut pc: u32 = 0x82677D08;
    'dispatch: loop {
        match pc {
            0x82677D08 => {
    //   block [0x82677D08..0x82677D78)
	// 82677D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82677D14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677D18: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677D1C: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82677D20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82677D24: 390B5810  addi r8, r11, 0x5810
	ctx.r[8].s64 = ctx.r[11].s64 + 22544;
	// 82677D28: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82677D2C: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 82677D30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82677D34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677D38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82677D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677D40: 386A4D70  addi r3, r10, 0x4d70
	ctx.r[3].s64 = ctx.r[10].s64 + 19824;
	// 82677D44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82677D48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677D50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82677D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677D58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677D5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677D60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677D64: 4BDEF0BD  bl 0x82466e20
	ctx.lr = 0x82677D68;
	sub_82466E20(ctx, base);
	// 82677D68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82677D78 size=36
    let mut pc: u32 = 0x82677D78;
    'dispatch: loop {
        match pc {
            0x82677D78 => {
    //   block [0x82677D78..0x82677D9C)
	// 82677D78: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677D7C: 814B58A4  lwz r10, 0x58a4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22692 as u32) ) } as u64;
	// 82677D80: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677D84: 396B6FF0  addi r11, r11, 0x6ff0
	ctx.r[11].s64 = ctx.r[11].s64 + 28656;
	// 82677D88: 914B00E0  stw r10, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[10].u32 ) };
	// 82677D8C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82677D90: 814A58A0  lwz r10, 0x58a0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(22688 as u32) ) } as u64;
	// 82677D94: 914B0200  stw r10, 0x200(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(512 as u32), ctx.r[10].u32 ) };
	// 82677D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677DA0 size=116
    let mut pc: u32 = 0x82677DA0;
    'dispatch: loop {
        match pc {
            0x82677DA0 => {
    //   block [0x82677DA0..0x82677E14)
	// 82677DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82677DAC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677DB0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82677DB4: 390B6FF0  addi r8, r11, 0x6ff0
	ctx.r[8].s64 = ctx.r[11].s64 + 28656;
	// 82677DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82677DBC: 392A341C  addi r9, r10, 0x341c
	ctx.r[9].s64 = ctx.r[10].s64 + 13340;
	// 82677DC0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677DC4: 38E00017  li r7, 0x17
	ctx.r[7].s64 = 23;
	// 82677DC8: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82677DCC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82677DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82677DD4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82677DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677DE4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82677DE8: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 82677DEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82677DF0: 386B4DA0  addi r3, r11, 0x4da0
	ctx.r[3].s64 = ctx.r[11].s64 + 19872;
	// 82677DF4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82677DF8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677E00: 4BDEF021  bl 0x82466e20
	ctx.lr = 0x82677E04;
	sub_82466E20(ctx, base);
	// 82677E04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677E18 size=116
    let mut pc: u32 = 0x82677E18;
    'dispatch: loop {
        match pc {
            0x82677E18 => {
    //   block [0x82677E18..0x82677E8C)
	// 82677E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82677E24: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677E28: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82677E2C: 390B58B0  addi r8, r11, 0x58b0
	ctx.r[8].s64 = ctx.r[11].s64 + 22704;
	// 82677E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82677E34: 392A3510  addi r9, r10, 0x3510
	ctx.r[9].s64 = ctx.r[10].s64 + 13584;
	// 82677E38: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677E3C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82677E40: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82677E44: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82677E48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82677E4C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82677E50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677E54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677E58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677E5C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82677E60: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 82677E64: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82677E68: 386B4DD0  addi r3, r11, 0x4dd0
	ctx.r[3].s64 = ctx.r[11].s64 + 19920;
	// 82677E6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82677E70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677E78: 4BDEEFA9  bl 0x82466e20
	ctx.lr = 0x82677E7C;
	sub_82466E20(ctx, base);
	// 82677E7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677E90 size=112
    let mut pc: u32 = 0x82677E90;
    'dispatch: loop {
        match pc {
            0x82677E90 => {
    //   block [0x82677E90..0x82677F00)
	// 82677E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82677E9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677EA0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677EA4: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82677EA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82677EAC: 390B5928  addi r8, r11, 0x5928
	ctx.r[8].s64 = ctx.r[11].s64 + 22824;
	// 82677EB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82677EB4: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 82677EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82677EBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677EC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82677EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677EC8: 386A4E00  addi r3, r10, 0x4e00
	ctx.r[3].s64 = ctx.r[10].s64 + 19968;
	// 82677ECC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82677ED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677ED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82677EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677EE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677EEC: 4BDEEF35  bl 0x82466e20
	ctx.lr = 0x82677EF0;
	sub_82466E20(ctx, base);
	// 82677EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677F00 size=112
    let mut pc: u32 = 0x82677F00;
    'dispatch: loop {
        match pc {
            0x82677F00 => {
    //   block [0x82677F00..0x82677F70)
	// 82677F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82677F0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677F10: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677F14: 38AA3720  addi r5, r10, 0x3720
	ctx.r[5].s64 = ctx.r[10].s64 + 14112;
	// 82677F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82677F1C: 390B5940  addi r8, r11, 0x5940
	ctx.r[8].s64 = ctx.r[11].s64 + 22848;
	// 82677F20: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82677F24: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 82677F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82677F2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677F30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82677F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677F38: 386A4E30  addi r3, r10, 0x4e30
	ctx.r[3].s64 = ctx.r[10].s64 + 20016;
	// 82677F3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82677F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82677F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677F5C: 4BDEEEC5  bl 0x82466e20
	ctx.lr = 0x82677F60;
	sub_82466E20(ctx, base);
	// 82677F60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677F70 size=108
    let mut pc: u32 = 0x82677F70;
    'dispatch: loop {
        match pc {
            0x82677F70 => {
    //   block [0x82677F70..0x82677FDC)
	// 82677F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82677F7C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82677F84: 38EB5958  addi r7, r11, 0x5958
	ctx.r[7].s64 = ctx.r[11].s64 + 22872;
	// 82677F88: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82677F8C: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 82677F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82677F94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82677F98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82677F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82677FA0: 386A4E60  addi r3, r10, 0x4e60
	ctx.r[3].s64 = ctx.r[10].s64 + 20064;
	// 82677FA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82677FA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82677FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82677FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82677FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82677FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82677FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82677FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82677FC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82677FC8: 4BDEEE59  bl 0x82466e20
	ctx.lr = 0x82677FCC;
	sub_82466E20(ctx, base);
	// 82677FCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82677FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82677FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82677FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82677FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82677FE0 size=108
    let mut pc: u32 = 0x82677FE0;
    'dispatch: loop {
        match pc {
            0x82677FE0 => {
    //   block [0x82677FE0..0x8267804C)
	// 82677FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82677FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82677FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82677FEC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82677FF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82677FF4: 38EB5970  addi r7, r11, 0x5970
	ctx.r[7].s64 = ctx.r[11].s64 + 22896;
	// 82677FF8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82677FFC: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 82678000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678004: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678008: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267800C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678010: 386A4E90  addi r3, r10, 0x4e90
	ctx.r[3].s64 = ctx.r[10].s64 + 20112;
	// 82678014: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82678018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267801C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267802C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678034: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82678038: 4BDEEDE9  bl 0x82466e20
	ctx.lr = 0x8267803C;
	sub_82466E20(ctx, base);
	// 8267803C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678050 size=108
    let mut pc: u32 = 0x82678050;
    'dispatch: loop {
        match pc {
            0x82678050 => {
    //   block [0x82678050..0x826780BC)
	// 82678050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82678054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267805C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82678060: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678064: 38EB59B8  addi r7, r11, 0x59b8
	ctx.r[7].s64 = ctx.r[11].s64 + 22968;
	// 82678068: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267806C: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 82678070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678074: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678078: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267807C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678080: 386A4EC0  addi r3, r10, 0x4ec0
	ctx.r[3].s64 = ctx.r[10].s64 + 20160;
	// 82678084: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82678088: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267808C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267809C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826780A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826780A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826780A8: 4BDEED79  bl 0x82466e20
	ctx.lr = 0x826780AC;
	sub_82466E20(ctx, base);
	// 826780AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826780B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826780B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826780B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826780C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826780C0 size=112
    let mut pc: u32 = 0x826780C0;
    'dispatch: loop {
        match pc {
            0x826780C0 => {
    //   block [0x826780C0..0x82678130)
	// 826780C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826780C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826780C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826780CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826780D0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826780D4: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 826780D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826780DC: 390B59D0  addi r8, r11, 0x59d0
	ctx.r[8].s64 = ctx.r[11].s64 + 22992;
	// 826780E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826780E4: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 826780E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826780EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826780F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826780F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826780F8: 386A4EF0  addi r3, r10, 0x4ef0
	ctx.r[3].s64 = ctx.r[10].s64 + 20208;
	// 826780FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82678100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267810C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267811C: 4BDEED05  bl 0x82466e20
	ctx.lr = 0x82678120;
	sub_82466E20(ctx, base);
	// 82678120: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267812C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678130 size=112
    let mut pc: u32 = 0x82678130;
    'dispatch: loop {
        match pc {
            0x82678130 => {
    //   block [0x82678130..0x826781A0)
	// 82678130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82678134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267813C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678140: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82678144: 38AA4230  addi r5, r10, 0x4230
	ctx.r[5].s64 = ctx.r[10].s64 + 16944;
	// 82678148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267814C: 390B5A00  addi r8, r11, 0x5a00
	ctx.r[8].s64 = ctx.r[11].s64 + 23040;
	// 82678150: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82678154: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 82678158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267815C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82678164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678168: 386A4F20  addi r3, r10, 0x4f20
	ctx.r[3].s64 = ctx.r[10].s64 + 20256;
	// 8267816C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82678170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267817C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267818C: 4BDEEC95  bl 0x82466e20
	ctx.lr = 0x82678190;
	sub_82466E20(ctx, base);
	// 82678190: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267819C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826781A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826781A0 size=112
    let mut pc: u32 = 0x826781A0;
    'dispatch: loop {
        match pc {
            0x826781A0 => {
    //   block [0x826781A0..0x82678210)
	// 826781A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826781A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826781A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826781AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826781B0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826781B4: 38AA4230  addi r5, r10, 0x4230
	ctx.r[5].s64 = ctx.r[10].s64 + 16944;
	// 826781B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826781BC: 390B5A48  addi r8, r11, 0x5a48
	ctx.r[8].s64 = ctx.r[11].s64 + 23112;
	// 826781C0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826781C4: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 826781C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826781CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826781D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826781D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826781D8: 386A4F50  addi r3, r10, 0x4f50
	ctx.r[3].s64 = ctx.r[10].s64 + 20304;
	// 826781DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826781E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826781E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826781E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826781EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826781F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826781F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826781F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826781FC: 4BDEEC25  bl 0x82466e20
	ctx.lr = 0x82678200;
	sub_82466E20(ctx, base);
	// 82678200: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267820C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678210 size=112
    let mut pc: u32 = 0x82678210;
    'dispatch: loop {
        match pc {
            0x82678210 => {
    //   block [0x82678210..0x82678280)
	// 82678210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82678214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267821C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678220: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82678224: 38AA4260  addi r5, r10, 0x4260
	ctx.r[5].s64 = ctx.r[10].s64 + 16992;
	// 82678228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267822C: 390B5AA8  addi r8, r11, 0x5aa8
	ctx.r[8].s64 = ctx.r[11].s64 + 23208;
	// 82678230: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82678234: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 82678238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267823C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678240: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82678244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678248: 386A4F80  addi r3, r10, 0x4f80
	ctx.r[3].s64 = ctx.r[10].s64 + 20352;
	// 8267824C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82678250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267825C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267826C: 4BDEEBB5  bl 0x82466e20
	ctx.lr = 0x82678270;
	sub_82466E20(ctx, base);
	// 82678270: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267827C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678280 size=112
    let mut pc: u32 = 0x82678280;
    'dispatch: loop {
        match pc {
            0x82678280 => {
    //   block [0x82678280..0x826782F0)
	// 82678280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82678284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267828C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678290: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82678294: 38AA4260  addi r5, r10, 0x4260
	ctx.r[5].s64 = ctx.r[10].s64 + 16992;
	// 82678298: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267829C: 390B5B08  addi r8, r11, 0x5b08
	ctx.r[8].s64 = ctx.r[11].s64 + 23304;
	// 826782A0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826782A4: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 826782A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826782AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826782B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826782B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826782B8: 386A4FB0  addi r3, r10, 0x4fb0
	ctx.r[3].s64 = ctx.r[10].s64 + 20400;
	// 826782BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826782C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826782C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826782C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826782CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826782D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826782D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826782D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826782DC: 4BDEEB45  bl 0x82466e20
	ctx.lr = 0x826782E0;
	sub_82466E20(ctx, base);
	// 826782E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826782E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826782E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826782EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826782F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826782F0 size=112
    let mut pc: u32 = 0x826782F0;
    'dispatch: loop {
        match pc {
            0x826782F0 => {
    //   block [0x826782F0..0x82678360)
	// 826782F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826782F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826782F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826782FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678300: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82678304: 38AA4230  addi r5, r10, 0x4230
	ctx.r[5].s64 = ctx.r[10].s64 + 16944;
	// 82678308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267830C: 390B5B68  addi r8, r11, 0x5b68
	ctx.r[8].s64 = ctx.r[11].s64 + 23400;
	// 82678310: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82678314: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 82678318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267831C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678320: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82678324: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678328: 386A4FE0  addi r3, r10, 0x4fe0
	ctx.r[3].s64 = ctx.r[10].s64 + 20448;
	// 8267832C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82678330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267833C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267834C: 4BDEEAD5  bl 0x82466e20
	ctx.lr = 0x82678350;
	sub_82466E20(ctx, base);
	// 82678350: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267835C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678360 size=108
    let mut pc: u32 = 0x82678360;
    'dispatch: loop {
        match pc {
            0x82678360 => {
    //   block [0x82678360..0x826783CC)
	// 82678360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82678364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267836C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82678370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678374: 38EB5C28  addi r7, r11, 0x5c28
	ctx.r[7].s64 = ctx.r[11].s64 + 23592;
	// 82678378: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8267837C: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 82678380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678384: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678388: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267838C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678390: 386A5010  addi r3, r10, 0x5010
	ctx.r[3].s64 = ctx.r[10].s64 + 20496;
	// 82678394: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82678398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267839C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826783A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826783A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826783A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826783AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826783B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826783B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826783B8: 4BDEEA69  bl 0x82466e20
	ctx.lr = 0x826783BC;
	sub_82466E20(ctx, base);
	// 826783BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826783C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826783C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826783C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826783D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826783D0 size=100
    let mut pc: u32 = 0x826783D0;
    'dispatch: loop {
        match pc {
            0x826783D0 => {
    //   block [0x826783D0..0x82678434)
	// 826783D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826783D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826783D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826783DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826783E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826783E4: 38AA37E0  addi r5, r10, 0x37e0
	ctx.r[5].s64 = ctx.r[10].s64 + 14304;
	// 826783E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826783EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826783F0: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 826783F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826783F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826783FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678404: 386A5040  addi r3, r10, 0x5040
	ctx.r[3].s64 = ctx.r[10].s64 + 20544;
	// 82678408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267840C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678410: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82678414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678418: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267841C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678420: 4BDEEA01  bl 0x82466e20
	ctx.lr = 0x82678424;
	sub_82466E20(ctx, base);
	// 82678424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267842C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678438 size=112
    let mut pc: u32 = 0x82678438;
    'dispatch: loop {
        match pc {
            0x82678438 => {
    //   block [0x82678438..0x826784A8)
	// 82678438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267843C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678444: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678448: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267844C: 38AA37E0  addi r5, r10, 0x37e0
	ctx.r[5].s64 = ctx.r[10].s64 + 14304;
	// 82678450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678454: 390B5D48  addi r8, r11, 0x5d48
	ctx.r[8].s64 = ctx.r[11].s64 + 23880;
	// 82678458: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267845C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 82678460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678464: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267846C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678470: 386A5070  addi r3, r10, 0x5070
	ctx.r[3].s64 = ctx.r[10].s64 + 20592;
	// 82678474: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82678478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267847C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678484: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82678488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267848C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678494: 4BDEE98D  bl 0x82466e20
	ctx.lr = 0x82678498;
	sub_82466E20(ctx, base);
	// 82678498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267849C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826784A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826784A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826784A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826784A8 size=112
    let mut pc: u32 = 0x826784A8;
    'dispatch: loop {
        match pc {
            0x826784A8 => {
    //   block [0x826784A8..0x82678518)
	// 826784A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826784AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826784B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826784B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826784B8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826784BC: 38AA37E0  addi r5, r10, 0x37e0
	ctx.r[5].s64 = ctx.r[10].s64 + 14304;
	// 826784C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826784C4: 390B5D60  addi r8, r11, 0x5d60
	ctx.r[8].s64 = ctx.r[11].s64 + 23904;
	// 826784C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826784CC: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 826784D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826784D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826784D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826784DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826784E0: 386A50A0  addi r3, r10, 0x50a0
	ctx.r[3].s64 = ctx.r[10].s64 + 20640;
	// 826784E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826784E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826784EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826784F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826784F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826784F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826784FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678504: 4BDEE91D  bl 0x82466e20
	ctx.lr = 0x82678508;
	sub_82466E20(ctx, base);
	// 82678508: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267850C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678518 size=108
    let mut pc: u32 = 0x82678518;
    'dispatch: loop {
        match pc {
            0x82678518 => {
    //   block [0x82678518..0x82678584)
	// 82678518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267851C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678524: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82678528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267852C: 38EB5D90  addi r7, r11, 0x5d90
	ctx.r[7].s64 = ctx.r[11].s64 + 23952;
	// 82678530: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82678534: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 82678538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267853C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678540: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82678544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678548: 386A50D0  addi r3, r10, 0x50d0
	ctx.r[3].s64 = ctx.r[10].s64 + 20688;
	// 8267854C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82678550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678554: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267855C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267856C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82678570: 4BDEE8B1  bl 0x82466e20
	ctx.lr = 0x82678574;
	sub_82466E20(ctx, base);
	// 82678574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267857C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678588 size=112
    let mut pc: u32 = 0x82678588;
    'dispatch: loop {
        match pc {
            0x82678588 => {
    //   block [0x82678588..0x826785F8)
	// 82678588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267858C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678594: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678598: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267859C: 38AA37E0  addi r5, r10, 0x37e0
	ctx.r[5].s64 = ctx.r[10].s64 + 14304;
	// 826785A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826785A4: 390B5DC0  addi r8, r11, 0x5dc0
	ctx.r[8].s64 = ctx.r[11].s64 + 24000;
	// 826785A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826785AC: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 826785B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826785B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826785B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826785BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826785C0: 386A5100  addi r3, r10, 0x5100
	ctx.r[3].s64 = ctx.r[10].s64 + 20736;
	// 826785C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826785C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826785CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826785D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826785D4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826785D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826785DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826785E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826785E4: 4BDEE83D  bl 0x82466e20
	ctx.lr = 0x826785E8;
	sub_82466E20(ctx, base);
	// 826785E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826785EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826785F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826785F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826785F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826785F8 size=112
    let mut pc: u32 = 0x826785F8;
    'dispatch: loop {
        match pc {
            0x826785F8 => {
    //   block [0x826785F8..0x82678668)
	// 826785F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826785FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678604: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678608: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267860C: 38AA4260  addi r5, r10, 0x4260
	ctx.r[5].s64 = ctx.r[10].s64 + 16992;
	// 82678610: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678614: 390B5DD8  addi r8, r11, 0x5dd8
	ctx.r[8].s64 = ctx.r[11].s64 + 24024;
	// 82678618: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8267861C: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 82678620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678624: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678628: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267862C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678630: 386A5130  addi r3, r10, 0x5130
	ctx.r[3].s64 = ctx.r[10].s64 + 20784;
	// 82678634: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82678638: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267863C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267864C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678654: 4BDEE7CD  bl 0x82466e20
	ctx.lr = 0x82678658;
	sub_82466E20(ctx, base);
	// 82678658: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267865C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678668 size=112
    let mut pc: u32 = 0x82678668;
    'dispatch: loop {
        match pc {
            0x82678668 => {
    //   block [0x82678668..0x826786D8)
	// 82678668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267866C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678674: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678678: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267867C: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82678680: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678684: 390B5E68  addi r8, r11, 0x5e68
	ctx.r[8].s64 = ctx.r[11].s64 + 24168;
	// 82678688: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267868C: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 82678690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678694: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678698: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267869C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826786A0: 386A5160  addi r3, r10, 0x5160
	ctx.r[3].s64 = ctx.r[10].s64 + 20832;
	// 826786A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826786A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826786AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826786B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826786B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826786B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826786BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826786C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826786C4: 4BDEE75D  bl 0x82466e20
	ctx.lr = 0x826786C8;
	sub_82466E20(ctx, base);
	// 826786C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826786CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826786D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826786D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826786D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826786D8 size=112
    let mut pc: u32 = 0x826786D8;
    'dispatch: loop {
        match pc {
            0x826786D8 => {
    //   block [0x826786D8..0x82678748)
	// 826786D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826786DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826786E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826786E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826786E8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826786EC: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 826786F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826786F4: 390B5E98  addi r8, r11, 0x5e98
	ctx.r[8].s64 = ctx.r[11].s64 + 24216;
	// 826786F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826786FC: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 82678700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678704: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267870C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678710: 386A5190  addi r3, r10, 0x5190
	ctx.r[3].s64 = ctx.r[10].s64 + 20880;
	// 82678714: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82678718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267871C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267872C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678734: 4BDEE6ED  bl 0x82466e20
	ctx.lr = 0x82678738;
	sub_82466E20(ctx, base);
	// 82678738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267873C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678748 size=112
    let mut pc: u32 = 0x82678748;
    'dispatch: loop {
        match pc {
            0x82678748 => {
    //   block [0x82678748..0x826787B8)
	// 82678748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267874C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678754: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678758: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267875C: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82678760: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678764: 390B5EC8  addi r8, r11, 0x5ec8
	ctx.r[8].s64 = ctx.r[11].s64 + 24264;
	// 82678768: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267876C: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 82678770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678774: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678778: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267877C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678780: 386A51C0  addi r3, r10, 0x51c0
	ctx.r[3].s64 = ctx.r[10].s64 + 20928;
	// 82678784: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82678788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267878C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267879C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826787A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826787A4: 4BDEE67D  bl 0x82466e20
	ctx.lr = 0x826787A8;
	sub_82466E20(ctx, base);
	// 826787A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826787AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826787B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826787B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826787B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826787B8 size=112
    let mut pc: u32 = 0x826787B8;
    'dispatch: loop {
        match pc {
            0x826787B8 => {
    //   block [0x826787B8..0x82678828)
	// 826787B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826787BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826787C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826787C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826787C8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826787CC: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 826787D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826787D4: 390B5EE0  addi r8, r11, 0x5ee0
	ctx.r[8].s64 = ctx.r[11].s64 + 24288;
	// 826787D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826787DC: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 826787E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826787E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826787E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826787EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826787F0: 386A51F0  addi r3, r10, 0x51f0
	ctx.r[3].s64 = ctx.r[10].s64 + 20976;
	// 826787F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826787F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826787FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267880C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678814: 4BDEE60D  bl 0x82466e20
	ctx.lr = 0x82678818;
	sub_82466E20(ctx, base);
	// 82678818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267881C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678828 size=108
    let mut pc: u32 = 0x82678828;
    'dispatch: loop {
        match pc {
            0x82678828 => {
    //   block [0x82678828..0x82678894)
	// 82678828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267882C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678834: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82678838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267883C: 38EB5EF8  addi r7, r11, 0x5ef8
	ctx.r[7].s64 = ctx.r[11].s64 + 24312;
	// 82678840: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82678844: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 82678848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267884C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82678854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678858: 386A5220  addi r3, r10, 0x5220
	ctx.r[3].s64 = ctx.r[10].s64 + 21024;
	// 8267885C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82678860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267886C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267887C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82678880: 4BDEE5A1  bl 0x82466e20
	ctx.lr = 0x82678884;
	sub_82466E20(ctx, base);
	// 82678884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267888C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678898 size=112
    let mut pc: u32 = 0x82678898;
    'dispatch: loop {
        match pc {
            0x82678898 => {
    //   block [0x82678898..0x82678908)
	// 82678898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267889C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826788A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826788A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826788A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826788AC: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 826788B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826788B4: 390B5F28  addi r8, r11, 0x5f28
	ctx.r[8].s64 = ctx.r[11].s64 + 24360;
	// 826788B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826788BC: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 826788C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826788C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826788C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826788CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826788D0: 386A5250  addi r3, r10, 0x5250
	ctx.r[3].s64 = ctx.r[10].s64 + 21072;
	// 826788D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826788D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826788DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826788E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826788E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826788E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826788EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826788F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826788F4: 4BDEE52D  bl 0x82466e20
	ctx.lr = 0x826788F8;
	sub_82466E20(ctx, base);
	// 826788F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826788FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678908 size=108
    let mut pc: u32 = 0x82678908;
    'dispatch: loop {
        match pc {
            0x82678908 => {
    //   block [0x82678908..0x82678974)
	// 82678908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267890C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678914: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82678918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267891C: 38EB5F40  addi r7, r11, 0x5f40
	ctx.r[7].s64 = ctx.r[11].s64 + 24384;
	// 82678920: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82678924: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 82678928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267892C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678930: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82678934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678938: 386A5280  addi r3, r10, 0x5280
	ctx.r[3].s64 = ctx.r[10].s64 + 21120;
	// 8267893C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82678940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267894C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267895C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82678960: 4BDEE4C1  bl 0x82466e20
	ctx.lr = 0x82678964;
	sub_82466E20(ctx, base);
	// 82678964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267896C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678978 size=112
    let mut pc: u32 = 0x82678978;
    'dispatch: loop {
        match pc {
            0x82678978 => {
    //   block [0x82678978..0x826789E8)
	// 82678978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267897C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678984: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678988: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267898C: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82678990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678994: 390B6018  addi r8, r11, 0x6018
	ctx.r[8].s64 = ctx.r[11].s64 + 24600;
	// 82678998: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 8267899C: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 826789A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826789A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826789A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826789AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826789B0: 386A52B0  addi r3, r10, 0x52b0
	ctx.r[3].s64 = ctx.r[10].s64 + 21168;
	// 826789B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826789B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826789BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826789C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826789C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826789C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826789CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826789D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826789D4: 4BDEE44D  bl 0x82466e20
	ctx.lr = 0x826789D8;
	sub_82466E20(ctx, base);
	// 826789D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826789DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826789E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826789E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826789E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826789E8 size=108
    let mut pc: u32 = 0x826789E8;
    'dispatch: loop {
        match pc {
            0x826789E8 => {
    //   block [0x826789E8..0x82678A54)
	// 826789E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826789EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826789F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826789F4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826789F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826789FC: 38EB61C8  addi r7, r11, 0x61c8
	ctx.r[7].s64 = ctx.r[11].s64 + 25032;
	// 82678A00: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82678A04: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 82678A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678A0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678A10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82678A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678A18: 386A52E0  addi r3, r10, 0x52e0
	ctx.r[3].s64 = ctx.r[10].s64 + 21216;
	// 82678A1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82678A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678A3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82678A40: 4BDEE3E1  bl 0x82466e20
	ctx.lr = 0x82678A44;
	sub_82466E20(ctx, base);
	// 82678A44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678A58 size=112
    let mut pc: u32 = 0x82678A58;
    'dispatch: loop {
        match pc {
            0x82678A58 => {
    //   block [0x82678A58..0x82678AC8)
	// 82678A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82678A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678A64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678A68: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82678A6C: 38AA4260  addi r5, r10, 0x4260
	ctx.r[5].s64 = ctx.r[10].s64 + 16992;
	// 82678A70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678A74: 390B6360  addi r8, r11, 0x6360
	ctx.r[8].s64 = ctx.r[11].s64 + 25440;
	// 82678A78: 3920001A  li r9, 0x1a
	ctx.r[9].s64 = 26;
	// 82678A7C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 82678A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678A84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678A88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82678A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678A90: 386A5310  addi r3, r10, 0x5310
	ctx.r[3].s64 = ctx.r[10].s64 + 21264;
	// 82678A94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82678A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678AAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678AB4: 4BDEE36D  bl 0x82466e20
	ctx.lr = 0x82678AB8;
	sub_82466E20(ctx, base);
	// 82678AB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678AC8 size=100
    let mut pc: u32 = 0x82678AC8;
    'dispatch: loop {
        match pc {
            0x82678AC8 => {
    //   block [0x82678AC8..0x82678B2C)
	// 82678AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82678ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678AD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678ADC: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82678AE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678AE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678AE8: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 82678AEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678AFC: 386A5340  addi r3, r10, 0x5340
	ctx.r[3].s64 = ctx.r[10].s64 + 21312;
	// 82678B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678B04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678B08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82678B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678B10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82678B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678B18: 4BDEE309  bl 0x82466e20
	ctx.lr = 0x82678B1C;
	sub_82466E20(ctx, base);
	// 82678B1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678B30 size=112
    let mut pc: u32 = 0x82678B30;
    'dispatch: loop {
        match pc {
            0x82678B30 => {
    //   block [0x82678B30..0x82678BA0)
	// 82678B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82678B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678B3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678B40: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82678B44: 38AA5340  addi r5, r10, 0x5340
	ctx.r[5].s64 = ctx.r[10].s64 + 21312;
	// 82678B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678B4C: 390B65D0  addi r8, r11, 0x65d0
	ctx.r[8].s64 = ctx.r[11].s64 + 26064;
	// 82678B50: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82678B54: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 82678B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678B5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678B60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82678B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678B68: 386A5370  addi r3, r10, 0x5370
	ctx.r[3].s64 = ctx.r[10].s64 + 21360;
	// 82678B6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82678B70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678B74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678B7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678B8C: 4BDEE295  bl 0x82466e20
	ctx.lr = 0x82678B90;
	sub_82466E20(ctx, base);
	// 82678B90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678BA0 size=100
    let mut pc: u32 = 0x82678BA0;
    'dispatch: loop {
        match pc {
            0x82678BA0 => {
    //   block [0x82678BA0..0x82678C04)
	// 82678BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82678BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678BAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678BB4: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82678BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678BC0: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 82678BC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678BD4: 386A53A0  addi r3, r10, 0x53a0
	ctx.r[3].s64 = ctx.r[10].s64 + 21408;
	// 82678BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678BDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678BE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82678BE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678BE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82678BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678BF0: 4BDEE231  bl 0x82466e20
	ctx.lr = 0x82678BF4;
	sub_82466E20(ctx, base);
	// 82678BF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678BF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678BFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678C08 size=108
    let mut pc: u32 = 0x82678C08;
    'dispatch: loop {
        match pc {
            0x82678C08 => {
    //   block [0x82678C08..0x82678C74)
	// 82678C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82678C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678C14: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82678C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678C1C: 38EB6648  addi r7, r11, 0x6648
	ctx.r[7].s64 = ctx.r[11].s64 + 26184;
	// 82678C20: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82678C24: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 82678C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678C2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678C30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82678C34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678C38: 386A53D0  addi r3, r10, 0x53d0
	ctx.r[3].s64 = ctx.r[10].s64 + 21456;
	// 82678C3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82678C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678C5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82678C60: 4BDEE1C1  bl 0x82466e20
	ctx.lr = 0x82678C64;
	sub_82466E20(ctx, base);
	// 82678C64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678C78 size=112
    let mut pc: u32 = 0x82678C78;
    'dispatch: loop {
        match pc {
            0x82678C78 => {
    //   block [0x82678C78..0x82678CE8)
	// 82678C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82678C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678C84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678C88: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82678C8C: 38AA53A0  addi r5, r10, 0x53a0
	ctx.r[5].s64 = ctx.r[10].s64 + 21408;
	// 82678C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678C94: 390B6690  addi r8, r11, 0x6690
	ctx.r[8].s64 = ctx.r[11].s64 + 26256;
	// 82678C98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82678C9C: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 82678CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678CA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678CA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82678CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678CB0: 386A5400  addi r3, r10, 0x5400
	ctx.r[3].s64 = ctx.r[10].s64 + 21504;
	// 82678CB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82678CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678CD4: 4BDEE14D  bl 0x82466e20
	ctx.lr = 0x82678CD8;
	sub_82466E20(ctx, base);
	// 82678CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678CE8 size=100
    let mut pc: u32 = 0x82678CE8;
    'dispatch: loop {
        match pc {
            0x82678CE8 => {
    //   block [0x82678CE8..0x82678D4C)
	// 82678CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82678CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678CF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678CFC: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82678D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678D04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678D08: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 82678D0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678D1C: 386A5430  addi r3, r10, 0x5430
	ctx.r[3].s64 = ctx.r[10].s64 + 21552;
	// 82678D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678D24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678D28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82678D2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678D30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82678D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678D38: 4BDEE0E9  bl 0x82466e20
	ctx.lr = 0x82678D3C;
	sub_82466E20(ctx, base);
	// 82678D3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678D50 size=100
    let mut pc: u32 = 0x82678D50;
    'dispatch: loop {
        match pc {
            0x82678D50 => {
    //   block [0x82678D50..0x82678DB4)
	// 82678D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82678D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678D5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678D64: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82678D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678D70: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 82678D74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678D84: 386A5460  addi r3, r10, 0x5460
	ctx.r[3].s64 = ctx.r[10].s64 + 21600;
	// 82678D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678D8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678D90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82678D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678D98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82678D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678DA0: 4BDEE081  bl 0x82466e20
	ctx.lr = 0x82678DA4;
	sub_82466E20(ctx, base);
	// 82678DA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678DA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678DAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678DB8 size=112
    let mut pc: u32 = 0x82678DB8;
    'dispatch: loop {
        match pc {
            0x82678DB8 => {
    //   block [0x82678DB8..0x82678E28)
	// 82678DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82678DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678DC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678DC8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82678DCC: 38AA5430  addi r5, r10, 0x5430
	ctx.r[5].s64 = ctx.r[10].s64 + 21552;
	// 82678DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678DD4: 390B66C0  addi r8, r11, 0x66c0
	ctx.r[8].s64 = ctx.r[11].s64 + 26304;
	// 82678DD8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82678DDC: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 82678DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678DE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678DE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82678DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678DF0: 386A5490  addi r3, r10, 0x5490
	ctx.r[3].s64 = ctx.r[10].s64 + 21648;
	// 82678DF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82678DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678E14: 4BDEE00D  bl 0x82466e20
	ctx.lr = 0x82678E18;
	sub_82466E20(ctx, base);
	// 82678E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678E28 size=112
    let mut pc: u32 = 0x82678E28;
    'dispatch: loop {
        match pc {
            0x82678E28 => {
    //   block [0x82678E28..0x82678E98)
	// 82678E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82678E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678E34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678E38: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82678E3C: 38AA5460  addi r5, r10, 0x5460
	ctx.r[5].s64 = ctx.r[10].s64 + 21600;
	// 82678E40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678E44: 390B6720  addi r8, r11, 0x6720
	ctx.r[8].s64 = ctx.r[11].s64 + 26400;
	// 82678E48: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82678E4C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 82678E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678E54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678E58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82678E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678E60: 386A54C0  addi r3, r10, 0x54c0
	ctx.r[3].s64 = ctx.r[10].s64 + 21696;
	// 82678E64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82678E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678E70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678E78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678E7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678E84: 4BDEDF9D  bl 0x82466e20
	ctx.lr = 0x82678E88;
	sub_82466E20(ctx, base);
	// 82678E88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678E98 size=100
    let mut pc: u32 = 0x82678E98;
    'dispatch: loop {
        match pc {
            0x82678E98 => {
    //   block [0x82678E98..0x82678EFC)
	// 82678E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82678E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678EA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678EAC: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82678EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678EB8: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 82678EBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678ECC: 386A54F0  addi r3, r10, 0x54f0
	ctx.r[3].s64 = ctx.r[10].s64 + 21744;
	// 82678ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678ED4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678ED8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82678EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678EE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82678EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678EE8: 4BDEDF39  bl 0x82466e20
	ctx.lr = 0x82678EEC;
	sub_82466E20(ctx, base);
	// 82678EEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678F00 size=112
    let mut pc: u32 = 0x82678F00;
    'dispatch: loop {
        match pc {
            0x82678F00 => {
    //   block [0x82678F00..0x82678F70)
	// 82678F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82678F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678F0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678F10: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82678F14: 38AA54F0  addi r5, r10, 0x54f0
	ctx.r[5].s64 = ctx.r[10].s64 + 21744;
	// 82678F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678F1C: 390B6780  addi r8, r11, 0x6780
	ctx.r[8].s64 = ctx.r[11].s64 + 26496;
	// 82678F20: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82678F24: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 82678F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678F2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678F30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82678F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678F38: 386A5520  addi r3, r10, 0x5520
	ctx.r[3].s64 = ctx.r[10].s64 + 21792;
	// 82678F3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82678F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678F5C: 4BDEDEC5  bl 0x82466e20
	ctx.lr = 0x82678F60;
	sub_82466E20(ctx, base);
	// 82678F60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678F70 size=100
    let mut pc: u32 = 0x82678F70;
    'dispatch: loop {
        match pc {
            0x82678F70 => {
    //   block [0x82678F70..0x82678FD4)
	// 82678F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82678F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678F7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82678F84: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82678F88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82678F90: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 82678F94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678F98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82678F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82678FA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82678FA4: 386A5550  addi r3, r10, 0x5550
	ctx.r[3].s64 = ctx.r[10].s64 + 21840;
	// 82678FA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82678FAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82678FB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82678FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82678FB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82678FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82678FC0: 4BDEDE61  bl 0x82466e20
	ctx.lr = 0x82678FC4;
	sub_82466E20(ctx, base);
	// 82678FC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82678FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82678FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82678FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82678FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82678FD8 size=112
    let mut pc: u32 = 0x82678FD8;
    'dispatch: loop {
        match pc {
            0x82678FD8 => {
    //   block [0x82678FD8..0x82679048)
	// 82678FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82678FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82678FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82678FE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82678FE8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82678FEC: 38AA5550  addi r5, r10, 0x5550
	ctx.r[5].s64 = ctx.r[10].s64 + 21840;
	// 82678FF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82678FF4: 390B6870  addi r8, r11, 0x6870
	ctx.r[8].s64 = ctx.r[11].s64 + 26736;
	// 82678FF8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82678FFC: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 82679000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679004: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267900C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679010: 386A5580  addi r3, r10, 0x5580
	ctx.r[3].s64 = ctx.r[10].s64 + 21888;
	// 82679014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82679018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267901C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267902C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679034: 4BDEDDED  bl 0x82466e20
	ctx.lr = 0x82679038;
	sub_82466E20(ctx, base);
	// 82679038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267903C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679048 size=108
    let mut pc: u32 = 0x82679048;
    'dispatch: loop {
        match pc {
            0x82679048 => {
    //   block [0x82679048..0x826790B4)
	// 82679048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267904C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679054: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267905C: 38EB68B8  addi r7, r11, 0x68b8
	ctx.r[7].s64 = ctx.r[11].s64 + 26808;
	// 82679060: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82679064: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 82679068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267906C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679070: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679078: 386A55B0  addi r3, r10, 0x55b0
	ctx.r[3].s64 = ctx.r[10].s64 + 21936;
	// 8267907C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267908C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267909C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826790A0: 4BDEDD81  bl 0x82466e20
	ctx.lr = 0x826790A4;
	sub_82466E20(ctx, base);
	// 826790A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826790A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826790AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826790B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826790B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826790B8 size=112
    let mut pc: u32 = 0x826790B8;
    'dispatch: loop {
        match pc {
            0x826790B8 => {
    //   block [0x826790B8..0x82679128)
	// 826790B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826790BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826790C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826790C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826790C8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826790CC: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 826790D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826790D4: 390B6900  addi r8, r11, 0x6900
	ctx.r[8].s64 = ctx.r[11].s64 + 26880;
	// 826790D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826790DC: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 826790E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826790E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826790E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826790EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826790F0: 386A55E0  addi r3, r10, 0x55e0
	ctx.r[3].s64 = ctx.r[10].s64 + 21984;
	// 826790F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826790F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826790FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267910C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679114: 4BDEDD0D  bl 0x82466e20
	ctx.lr = 0x82679118;
	sub_82466E20(ctx, base);
	// 82679118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267911C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679128 size=108
    let mut pc: u32 = 0x82679128;
    'dispatch: loop {
        match pc {
            0x82679128 => {
    //   block [0x82679128..0x82679194)
	// 82679128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267912C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679134: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267913C: 38EB6918  addi r7, r11, 0x6918
	ctx.r[7].s64 = ctx.r[11].s64 + 26904;
	// 82679140: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82679144: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 82679148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267914C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679150: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679158: 386A5610  addi r3, r10, 0x5610
	ctx.r[3].s64 = ctx.r[10].s64 + 22032;
	// 8267915C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267916C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267917C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679180: 4BDEDCA1  bl 0x82466e20
	ctx.lr = 0x82679184;
	sub_82466E20(ctx, base);
	// 82679184: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267918C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679198 size=112
    let mut pc: u32 = 0x82679198;
    'dispatch: loop {
        match pc {
            0x82679198 => {
    //   block [0x82679198..0x82679208)
	// 82679198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267919C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826791A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826791A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826791A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826791AC: 38AA55E0  addi r5, r10, 0x55e0
	ctx.r[5].s64 = ctx.r[10].s64 + 21984;
	// 826791B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826791B4: 390B6960  addi r8, r11, 0x6960
	ctx.r[8].s64 = ctx.r[11].s64 + 26976;
	// 826791B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826791BC: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 826791C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826791C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826791C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826791CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826791D0: 386A5640  addi r3, r10, 0x5640
	ctx.r[3].s64 = ctx.r[10].s64 + 22080;
	// 826791D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826791D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826791DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826791E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826791E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826791E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826791EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826791F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826791F4: 4BDEDC2D  bl 0x82466e20
	ctx.lr = 0x826791F8;
	sub_82466E20(ctx, base);
	// 826791F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826791FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679208 size=100
    let mut pc: u32 = 0x82679208;
    'dispatch: loop {
        match pc {
            0x82679208 => {
    //   block [0x82679208..0x8267926C)
	// 82679208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267920C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679214: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267921C: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82679220: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679228: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 8267922C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267923C: 386A5670  addi r3, r10, 0x5670
	ctx.r[3].s64 = ctx.r[10].s64 + 22128;
	// 82679240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679244: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679248: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267924C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679250: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82679254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679258: 4BDEDBC9  bl 0x82466e20
	ctx.lr = 0x8267925C;
	sub_82466E20(ctx, base);
	// 8267925C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679270 size=112
    let mut pc: u32 = 0x82679270;
    'dispatch: loop {
        match pc {
            0x82679270 => {
    //   block [0x82679270..0x826792E0)
	// 82679270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267927C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679280: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679284: 38AA5670  addi r5, r10, 0x5670
	ctx.r[5].s64 = ctx.r[10].s64 + 22128;
	// 82679288: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267928C: 390B6978  addi r8, r11, 0x6978
	ctx.r[8].s64 = ctx.r[11].s64 + 27000;
	// 82679290: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82679294: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 82679298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267929C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826792A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826792A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826792A8: 386A56A0  addi r3, r10, 0x56a0
	ctx.r[3].s64 = ctx.r[10].s64 + 22176;
	// 826792AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826792B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826792B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826792B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826792BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826792C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826792C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826792C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826792CC: 4BDEDB55  bl 0x82466e20
	ctx.lr = 0x826792D0;
	sub_82466E20(ctx, base);
	// 826792D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826792D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826792D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826792DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826792E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826792E0 size=108
    let mut pc: u32 = 0x826792E0;
    'dispatch: loop {
        match pc {
            0x826792E0 => {
    //   block [0x826792E0..0x8267934C)
	// 826792E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826792E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826792E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826792EC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826792F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826792F4: 38EB6A20  addi r7, r11, 0x6a20
	ctx.r[7].s64 = ctx.r[11].s64 + 27168;
	// 826792F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826792FC: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 82679300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679304: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679308: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267930C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679310: 386A56D0  addi r3, r10, 0x56d0
	ctx.r[3].s64 = ctx.r[10].s64 + 22224;
	// 82679314: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267931C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679324: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267932C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679334: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679338: 4BDEDAE9  bl 0x82466e20
	ctx.lr = 0x8267933C;
	sub_82466E20(ctx, base);
	// 8267933C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679350 size=112
    let mut pc: u32 = 0x82679350;
    'dispatch: loop {
        match pc {
            0x82679350 => {
    //   block [0x82679350..0x826793C0)
	// 82679350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267935C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679360: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679364: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82679368: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267936C: 390B6A50  addi r8, r11, 0x6a50
	ctx.r[8].s64 = ctx.r[11].s64 + 27216;
	// 82679370: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82679374: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 82679378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267937C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679380: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82679384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679388: 386A5700  addi r3, r10, 0x5700
	ctx.r[3].s64 = ctx.r[10].s64 + 22272;
	// 8267938C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82679390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267939C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826793A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826793A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826793A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826793AC: 4BDEDA75  bl 0x82466e20
	ctx.lr = 0x826793B0;
	sub_82466E20(ctx, base);
	// 826793B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826793B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826793B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826793BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826793C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826793C0 size=112
    let mut pc: u32 = 0x826793C0;
    'dispatch: loop {
        match pc {
            0x826793C0 => {
    //   block [0x826793C0..0x82679430)
	// 826793C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826793C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826793C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826793CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826793D0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826793D4: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 826793D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826793DC: 390B6A98  addi r8, r11, 0x6a98
	ctx.r[8].s64 = ctx.r[11].s64 + 27288;
	// 826793E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826793E4: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 826793E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826793EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826793F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826793F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826793F8: 386A5730  addi r3, r10, 0x5730
	ctx.r[3].s64 = ctx.r[10].s64 + 22320;
	// 826793FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82679400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267940C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267941C: 4BDEDA05  bl 0x82466e20
	ctx.lr = 0x82679420;
	sub_82466E20(ctx, base);
	// 82679420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267942C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679430 size=100
    let mut pc: u32 = 0x82679430;
    'dispatch: loop {
        match pc {
            0x82679430 => {
    //   block [0x82679430..0x82679494)
	// 82679430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267943C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679444: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82679448: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267944C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679450: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 82679454: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267945C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679464: 386A5760  addi r3, r10, 0x5760
	ctx.r[3].s64 = ctx.r[10].s64 + 22368;
	// 82679468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267946C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679470: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82679474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679478: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267947C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679480: 4BDED9A1  bl 0x82466e20
	ctx.lr = 0x82679484;
	sub_82466E20(ctx, base);
	// 82679484: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267948C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679498 size=112
    let mut pc: u32 = 0x82679498;
    'dispatch: loop {
        match pc {
            0x82679498 => {
    //   block [0x82679498..0x82679508)
	// 82679498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267949C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826794A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826794A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826794A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826794AC: 38AA5760  addi r5, r10, 0x5760
	ctx.r[5].s64 = ctx.r[10].s64 + 22368;
	// 826794B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826794B4: 390B6AE0  addi r8, r11, 0x6ae0
	ctx.r[8].s64 = ctx.r[11].s64 + 27360;
	// 826794B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826794BC: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 826794C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826794C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826794C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826794CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826794D0: 386A5790  addi r3, r10, 0x5790
	ctx.r[3].s64 = ctx.r[10].s64 + 22416;
	// 826794D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826794D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826794DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826794E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826794E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826794E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826794EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826794F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826794F4: 4BDED92D  bl 0x82466e20
	ctx.lr = 0x826794F8;
	sub_82466E20(ctx, base);
	// 826794F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826794FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679508 size=112
    let mut pc: u32 = 0x82679508;
    'dispatch: loop {
        match pc {
            0x82679508 => {
    //   block [0x82679508..0x82679578)
	// 82679508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267950C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679514: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679518: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267951C: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82679520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679524: 390B6B28  addi r8, r11, 0x6b28
	ctx.r[8].s64 = ctx.r[11].s64 + 27432;
	// 82679528: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267952C: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 82679530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679534: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679538: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267953C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679540: 386A57C0  addi r3, r10, 0x57c0
	ctx.r[3].s64 = ctx.r[10].s64 + 22464;
	// 82679544: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82679548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267954C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679554: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267955C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679564: 4BDED8BD  bl 0x82466e20
	ctx.lr = 0x82679568;
	sub_82466E20(ctx, base);
	// 82679568: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267956C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679578 size=112
    let mut pc: u32 = 0x82679578;
    'dispatch: loop {
        match pc {
            0x82679578 => {
    //   block [0x82679578..0x826795E8)
	// 82679578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267957C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679584: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679588: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267958C: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82679590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679594: 390B6B40  addi r8, r11, 0x6b40
	ctx.r[8].s64 = ctx.r[11].s64 + 27456;
	// 82679598: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267959C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 826795A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826795A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826795A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826795AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826795B0: 386A57F0  addi r3, r10, 0x57f0
	ctx.r[3].s64 = ctx.r[10].s64 + 22512;
	// 826795B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826795B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826795BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826795C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826795C4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826795C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826795CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826795D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826795D4: 4BDED84D  bl 0x82466e20
	ctx.lr = 0x826795D8;
	sub_82466E20(ctx, base);
	// 826795D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826795DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826795E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826795E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826795E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826795E8 size=112
    let mut pc: u32 = 0x826795E8;
    'dispatch: loop {
        match pc {
            0x826795E8 => {
    //   block [0x826795E8..0x82679658)
	// 826795E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826795EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826795F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826795F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826795F8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826795FC: 38AA57C0  addi r5, r10, 0x57c0
	ctx.r[5].s64 = ctx.r[10].s64 + 22464;
	// 82679600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679604: 390B6B58  addi r8, r11, 0x6b58
	ctx.r[8].s64 = ctx.r[11].s64 + 27480;
	// 82679608: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267960C: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 82679610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679614: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679618: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267961C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679620: 386A5820  addi r3, r10, 0x5820
	ctx.r[3].s64 = ctx.r[10].s64 + 22560;
	// 82679624: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82679628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267962C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267963C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679644: 4BDED7DD  bl 0x82466e20
	ctx.lr = 0x82679648;
	sub_82466E20(ctx, base);
	// 82679648: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267964C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679658 size=72
    let mut pc: u32 = 0x82679658;
    'dispatch: loop {
        match pc {
            0x82679658 => {
    //   block [0x82679658..0x826796A0)
	// 82679658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267965C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679664: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82679668: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8267966C: 38CB2F88  addi r6, r11, 0x2f88
	ctx.r[6].s64 = ctx.r[11].s64 + 12168;
	// 82679670: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82679674: 388B3528  addi r4, r11, 0x3528
	ctx.r[4].s64 = ctx.r[11].s64 + 13608;
	// 82679678: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267967C: 386B5850  addi r3, r11, 0x5850
	ctx.r[3].s64 = ctx.r[11].s64 + 22608;
	// 82679680: 4BE02409  bl 0x8247ba88
	ctx.lr = 0x82679684;
	sub_8247BA88(ctx, base);
	// 82679684: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82679688: 386BCE28  addi r3, r11, -0x31d8
	ctx.r[3].s64 = ctx.r[11].s64 + -12760;
	// 8267968C: 4BEB94AD  bl 0x82532b38
	ctx.lr = 0x82679690;
	sub_82532B38(ctx, base);
	// 82679690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82679694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267969C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826796A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826796A0 size=108
    let mut pc: u32 = 0x826796A0;
    'dispatch: loop {
        match pc {
            0x826796A0 => {
    //   block [0x826796A0..0x8267970C)
	// 826796A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826796A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826796A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826796AC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826796B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826796B4: 38EB7238  addi r7, r11, 0x7238
	ctx.r[7].s64 = ctx.r[11].s64 + 29240;
	// 826796B8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826796BC: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 826796C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826796C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826796C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826796CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826796D0: 386A5868  addi r3, r10, 0x5868
	ctx.r[3].s64 = ctx.r[10].s64 + 22632;
	// 826796D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826796D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826796DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826796E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826796E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826796E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826796EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826796F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826796F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826796F8: 4BDED729  bl 0x82466e20
	ctx.lr = 0x826796FC;
	sub_82466E20(ctx, base);
	// 826796FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82679710 size=24
    let mut pc: u32 = 0x82679710;
    'dispatch: loop {
        match pc {
            0x82679710 => {
    //   block [0x82679710..0x82679728)
	// 82679710: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679714: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82679718: 394AE3D8  addi r10, r10, -0x1c28
	ctx.r[10].s64 = ctx.r[10].s64 + -7208;
	// 8267971C: 816B72B0  lwz r11, 0x72b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29360 as u32) ) } as u64;
	// 82679720: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82679724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679728 size=112
    let mut pc: u32 = 0x82679728;
    'dispatch: loop {
        match pc {
            0x82679728 => {
    //   block [0x82679728..0x82679798)
	// 82679728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267972C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679734: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82679738: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267973C: 392A3BE4  addi r9, r10, 0x3be4
	ctx.r[9].s64 = ctx.r[10].s64 + 15332;
	// 82679740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679744: 390BE3D8  addi r8, r11, -0x1c28
	ctx.r[8].s64 = ctx.r[11].s64 + -7208;
	// 82679748: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8267974C: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 82679750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679754: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267975C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679760: 386A5898  addi r3, r10, 0x5898
	ctx.r[3].s64 = ctx.r[10].s64 + 22680;
	// 82679764: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82679768: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267976C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267977C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679784: 4BDED69D  bl 0x82466e20
	ctx.lr = 0x82679788;
	sub_82466E20(ctx, base);
	// 82679788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267978C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679798 size=108
    let mut pc: u32 = 0x82679798;
    'dispatch: loop {
        match pc {
            0x82679798 => {
    //   block [0x82679798..0x82679804)
	// 82679798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267979C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826797A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826797A4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826797A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826797AC: 38EB72B4  addi r7, r11, 0x72b4
	ctx.r[7].s64 = ctx.r[11].s64 + 29364;
	// 826797B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826797B4: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 826797B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826797BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826797C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826797C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826797C8: 386A58C8  addi r3, r10, 0x58c8
	ctx.r[3].s64 = ctx.r[10].s64 + 22728;
	// 826797CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826797D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826797D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826797D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826797DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826797E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826797E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826797E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826797EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826797F0: 4BDED631  bl 0x82466e20
	ctx.lr = 0x826797F4;
	sub_82466E20(ctx, base);
	// 826797F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826797F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826797FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679808 size=108
    let mut pc: u32 = 0x82679808;
    'dispatch: loop {
        match pc {
            0x82679808 => {
    //   block [0x82679808..0x82679874)
	// 82679808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267980C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679814: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267981C: 38EB72E4  addi r7, r11, 0x72e4
	ctx.r[7].s64 = ctx.r[11].s64 + 29412;
	// 82679820: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82679824: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 82679828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267982C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679830: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679838: 386A58F8  addi r3, r10, 0x58f8
	ctx.r[3].s64 = ctx.r[10].s64 + 22776;
	// 8267983C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267984C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267985C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679860: 4BDED5C1  bl 0x82466e20
	ctx.lr = 0x82679864;
	sub_82466E20(ctx, base);
	// 82679864: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267986C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82679878 size=24
    let mut pc: u32 = 0x82679878;
    'dispatch: loop {
        match pc {
            0x82679878 => {
    //   block [0x82679878..0x82679890)
	// 82679878: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267987C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82679880: 394AE420  addi r10, r10, -0x1be0
	ctx.r[10].s64 = ctx.r[10].s64 + -7136;
	// 82679884: 816B7314  lwz r11, 0x7314(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29460 as u32) ) } as u64;
	// 82679888: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8267988C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679890 size=116
    let mut pc: u32 = 0x82679890;
    'dispatch: loop {
        match pc {
            0x82679890 => {
    //   block [0x82679890..0x82679904)
	// 82679890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267989C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826798A0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826798A4: 390BE420  addi r8, r11, -0x1be0
	ctx.r[8].s64 = ctx.r[11].s64 + -7136;
	// 826798A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826798AC: 392A3C18  addi r9, r10, 0x3c18
	ctx.r[9].s64 = ctx.r[10].s64 + 15384;
	// 826798B0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826798B4: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826798B8: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 826798BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826798C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826798C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826798C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826798CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826798D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826798D4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826798D8: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 826798DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826798E0: 386B5928  addi r3, r11, 0x5928
	ctx.r[3].s64 = ctx.r[11].s64 + 22824;
	// 826798E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826798E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826798EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826798F0: 4BDED531  bl 0x82466e20
	ctx.lr = 0x826798F4;
	sub_82466E20(ctx, base);
	// 826798F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826798F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826798FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


