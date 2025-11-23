pub fn sub_82665FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665FB0 size=108
    let mut pc: u32 = 0x82665FB0;
    'dispatch: loop {
        match pc {
            0x82665FB0 => {
    //   block [0x82665FB0..0x8266601C)
	// 82665FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665FBC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665FC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82665FC4: 38EB7A50  addi r7, r11, 0x7a50
	ctx.r[7].s64 = ctx.r[11].s64 + 31312;
	// 82665FC8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82665FCC: 388A0DDC  addi r4, r10, 0xddc
	ctx.r[4].s64 = ctx.r[10].s64 + 3548;
	// 82665FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665FD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665FD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82665FDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665FE0: 386AD3F0  addi r3, r10, -0x2c10
	ctx.r[3].s64 = ctx.r[10].s64 + -11280;
	// 82665FE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82665FE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665FEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665FF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665FF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665FFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666004: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82666008: 4BE00E19  bl 0x82466e20
	ctx.lr = 0x8266600C;
	sub_82466E20(ctx, base);
	// 8266600C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666020 size=112
    let mut pc: u32 = 0x82666020;
    'dispatch: loop {
        match pc {
            0x82666020 => {
    //   block [0x82666020..0x82666090)
	// 82666020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266602C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666030: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666034: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 82666038: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266603C: 390B7AB0  addi r8, r11, 0x7ab0
	ctx.r[8].s64 = ctx.r[11].s64 + 31408;
	// 82666040: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82666044: 388A3EF8  addi r4, r10, 0x3ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 16120;
	// 82666048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266604C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666050: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666058: 386AD420  addi r3, r10, -0x2be0
	ctx.r[3].s64 = ctx.r[10].s64 + -11232;
	// 8266605C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266606C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266607C: 4BE00DA5  bl 0x82466e20
	ctx.lr = 0x82666080;
	sub_82466E20(ctx, base);
	// 82666080: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266608C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666090 size=100
    let mut pc: u32 = 0x82666090;
    'dispatch: loop {
        match pc {
            0x82666090 => {
    //   block [0x82666090..0x826660F4)
	// 82666090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266609C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826660A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826660A4: 38AACD60  addi r5, r10, -0x32a0
	ctx.r[5].s64 = ctx.r[10].s64 + -12960;
	// 826660A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826660AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826660B0: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 826660B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826660B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826660BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826660C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826660C4: 386AD450  addi r3, r10, -0x2bb0
	ctx.r[3].s64 = ctx.r[10].s64 + -11184;
	// 826660C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826660CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826660D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826660D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826660D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826660DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826660E0: 4BE00D41  bl 0x82466e20
	ctx.lr = 0x826660E4;
	sub_82466E20(ctx, base);
	// 826660E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826660E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826660EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826660F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826660F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826660F8 size=112
    let mut pc: u32 = 0x826660F8;
    'dispatch: loop {
        match pc {
            0x826660F8 => {
    //   block [0x826660F8..0x82666168)
	// 826660F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826660FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666104: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666108: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266610C: 38AAD180  addi r5, r10, -0x2e80
	ctx.r[5].s64 = ctx.r[10].s64 + -11904;
	// 82666110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666114: 390B7B10  addi r8, r11, 0x7b10
	ctx.r[8].s64 = ctx.r[11].s64 + 31504;
	// 82666118: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266611C: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 82666120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666124: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666128: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266612C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666130: 386AD480  addi r3, r10, -0x2b80
	ctx.r[3].s64 = ctx.r[10].s64 + -11136;
	// 82666134: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266613C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266614C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666154: 4BE00CCD  bl 0x82466e20
	ctx.lr = 0x82666158;
	sub_82466E20(ctx, base);
	// 82666158: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266615C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666168 size=112
    let mut pc: u32 = 0x82666168;
    'dispatch: loop {
        match pc {
            0x82666168 => {
    //   block [0x82666168..0x826661D8)
	// 82666168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266616C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666174: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666178: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266617C: 38AAD180  addi r5, r10, -0x2e80
	ctx.r[5].s64 = ctx.r[10].s64 + -11904;
	// 82666180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666184: 390B7B58  addi r8, r11, 0x7b58
	ctx.r[8].s64 = ctx.r[11].s64 + 31576;
	// 82666188: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8266618C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 82666190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666194: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666198: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266619C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826661A0: 386AD4B0  addi r3, r10, -0x2b50
	ctx.r[3].s64 = ctx.r[10].s64 + -11088;
	// 826661A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826661A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826661AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826661B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826661B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826661B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826661BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826661C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826661C4: 4BE00C5D  bl 0x82466e20
	ctx.lr = 0x826661C8;
	sub_82466E20(ctx, base);
	// 826661C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826661CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826661D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826661D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826661D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826661D8 size=108
    let mut pc: u32 = 0x826661D8;
    'dispatch: loop {
        match pc {
            0x826661D8 => {
    //   block [0x826661D8..0x82666244)
	// 826661D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826661DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826661E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826661E4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826661E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826661EC: 38EB7C00  addi r7, r11, 0x7c00
	ctx.r[7].s64 = ctx.r[11].s64 + 31744;
	// 826661F0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826661F4: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 826661F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826661FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666200: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82666204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666208: 386AD4E0  addi r3, r10, -0x2b20
	ctx.r[3].s64 = ctx.r[10].s64 + -11040;
	// 8266620C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82666210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266621C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266622C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82666230: 4BE00BF1  bl 0x82466e20
	ctx.lr = 0x82666234;
	sub_82466E20(ctx, base);
	// 82666234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266623C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82666248 size=24
    let mut pc: u32 = 0x82666248;
    'dispatch: loop {
        match pc {
            0x82666248 => {
    //   block [0x82666248..0x82666260)
	// 82666248: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266624C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82666250: 394AC620  addi r10, r10, -0x39e0
	ctx.r[10].s64 = ctx.r[10].s64 + -14816;
	// 82666254: 816B7F18  lwz r11, 0x7f18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32536 as u32) ) } as u64;
	// 82666258: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8266625C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666260 size=116
    let mut pc: u32 = 0x82666260;
    'dispatch: loop {
        match pc {
            0x82666260 => {
    //   block [0x82666260..0x826662D4)
	// 82666260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266626C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82666270: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82666274: 390AC620  addi r8, r10, -0x39e0
	ctx.r[8].s64 = ctx.r[10].s64 + -14816;
	// 82666278: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266627C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82666280: 38AACEE0  addi r5, r10, -0x3120
	ctx.r[5].s64 = ctx.r[10].s64 + -12576;
	// 82666284: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666288: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266628C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666294: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 82666298: 396BF528  addi r11, r11, -0xad8
	ctx.r[11].s64 = ctx.r[11].s64 + -2776;
	// 8266629C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826662A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826662A4: 386AD510  addi r3, r10, -0x2af0
	ctx.r[3].s64 = ctx.r[10].s64 + -10992;
	// 826662A8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826662AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826662B0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826662B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826662B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826662BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826662C0: 4BE00B61  bl 0x82466e20
	ctx.lr = 0x826662C4;
	sub_82466E20(ctx, base);
	// 826662C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826662C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826662CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826662D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826662D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826662D8 size=112
    let mut pc: u32 = 0x826662D8;
    'dispatch: loop {
        match pc {
            0x826662D8 => {
    //   block [0x826662D8..0x82666348)
	// 826662D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826662DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826662E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826662E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826662E8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826662EC: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 826662F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826662F4: 390B7C60  addi r8, r11, 0x7c60
	ctx.r[8].s64 = ctx.r[11].s64 + 31840;
	// 826662F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826662FC: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 82666300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666304: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666308: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266630C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666310: 386AD540  addi r3, r10, -0x2ac0
	ctx.r[3].s64 = ctx.r[10].s64 + -10944;
	// 82666314: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266631C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266632C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666334: 4BE00AED  bl 0x82466e20
	ctx.lr = 0x82666338;
	sub_82466E20(ctx, base);
	// 82666338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266633C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666348 size=100
    let mut pc: u32 = 0x82666348;
    'dispatch: loop {
        match pc {
            0x82666348 => {
    //   block [0x82666348..0x826663AC)
	// 82666348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266634C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666354: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266635C: 38AACD60  addi r5, r10, -0x32a0
	ctx.r[5].s64 = ctx.r[10].s64 + -12960;
	// 82666360: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666368: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 8266636C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266637C: 386AD570  addi r3, r10, -0x2a90
	ctx.r[3].s64 = ctx.r[10].s64 + -10896;
	// 82666380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666384: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666388: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266638C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666390: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82666394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666398: 4BE00A89  bl 0x82466e20
	ctx.lr = 0x8266639C;
	sub_82466E20(ctx, base);
	// 8266639C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826663A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826663A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826663A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826663B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826663B0 size=112
    let mut pc: u32 = 0x826663B0;
    'dispatch: loop {
        match pc {
            0x826663B0 => {
    //   block [0x826663B0..0x82666420)
	// 826663B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826663B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826663B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826663BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826663C0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826663C4: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826663C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826663CC: 390B7C78  addi r8, r11, 0x7c78
	ctx.r[8].s64 = ctx.r[11].s64 + 31864;
	// 826663D0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826663D4: 388A3FAC  addi r4, r10, 0x3fac
	ctx.r[4].s64 = ctx.r[10].s64 + 16300;
	// 826663D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826663DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826663E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826663E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826663E8: 386AD5A0  addi r3, r10, -0x2a60
	ctx.r[3].s64 = ctx.r[10].s64 + -10848;
	// 826663EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826663F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826663F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826663F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826663FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266640C: 4BE00A15  bl 0x82466e20
	ctx.lr = 0x82666410;
	sub_82466E20(ctx, base);
	// 82666410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266641C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666420 size=112
    let mut pc: u32 = 0x82666420;
    'dispatch: loop {
        match pc {
            0x82666420 => {
    //   block [0x82666420..0x82666490)
	// 82666420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266642C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666430: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666434: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82666438: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266643C: 390B7D08  addi r8, r11, 0x7d08
	ctx.r[8].s64 = ctx.r[11].s64 + 32008;
	// 82666440: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82666444: 388A3FDC  addi r4, r10, 0x3fdc
	ctx.r[4].s64 = ctx.r[10].s64 + 16348;
	// 82666448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266644C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666458: 386AD5D0  addi r3, r10, -0x2a30
	ctx.r[3].s64 = ctx.r[10].s64 + -10800;
	// 8266645C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266646C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266647C: 4BE009A5  bl 0x82466e20
	ctx.lr = 0x82666480;
	sub_82466E20(ctx, base);
	// 82666480: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266648C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666490 size=112
    let mut pc: u32 = 0x82666490;
    'dispatch: loop {
        match pc {
            0x82666490 => {
    //   block [0x82666490..0x82666500)
	// 82666490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266649C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826664A0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826664A4: 38AAD120  addi r5, r10, -0x2ee0
	ctx.r[5].s64 = ctx.r[10].s64 + -12000;
	// 826664A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826664AC: 390B7D68  addi r8, r11, 0x7d68
	ctx.r[8].s64 = ctx.r[11].s64 + 32104;
	// 826664B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826664B4: 388A400C  addi r4, r10, 0x400c
	ctx.r[4].s64 = ctx.r[10].s64 + 16396;
	// 826664B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826664BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826664C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826664C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826664C8: 386AD600  addi r3, r10, -0x2a00
	ctx.r[3].s64 = ctx.r[10].s64 + -10752;
	// 826664CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826664D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826664D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826664D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826664DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826664E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826664E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826664E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826664EC: 4BE00935  bl 0x82466e20
	ctx.lr = 0x826664F0;
	sub_82466E20(ctx, base);
	// 826664F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826664F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826664F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826664FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666500 size=112
    let mut pc: u32 = 0x82666500;
    'dispatch: loop {
        match pc {
            0x82666500 => {
    //   block [0x82666500..0x82666570)
	// 82666500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266650C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666510: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666514: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82666518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266651C: 390B7D98  addi r8, r11, 0x7d98
	ctx.r[8].s64 = ctx.r[11].s64 + 32152;
	// 82666520: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82666524: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 82666528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266652C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666538: 386AD630  addi r3, r10, -0x29d0
	ctx.r[3].s64 = ctx.r[10].s64 + -10704;
	// 8266653C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266654C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266655C: 4BE008C5  bl 0x82466e20
	ctx.lr = 0x82666560;
	sub_82466E20(ctx, base);
	// 82666560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266656C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666570 size=112
    let mut pc: u32 = 0x82666570;
    'dispatch: loop {
        match pc {
            0x82666570 => {
    //   block [0x82666570..0x826665E0)
	// 82666570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266657C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666580: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666584: 38AAD270  addi r5, r10, -0x2d90
	ctx.r[5].s64 = ctx.r[10].s64 + -11664;
	// 82666588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266658C: 390B7E28  addi r8, r11, 0x7e28
	ctx.r[8].s64 = ctx.r[11].s64 + 32296;
	// 82666590: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82666594: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 82666598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266659C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826665A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826665A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826665A8: 386AD660  addi r3, r10, -0x29a0
	ctx.r[3].s64 = ctx.r[10].s64 + -10656;
	// 826665AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826665B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826665B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826665B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826665BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826665C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826665C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826665C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826665CC: 4BE00855  bl 0x82466e20
	ctx.lr = 0x826665D0;
	sub_82466E20(ctx, base);
	// 826665D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826665D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826665D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826665DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826665E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826665E0 size=112
    let mut pc: u32 = 0x826665E0;
    'dispatch: loop {
        match pc {
            0x826665E0 => {
    //   block [0x826665E0..0x82666650)
	// 826665E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826665E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826665E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826665EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826665F0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826665F4: 38AAD4B0  addi r5, r10, -0x2b50
	ctx.r[5].s64 = ctx.r[10].s64 + -11088;
	// 826665F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826665FC: 390B7E40  addi r8, r11, 0x7e40
	ctx.r[8].s64 = ctx.r[11].s64 + 32320;
	// 82666600: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82666604: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 82666608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266660C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666618: 386AD690  addi r3, r10, -0x2970
	ctx.r[3].s64 = ctx.r[10].s64 + -10608;
	// 8266661C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266662C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266663C: 4BE007E5  bl 0x82466e20
	ctx.lr = 0x82666640;
	sub_82466E20(ctx, base);
	// 82666640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266664C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666650 size=112
    let mut pc: u32 = 0x82666650;
    'dispatch: loop {
        match pc {
            0x82666650 => {
    //   block [0x82666650..0x826666C0)
	// 82666650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266665C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666660: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666664: 38AACD60  addi r5, r10, -0x32a0
	ctx.r[5].s64 = ctx.r[10].s64 + -12960;
	// 82666668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266666C: 390B7E70  addi r8, r11, 0x7e70
	ctx.r[8].s64 = ctx.r[11].s64 + 32368;
	// 82666670: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82666674: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 82666678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266667C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666688: 386AD6C0  addi r3, r10, -0x2940
	ctx.r[3].s64 = ctx.r[10].s64 + -10560;
	// 8266668C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266669C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826666A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826666A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826666A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826666AC: 4BE00775  bl 0x82466e20
	ctx.lr = 0x826666B0;
	sub_82466E20(ctx, base);
	// 826666B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826666B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826666B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826666BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826666C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826666C0 size=24
    let mut pc: u32 = 0x826666C0;
    'dispatch: loop {
        match pc {
            0x826666C0 => {
    //   block [0x826666C0..0x826666D8)
	// 826666C0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826666C4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826666C8: 394AC698  addi r10, r10, -0x3968
	ctx.r[10].s64 = ctx.r[10].s64 + -14696;
	// 826666CC: 816B7F18  lwz r11, 0x7f18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32536 as u32) ) } as u64;
	// 826666D0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826666D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826666D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826666D8 size=116
    let mut pc: u32 = 0x826666D8;
    'dispatch: loop {
        match pc {
            0x826666D8 => {
    //   block [0x826666D8..0x8266674C)
	// 826666D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826666DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826666E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826666E4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826666E8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826666EC: 390AC698  addi r8, r10, -0x3968
	ctx.r[8].s64 = ctx.r[10].s64 + -14696;
	// 826666F0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826666F4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826666F8: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 826666FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666700: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82666704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266670C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 82666710: 396BF540  addi r11, r11, -0xac0
	ctx.r[11].s64 = ctx.r[11].s64 + -2752;
	// 82666714: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666718: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266671C: 386AD6F0  addi r3, r10, -0x2910
	ctx.r[3].s64 = ctx.r[10].s64 + -10512;
	// 82666720: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82666724: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666728: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8266672C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666738: 4BE006E9  bl 0x82466e20
	ctx.lr = 0x8266673C;
	sub_82466E20(ctx, base);
	// 8266673C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666750 size=112
    let mut pc: u32 = 0x82666750;
    'dispatch: loop {
        match pc {
            0x82666750 => {
    //   block [0x82666750..0x826667C0)
	// 82666750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266675C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666760: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666764: 38AACE80  addi r5, r10, -0x3180
	ctx.r[5].s64 = ctx.r[10].s64 + -12672;
	// 82666768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266676C: 390B7EB8  addi r8, r11, 0x7eb8
	ctx.r[8].s64 = ctx.r[11].s64 + 32440;
	// 82666770: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82666774: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 82666778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266677C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666780: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666788: 386AD720  addi r3, r10, -0x28e0
	ctx.r[3].s64 = ctx.r[10].s64 + -10464;
	// 8266678C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266679C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826667A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826667A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826667A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826667AC: 4BE00675  bl 0x82466e20
	ctx.lr = 0x826667B0;
	sub_82466E20(ctx, base);
	// 826667B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826667B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826667B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826667BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826667C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826667C0 size=112
    let mut pc: u32 = 0x826667C0;
    'dispatch: loop {
        match pc {
            0x826667C0 => {
    //   block [0x826667C0..0x82666830)
	// 826667C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826667C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826667C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826667CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826667D0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826667D4: 38AACEE0  addi r5, r10, -0x3120
	ctx.r[5].s64 = ctx.r[10].s64 + -12576;
	// 826667D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826667DC: 390B7EE8  addi r8, r11, 0x7ee8
	ctx.r[8].s64 = ctx.r[11].s64 + 32488;
	// 826667E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826667E4: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 826667E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826667EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826667F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826667F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826667F8: 386AD750  addi r3, r10, -0x28b0
	ctx.r[3].s64 = ctx.r[10].s64 + -10416;
	// 826667FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266680C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266681C: 4BE00605  bl 0x82466e20
	ctx.lr = 0x82666820;
	sub_82466E20(ctx, base);
	// 82666820: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266682C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666830 size=100
    let mut pc: u32 = 0x82666830;
    'dispatch: loop {
        match pc {
            0x82666830 => {
    //   block [0x82666830..0x82666894)
	// 82666830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266683C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82666840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666844: 392AF5B0  addi r9, r10, -0xa50
	ctx.r[9].s64 = ctx.r[10].s64 + -2640;
	// 82666848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266684C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666850: 388A4110  addi r4, r10, 0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + 16656;
	// 82666854: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266685C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666864: 386AD780  addi r3, r10, -0x2880
	ctx.r[3].s64 = ctx.r[10].s64 + -10368;
	// 82666868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266686C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82666870: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82666874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666878: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266687C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82666880: 4BE005A1  bl 0x82466e20
	ctx.lr = 0x82666884;
	sub_82466E20(ctx, base);
	// 82666884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266688C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82666898 size=24
    let mut pc: u32 = 0x82666898;
    'dispatch: loop {
        match pc {
            0x82666898 => {
    //   block [0x82666898..0x826668B0)
	// 82666898: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266689C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826668A0: 394AC740  addi r10, r10, -0x38c0
	ctx.r[10].s64 = ctx.r[10].s64 + -14528;
	// 826668A4: 816B7F24  lwz r11, 0x7f24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32548 as u32) ) } as u64;
	// 826668A8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826668AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826668B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826668B0 size=112
    let mut pc: u32 = 0x826668B0;
    'dispatch: loop {
        match pc {
            0x826668B0 => {
    //   block [0x826668B0..0x82666920)
	// 826668B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826668B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826668B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826668BC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826668C0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826668C4: 392AF6F0  addi r9, r10, -0x910
	ctx.r[9].s64 = ctx.r[10].s64 + -2320;
	// 826668C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826668CC: 390BC740  addi r8, r11, -0x38c0
	ctx.r[8].s64 = ctx.r[11].s64 + -14528;
	// 826668D0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826668D4: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 826668D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826668DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826668E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826668E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826668E8: 386AD7B0  addi r3, r10, -0x2850
	ctx.r[3].s64 = ctx.r[10].s64 + -10320;
	// 826668EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826668F0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826668F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826668F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826668FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666904: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82666908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266690C: 4BE00515  bl 0x82466e20
	ctx.lr = 0x82666910;
	sub_82466E20(ctx, base);
	// 82666910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266691C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666920 size=112
    let mut pc: u32 = 0x82666920;
    'dispatch: loop {
        match pc {
            0x82666920 => {
    //   block [0x82666920..0x82666990)
	// 82666920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266692C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666930: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666934: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266693C: 390B7F2C  addi r8, r11, 0x7f2c
	ctx.r[8].s64 = ctx.r[11].s64 + 32556;
	// 82666940: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82666944: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 82666948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266694C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666958: 386AD7E0  addi r3, r10, -0x2820
	ctx.r[3].s64 = ctx.r[10].s64 + -10272;
	// 8266695C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266696C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266697C: 4BE004A5  bl 0x82466e20
	ctx.lr = 0x82666980;
	sub_82466E20(ctx, base);
	// 82666980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266698C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666990 size=108
    let mut pc: u32 = 0x82666990;
    'dispatch: loop {
        match pc {
            0x82666990 => {
    //   block [0x82666990..0x826669FC)
	// 82666990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266699C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826669A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826669A4: 38EB7F5C  addi r7, r11, 0x7f5c
	ctx.r[7].s64 = ctx.r[11].s64 + 32604;
	// 826669A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826669AC: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 826669B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826669B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826669B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826669BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826669C0: 386AD810  addi r3, r10, -0x27f0
	ctx.r[3].s64 = ctx.r[10].s64 + -10224;
	// 826669C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826669C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826669CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826669D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826669D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826669D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826669DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826669E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826669E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826669E8: 4BE00439  bl 0x82466e20
	ctx.lr = 0x826669EC;
	sub_82466E20(ctx, base);
	// 826669EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826669F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826669F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826669F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666A00 size=112
    let mut pc: u32 = 0x82666A00;
    'dispatch: loop {
        match pc {
            0x82666A00 => {
    //   block [0x82666A00..0x82666A70)
	// 82666A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666A0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666A10: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666A14: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666A18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82666A1C: 390B7F78  addi r8, r11, 0x7f78
	ctx.r[8].s64 = ctx.r[11].s64 + 32632;
	// 82666A20: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82666A24: 388A0E04  addi r4, r10, 0xe04
	ctx.r[4].s64 = ctx.r[10].s64 + 3588;
	// 82666A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666A2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666A30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666A38: 386AD840  addi r3, r10, -0x27c0
	ctx.r[3].s64 = ctx.r[10].s64 + -10176;
	// 82666A3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666A40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666A5C: 4BE003C5  bl 0x82466e20
	ctx.lr = 0x82666A60;
	sub_82466E20(ctx, base);
	// 82666A60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666A70 size=100
    let mut pc: u32 = 0x82666A70;
    'dispatch: loop {
        match pc {
            0x82666A70 => {
    //   block [0x82666A70..0x82666AD4)
	// 82666A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666A7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666A84: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666A88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666A90: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 82666A94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666A9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666AA4: 386AD870  addi r3, r10, -0x2790
	ctx.r[3].s64 = ctx.r[10].s64 + -10128;
	// 82666AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666AAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666AB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82666AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666AB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82666ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666AC0: 4BE00361  bl 0x82466e20
	ctx.lr = 0x82666AC4;
	sub_82466E20(ctx, base);
	// 82666AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666AD8 size=112
    let mut pc: u32 = 0x82666AD8;
    'dispatch: loop {
        match pc {
            0x82666AD8 => {
    //   block [0x82666AD8..0x82666B48)
	// 82666AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666AE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666AE8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666AEC: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666AF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666AF4: 390B7FD8  addi r8, r11, 0x7fd8
	ctx.r[8].s64 = ctx.r[11].s64 + 32728;
	// 82666AF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82666AFC: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 82666B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666B04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666B08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666B10: 386AD8A0  addi r3, r10, -0x2760
	ctx.r[3].s64 = ctx.r[10].s64 + -10080;
	// 82666B14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666B34: 4BE002ED  bl 0x82466e20
	ctx.lr = 0x82666B38;
	sub_82466E20(ctx, base);
	// 82666B38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666B48 size=112
    let mut pc: u32 = 0x82666B48;
    'dispatch: loop {
        match pc {
            0x82666B48 => {
    //   block [0x82666B48..0x82666BB8)
	// 82666B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666B54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666B58: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666B5C: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666B64: 390B7FF0  addi r8, r11, 0x7ff0
	ctx.r[8].s64 = ctx.r[11].s64 + 32752;
	// 82666B68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82666B6C: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 82666B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666B74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666B78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666B80: 386AD8D0  addi r3, r10, -0x2730
	ctx.r[3].s64 = ctx.r[10].s64 + -10032;
	// 82666B84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666BA4: 4BE0027D  bl 0x82466e20
	ctx.lr = 0x82666BA8;
	sub_82466E20(ctx, base);
	// 82666BA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666BB8 size=112
    let mut pc: u32 = 0x82666BB8;
    'dispatch: loop {
        match pc {
            0x82666BB8 => {
    //   block [0x82666BB8..0x82666C28)
	// 82666BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666BC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666BC8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82666BCC: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666BD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666BD4: 390B8020  addi r8, r11, -0x7fe0
	ctx.r[8].s64 = ctx.r[11].s64 + -32736;
	// 82666BD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82666BDC: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 82666BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666BE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666BE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666BF0: 386AD900  addi r3, r10, -0x2700
	ctx.r[3].s64 = ctx.r[10].s64 + -9984;
	// 82666BF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666C14: 4BE0020D  bl 0x82466e20
	ctx.lr = 0x82666C18;
	sub_82466E20(ctx, base);
	// 82666C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666C28 size=112
    let mut pc: u32 = 0x82666C28;
    'dispatch: loop {
        match pc {
            0x82666C28 => {
    //   block [0x82666C28..0x82666C98)
	// 82666C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666C34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666C38: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82666C3C: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666C40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666C44: 390B8050  addi r8, r11, -0x7fb0
	ctx.r[8].s64 = ctx.r[11].s64 + -32688;
	// 82666C48: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82666C4C: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 82666C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666C54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666C58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666C60: 386AD930  addi r3, r10, -0x26d0
	ctx.r[3].s64 = ctx.r[10].s64 + -9936;
	// 82666C64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666C70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666C74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666C78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666C7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666C80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666C84: 4BE0019D  bl 0x82466e20
	ctx.lr = 0x82666C88;
	sub_82466E20(ctx, base);
	// 82666C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666C98 size=112
    let mut pc: u32 = 0x82666C98;
    'dispatch: loop {
        match pc {
            0x82666C98 => {
    //   block [0x82666C98..0x82666D08)
	// 82666C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666CA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666CA8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82666CAC: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666CB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666CB4: 390B8080  addi r8, r11, -0x7f80
	ctx.r[8].s64 = ctx.r[11].s64 + -32640;
	// 82666CB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82666CBC: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 82666CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666CC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666CC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666CCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666CD0: 386AD960  addi r3, r10, -0x26a0
	ctx.r[3].s64 = ctx.r[10].s64 + -9888;
	// 82666CD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666CD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666CE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666CE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666CEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666CF4: 4BE0012D  bl 0x82466e20
	ctx.lr = 0x82666CF8;
	sub_82466E20(ctx, base);
	// 82666CF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666D08 size=112
    let mut pc: u32 = 0x82666D08;
    'dispatch: loop {
        match pc {
            0x82666D08 => {
    //   block [0x82666D08..0x82666D78)
	// 82666D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666D14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666D18: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82666D1C: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666D20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666D24: 390B8098  addi r8, r11, -0x7f68
	ctx.r[8].s64 = ctx.r[11].s64 + -32616;
	// 82666D28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82666D2C: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 82666D30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666D34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666D38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666D40: 386AD990  addi r3, r10, -0x2670
	ctx.r[3].s64 = ctx.r[10].s64 + -9840;
	// 82666D44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666D48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666D50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666D58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666D5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666D60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666D64: 4BE000BD  bl 0x82466e20
	ctx.lr = 0x82666D68;
	sub_82466E20(ctx, base);
	// 82666D68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666D78 size=112
    let mut pc: u32 = 0x82666D78;
    'dispatch: loop {
        match pc {
            0x82666D78 => {
    //   block [0x82666D78..0x82666DE8)
	// 82666D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666D80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666D84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666D88: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82666D8C: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666D90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666D94: 390B80B0  addi r8, r11, -0x7f50
	ctx.r[8].s64 = ctx.r[11].s64 + -32592;
	// 82666D98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82666D9C: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 82666DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666DA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666DA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666DAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666DB0: 386AD9C0  addi r3, r10, -0x2640
	ctx.r[3].s64 = ctx.r[10].s64 + -9792;
	// 82666DB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666DB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666DC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666DC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666DCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666DD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666DD4: 4BE0004D  bl 0x82466e20
	ctx.lr = 0x82666DD8;
	sub_82466E20(ctx, base);
	// 82666DD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666DDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666DE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666DE8 size=112
    let mut pc: u32 = 0x82666DE8;
    'dispatch: loop {
        match pc {
            0x82666DE8 => {
    //   block [0x82666DE8..0x82666E58)
	// 82666DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666DF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666DF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666DF8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82666DFC: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666E00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666E04: 390B80F8  addi r8, r11, -0x7f08
	ctx.r[8].s64 = ctx.r[11].s64 + -32520;
	// 82666E08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82666E0C: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 82666E10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666E14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666E18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666E1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666E20: 386AD9F0  addi r3, r10, -0x2610
	ctx.r[3].s64 = ctx.r[10].s64 + -9744;
	// 82666E24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666E28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666E2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666E30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666E34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666E38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666E40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666E44: 4BDFFFDD  bl 0x82466e20
	ctx.lr = 0x82666E48;
	sub_82466E20(ctx, base);
	// 82666E48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666E4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666E50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666E54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666E58 size=112
    let mut pc: u32 = 0x82666E58;
    'dispatch: loop {
        match pc {
            0x82666E58 => {
    //   block [0x82666E58..0x82666EC8)
	// 82666E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666E60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666E64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666E68: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82666E6C: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666E70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666E74: 390B8140  addi r8, r11, -0x7ec0
	ctx.r[8].s64 = ctx.r[11].s64 + -32448;
	// 82666E78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82666E7C: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 82666E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666E84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666E88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666E8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666E90: 386ADA20  addi r3, r10, -0x25e0
	ctx.r[3].s64 = ctx.r[10].s64 + -9696;
	// 82666E94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666E98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666E9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666EA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666EA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666EA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666EB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666EB4: 4BDFFF6D  bl 0x82466e20
	ctx.lr = 0x82666EB8;
	sub_82466E20(ctx, base);
	// 82666EB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666EC8 size=112
    let mut pc: u32 = 0x82666EC8;
    'dispatch: loop {
        match pc {
            0x82666EC8 => {
    //   block [0x82666EC8..0x82666F38)
	// 82666EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666ED4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666ED8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82666EDC: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666EE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666EE4: 390B8158  addi r8, r11, -0x7ea8
	ctx.r[8].s64 = ctx.r[11].s64 + -32424;
	// 82666EE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82666EEC: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 82666EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666EF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666EF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666F00: 386ADA50  addi r3, r10, -0x25b0
	ctx.r[3].s64 = ctx.r[10].s64 + -9648;
	// 82666F04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666F1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666F24: 4BDFFEFD  bl 0x82466e20
	ctx.lr = 0x82666F28;
	sub_82466E20(ctx, base);
	// 82666F28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666F38 size=116
    let mut pc: u32 = 0x82666F38;
    'dispatch: loop {
        match pc {
            0x82666F38 => {
    //   block [0x82666F38..0x82666FAC)
	// 82666F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666F40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666F44: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82666F48: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82666F4C: 390A8188  addi r8, r10, -0x7e78
	ctx.r[8].s64 = ctx.r[10].s64 + -32376;
	// 82666F50: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666F54: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82666F58: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666F5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666F60: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82666F64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666F68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666F6C: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 82666F70: 396BF718  addi r11, r11, -0x8e8
	ctx.r[11].s64 = ctx.r[11].s64 + -2280;
	// 82666F74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666F78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666F7C: 386ADA80  addi r3, r10, -0x2580
	ctx.r[3].s64 = ctx.r[10].s64 + -9600;
	// 82666F80: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82666F84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666F88: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82666F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666F94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666F98: 4BDFFE89  bl 0x82466e20
	ctx.lr = 0x82666F9C;
	sub_82466E20(ctx, base);
	// 82666F9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666FB0 size=116
    let mut pc: u32 = 0x82666FB0;
    'dispatch: loop {
        match pc {
            0x82666FB0 => {
    //   block [0x82666FB0..0x82667024)
	// 82666FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666FBC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82666FC0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82666FC4: 390A8200  addi r8, r10, -0x7e00
	ctx.r[8].s64 = ctx.r[10].s64 + -32256;
	// 82666FC8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666FCC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82666FD0: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666FD4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666FD8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82666FDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666FE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666FE4: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 82666FE8: 396BF730  addi r11, r11, -0x8d0
	ctx.r[11].s64 = ctx.r[11].s64 + -2256;
	// 82666FEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666FF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666FF4: 386ADAB0  addi r3, r10, -0x2550
	ctx.r[3].s64 = ctx.r[10].s64 + -9552;
	// 82666FF8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82666FFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667000: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82667004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266700C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667010: 4BDFFE11  bl 0x82466e20
	ctx.lr = 0x82667014;
	sub_82466E20(ctx, base);
	// 82667014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266701C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82667028 size=24
    let mut pc: u32 = 0x82667028;
    'dispatch: loop {
        match pc {
            0x82667028 => {
    //   block [0x82667028..0x82667040)
	// 82667028: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266702C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82667030: 394AC758  addi r10, r10, -0x38a8
	ctx.r[10].s64 = ctx.r[10].s64 + -14504;
	// 82667034: 816B7F74  lwz r11, 0x7f74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32628 as u32) ) } as u64;
	// 82667038: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8266703C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667040 size=116
    let mut pc: u32 = 0x82667040;
    'dispatch: loop {
        match pc {
            0x82667040 => {
    //   block [0x82667040..0x826670B4)
	// 82667040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266704C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82667050: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667054: 392BF75C  addi r9, r11, -0x8a4
	ctx.r[9].s64 = ctx.r[11].s64 + -2212;
	// 82667058: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 8266705C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667060: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82667064: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82667068: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266706C: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 82667070: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667074: 396BC758  addi r11, r11, -0x38a8
	ctx.r[11].s64 = ctx.r[11].s64 + -14504;
	// 82667078: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8266707C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667080: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82667084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667088: 386ADAE0  addi r3, r10, -0x2520
	ctx.r[3].s64 = ctx.r[10].s64 + -9504;
	// 8266708C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82667090: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82667094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667098: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8266709C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826670A0: 4BDFFD81  bl 0x82466e20
	ctx.lr = 0x826670A4;
	sub_82466E20(ctx, base);
	// 826670A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826670A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826670AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826670B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826670B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826670B8 size=112
    let mut pc: u32 = 0x826670B8;
    'dispatch: loop {
        match pc {
            0x826670B8 => {
    //   block [0x826670B8..0x82667128)
	// 826670B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826670BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826670C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826670C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826670C8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826670CC: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 826670D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826670D4: 390B8290  addi r8, r11, -0x7d70
	ctx.r[8].s64 = ctx.r[11].s64 + -32112;
	// 826670D8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826670DC: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 826670E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826670E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826670E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826670EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826670F0: 386ADB10  addi r3, r10, -0x24f0
	ctx.r[3].s64 = ctx.r[10].s64 + -9456;
	// 826670F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826670F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826670FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266710C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667114: 4BDFFD0D  bl 0x82466e20
	ctx.lr = 0x82667118;
	sub_82466E20(ctx, base);
	// 82667118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266711C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667128 size=112
    let mut pc: u32 = 0x82667128;
    'dispatch: loop {
        match pc {
            0x82667128 => {
    //   block [0x82667128..0x82667198)
	// 82667128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266712C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667134: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667138: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266713C: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82667140: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667144: 390B82F0  addi r8, r11, -0x7d10
	ctx.r[8].s64 = ctx.r[11].s64 + -32016;
	// 82667148: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8266714C: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 82667150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667154: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667158: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266715C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667160: 386ADB40  addi r3, r10, -0x24c0
	ctx.r[3].s64 = ctx.r[10].s64 + -9408;
	// 82667164: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266716C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266717C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667184: 4BDFFC9D  bl 0x82466e20
	ctx.lr = 0x82667188;
	sub_82466E20(ctx, base);
	// 82667188: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266718C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667198 size=112
    let mut pc: u32 = 0x82667198;
    'dispatch: loop {
        match pc {
            0x82667198 => {
    //   block [0x82667198..0x82667208)
	// 82667198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266719C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826671A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826671A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826671A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826671AC: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 826671B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826671B4: 390B8398  addi r8, r11, -0x7c68
	ctx.r[8].s64 = ctx.r[11].s64 + -31848;
	// 826671B8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826671BC: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 826671C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826671C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826671C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826671CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826671D0: 386ADB70  addi r3, r10, -0x2490
	ctx.r[3].s64 = ctx.r[10].s64 + -9360;
	// 826671D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826671D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826671DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826671E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826671E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826671E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826671EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826671F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826671F4: 4BDFFC2D  bl 0x82466e20
	ctx.lr = 0x826671F8;
	sub_82466E20(ctx, base);
	// 826671F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826671FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667208 size=112
    let mut pc: u32 = 0x82667208;
    'dispatch: loop {
        match pc {
            0x82667208 => {
    //   block [0x82667208..0x82667278)
	// 82667208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266720C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667214: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667218: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266721C: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82667220: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667224: 390B8410  addi r8, r11, -0x7bf0
	ctx.r[8].s64 = ctx.r[11].s64 + -31728;
	// 82667228: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266722C: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 82667230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667234: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667238: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266723C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667240: 386ADBA0  addi r3, r10, -0x2460
	ctx.r[3].s64 = ctx.r[10].s64 + -9312;
	// 82667244: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266724C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266725C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667264: 4BDFFBBD  bl 0x82466e20
	ctx.lr = 0x82667268;
	sub_82466E20(ctx, base);
	// 82667268: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266726C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667278 size=112
    let mut pc: u32 = 0x82667278;
    'dispatch: loop {
        match pc {
            0x82667278 => {
    //   block [0x82667278..0x826672E8)
	// 82667278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266727C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667284: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667288: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266728C: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82667290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667294: 390B8458  addi r8, r11, -0x7ba8
	ctx.r[8].s64 = ctx.r[11].s64 + -31656;
	// 82667298: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8266729C: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 826672A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826672A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826672A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826672AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826672B0: 386ADBD0  addi r3, r10, -0x2430
	ctx.r[3].s64 = ctx.r[10].s64 + -9264;
	// 826672B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826672B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826672BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826672C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826672C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826672C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826672CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826672D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826672D4: 4BDFFB4D  bl 0x82466e20
	ctx.lr = 0x826672D8;
	sub_82466E20(ctx, base);
	// 826672D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826672DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826672E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826672E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826672E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826672E8 size=112
    let mut pc: u32 = 0x826672E8;
    'dispatch: loop {
        match pc {
            0x826672E8 => {
    //   block [0x826672E8..0x82667358)
	// 826672E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826672EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826672F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826672F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826672F8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826672FC: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82667300: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667304: 390B84E8  addi r8, r11, -0x7b18
	ctx.r[8].s64 = ctx.r[11].s64 + -31512;
	// 82667308: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8266730C: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 82667310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667314: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266731C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667320: 386ADC00  addi r3, r10, -0x2400
	ctx.r[3].s64 = ctx.r[10].s64 + -9216;
	// 82667324: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266732C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266733C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667344: 4BDFFADD  bl 0x82466e20
	ctx.lr = 0x82667348;
	sub_82466E20(ctx, base);
	// 82667348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266734C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667358 size=112
    let mut pc: u32 = 0x82667358;
    'dispatch: loop {
        match pc {
            0x82667358 => {
    //   block [0x82667358..0x826673C8)
	// 82667358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266735C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667364: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667368: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266736C: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82667370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667374: 390B8548  addi r8, r11, -0x7ab8
	ctx.r[8].s64 = ctx.r[11].s64 + -31416;
	// 82667378: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8266737C: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 82667380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667384: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266738C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667390: 386ADC30  addi r3, r10, -0x23d0
	ctx.r[3].s64 = ctx.r[10].s64 + -9168;
	// 82667394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266739C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826673A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826673A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826673A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826673AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826673B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826673B4: 4BDFFA6D  bl 0x82466e20
	ctx.lr = 0x826673B8;
	sub_82466E20(ctx, base);
	// 826673B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826673BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826673C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826673C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826673C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826673C8 size=112
    let mut pc: u32 = 0x826673C8;
    'dispatch: loop {
        match pc {
            0x826673C8 => {
    //   block [0x826673C8..0x82667438)
	// 826673C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826673CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826673D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826673D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826673D8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826673DC: 38AADC30  addi r5, r10, -0x23d0
	ctx.r[5].s64 = ctx.r[10].s64 + -9168;
	// 826673E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826673E4: 390B85A8  addi r8, r11, -0x7a58
	ctx.r[8].s64 = ctx.r[11].s64 + -31320;
	// 826673E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826673EC: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 826673F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826673F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826673F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826673FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667400: 386ADC60  addi r3, r10, -0x23a0
	ctx.r[3].s64 = ctx.r[10].s64 + -9120;
	// 82667404: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266740C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266741C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667424: 4BDFF9FD  bl 0x82466e20
	ctx.lr = 0x82667428;
	sub_82466E20(ctx, base);
	// 82667428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266742C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667438 size=112
    let mut pc: u32 = 0x82667438;
    'dispatch: loop {
        match pc {
            0x82667438 => {
    //   block [0x82667438..0x826674A8)
	// 82667438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266743C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667444: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667448: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266744C: 38AADC30  addi r5, r10, -0x23d0
	ctx.r[5].s64 = ctx.r[10].s64 + -9168;
	// 82667450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667454: 390B85D8  addi r8, r11, -0x7a28
	ctx.r[8].s64 = ctx.r[11].s64 + -31272;
	// 82667458: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266745C: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 82667460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667464: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266746C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667470: 386ADC90  addi r3, r10, -0x2370
	ctx.r[3].s64 = ctx.r[10].s64 + -9072;
	// 82667474: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266747C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266748C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667494: 4BDFF98D  bl 0x82466e20
	ctx.lr = 0x82667498;
	sub_82466E20(ctx, base);
	// 82667498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266749C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826674A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826674A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826674A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826674A8 size=100
    let mut pc: u32 = 0x826674A8;
    'dispatch: loop {
        match pc {
            0x826674A8 => {
    //   block [0x826674A8..0x8266750C)
	// 826674A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826674AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826674B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826674B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826674B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826674BC: 38AADC30  addi r5, r10, -0x23d0
	ctx.r[5].s64 = ctx.r[10].s64 + -9168;
	// 826674C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826674C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826674C8: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 826674CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826674D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826674D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826674D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826674DC: 386ADCC0  addi r3, r10, -0x2340
	ctx.r[3].s64 = ctx.r[10].s64 + -9024;
	// 826674E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826674E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826674E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826674EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826674F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826674F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826674F8: 4BDFF929  bl 0x82466e20
	ctx.lr = 0x826674FC;
	sub_82466E20(ctx, base);
	// 826674FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667510 size=112
    let mut pc: u32 = 0x82667510;
    'dispatch: loop {
        match pc {
            0x82667510 => {
    //   block [0x82667510..0x82667580)
	// 82667510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266751C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667520: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667524: 38AADC30  addi r5, r10, -0x23d0
	ctx.r[5].s64 = ctx.r[10].s64 + -9168;
	// 82667528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266752C: 390B8620  addi r8, r11, -0x79e0
	ctx.r[8].s64 = ctx.r[11].s64 + -31200;
	// 82667530: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82667534: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 82667538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266753C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667540: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667548: 386ADCF0  addi r3, r10, -0x2310
	ctx.r[3].s64 = ctx.r[10].s64 + -8976;
	// 8266754C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266755C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266756C: 4BDFF8B5  bl 0x82466e20
	ctx.lr = 0x82667570;
	sub_82466E20(ctx, base);
	// 82667570: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266757C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667580 size=108
    let mut pc: u32 = 0x82667580;
    'dispatch: loop {
        match pc {
            0x82667580 => {
    //   block [0x82667580..0x826675EC)
	// 82667580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266758C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667590: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82667594: 38EB8638  addi r7, r11, -0x79c8
	ctx.r[7].s64 = ctx.r[11].s64 + -31176;
	// 82667598: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266759C: 388A0E24  addi r4, r10, 0xe24
	ctx.r[4].s64 = ctx.r[10].s64 + 3620;
	// 826675A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826675A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826675A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826675AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826675B0: 386ADD20  addi r3, r10, -0x22e0
	ctx.r[3].s64 = ctx.r[10].s64 + -8928;
	// 826675B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826675B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826675BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826675C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826675C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826675C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826675CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826675D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826675D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826675D8: 4BDFF849  bl 0x82466e20
	ctx.lr = 0x826675DC;
	sub_82466E20(ctx, base);
	// 826675DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826675E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826675E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826675E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826675F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826675F0 size=112
    let mut pc: u32 = 0x826675F0;
    'dispatch: loop {
        match pc {
            0x826675F0 => {
    //   block [0x826675F0..0x82667660)
	// 826675F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826675F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826675F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826675FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667600: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667604: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82667608: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266760C: 390B8680  addi r8, r11, -0x7980
	ctx.r[8].s64 = ctx.r[11].s64 + -31104;
	// 82667610: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82667614: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 82667618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266761C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667620: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667628: 386ADD50  addi r3, r10, -0x22b0
	ctx.r[3].s64 = ctx.r[10].s64 + -8880;
	// 8266762C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266763C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266764C: 4BDFF7D5  bl 0x82466e20
	ctx.lr = 0x82667650;
	sub_82466E20(ctx, base);
	// 82667650: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266765C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667660 size=112
    let mut pc: u32 = 0x82667660;
    'dispatch: loop {
        match pc {
            0x82667660 => {
    //   block [0x82667660..0x826676D0)
	// 82667660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266766C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667670: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667674: 38AADD50  addi r5, r10, -0x22b0
	ctx.r[5].s64 = ctx.r[10].s64 + -8880;
	// 82667678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266767C: 390B86E0  addi r8, r11, -0x7920
	ctx.r[8].s64 = ctx.r[11].s64 + -31008;
	// 82667680: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82667684: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 82667688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266768C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667690: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667698: 386ADD80  addi r3, r10, -0x2280
	ctx.r[3].s64 = ctx.r[10].s64 + -8832;
	// 8266769C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826676A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826676A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826676A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826676AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826676B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826676B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826676B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826676BC: 4BDFF765  bl 0x82466e20
	ctx.lr = 0x826676C0;
	sub_82466E20(ctx, base);
	// 826676C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826676C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826676C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826676CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826676D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826676D0 size=112
    let mut pc: u32 = 0x826676D0;
    'dispatch: loop {
        match pc {
            0x826676D0 => {
    //   block [0x826676D0..0x82667740)
	// 826676D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826676D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826676D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826676DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826676E0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826676E4: 38AADD50  addi r5, r10, -0x22b0
	ctx.r[5].s64 = ctx.r[10].s64 + -8880;
	// 826676E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826676EC: 390B86F8  addi r8, r11, -0x7908
	ctx.r[8].s64 = ctx.r[11].s64 + -30984;
	// 826676F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826676F4: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 826676F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826676FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667700: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667708: 386ADDB0  addi r3, r10, -0x2250
	ctx.r[3].s64 = ctx.r[10].s64 + -8784;
	// 8266770C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266771C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266772C: 4BDFF6F5  bl 0x82466e20
	ctx.lr = 0x82667730;
	sub_82466E20(ctx, base);
	// 82667730: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266773C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667740 size=112
    let mut pc: u32 = 0x82667740;
    'dispatch: loop {
        match pc {
            0x82667740 => {
    //   block [0x82667740..0x826677B0)
	// 82667740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266774C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667750: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667754: 38AADD50  addi r5, r10, -0x22b0
	ctx.r[5].s64 = ctx.r[10].s64 + -8880;
	// 82667758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266775C: 390B8728  addi r8, r11, -0x78d8
	ctx.r[8].s64 = ctx.r[11].s64 + -30936;
	// 82667760: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82667764: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 82667768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266776C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667770: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667778: 386ADDE0  addi r3, r10, -0x2220
	ctx.r[3].s64 = ctx.r[10].s64 + -8736;
	// 8266777C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266778C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266779C: 4BDFF685  bl 0x82466e20
	ctx.lr = 0x826677A0;
	sub_82466E20(ctx, base);
	// 826677A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826677A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826677A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826677AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826677B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826677B0 size=24
    let mut pc: u32 = 0x826677B0;
    'dispatch: loop {
        match pc {
            0x826677B0 => {
    //   block [0x826677B0..0x826677C8)
	// 826677B0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826677B4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826677B8: 394AC800  addi r10, r10, -0x3800
	ctx.r[10].s64 = ctx.r[10].s64 + -14336;
	// 826677BC: 816B8740  lwz r11, -0x78c0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30912 as u32) ) } as u64;
	// 826677C0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826677C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826677C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826677C8 size=112
    let mut pc: u32 = 0x826677C8;
    'dispatch: loop {
        match pc {
            0x826677C8 => {
    //   block [0x826677C8..0x82667838)
	// 826677C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826677CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826677D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826677D4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826677D8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826677DC: 392AF7B8  addi r9, r10, -0x848
	ctx.r[9].s64 = ctx.r[10].s64 + -2120;
	// 826677E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826677E4: 390BC800  addi r8, r11, -0x3800
	ctx.r[8].s64 = ctx.r[11].s64 + -14336;
	// 826677E8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826677EC: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 826677F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826677F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826677F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826677FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667800: 386ADE10  addi r3, r10, -0x21f0
	ctx.r[3].s64 = ctx.r[10].s64 + -8688;
	// 82667804: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82667808: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266780C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266781C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82667820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667824: 4BDFF5FD  bl 0x82466e20
	ctx.lr = 0x82667828;
	sub_82466E20(ctx, base);
	// 82667828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266782C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667838 size=108
    let mut pc: u32 = 0x82667838;
    'dispatch: loop {
        match pc {
            0x82667838 => {
    //   block [0x82667838..0x826678A4)
	// 82667838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266783C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667844: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266784C: 38EB8744  addi r7, r11, -0x78bc
	ctx.r[7].s64 = ctx.r[11].s64 + -30908;
	// 82667850: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82667854: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 82667858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266785C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667860: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82667864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667868: 386ADE40  addi r3, r10, -0x21c0
	ctx.r[3].s64 = ctx.r[10].s64 + -8640;
	// 8266786C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82667870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266787C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266788C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82667890: 4BDFF591  bl 0x82466e20
	ctx.lr = 0x82667894;
	sub_82466E20(ctx, base);
	// 82667894: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266789C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826678A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826678A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826678A8 size=108
    let mut pc: u32 = 0x826678A8;
    'dispatch: loop {
        match pc {
            0x826678A8 => {
    //   block [0x826678A8..0x82667914)
	// 826678A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826678AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826678B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826678B4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826678B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826678BC: 38EB8760  addi r7, r11, -0x78a0
	ctx.r[7].s64 = ctx.r[11].s64 + -30880;
	// 826678C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826678C4: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 826678C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826678CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826678D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826678D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826678D8: 386ADE70  addi r3, r10, -0x2190
	ctx.r[3].s64 = ctx.r[10].s64 + -8592;
	// 826678DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826678E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826678E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826678E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826678EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826678F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826678F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826678F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826678FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82667900: 4BDFF521  bl 0x82466e20
	ctx.lr = 0x82667904;
	sub_82466E20(ctx, base);
	// 82667904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266790C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667918 size=116
    let mut pc: u32 = 0x82667918;
    'dispatch: loop {
        match pc {
            0x82667918 => {
    //   block [0x82667918..0x8266798C)
	// 82667918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266791C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667924: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667928: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266792C: 390B87A8  addi r8, r11, -0x7858
	ctx.r[8].s64 = ctx.r[11].s64 + -30808;
	// 82667930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667934: 392AF870  addi r9, r10, -0x790
	ctx.r[9].s64 = ctx.r[10].s64 + -1936;
	// 82667938: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266793C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82667940: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82667944: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266794C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266795C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82667960: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 82667964: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82667968: 386BDEA0  addi r3, r11, -0x2160
	ctx.r[3].s64 = ctx.r[11].s64 + -8544;
	// 8266796C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82667970: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667978: 4BDFF4A9  bl 0x82466e20
	ctx.lr = 0x8266797C;
	sub_82466E20(ctx, base);
	// 8266797C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82667990 size=24
    let mut pc: u32 = 0x82667990;
    'dispatch: loop {
        match pc {
            0x82667990 => {
    //   block [0x82667990..0x826679A8)
	// 82667990: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667994: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82667998: 394AC848  addi r10, r10, -0x37b8
	ctx.r[10].s64 = ctx.r[10].s64 + -14264;
	// 8266799C: 816B87C0  lwz r11, -0x7840(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30784 as u32) ) } as u64;
	// 826679A0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826679A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826679A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826679A8 size=116
    let mut pc: u32 = 0x826679A8;
    'dispatch: loop {
        match pc {
            0x826679A8 => {
    //   block [0x826679A8..0x82667A1C)
	// 826679A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826679AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826679B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826679B4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826679B8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826679BC: 390BC848  addi r8, r11, -0x37b8
	ctx.r[8].s64 = ctx.r[11].s64 + -14264;
	// 826679C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826679C4: 392AF8E0  addi r9, r10, -0x720
	ctx.r[9].s64 = ctx.r[10].s64 + -1824;
	// 826679C8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826679CC: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826679D0: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826679D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826679D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826679DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826679E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826679E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826679E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826679EC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826679F0: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 826679F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826679F8: 386BDED0  addi r3, r11, -0x2130
	ctx.r[3].s64 = ctx.r[11].s64 + -8496;
	// 826679FC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82667A00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667A08: 4BDFF419  bl 0x82466e20
	ctx.lr = 0x82667A0C;
	sub_82466E20(ctx, base);
	// 82667A0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667A20 size=108
    let mut pc: u32 = 0x82667A20;
    'dispatch: loop {
        match pc {
            0x82667A20 => {
    //   block [0x82667A20..0x82667A8C)
	// 82667A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667A2C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667A30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667A34: 38EB87D0  addi r7, r11, -0x7830
	ctx.r[7].s64 = ctx.r[11].s64 + -30768;
	// 82667A38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82667A3C: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 82667A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667A44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667A48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82667A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667A50: 386ADF00  addi r3, r10, -0x2100
	ctx.r[3].s64 = ctx.r[10].s64 + -8448;
	// 82667A54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82667A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667A74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82667A78: 4BDFF3A9  bl 0x82466e20
	ctx.lr = 0x82667A7C;
	sub_82466E20(ctx, base);
	// 82667A7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667A90 size=112
    let mut pc: u32 = 0x82667A90;
    'dispatch: loop {
        match pc {
            0x82667A90 => {
    //   block [0x82667A90..0x82667B00)
	// 82667A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667A9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667AA0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667AA4: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82667AA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667AAC: 390B8800  addi r8, r11, -0x7800
	ctx.r[8].s64 = ctx.r[11].s64 + -30720;
	// 82667AB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82667AB4: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 82667AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667ABC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667AC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667AC8: 386ADF30  addi r3, r10, -0x20d0
	ctx.r[3].s64 = ctx.r[10].s64 + -8400;
	// 82667ACC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667AD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667AD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667ADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667AE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667AE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667AE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667AEC: 4BDFF335  bl 0x82466e20
	ctx.lr = 0x82667AF0;
	sub_82466E20(ctx, base);
	// 82667AF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667B00 size=112
    let mut pc: u32 = 0x82667B00;
    'dispatch: loop {
        match pc {
            0x82667B00 => {
    //   block [0x82667B00..0x82667B70)
	// 82667B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667B0C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82667B10: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667B14: 392AF938  addi r9, r10, -0x6c8
	ctx.r[9].s64 = ctx.r[10].s64 + -1736;
	// 82667B18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667B1C: 390B8820  addi r8, r11, -0x77e0
	ctx.r[8].s64 = ctx.r[11].s64 + -30688;
	// 82667B20: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82667B24: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 82667B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667B2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667B30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667B38: 386ADF60  addi r3, r10, -0x20a0
	ctx.r[3].s64 = ctx.r[10].s64 + -8352;
	// 82667B3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82667B40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82667B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667B54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82667B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667B5C: 4BDFF2C5  bl 0x82466e20
	ctx.lr = 0x82667B60;
	sub_82466E20(ctx, base);
	// 82667B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667B70 size=112
    let mut pc: u32 = 0x82667B70;
    'dispatch: loop {
        match pc {
            0x82667B70 => {
    //   block [0x82667B70..0x82667BE0)
	// 82667B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667B7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667B80: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667B84: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82667B88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667B8C: 390B8868  addi r8, r11, -0x7798
	ctx.r[8].s64 = ctx.r[11].s64 + -30616;
	// 82667B90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82667B94: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 82667B98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667B9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667BA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667BA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667BA8: 386ADF90  addi r3, r10, -0x2070
	ctx.r[3].s64 = ctx.r[10].s64 + -8304;
	// 82667BAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667BB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667BB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667BC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667BC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667BC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667BCC: 4BDFF255  bl 0x82466e20
	ctx.lr = 0x82667BD0;
	sub_82466E20(ctx, base);
	// 82667BD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667BE0 size=112
    let mut pc: u32 = 0x82667BE0;
    'dispatch: loop {
        match pc {
            0x82667BE0 => {
    //   block [0x82667BE0..0x82667C50)
	// 82667BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667BEC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82667BF0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667BF4: 392AF964  addi r9, r10, -0x69c
	ctx.r[9].s64 = ctx.r[10].s64 + -1692;
	// 82667BF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667BFC: 390B8880  addi r8, r11, -0x7780
	ctx.r[8].s64 = ctx.r[11].s64 + -30592;
	// 82667C00: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82667C04: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 82667C08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667C0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667C10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667C18: 386ADFC0  addi r3, r10, -0x2040
	ctx.r[3].s64 = ctx.r[10].s64 + -8256;
	// 82667C1C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82667C20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82667C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667C28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667C30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667C34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82667C38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667C3C: 4BDFF1E5  bl 0x82466e20
	ctx.lr = 0x82667C40;
	sub_82466E20(ctx, base);
	// 82667C40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667C50 size=112
    let mut pc: u32 = 0x82667C50;
    'dispatch: loop {
        match pc {
            0x82667C50 => {
    //   block [0x82667C50..0x82667CC0)
	// 82667C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667C5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667C60: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667C64: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82667C68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667C6C: 390B8910  addi r8, r11, -0x76f0
	ctx.r[8].s64 = ctx.r[11].s64 + -30448;
	// 82667C70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82667C74: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 82667C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667C7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667C80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667C88: 386ADFF0  addi r3, r10, -0x2010
	ctx.r[3].s64 = ctx.r[10].s64 + -8208;
	// 82667C8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667C94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667CAC: 4BDFF175  bl 0x82466e20
	ctx.lr = 0x82667CB0;
	sub_82466E20(ctx, base);
	// 82667CB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667CC0 size=112
    let mut pc: u32 = 0x82667CC0;
    'dispatch: loop {
        match pc {
            0x82667CC0 => {
    //   block [0x82667CC0..0x82667D30)
	// 82667CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667CCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667CD0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667CD4: 38AAE050  addi r5, r10, -0x1fb0
	ctx.r[5].s64 = ctx.r[10].s64 + -8112;
	// 82667CD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667CDC: 390B8928  addi r8, r11, -0x76d8
	ctx.r[8].s64 = ctx.r[11].s64 + -30424;
	// 82667CE0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82667CE4: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 82667CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667CEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667CF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667CF8: 386AE020  addi r3, r10, -0x1fe0
	ctx.r[3].s64 = ctx.r[10].s64 + -8160;
	// 82667CFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667D04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667D1C: 4BDFF105  bl 0x82466e20
	ctx.lr = 0x82667D20;
	sub_82466E20(ctx, base);
	// 82667D20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667D30 size=100
    let mut pc: u32 = 0x82667D30;
    'dispatch: loop {
        match pc {
            0x82667D30 => {
    //   block [0x82667D30..0x82667D94)
	// 82667D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667D3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667D44: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82667D48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667D50: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 82667D54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667D64: 386AE050  addi r3, r10, -0x1fb0
	ctx.r[3].s64 = ctx.r[10].s64 + -8112;
	// 82667D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667D6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667D70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82667D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667D78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82667D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667D80: 4BDFF0A1  bl 0x82466e20
	ctx.lr = 0x82667D84;
	sub_82466E20(ctx, base);
	// 82667D84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667D88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667D8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667D90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82667D98 size=24
    let mut pc: u32 = 0x82667D98;
    'dispatch: loop {
        match pc {
            0x82667D98 => {
    //   block [0x82667D98..0x82667DB0)
	// 82667D98: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667D9C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82667DA0: 394AC920  addi r10, r10, -0x36e0
	ctx.r[10].s64 = ctx.r[10].s64 + -14048;
	// 82667DA4: 816B89A0  lwz r11, -0x7660(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30304 as u32) ) } as u64;
	// 82667DA8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82667DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667DB0 size=116
    let mut pc: u32 = 0x82667DB0;
    'dispatch: loop {
        match pc {
            0x82667DB0 => {
    //   block [0x82667DB0..0x82667E24)
	// 82667DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667DB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667DBC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667DC0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82667DC4: 390BC920  addi r8, r11, -0x36e0
	ctx.r[8].s64 = ctx.r[11].s64 + -14048;
	// 82667DC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667DCC: 392AF9A0  addi r9, r10, -0x660
	ctx.r[9].s64 = ctx.r[10].s64 + -1632;
	// 82667DD0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667DD4: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82667DD8: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82667DDC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667DE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667DE4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667DE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667DEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667DF4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82667DF8: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 82667DFC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82667E00: 386BE080  addi r3, r11, -0x1f80
	ctx.r[3].s64 = ctx.r[11].s64 + -8064;
	// 82667E04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82667E08: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667E10: 4BDFF011  bl 0x82466e20
	ctx.lr = 0x82667E14;
	sub_82466E20(ctx, base);
	// 82667E14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667E18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667E1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667E28 size=108
    let mut pc: u32 = 0x82667E28;
    'dispatch: loop {
        match pc {
            0x82667E28 => {
    //   block [0x82667E28..0x82667E94)
	// 82667E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667E34: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667E3C: 38EB89A4  addi r7, r11, -0x765c
	ctx.r[7].s64 = ctx.r[11].s64 + -30300;
	// 82667E40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82667E44: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 82667E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667E4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667E50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82667E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667E58: 386AE0B0  addi r3, r10, -0x1f50
	ctx.r[3].s64 = ctx.r[10].s64 + -8016;
	// 82667E5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82667E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667E7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82667E80: 4BDFEFA1  bl 0x82466e20
	ctx.lr = 0x82667E84;
	sub_82466E20(ctx, base);
	// 82667E84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667E98 size=112
    let mut pc: u32 = 0x82667E98;
    'dispatch: loop {
        match pc {
            0x82667E98 => {
    //   block [0x82667E98..0x82667F08)
	// 82667E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667EA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667EA8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667EAC: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82667EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667EB4: 390B89D4  addi r8, r11, -0x762c
	ctx.r[8].s64 = ctx.r[11].s64 + -30252;
	// 82667EB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82667EBC: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 82667EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667EC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667EC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667ED0: 386AE0E0  addi r3, r10, -0x1f20
	ctx.r[3].s64 = ctx.r[10].s64 + -7968;
	// 82667ED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667EDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667EF4: 4BDFEF2D  bl 0x82466e20
	ctx.lr = 0x82667EF8;
	sub_82466E20(ctx, base);
	// 82667EF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667F08 size=112
    let mut pc: u32 = 0x82667F08;
    'dispatch: loop {
        match pc {
            0x82667F08 => {
    //   block [0x82667F08..0x82667F78)
	// 82667F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667F14: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82667F18: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667F1C: 392AF9C4  addi r9, r10, -0x63c
	ctx.r[9].s64 = ctx.r[10].s64 + -1596;
	// 82667F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667F24: 390B89F0  addi r8, r11, -0x7610
	ctx.r[8].s64 = ctx.r[11].s64 + -30224;
	// 82667F28: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82667F2C: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 82667F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667F34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667F38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667F40: 386AE110  addi r3, r10, -0x1ef0
	ctx.r[3].s64 = ctx.r[10].s64 + -7920;
	// 82667F44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82667F48: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82667F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667F5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82667F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667F64: 4BDFEEBD  bl 0x82466e20
	ctx.lr = 0x82667F68;
	sub_82466E20(ctx, base);
	// 82667F68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667F78 size=112
    let mut pc: u32 = 0x82667F78;
    'dispatch: loop {
        match pc {
            0x82667F78 => {
    //   block [0x82667F78..0x82667FE8)
	// 82667F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667F84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667F88: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667F8C: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82667F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667F94: 390B8A98  addi r8, r11, -0x7568
	ctx.r[8].s64 = ctx.r[11].s64 + -30056;
	// 82667F98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82667F9C: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 82667FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667FA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667FB0: 386AE140  addi r3, r10, -0x1ec0
	ctx.r[3].s64 = ctx.r[10].s64 + -7872;
	// 82667FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667FD4: 4BDFEE4D  bl 0x82466e20
	ctx.lr = 0x82667FD8;
	sub_82466E20(ctx, base);
	// 82667FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667FE8 size=112
    let mut pc: u32 = 0x82667FE8;
    'dispatch: loop {
        match pc {
            0x82667FE8 => {
    //   block [0x82667FE8..0x82668058)
	// 82667FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667FF4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82667FF8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667FFC: 392AFA1C  addi r9, r10, -0x5e4
	ctx.r[9].s64 = ctx.r[10].s64 + -1508;
	// 82668000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668004: 390B8AB8  addi r8, r11, -0x7548
	ctx.r[8].s64 = ctx.r[11].s64 + -30024;
	// 82668008: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8266800C: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 82668010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668014: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266801C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668020: 386AE170  addi r3, r10, -0x1e90
	ctx.r[3].s64 = ctx.r[10].s64 + -7824;
	// 82668024: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82668028: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266802C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266803C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668044: 4BDFEDDD  bl 0x82466e20
	ctx.lr = 0x82668048;
	sub_82466E20(ctx, base);
	// 82668048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266804C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668058 size=116
    let mut pc: u32 = 0x82668058;
    'dispatch: loop {
        match pc {
            0x82668058 => {
    //   block [0x82668058..0x826680CC)
	// 82668058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266805C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668064: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668068: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266806C: 390B8B60  addi r8, r11, -0x74a0
	ctx.r[8].s64 = ctx.r[11].s64 + -29856;
	// 82668070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668074: 392AF9F0  addi r9, r10, -0x610
	ctx.r[9].s64 = ctx.r[10].s64 + -1552;
	// 82668078: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266807C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82668080: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82668084: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266808C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266809C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826680A0: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 826680A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826680A8: 386BE1A0  addi r3, r11, -0x1e60
	ctx.r[3].s64 = ctx.r[11].s64 + -7776;
	// 826680AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826680B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826680B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826680B8: 4BDFED69  bl 0x82466e20
	ctx.lr = 0x826680BC;
	sub_82466E20(ctx, base);
	// 826680BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826680C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826680C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826680C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826680D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826680D0 size=108
    let mut pc: u32 = 0x826680D0;
    'dispatch: loop {
        match pc {
            0x826680D0 => {
    //   block [0x826680D0..0x8266813C)
	// 826680D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826680D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826680D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826680DC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826680E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826680E4: 38EB8B78  addi r7, r11, -0x7488
	ctx.r[7].s64 = ctx.r[11].s64 + -29832;
	// 826680E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826680EC: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 826680F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826680F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826680F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826680FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668100: 386AE1D0  addi r3, r10, -0x1e30
	ctx.r[3].s64 = ctx.r[10].s64 + -7728;
	// 82668104: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82668108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266810C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266811C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668128: 4BDFECF9  bl 0x82466e20
	ctx.lr = 0x8266812C;
	sub_82466E20(ctx, base);
	// 8266812C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668140 size=112
    let mut pc: u32 = 0x82668140;
    'dispatch: loop {
        match pc {
            0x82668140 => {
    //   block [0x82668140..0x826681B0)
	// 82668140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266814C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668150: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668154: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82668158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266815C: 390B8BA8  addi r8, r11, -0x7458
	ctx.r[8].s64 = ctx.r[11].s64 + -29784;
	// 82668160: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82668164: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 82668168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266816C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668178: 386AE200  addi r3, r10, -0x1e00
	ctx.r[3].s64 = ctx.r[10].s64 + -7680;
	// 8266817C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266818C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266819C: 4BDFEC85  bl 0x82466e20
	ctx.lr = 0x826681A0;
	sub_82466E20(ctx, base);
	// 826681A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826681A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826681A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826681AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826681B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826681B0 size=112
    let mut pc: u32 = 0x826681B0;
    'dispatch: loop {
        match pc {
            0x826681B0 => {
    //   block [0x826681B0..0x82668220)
	// 826681B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826681B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826681B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826681BC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826681C0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826681C4: 392AFA50  addi r9, r10, -0x5b0
	ctx.r[9].s64 = ctx.r[10].s64 + -1456;
	// 826681C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826681CC: 390B8BC8  addi r8, r11, -0x7438
	ctx.r[8].s64 = ctx.r[11].s64 + -29752;
	// 826681D0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826681D4: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 826681D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826681DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826681E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826681E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826681E8: 386AE230  addi r3, r10, -0x1dd0
	ctx.r[3].s64 = ctx.r[10].s64 + -7632;
	// 826681EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826681F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826681F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826681F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826681FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266820C: 4BDFEC15  bl 0x82466e20
	ctx.lr = 0x82668210;
	sub_82466E20(ctx, base);
	// 82668210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266821C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668220 size=112
    let mut pc: u32 = 0x82668220;
    'dispatch: loop {
        match pc {
            0x82668220 => {
    //   block [0x82668220..0x82668290)
	// 82668220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266822C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668230: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668234: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82668238: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266823C: 390B8C70  addi r8, r11, -0x7390
	ctx.r[8].s64 = ctx.r[11].s64 + -29584;
	// 82668240: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82668244: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 82668248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266824C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668250: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668258: 386AE260  addi r3, r10, -0x1da0
	ctx.r[3].s64 = ctx.r[10].s64 + -7584;
	// 8266825C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266826C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266827C: 4BDFEBA5  bl 0x82466e20
	ctx.lr = 0x82668280;
	sub_82466E20(ctx, base);
	// 82668280: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266828C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668290 size=112
    let mut pc: u32 = 0x82668290;
    'dispatch: loop {
        match pc {
            0x82668290 => {
    //   block [0x82668290..0x82668300)
	// 82668290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266829C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826682A0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826682A4: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 826682A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826682AC: 390B8CB8  addi r8, r11, -0x7348
	ctx.r[8].s64 = ctx.r[11].s64 + -29512;
	// 826682B0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826682B4: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 826682B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826682BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826682C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826682C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826682C8: 386AE290  addi r3, r10, -0x1d70
	ctx.r[3].s64 = ctx.r[10].s64 + -7536;
	// 826682CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826682D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826682D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826682D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826682DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826682E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826682E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826682E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826682EC: 4BDFEB35  bl 0x82466e20
	ctx.lr = 0x826682F0;
	sub_82466E20(ctx, base);
	// 826682F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826682F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826682F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826682FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668300 size=100
    let mut pc: u32 = 0x82668300;
    'dispatch: loop {
        match pc {
            0x82668300 => {
    //   block [0x82668300..0x82668364)
	// 82668300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266830C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668314: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82668318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266831C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668320: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 82668324: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266832C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668334: 386AE2C0  addi r3, r10, -0x1d40
	ctx.r[3].s64 = ctx.r[10].s64 + -7488;
	// 82668338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266833C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668340: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82668344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668348: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266834C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668350: 4BDFEAD1  bl 0x82466e20
	ctx.lr = 0x82668354;
	sub_82466E20(ctx, base);
	// 82668354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266835C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668368 size=112
    let mut pc: u32 = 0x82668368;
    'dispatch: loop {
        match pc {
            0x82668368 => {
    //   block [0x82668368..0x826683D8)
	// 82668368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266836C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668374: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668378: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266837C: 38AADED0  addi r5, r10, -0x2130
	ctx.r[5].s64 = ctx.r[10].s64 + -8496;
	// 82668380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668384: 390B8D78  addi r8, r11, -0x7288
	ctx.r[8].s64 = ctx.r[11].s64 + -29320;
	// 82668388: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266838C: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 82668390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668394: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266839C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826683A0: 386AE2F0  addi r3, r10, -0x1d10
	ctx.r[3].s64 = ctx.r[10].s64 + -7440;
	// 826683A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826683A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826683AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826683B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826683B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826683B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826683BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826683C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826683C4: 4BDFEA5D  bl 0x82466e20
	ctx.lr = 0x826683C8;
	sub_82466E20(ctx, base);
	// 826683C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826683CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826683D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826683D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826683D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826683D8 size=112
    let mut pc: u32 = 0x826683D8;
    'dispatch: loop {
        match pc {
            0x826683D8 => {
    //   block [0x826683D8..0x82668448)
	// 826683D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826683DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826683E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826683E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826683E8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826683EC: 38AADD50  addi r5, r10, -0x22b0
	ctx.r[5].s64 = ctx.r[10].s64 + -8880;
	// 826683F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826683F4: 390B8DA8  addi r8, r11, -0x7258
	ctx.r[8].s64 = ctx.r[11].s64 + -29272;
	// 826683F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826683FC: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 82668400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668404: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266840C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668410: 386AE320  addi r3, r10, -0x1ce0
	ctx.r[3].s64 = ctx.r[10].s64 + -7392;
	// 82668414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266841C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266842C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668434: 4BDFE9ED  bl 0x82466e20
	ctx.lr = 0x82668438;
	sub_82466E20(ctx, base);
	// 82668438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266843C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668448 size=108
    let mut pc: u32 = 0x82668448;
    'dispatch: loop {
        match pc {
            0x82668448 => {
    //   block [0x82668448..0x826684B4)
	// 82668448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266844C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668454: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266845C: 38EB8DC0  addi r7, r11, -0x7240
	ctx.r[7].s64 = ctx.r[11].s64 + -29248;
	// 82668460: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82668464: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 82668468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266846C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82668474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668478: 386AE350  addi r3, r10, -0x1cb0
	ctx.r[3].s64 = ctx.r[10].s64 + -7344;
	// 8266847C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82668480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266848C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266849C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826684A0: 4BDFE981  bl 0x82466e20
	ctx.lr = 0x826684A4;
	sub_82466E20(ctx, base);
	// 826684A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826684A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826684AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826684B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826684B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826684B8 size=112
    let mut pc: u32 = 0x826684B8;
    'dispatch: loop {
        match pc {
            0x826684B8 => {
    //   block [0x826684B8..0x82668528)
	// 826684B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826684BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826684C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826684C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826684C8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826684CC: 38AAE2C0  addi r5, r10, -0x1d40
	ctx.r[5].s64 = ctx.r[10].s64 + -7488;
	// 826684D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826684D4: 390B8DF0  addi r8, r11, -0x7210
	ctx.r[8].s64 = ctx.r[11].s64 + -29200;
	// 826684D8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826684DC: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 826684E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826684E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826684E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826684EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826684F0: 386AE380  addi r3, r10, -0x1c80
	ctx.r[3].s64 = ctx.r[10].s64 + -7296;
	// 826684F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826684F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826684FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266850C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668514: 4BDFE90D  bl 0x82466e20
	ctx.lr = 0x82668518;
	sub_82466E20(ctx, base);
	// 82668518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266851C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668528 size=112
    let mut pc: u32 = 0x82668528;
    'dispatch: loop {
        match pc {
            0x82668528 => {
    //   block [0x82668528..0x82668598)
	// 82668528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266852C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668534: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82668538: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266853C: 392AFA7C  addi r9, r10, -0x584
	ctx.r[9].s64 = ctx.r[10].s64 + -1412;
	// 82668540: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668544: 390B8E80  addi r8, r11, -0x7180
	ctx.r[8].s64 = ctx.r[11].s64 + -29056;
	// 82668548: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8266854C: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 82668550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668554: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668558: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266855C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668560: 386AE3B0  addi r3, r10, -0x1c50
	ctx.r[3].s64 = ctx.r[10].s64 + -7248;
	// 82668564: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82668568: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266856C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266857C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668584: 4BDFE89D  bl 0x82466e20
	ctx.lr = 0x82668588;
	sub_82466E20(ctx, base);
	// 82668588: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266858C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668598 size=112
    let mut pc: u32 = 0x82668598;
    'dispatch: loop {
        match pc {
            0x82668598 => {
    //   block [0x82668598..0x82668608)
	// 82668598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266859C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826685A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826685A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826685A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826685AC: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 826685B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826685B4: 390B8EC8  addi r8, r11, -0x7138
	ctx.r[8].s64 = ctx.r[11].s64 + -28984;
	// 826685B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826685BC: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 826685C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826685C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826685C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826685CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826685D0: 386AE3E0  addi r3, r10, -0x1c20
	ctx.r[3].s64 = ctx.r[10].s64 + -7200;
	// 826685D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826685D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826685DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826685E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826685E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826685E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826685EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826685F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826685F4: 4BDFE82D  bl 0x82466e20
	ctx.lr = 0x826685F8;
	sub_82466E20(ctx, base);
	// 826685F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826685FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668608 size=108
    let mut pc: u32 = 0x82668608;
    'dispatch: loop {
        match pc {
            0x82668608 => {
    //   block [0x82668608..0x82668674)
	// 82668608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266860C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668614: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266861C: 38EB8EE0  addi r7, r11, -0x7120
	ctx.r[7].s64 = ctx.r[11].s64 + -28960;
	// 82668620: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82668624: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 82668628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266862C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82668634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668638: 386AE410  addi r3, r10, -0x1bf0
	ctx.r[3].s64 = ctx.r[10].s64 + -7152;
	// 8266863C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82668640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266864C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266865C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668660: 4BDFE7C1  bl 0x82466e20
	ctx.lr = 0x82668664;
	sub_82466E20(ctx, base);
	// 82668664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266866C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668678 size=116
    let mut pc: u32 = 0x82668678;
    'dispatch: loop {
        match pc {
            0x82668678 => {
    //   block [0x82668678..0x826686EC)
	// 82668678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266867C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668684: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82668688: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8266868C: 390A8F70  addi r8, r10, -0x7090
	ctx.r[8].s64 = ctx.r[10].s64 + -28816;
	// 82668690: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668694: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82668698: 38AAE2C0  addi r5, r10, -0x1d40
	ctx.r[5].s64 = ctx.r[10].s64 + -7488;
	// 8266869C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826686A0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826686A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826686A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826686AC: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 826686B0: 396BFA90  addi r11, r11, -0x570
	ctx.r[11].s64 = ctx.r[11].s64 + -1392;
	// 826686B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826686B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826686BC: 386AE440  addi r3, r10, -0x1bc0
	ctx.r[3].s64 = ctx.r[10].s64 + -7104;
	// 826686C0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826686C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826686C8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826686CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826686D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826686D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826686D8: 4BDFE749  bl 0x82466e20
	ctx.lr = 0x826686DC;
	sub_82466E20(ctx, base);
	// 826686DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826686E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826686E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826686E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826686F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826686F0 size=112
    let mut pc: u32 = 0x826686F0;
    'dispatch: loop {
        match pc {
            0x826686F0 => {
    //   block [0x826686F0..0x82668760)
	// 826686F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826686F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826686F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826686FC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82668700: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668704: 392AFADC  addi r9, r10, -0x524
	ctx.r[9].s64 = ctx.r[10].s64 + -1316;
	// 82668708: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266870C: 390B9050  addi r8, r11, -0x6fb0
	ctx.r[8].s64 = ctx.r[11].s64 + -28592;
	// 82668710: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82668714: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 82668718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266871C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668728: 386AE470  addi r3, r10, -0x1b90
	ctx.r[3].s64 = ctx.r[10].s64 + -7056;
	// 8266872C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82668730: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82668734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266873C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668744: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266874C: 4BDFE6D5  bl 0x82466e20
	ctx.lr = 0x82668750;
	sub_82466E20(ctx, base);
	// 82668750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266875C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668760 size=112
    let mut pc: u32 = 0x82668760;
    'dispatch: loop {
        match pc {
            0x82668760 => {
    //   block [0x82668760..0x826687D0)
	// 82668760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266876C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668770: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668774: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82668778: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266877C: 390B90B0  addi r8, r11, -0x6f50
	ctx.r[8].s64 = ctx.r[11].s64 + -28496;
	// 82668780: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82668784: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 82668788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266878C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668798: 386AE4A0  addi r3, r10, -0x1b60
	ctx.r[3].s64 = ctx.r[10].s64 + -7008;
	// 8266879C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826687A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826687A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826687A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826687AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826687B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826687B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826687B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826687BC: 4BDFE665  bl 0x82466e20
	ctx.lr = 0x826687C0;
	sub_82466E20(ctx, base);
	// 826687C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826687C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826687C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826687CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826687D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826687D0 size=108
    let mut pc: u32 = 0x826687D0;
    'dispatch: loop {
        match pc {
            0x826687D0 => {
    //   block [0x826687D0..0x8266883C)
	// 826687D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826687D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826687D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826687DC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826687E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826687E4: 38EB90C8  addi r7, r11, -0x6f38
	ctx.r[7].s64 = ctx.r[11].s64 + -28472;
	// 826687E8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826687EC: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 826687F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826687F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826687F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826687FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668800: 386AE4D0  addi r3, r10, -0x1b30
	ctx.r[3].s64 = ctx.r[10].s64 + -6960;
	// 82668804: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82668808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266880C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266881C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668828: 4BDFE5F9  bl 0x82466e20
	ctx.lr = 0x8266882C;
	sub_82466E20(ctx, base);
	// 8266882C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668840 size=112
    let mut pc: u32 = 0x82668840;
    'dispatch: loop {
        match pc {
            0x82668840 => {
    //   block [0x82668840..0x826688B0)
	// 82668840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266884C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668850: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668854: 38AAE2C0  addi r5, r10, -0x1d40
	ctx.r[5].s64 = ctx.r[10].s64 + -7488;
	// 82668858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266885C: 390B9110  addi r8, r11, -0x6ef0
	ctx.r[8].s64 = ctx.r[11].s64 + -28400;
	// 82668860: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82668864: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 82668868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266886C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668878: 386AE500  addi r3, r10, -0x1b00
	ctx.r[3].s64 = ctx.r[10].s64 + -6912;
	// 8266887C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266888C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266889C: 4BDFE585  bl 0x82466e20
	ctx.lr = 0x826688A0;
	sub_82466E20(ctx, base);
	// 826688A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826688A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826688A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826688AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826688B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826688B0 size=112
    let mut pc: u32 = 0x826688B0;
    'dispatch: loop {
        match pc {
            0x826688B0 => {
    //   block [0x826688B0..0x82668920)
	// 826688B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826688B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826688B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826688BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826688C0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826688C4: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 826688C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826688CC: 390B9188  addi r8, r11, -0x6e78
	ctx.r[8].s64 = ctx.r[11].s64 + -28280;
	// 826688D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826688D4: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 826688D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826688DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826688E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826688E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826688E8: 386AE530  addi r3, r10, -0x1ad0
	ctx.r[3].s64 = ctx.r[10].s64 + -6864;
	// 826688EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826688F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826688F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826688F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826688FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266890C: 4BDFE515  bl 0x82466e20
	ctx.lr = 0x82668910;
	sub_82466E20(ctx, base);
	// 82668910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266891C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668920 size=108
    let mut pc: u32 = 0x82668920;
    'dispatch: loop {
        match pc {
            0x82668920 => {
    //   block [0x82668920..0x8266898C)
	// 82668920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266892C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668934: 38EB91B8  addi r7, r11, -0x6e48
	ctx.r[7].s64 = ctx.r[11].s64 + -28232;
	// 82668938: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8266893C: 388A4864  addi r4, r10, 0x4864
	ctx.r[4].s64 = ctx.r[10].s64 + 18532;
	// 82668940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668944: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668948: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266894C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668950: 386AE560  addi r3, r10, -0x1aa0
	ctx.r[3].s64 = ctx.r[10].s64 + -6816;
	// 82668954: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82668958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266895C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266896C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668978: 4BDFE4A9  bl 0x82466e20
	ctx.lr = 0x8266897C;
	sub_82466E20(ctx, base);
	// 8266897C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668990 size=108
    let mut pc: u32 = 0x82668990;
    'dispatch: loop {
        match pc {
            0x82668990 => {
    //   block [0x82668990..0x826689FC)
	// 82668990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266899C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826689A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826689A4: 38EB9218  addi r7, r11, -0x6de8
	ctx.r[7].s64 = ctx.r[11].s64 + -28136;
	// 826689A8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826689AC: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 826689B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826689B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826689B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826689BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826689C0: 386AE590  addi r3, r10, -0x1a70
	ctx.r[3].s64 = ctx.r[10].s64 + -6768;
	// 826689C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826689C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826689CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826689D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826689D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826689D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826689DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826689E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826689E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826689E8: 4BDFE439  bl 0x82466e20
	ctx.lr = 0x826689EC;
	sub_82466E20(ctx, base);
	// 826689EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826689F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826689F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826689F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668A00 size=112
    let mut pc: u32 = 0x82668A00;
    'dispatch: loop {
        match pc {
            0x82668A00 => {
    //   block [0x82668A00..0x82668A70)
	// 82668A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668A0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668A10: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668A14: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82668A18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668A1C: 390B9290  addi r8, r11, -0x6d70
	ctx.r[8].s64 = ctx.r[11].s64 + -28016;
	// 82668A20: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82668A24: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 82668A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668A2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668A30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668A38: 386AE5C0  addi r3, r10, -0x1a40
	ctx.r[3].s64 = ctx.r[10].s64 + -6720;
	// 82668A3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668A40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668A5C: 4BDFE3C5  bl 0x82466e20
	ctx.lr = 0x82668A60;
	sub_82466E20(ctx, base);
	// 82668A60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82668A70 size=24
    let mut pc: u32 = 0x82668A70;
    'dispatch: loop {
        match pc {
            0x82668A70 => {
    //   block [0x82668A70..0x82668A88)
	// 82668A70: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668A74: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82668A78: 394AC998  addi r10, r10, -0x3668
	ctx.r[10].s64 = ctx.r[10].s64 + -13928;
	// 82668A7C: 816B904C  lwz r11, -0x6fb4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28596 as u32) ) } as u64;
	// 82668A80: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82668A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668A88 size=116
    let mut pc: u32 = 0x82668A88;
    'dispatch: loop {
        match pc {
            0x82668A88 => {
    //   block [0x82668A88..0x82668AFC)
	// 82668A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668A94: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668A98: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82668A9C: 390BC998  addi r8, r11, -0x3668
	ctx.r[8].s64 = ctx.r[11].s64 + -13928;
	// 82668AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668AA4: 392AFB20  addi r9, r10, -0x4e0
	ctx.r[9].s64 = ctx.r[10].s64 + -1248;
	// 82668AA8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668AAC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82668AB0: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82668AB4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668ABC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668ACC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82668AD0: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 82668AD4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82668AD8: 386BE5F0  addi r3, r11, -0x1a10
	ctx.r[3].s64 = ctx.r[11].s64 + -6672;
	// 82668ADC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82668AE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668AE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668AE8: 4BDFE339  bl 0x82466e20
	ctx.lr = 0x82668AEC;
	sub_82466E20(ctx, base);
	// 82668AEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668B00 size=112
    let mut pc: u32 = 0x82668B00;
    'dispatch: loop {
        match pc {
            0x82668B00 => {
    //   block [0x82668B00..0x82668B70)
	// 82668B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668B0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668B10: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668B14: 38AAE5F0  addi r5, r10, -0x1a10
	ctx.r[5].s64 = ctx.r[10].s64 + -6672;
	// 82668B18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668B1C: 390B92D8  addi r8, r11, -0x6d28
	ctx.r[8].s64 = ctx.r[11].s64 + -27944;
	// 82668B20: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82668B24: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 82668B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668B2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668B30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668B38: 386AE620  addi r3, r10, -0x19e0
	ctx.r[3].s64 = ctx.r[10].s64 + -6624;
	// 82668B3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668B40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668B44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668B54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668B5C: 4BDFE2C5  bl 0x82466e20
	ctx.lr = 0x82668B60;
	sub_82466E20(ctx, base);
	// 82668B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82668B70 size=24
    let mut pc: u32 = 0x82668B70;
    'dispatch: loop {
        match pc {
            0x82668B70 => {
    //   block [0x82668B70..0x82668B88)
	// 82668B70: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668B74: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82668B78: 394AC9B0  addi r10, r10, -0x3650
	ctx.r[10].s64 = ctx.r[10].s64 + -13904;
	// 82668B7C: 816B9308  lwz r11, -0x6cf8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27896 as u32) ) } as u64;
	// 82668B80: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82668B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668B88 size=116
    let mut pc: u32 = 0x82668B88;
    'dispatch: loop {
        match pc {
            0x82668B88 => {
    //   block [0x82668B88..0x82668BFC)
	// 82668B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668B94: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668B98: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82668B9C: 390BC9B0  addi r8, r11, -0x3650
	ctx.r[8].s64 = ctx.r[11].s64 + -13904;
	// 82668BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668BA4: 392AFB5C  addi r9, r10, -0x4a4
	ctx.r[9].s64 = ctx.r[10].s64 + -1188;
	// 82668BA8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668BAC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82668BB0: 38AAE620  addi r5, r10, -0x19e0
	ctx.r[5].s64 = ctx.r[10].s64 + -6624;
	// 82668BB4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668BB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668BBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668BC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668BC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668BCC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82668BD0: 388A490C  addi r4, r10, 0x490c
	ctx.r[4].s64 = ctx.r[10].s64 + 18700;
	// 82668BD4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82668BD8: 386BE650  addi r3, r11, -0x19b0
	ctx.r[3].s64 = ctx.r[11].s64 + -6576;
	// 82668BDC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82668BE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668BE8: 4BDFE239  bl 0x82466e20
	ctx.lr = 0x82668BEC;
	sub_82466E20(ctx, base);
	// 82668BEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668BF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668BF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668C00 size=112
    let mut pc: u32 = 0x82668C00;
    'dispatch: loop {
        match pc {
            0x82668C00 => {
    //   block [0x82668C00..0x82668C70)
	// 82668C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668C0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668C10: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668C14: 38AAE620  addi r5, r10, -0x19e0
	ctx.r[5].s64 = ctx.r[10].s64 + -6624;
	// 82668C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668C1C: 390B9310  addi r8, r11, -0x6cf0
	ctx.r[8].s64 = ctx.r[11].s64 + -27888;
	// 82668C20: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82668C24: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 82668C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668C2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668C30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668C34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668C38: 386AE680  addi r3, r10, -0x1980
	ctx.r[3].s64 = ctx.r[10].s64 + -6528;
	// 82668C3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668C44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668C5C: 4BDFE1C5  bl 0x82466e20
	ctx.lr = 0x82668C60;
	sub_82466E20(ctx, base);
	// 82668C60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668C70 size=112
    let mut pc: u32 = 0x82668C70;
    'dispatch: loop {
        match pc {
            0x82668C70 => {
    //   block [0x82668C70..0x82668CE0)
	// 82668C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668C7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668C80: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668C84: 38AAE620  addi r5, r10, -0x19e0
	ctx.r[5].s64 = ctx.r[10].s64 + -6624;
	// 82668C88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668C8C: 390B9370  addi r8, r11, -0x6c90
	ctx.r[8].s64 = ctx.r[11].s64 + -27792;
	// 82668C90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82668C94: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 82668C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668C9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668CA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668CA8: 386AE6B0  addi r3, r10, -0x1950
	ctx.r[3].s64 = ctx.r[10].s64 + -6480;
	// 82668CAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668CCC: 4BDFE155  bl 0x82466e20
	ctx.lr = 0x82668CD0;
	sub_82466E20(ctx, base);
	// 82668CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668CE0 size=112
    let mut pc: u32 = 0x82668CE0;
    'dispatch: loop {
        match pc {
            0x82668CE0 => {
    //   block [0x82668CE0..0x82668D50)
	// 82668CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668CE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668CEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668CF0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668CF4: 38AAE620  addi r5, r10, -0x19e0
	ctx.r[5].s64 = ctx.r[10].s64 + -6624;
	// 82668CF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668CFC: 390B93A0  addi r8, r11, -0x6c60
	ctx.r[8].s64 = ctx.r[11].s64 + -27744;
	// 82668D00: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82668D04: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 82668D08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668D0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668D10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668D18: 386AE6E0  addi r3, r10, -0x1920
	ctx.r[3].s64 = ctx.r[10].s64 + -6432;
	// 82668D1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668D20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668D28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668D2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668D30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668D38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668D3C: 4BDFE0E5  bl 0x82466e20
	ctx.lr = 0x82668D40;
	sub_82466E20(ctx, base);
	// 82668D40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668D50 size=108
    let mut pc: u32 = 0x82668D50;
    'dispatch: loop {
        match pc {
            0x82668D50 => {
    //   block [0x82668D50..0x82668DBC)
	// 82668D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668D5C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668D60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668D64: 38EB93E8  addi r7, r11, -0x6c18
	ctx.r[7].s64 = ctx.r[11].s64 + -27672;
	// 82668D68: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82668D6C: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 82668D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668D74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668D78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82668D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668D80: 386AE710  addi r3, r10, -0x18f0
	ctx.r[3].s64 = ctx.r[10].s64 + -6384;
	// 82668D84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82668D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668D8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668DA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668DA8: 4BDFE079  bl 0x82466e20
	ctx.lr = 0x82668DAC;
	sub_82466E20(ctx, base);
	// 82668DAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668DC0 size=112
    let mut pc: u32 = 0x82668DC0;
    'dispatch: loop {
        match pc {
            0x82668DC0 => {
    //   block [0x82668DC0..0x82668E30)
	// 82668DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668DCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668DD0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668DD4: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82668DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668DDC: 390B9418  addi r8, r11, -0x6be8
	ctx.r[8].s64 = ctx.r[11].s64 + -27624;
	// 82668DE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82668DE4: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 82668DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668DEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668DF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668DF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668DF8: 386AE740  addi r3, r10, -0x18c0
	ctx.r[3].s64 = ctx.r[10].s64 + -6336;
	// 82668DFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668E1C: 4BDFE005  bl 0x82466e20
	ctx.lr = 0x82668E20;
	sub_82466E20(ctx, base);
	// 82668E20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668E30 size=108
    let mut pc: u32 = 0x82668E30;
    'dispatch: loop {
        match pc {
            0x82668E30 => {
    //   block [0x82668E30..0x82668E9C)
	// 82668E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668E3C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668E40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668E44: 38EB9430  addi r7, r11, -0x6bd0
	ctx.r[7].s64 = ctx.r[11].s64 + -27600;
	// 82668E48: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82668E4C: 388A49B4  addi r4, r10, 0x49b4
	ctx.r[4].s64 = ctx.r[10].s64 + 18868;
	// 82668E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668E54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668E58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82668E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668E60: 386AE770  addi r3, r10, -0x1890
	ctx.r[3].s64 = ctx.r[10].s64 + -6288;
	// 82668E64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82668E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668E70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668E74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668E78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668E7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668E84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668E88: 4BDFDF99  bl 0x82466e20
	ctx.lr = 0x82668E8C;
	sub_82466E20(ctx, base);
	// 82668E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668EA0 size=108
    let mut pc: u32 = 0x82668EA0;
    'dispatch: loop {
        match pc {
            0x82668EA0 => {
    //   block [0x82668EA0..0x82668F0C)
	// 82668EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668EAC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668EB4: 38EB9478  addi r7, r11, -0x6b88
	ctx.r[7].s64 = ctx.r[11].s64 + -27528;
	// 82668EB8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82668EBC: 388A49DC  addi r4, r10, 0x49dc
	ctx.r[4].s64 = ctx.r[10].s64 + 18908;
	// 82668EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668EC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668EC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82668ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668ED0: 386AE7A0  addi r3, r10, -0x1860
	ctx.r[3].s64 = ctx.r[10].s64 + -6240;
	// 82668ED4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82668ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668EF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668EF8: 4BDFDF29  bl 0x82466e20
	ctx.lr = 0x82668EFC;
	sub_82466E20(ctx, base);
	// 82668EFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668F10 size=116
    let mut pc: u32 = 0x82668F10;
    'dispatch: loop {
        match pc {
            0x82668F10 => {
    //   block [0x82668F10..0x82668F84)
	// 82668F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668F1C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82668F20: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668F24: 392BFB90  addi r9, r11, -0x470
	ctx.r[9].s64 = ctx.r[11].s64 + -1136;
	// 82668F28: 38AAEC20  addi r5, r10, -0x13e0
	ctx.r[5].s64 = ctx.r[10].s64 + -5088;
	// 82668F2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668F30: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 82668F34: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 82668F38: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668F3C: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 82668F40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668F44: 396B94D8  addi r11, r11, -0x6b28
	ctx.r[11].s64 = ctx.r[11].s64 + -27432;
	// 82668F48: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82668F4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668F50: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82668F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668F58: 386AE7D0  addi r3, r10, -0x1830
	ctx.r[3].s64 = ctx.r[10].s64 + -6192;
	// 82668F5C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82668F60: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82668F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668F68: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82668F6C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82668F70: 4BDFDEB1  bl 0x82466e20
	ctx.lr = 0x82668F74;
	sub_82466E20(ctx, base);
	// 82668F74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668F88 size=100
    let mut pc: u32 = 0x82668F88;
    'dispatch: loop {
        match pc {
            0x82668F88 => {
    //   block [0x82668F88..0x82668FEC)
	// 82668F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668F94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668F98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668F9C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82668FA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668FA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668FA8: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 82668FAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668FBC: 386AE800  addi r3, r10, -0x1800
	ctx.r[3].s64 = ctx.r[10].s64 + -6144;
	// 82668FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668FC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668FC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82668FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668FD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82668FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668FD8: 4BDFDE49  bl 0x82466e20
	ctx.lr = 0x82668FDC;
	sub_82466E20(ctx, base);
	// 82668FDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668FF0 size=100
    let mut pc: u32 = 0x82668FF0;
    'dispatch: loop {
        match pc {
            0x82668FF0 => {
    //   block [0x82668FF0..0x82669054)
	// 82668FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668FFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669004: 38AAE890  addi r5, r10, -0x1770
	ctx.r[5].s64 = ctx.r[10].s64 + -6000;
	// 82669008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266900C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669010: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 82669014: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266901C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669024: 386AE830  addi r3, r10, -0x17d0
	ctx.r[3].s64 = ctx.r[10].s64 + -6096;
	// 82669028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266902C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669030: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82669034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669038: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266903C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669040: 4BDFDDE1  bl 0x82466e20
	ctx.lr = 0x82669044;
	sub_82466E20(ctx, base);
	// 82669044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266904C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669058 size=100
    let mut pc: u32 = 0x82669058;
    'dispatch: loop {
        match pc {
            0x82669058 => {
    //   block [0x82669058..0x826690BC)
	// 82669058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266905C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669064: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266906C: 38AAE7D0  addi r5, r10, -0x1830
	ctx.r[5].s64 = ctx.r[10].s64 + -6192;
	// 82669070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669078: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 8266907C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266908C: 386AE860  addi r3, r10, -0x17a0
	ctx.r[3].s64 = ctx.r[10].s64 + -6048;
	// 82669090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669094: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669098: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266909C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826690A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826690A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826690A8: 4BDFDD79  bl 0x82466e20
	ctx.lr = 0x826690AC;
	sub_82466E20(ctx, base);
	// 826690AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826690B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826690B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826690B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826690C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826690C0 size=104
    let mut pc: u32 = 0x826690C0;
    'dispatch: loop {
        match pc {
            0x826690C0 => {
    //   block [0x826690C0..0x82669128)
	// 826690C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826690C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826690C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826690CC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826690D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826690D4: 392AFC14  addi r9, r10, -0x3ec
	ctx.r[9].s64 = ctx.r[10].s64 + -1004;
	// 826690D8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826690DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826690E0: 38AAE800  addi r5, r10, -0x1800
	ctx.r[5].s64 = ctx.r[10].s64 + -6144;
	// 826690E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826690E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826690EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826690F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826690F4: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 826690F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826690FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669100: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82669104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669108: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266910C: 386AE890  addi r3, r10, -0x1770
	ctx.r[3].s64 = ctx.r[10].s64 + -6000;
	// 82669110: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82669114: 4BDFDD0D  bl 0x82466e20
	ctx.lr = 0x82669118;
	sub_82466E20(ctx, base);
	// 82669118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266911C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669128 size=108
    let mut pc: u32 = 0x82669128;
    'dispatch: loop {
        match pc {
            0x82669128 => {
    //   block [0x82669128..0x82669194)
	// 82669128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266912C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669134: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266913C: 38EB968C  addi r7, r11, -0x6974
	ctx.r[7].s64 = ctx.r[11].s64 + -26996;
	// 82669140: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82669144: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 82669148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266914C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669150: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82669154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669158: 386AE8C0  addi r3, r10, -0x1740
	ctx.r[3].s64 = ctx.r[10].s64 + -5952;
	// 8266915C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82669160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266916C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266917C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82669180: 4BDFDCA1  bl 0x82466e20
	ctx.lr = 0x82669184;
	sub_82466E20(ctx, base);
	// 82669184: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266918C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669198 size=112
    let mut pc: u32 = 0x82669198;
    'dispatch: loop {
        match pc {
            0x82669198 => {
    //   block [0x82669198..0x82669208)
	// 82669198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266919C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826691A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826691A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826691A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826691AC: 38AAE890  addi r5, r10, -0x1770
	ctx.r[5].s64 = ctx.r[10].s64 + -6000;
	// 826691B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826691B4: 390B96C0  addi r8, r11, -0x6940
	ctx.r[8].s64 = ctx.r[11].s64 + -26944;
	// 826691B8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826691BC: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 826691C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826691C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826691C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826691CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826691D0: 386AE8F0  addi r3, r10, -0x1710
	ctx.r[3].s64 = ctx.r[10].s64 + -5904;
	// 826691D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826691D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826691DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826691E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826691E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826691E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826691EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826691F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826691F4: 4BDFDC2D  bl 0x82466e20
	ctx.lr = 0x826691F8;
	sub_82466E20(ctx, base);
	// 826691F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826691FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82669208 size=24
    let mut pc: u32 = 0x82669208;
    'dispatch: loop {
        match pc {
            0x82669208 => {
    //   block [0x82669208..0x82669220)
	// 82669208: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266920C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82669210: 394ACA28  addi r10, r10, -0x35d8
	ctx.r[10].s64 = ctx.r[10].s64 + -13784;
	// 82669214: 816B96BC  lwz r11, -0x6944(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26948 as u32) ) } as u64;
	// 82669218: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8266921C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669220 size=116
    let mut pc: u32 = 0x82669220;
    'dispatch: loop {
        match pc {
            0x82669220 => {
    //   block [0x82669220..0x82669294)
	// 82669220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266922C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669230: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82669234: 390BCA28  addi r8, r11, -0x35d8
	ctx.r[8].s64 = ctx.r[11].s64 + -13784;
	// 82669238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266923C: 392AFC78  addi r9, r10, -0x388
	ctx.r[9].s64 = ctx.r[10].s64 + -904;
	// 82669240: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669244: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82669248: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266924C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669254: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266925C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669264: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82669268: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 8266926C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82669270: 386BE920  addi r3, r11, -0x16e0
	ctx.r[3].s64 = ctx.r[11].s64 + -5856;
	// 82669274: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82669278: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266927C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669280: 4BDFDBA1  bl 0x82466e20
	ctx.lr = 0x82669284;
	sub_82466E20(ctx, base);
	// 82669284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266928C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669298 size=100
    let mut pc: u32 = 0x82669298;
    'dispatch: loop {
        match pc {
            0x82669298 => {
    //   block [0x82669298..0x826692FC)
	// 82669298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266929C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826692A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826692A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826692A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826692AC: 38AAE920  addi r5, r10, -0x16e0
	ctx.r[5].s64 = ctx.r[10].s64 + -5856;
	// 826692B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826692B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826692B8: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 826692BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826692C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826692C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826692C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826692CC: 386AE950  addi r3, r10, -0x16b0
	ctx.r[3].s64 = ctx.r[10].s64 + -5808;
	// 826692D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826692D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826692D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826692DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826692E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826692E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826692E8: 4BDFDB39  bl 0x82466e20
	ctx.lr = 0x826692EC;
	sub_82466E20(ctx, base);
	// 826692EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826692F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826692F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826692F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669300 size=100
    let mut pc: u32 = 0x82669300;
    'dispatch: loop {
        match pc {
            0x82669300 => {
    //   block [0x82669300..0x82669364)
	// 82669300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266930C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669314: 38AAE9B0  addi r5, r10, -0x1650
	ctx.r[5].s64 = ctx.r[10].s64 + -5712;
	// 82669318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266931C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669320: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 82669324: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266932C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669334: 386AE980  addi r3, r10, -0x1680
	ctx.r[3].s64 = ctx.r[10].s64 + -5760;
	// 82669338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266933C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669340: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82669344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669348: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266934C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669350: 4BDFDAD1  bl 0x82466e20
	ctx.lr = 0x82669354;
	sub_82466E20(ctx, base);
	// 82669354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266935C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669368 size=112
    let mut pc: u32 = 0x82669368;
    'dispatch: loop {
        match pc {
            0x82669368 => {
    //   block [0x82669368..0x826693D8)
	// 82669368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266936C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669374: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669378: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266937C: 38AAE920  addi r5, r10, -0x16e0
	ctx.r[5].s64 = ctx.r[10].s64 + -5856;
	// 82669380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669384: 390B9768  addi r8, r11, -0x6898
	ctx.r[8].s64 = ctx.r[11].s64 + -26776;
	// 82669388: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266938C: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 82669390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669394: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266939C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826693A0: 386AE9B0  addi r3, r10, -0x1650
	ctx.r[3].s64 = ctx.r[10].s64 + -5712;
	// 826693A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826693A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826693AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826693B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826693B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826693B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826693BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826693C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826693C4: 4BDFDA5D  bl 0x82466e20
	ctx.lr = 0x826693C8;
	sub_82466E20(ctx, base);
	// 826693C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826693CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826693D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826693D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826693D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826693D8 size=100
    let mut pc: u32 = 0x826693D8;
    'dispatch: loop {
        match pc {
            0x826693D8 => {
    //   block [0x826693D8..0x8266943C)
	// 826693D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826693DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826693E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826693E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826693E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826693EC: 38AAE9B0  addi r5, r10, -0x1650
	ctx.r[5].s64 = ctx.r[10].s64 + -5712;
	// 826693F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826693F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826693F8: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 826693FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266940C: 386AE9E0  addi r3, r10, -0x1620
	ctx.r[3].s64 = ctx.r[10].s64 + -5664;
	// 82669410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669414: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669418: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266941C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669420: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82669424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669428: 4BDFD9F9  bl 0x82466e20
	ctx.lr = 0x8266942C;
	sub_82466E20(ctx, base);
	// 8266942C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669440 size=100
    let mut pc: u32 = 0x82669440;
    'dispatch: loop {
        match pc {
            0x82669440 => {
    //   block [0x82669440..0x826694A4)
	// 82669440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266944C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669454: 38AAE920  addi r5, r10, -0x16e0
	ctx.r[5].s64 = ctx.r[10].s64 + -5856;
	// 82669458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266945C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669460: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 82669464: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266946C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669474: 386AEA10  addi r3, r10, -0x15f0
	ctx.r[3].s64 = ctx.r[10].s64 + -5616;
	// 82669478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266947C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669480: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82669484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669488: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266948C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669490: 4BDFD991  bl 0x82466e20
	ctx.lr = 0x82669494;
	sub_82466E20(ctx, base);
	// 82669494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266949C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826694A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826694A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826694A8 size=100
    let mut pc: u32 = 0x826694A8;
    'dispatch: loop {
        match pc {
            0x826694A8 => {
    //   block [0x826694A8..0x8266950C)
	// 826694A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826694AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826694B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826694B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826694B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826694BC: 38AAE950  addi r5, r10, -0x16b0
	ctx.r[5].s64 = ctx.r[10].s64 + -5808;
	// 826694C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826694C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826694C8: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 826694CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826694D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826694D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826694D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826694DC: 386AEA40  addi r3, r10, -0x15c0
	ctx.r[3].s64 = ctx.r[10].s64 + -5568;
	// 826694E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826694E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826694E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826694EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826694F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826694F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826694F8: 4BDFD929  bl 0x82466e20
	ctx.lr = 0x826694FC;
	sub_82466E20(ctx, base);
	// 826694FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669510 size=100
    let mut pc: u32 = 0x82669510;
    'dispatch: loop {
        match pc {
            0x82669510 => {
    //   block [0x82669510..0x82669574)
	// 82669510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266951C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669524: 38AAEA10  addi r5, r10, -0x15f0
	ctx.r[5].s64 = ctx.r[10].s64 + -5616;
	// 82669528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266952C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669530: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 82669534: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266953C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669544: 386AEA70  addi r3, r10, -0x1590
	ctx.r[3].s64 = ctx.r[10].s64 + -5520;
	// 82669548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266954C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669550: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82669554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669558: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266955C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669560: 4BDFD8C1  bl 0x82466e20
	ctx.lr = 0x82669564;
	sub_82466E20(ctx, base);
	// 82669564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266956C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669578 size=100
    let mut pc: u32 = 0x82669578;
    'dispatch: loop {
        match pc {
            0x82669578 => {
    //   block [0x82669578..0x826695DC)
	// 82669578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266957C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669584: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266958C: 38AAE950  addi r5, r10, -0x16b0
	ctx.r[5].s64 = ctx.r[10].s64 + -5808;
	// 82669590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669598: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 8266959C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826695A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826695A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826695A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826695AC: 386AEAA0  addi r3, r10, -0x1560
	ctx.r[3].s64 = ctx.r[10].s64 + -5472;
	// 826695B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826695B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826695B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826695BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826695C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826695C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826695C8: 4BDFD859  bl 0x82466e20
	ctx.lr = 0x826695CC;
	sub_82466E20(ctx, base);
	// 826695CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826695D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826695D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826695D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826695E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826695E0 size=112
    let mut pc: u32 = 0x826695E0;
    'dispatch: loop {
        match pc {
            0x826695E0 => {
    //   block [0x826695E0..0x82669650)
	// 826695E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826695E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826695E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826695EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826695F0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826695F4: 38AAEB30  addi r5, r10, -0x14d0
	ctx.r[5].s64 = ctx.r[10].s64 + -5328;
	// 826695F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826695FC: 390B9798  addi r8, r11, -0x6868
	ctx.r[8].s64 = ctx.r[11].s64 + -26728;
	// 82669600: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82669604: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 82669608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266960C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669618: 386AEAD0  addi r3, r10, -0x1530
	ctx.r[3].s64 = ctx.r[10].s64 + -5424;
	// 8266961C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266962C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266963C: 4BDFD7E5  bl 0x82466e20
	ctx.lr = 0x82669640;
	sub_82466E20(ctx, base);
	// 82669640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266964C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669650 size=112
    let mut pc: u32 = 0x82669650;
    'dispatch: loop {
        match pc {
            0x82669650 => {
    //   block [0x82669650..0x826696C0)
	// 82669650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266965C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669660: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669664: 38AAEB60  addi r5, r10, -0x14a0
	ctx.r[5].s64 = ctx.r[10].s64 + -5280;
	// 82669668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266966C: 390B97C8  addi r8, r11, -0x6838
	ctx.r[8].s64 = ctx.r[11].s64 + -26680;
	// 82669670: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82669674: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 82669678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266967C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669688: 386AEB00  addi r3, r10, -0x1500
	ctx.r[3].s64 = ctx.r[10].s64 + -5376;
	// 8266968C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266969C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826696A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826696A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826696A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826696AC: 4BDFD775  bl 0x82466e20
	ctx.lr = 0x826696B0;
	sub_82466E20(ctx, base);
	// 826696B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826696B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826696B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826696BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826696C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826696C0 size=112
    let mut pc: u32 = 0x826696C0;
    'dispatch: loop {
        match pc {
            0x826696C0 => {
    //   block [0x826696C0..0x82669730)
	// 826696C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826696C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826696C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826696CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826696D0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826696D4: 38AAEC20  addi r5, r10, -0x13e0
	ctx.r[5].s64 = ctx.r[10].s64 + -5088;
	// 826696D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826696DC: 390B97E0  addi r8, r11, -0x6820
	ctx.r[8].s64 = ctx.r[11].s64 + -26656;
	// 826696E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826696E4: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 826696E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826696EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826696F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826696F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826696F8: 386AEB30  addi r3, r10, -0x14d0
	ctx.r[3].s64 = ctx.r[10].s64 + -5328;
	// 826696FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266970C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266971C: 4BDFD705  bl 0x82466e20
	ctx.lr = 0x82669720;
	sub_82466E20(ctx, base);
	// 82669720: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266972C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669730 size=112
    let mut pc: u32 = 0x82669730;
    'dispatch: loop {
        match pc {
            0x82669730 => {
    //   block [0x82669730..0x826697A0)
	// 82669730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266973C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669740: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669744: 38AAEB30  addi r5, r10, -0x14d0
	ctx.r[5].s64 = ctx.r[10].s64 + -5328;
	// 82669748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266974C: 390B9810  addi r8, r11, -0x67f0
	ctx.r[8].s64 = ctx.r[11].s64 + -26608;
	// 82669750: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82669754: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 82669758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266975C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669768: 386AEB60  addi r3, r10, -0x14a0
	ctx.r[3].s64 = ctx.r[10].s64 + -5280;
	// 8266976C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266977C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266978C: 4BDFD695  bl 0x82466e20
	ctx.lr = 0x82669790;
	sub_82466E20(ctx, base);
	// 82669790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266979C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826697A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826697A0 size=112
    let mut pc: u32 = 0x826697A0;
    'dispatch: loop {
        match pc {
            0x826697A0 => {
    //   block [0x826697A0..0x82669810)
	// 826697A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826697A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826697A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826697AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826697B0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826697B4: 38AAEB60  addi r5, r10, -0x14a0
	ctx.r[5].s64 = ctx.r[10].s64 + -5280;
	// 826697B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826697BC: 390B9828  addi r8, r11, -0x67d8
	ctx.r[8].s64 = ctx.r[11].s64 + -26584;
	// 826697C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826697C4: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 826697C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826697CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826697D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826697D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826697D8: 386AEB90  addi r3, r10, -0x1470
	ctx.r[3].s64 = ctx.r[10].s64 + -5232;
	// 826697DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826697E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826697E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826697E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826697EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826697F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826697F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826697F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826697FC: 4BDFD625  bl 0x82466e20
	ctx.lr = 0x82669800;
	sub_82466E20(ctx, base);
	// 82669800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266980C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669810 size=116
    let mut pc: u32 = 0x82669810;
    'dispatch: loop {
        match pc {
            0x82669810 => {
    //   block [0x82669810..0x82669884)
	// 82669810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266981C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82669820: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82669824: 390A9840  addi r8, r10, -0x67c0
	ctx.r[8].s64 = ctx.r[10].s64 + -26560;
	// 82669828: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266982C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82669830: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82669834: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669838: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266983C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669844: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 82669848: 396BFC8C  addi r11, r11, -0x374
	ctx.r[11].s64 = ctx.r[11].s64 + -884;
	// 8266984C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669850: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669854: 386AEBC0  addi r3, r10, -0x1440
	ctx.r[3].s64 = ctx.r[10].s64 + -5184;
	// 82669858: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8266985C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669860: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82669864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266986C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669870: 4BDFD5B1  bl 0x82466e20
	ctx.lr = 0x82669874;
	sub_82466E20(ctx, base);
	// 82669874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266987C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82669888 size=48
    let mut pc: u32 = 0x82669888;
    'dispatch: loop {
        match pc {
            0x82669888 => {
    //   block [0x82669888..0x826698B8)
	// 82669888: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266988C: 814B98F4  lwz r10, -0x670c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26380 as u32) ) } as u64;
	// 82669890: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669894: 396BCAE8  addi r11, r11, -0x3518
	ctx.r[11].s64 = ctx.r[11].s64 + -13592;
	// 82669898: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8266989C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826698A0: 814A98F0  lwz r10, -0x6710(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26384 as u32) ) } as u64;
	// 826698A4: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826698A8: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826698AC: 814A98EC  lwz r10, -0x6714(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26388 as u32) ) } as u64;
	// 826698B0: 914B0350  stw r10, 0x350(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(848 as u32), ctx.r[10].u32 ) };
	// 826698B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826698B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826698B8 size=116
    let mut pc: u32 = 0x826698B8;
    'dispatch: loop {
        match pc {
            0x826698B8 => {
    //   block [0x826698B8..0x8266992C)
	// 826698B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826698BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826698C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826698C4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826698C8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826698CC: 392BFD60  addi r9, r11, -0x2a0
	ctx.r[9].s64 = ctx.r[11].s64 + -672;
	// 826698D0: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826698D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826698D8: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 826698DC: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 826698E0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826698E4: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 826698E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826698EC: 396BCAE8  addi r11, r11, -0x3518
	ctx.r[11].s64 = ctx.r[11].s64 + -13592;
	// 826698F0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826698F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826698F8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826698FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669900: 386AEBF0  addi r3, r10, -0x1410
	ctx.r[3].s64 = ctx.r[10].s64 + -5136;
	// 82669904: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82669908: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8266990C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669910: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82669914: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82669918: 4BDFD509  bl 0x82466e20
	ctx.lr = 0x8266991C;
	sub_82466E20(ctx, base);
	// 8266991C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669930 size=116
    let mut pc: u32 = 0x82669930;
    'dispatch: loop {
        match pc {
            0x82669930 => {
    //   block [0x82669930..0x826699A4)
	// 82669930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266993C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669940: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82669944: 390B9900  addi r8, r11, -0x6700
	ctx.r[8].s64 = ctx.r[11].s64 + -26368;
	// 82669948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266994C: 392AFF00  addi r9, r10, -0x100
	ctx.r[9].s64 = ctx.r[10].s64 + -256;
	// 82669950: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669954: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82669958: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266995C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669964: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266996C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669974: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82669978: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 8266997C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82669980: 386BEC20  addi r3, r11, -0x13e0
	ctx.r[3].s64 = ctx.r[11].s64 + -5088;
	// 82669984: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82669988: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266998C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669990: 4BDFD491  bl 0x82466e20
	ctx.lr = 0x82669994;
	sub_82466E20(ctx, base);
	// 82669994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266999C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826699A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826699A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826699A8 size=112
    let mut pc: u32 = 0x826699A8;
    'dispatch: loop {
        match pc {
            0x826699A8 => {
    //   block [0x826699A8..0x82669A18)
	// 826699A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826699AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826699B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826699B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826699B8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826699BC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826699C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826699C4: 390B9990  addi r8, r11, -0x6670
	ctx.r[8].s64 = ctx.r[11].s64 + -26224;
	// 826699C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826699CC: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 826699D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826699D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826699D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826699DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826699E0: 386AEC50  addi r3, r10, -0x13b0
	ctx.r[3].s64 = ctx.r[10].s64 + -5040;
	// 826699E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826699E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826699EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826699F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826699F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826699F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826699FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669A04: 4BDFD41D  bl 0x82466e20
	ctx.lr = 0x82669A08;
	sub_82466E20(ctx, base);
	// 82669A08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669A18 size=108
    let mut pc: u32 = 0x82669A18;
    'dispatch: loop {
        match pc {
            0x82669A18 => {
    //   block [0x82669A18..0x82669A84)
	// 82669A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669A24: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669A28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82669A2C: 38EB99A8  addi r7, r11, -0x6658
	ctx.r[7].s64 = ctx.r[11].s64 + -26200;
	// 82669A30: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82669A34: 388A0E48  addi r4, r10, 0xe48
	ctx.r[4].s64 = ctx.r[10].s64 + 3656;
	// 82669A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669A3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669A40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82669A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669A48: 386AEC80  addi r3, r10, -0x1380
	ctx.r[3].s64 = ctx.r[10].s64 + -4992;
	// 82669A4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82669A50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669A6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82669A70: 4BDFD3B1  bl 0x82466e20
	ctx.lr = 0x82669A74;
	sub_82466E20(ctx, base);
	// 82669A74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669A88 size=112
    let mut pc: u32 = 0x82669A88;
    'dispatch: loop {
        match pc {
            0x82669A88 => {
    //   block [0x82669A88..0x82669AF8)
	// 82669A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669A94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669A98: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669A9C: 38AACB20  addi r5, r10, -0x34e0
	ctx.r[5].s64 = ctx.r[10].s64 + -13536;
	// 82669AA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669AA4: 390B9A20  addi r8, r11, -0x65e0
	ctx.r[8].s64 = ctx.r[11].s64 + -26080;
	// 82669AA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82669AAC: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 82669AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669AB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669AB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669AC0: 386AECB0  addi r3, r10, -0x1350
	ctx.r[3].s64 = ctx.r[10].s64 + -4944;
	// 82669AC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669ACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669AD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669AE4: 4BDFD33D  bl 0x82466e20
	ctx.lr = 0x82669AE8;
	sub_82466E20(ctx, base);
	// 82669AE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669AF8 size=108
    let mut pc: u32 = 0x82669AF8;
    'dispatch: loop {
        match pc {
            0x82669AF8 => {
    //   block [0x82669AF8..0x82669B64)
	// 82669AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669B04: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669B08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669B0C: 38EB9A38  addi r7, r11, -0x65c8
	ctx.r[7].s64 = ctx.r[11].s64 + -26056;
	// 82669B10: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82669B14: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 82669B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669B1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669B20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82669B24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669B28: 386AECE0  addi r3, r10, -0x1320
	ctx.r[3].s64 = ctx.r[10].s64 + -4896;
	// 82669B2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82669B30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669B44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669B4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82669B50: 4BDFD2D1  bl 0x82466e20
	ctx.lr = 0x82669B54;
	sub_82466E20(ctx, base);
	// 82669B54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669B68 size=112
    let mut pc: u32 = 0x82669B68;
    'dispatch: loop {
        match pc {
            0x82669B68 => {
    //   block [0x82669B68..0x82669BD8)
	// 82669B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669B74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669B78: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669B7C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82669B80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669B84: 390B9A50  addi r8, r11, -0x65b0
	ctx.r[8].s64 = ctx.r[11].s64 + -26032;
	// 82669B88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82669B8C: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 82669B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669B94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669B98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669BA0: 386AED10  addi r3, r10, -0x12f0
	ctx.r[3].s64 = ctx.r[10].s64 + -4848;
	// 82669BA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669BA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669BAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669BB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669BB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669BB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669BC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669BC4: 4BDFD25D  bl 0x82466e20
	ctx.lr = 0x82669BC8;
	sub_82466E20(ctx, base);
	// 82669BC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669BD8 size=108
    let mut pc: u32 = 0x82669BD8;
    'dispatch: loop {
        match pc {
            0x82669BD8 => {
    //   block [0x82669BD8..0x82669C44)
	// 82669BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669BE4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669BE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669BEC: 38EB9A98  addi r7, r11, -0x6568
	ctx.r[7].s64 = ctx.r[11].s64 + -25960;
	// 82669BF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82669BF4: 388A5040  addi r4, r10, 0x5040
	ctx.r[4].s64 = ctx.r[10].s64 + 20544;
	// 82669BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669BFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669C00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82669C04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669C08: 386AED40  addi r3, r10, -0x12c0
	ctx.r[3].s64 = ctx.r[10].s64 + -4800;
	// 82669C0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82669C10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669C18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669C20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669C28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669C2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82669C30: 4BDFD1F1  bl 0x82466e20
	ctx.lr = 0x82669C34;
	sub_82466E20(ctx, base);
	// 82669C34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669C38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669C3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669C48 size=108
    let mut pc: u32 = 0x82669C48;
    'dispatch: loop {
        match pc {
            0x82669C48 => {
    //   block [0x82669C48..0x82669CB4)
	// 82669C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669C54: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669C58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669C5C: 38EB9AC8  addi r7, r11, -0x6538
	ctx.r[7].s64 = ctx.r[11].s64 + -25912;
	// 82669C60: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82669C64: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 82669C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669C6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669C70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82669C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669C78: 386AED70  addi r3, r10, -0x1290
	ctx.r[3].s64 = ctx.r[10].s64 + -4752;
	// 82669C7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82669C80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669C9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82669CA0: 4BDFD181  bl 0x82466e20
	ctx.lr = 0x82669CA4;
	sub_82466E20(ctx, base);
	// 82669CA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669CB8 size=112
    let mut pc: u32 = 0x82669CB8;
    'dispatch: loop {
        match pc {
            0x82669CB8 => {
    //   block [0x82669CB8..0x82669D28)
	// 82669CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669CC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669CC8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669CCC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82669CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669CD4: 390B9AE0  addi r8, r11, -0x6520
	ctx.r[8].s64 = ctx.r[11].s64 + -25888;
	// 82669CD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82669CDC: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 82669CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669CE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669CE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669CF0: 386AEDA0  addi r3, r10, -0x1260
	ctx.r[3].s64 = ctx.r[10].s64 + -4704;
	// 82669CF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669D14: 4BDFD10D  bl 0x82466e20
	ctx.lr = 0x82669D18;
	sub_82466E20(ctx, base);
	// 82669D18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669D28 size=112
    let mut pc: u32 = 0x82669D28;
    'dispatch: loop {
        match pc {
            0x82669D28 => {
    //   block [0x82669D28..0x82669D98)
	// 82669D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669D30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669D34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669D38: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669D3C: 38AADDB0  addi r5, r10, -0x2250
	ctx.r[5].s64 = ctx.r[10].s64 + -8784;
	// 82669D40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669D44: 390B9B10  addi r8, r11, -0x64f0
	ctx.r[8].s64 = ctx.r[11].s64 + -25840;
	// 82669D48: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82669D4C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 82669D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669D54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669D58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669D60: 386AEDD0  addi r3, r10, -0x1230
	ctx.r[3].s64 = ctx.r[10].s64 + -4656;
	// 82669D64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669D84: 4BDFD09D  bl 0x82466e20
	ctx.lr = 0x82669D88;
	sub_82466E20(ctx, base);
	// 82669D88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669D98 size=112
    let mut pc: u32 = 0x82669D98;
    'dispatch: loop {
        match pc {
            0x82669D98 => {
    //   block [0x82669D98..0x82669E08)
	// 82669D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669DA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669DA8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669DAC: 38AADDB0  addi r5, r10, -0x2250
	ctx.r[5].s64 = ctx.r[10].s64 + -8784;
	// 82669DB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669DB4: 390B9B58  addi r8, r11, -0x64a8
	ctx.r[8].s64 = ctx.r[11].s64 + -25768;
	// 82669DB8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82669DBC: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 82669DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669DC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669DC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669DD0: 386AEE00  addi r3, r10, -0x1200
	ctx.r[3].s64 = ctx.r[10].s64 + -4608;
	// 82669DD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669DD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669DDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669DE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669DE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669DEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669DF4: 4BDFD02D  bl 0x82466e20
	ctx.lr = 0x82669DF8;
	sub_82466E20(ctx, base);
	// 82669DF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669E08 size=112
    let mut pc: u32 = 0x82669E08;
    'dispatch: loop {
        match pc {
            0x82669E08 => {
    //   block [0x82669E08..0x82669E78)
	// 82669E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669E14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669E18: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669E1C: 38AADDE0  addi r5, r10, -0x2220
	ctx.r[5].s64 = ctx.r[10].s64 + -8736;
	// 82669E20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669E24: 390B9BB8  addi r8, r11, -0x6448
	ctx.r[8].s64 = ctx.r[11].s64 + -25672;
	// 82669E28: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82669E2C: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 82669E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669E34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669E38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669E40: 386AEE30  addi r3, r10, -0x11d0
	ctx.r[3].s64 = ctx.r[10].s64 + -4560;
	// 82669E44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669E48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669E4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669E50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669E54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669E58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669E64: 4BDFCFBD  bl 0x82466e20
	ctx.lr = 0x82669E68;
	sub_82466E20(ctx, base);
	// 82669E68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669E78 size=112
    let mut pc: u32 = 0x82669E78;
    'dispatch: loop {
        match pc {
            0x82669E78 => {
    //   block [0x82669E78..0x82669EE8)
	// 82669E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669E84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669E88: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669E8C: 38AADDE0  addi r5, r10, -0x2220
	ctx.r[5].s64 = ctx.r[10].s64 + -8736;
	// 82669E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669E94: 390B9C18  addi r8, r11, -0x63e8
	ctx.r[8].s64 = ctx.r[11].s64 + -25576;
	// 82669E98: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82669E9C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 82669EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669EA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669EA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669EB0: 386AEE60  addi r3, r10, -0x11a0
	ctx.r[3].s64 = ctx.r[10].s64 + -4512;
	// 82669EB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669ED4: 4BDFCF4D  bl 0x82466e20
	ctx.lr = 0x82669ED8;
	sub_82466E20(ctx, base);
	// 82669ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669EE8 size=112
    let mut pc: u32 = 0x82669EE8;
    'dispatch: loop {
        match pc {
            0x82669EE8 => {
    //   block [0x82669EE8..0x82669F58)
	// 82669EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669EF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669EF8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669EFC: 38AADDB0  addi r5, r10, -0x2250
	ctx.r[5].s64 = ctx.r[10].s64 + -8784;
	// 82669F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669F04: 390B9C78  addi r8, r11, -0x6388
	ctx.r[8].s64 = ctx.r[11].s64 + -25480;
	// 82669F08: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82669F0C: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 82669F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669F14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669F18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669F1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669F20: 386AEE90  addi r3, r10, -0x1170
	ctx.r[3].s64 = ctx.r[10].s64 + -4464;
	// 82669F24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669F44: 4BDFCEDD  bl 0x82466e20
	ctx.lr = 0x82669F48;
	sub_82466E20(ctx, base);
	// 82669F48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669F58 size=112
    let mut pc: u32 = 0x82669F58;
    'dispatch: loop {
        match pc {
            0x82669F58 => {
    //   block [0x82669F58..0x82669FC8)
	// 82669F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669F64: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82669F68: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 82669F6C: 38EA9D38  addi r7, r10, -0x62c8
	ctx.r[7].s64 = ctx.r[10].s64 + -25288;
	// 82669F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669F74: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82669F78: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 82669F7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669F80: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82669F84: 396BFF18  addi r11, r11, -0xe8
	ctx.r[11].s64 = ctx.r[11].s64 + -232;
	// 82669F88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82669F8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669F90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669F94: 386AEEC0  addi r3, r10, -0x1140
	ctx.r[3].s64 = ctx.r[10].s64 + -4416;
	// 82669F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669F9C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82669FA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669FA4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82669FA8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669FAC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669FB0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82669FB4: 4BDFCE6D  bl 0x82466e20
	ctx.lr = 0x82669FB8;
	sub_82466E20(ctx, base);
	// 82669FB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669FC8 size=112
    let mut pc: u32 = 0x82669FC8;
    'dispatch: loop {
        match pc {
            0x82669FC8 => {
    //   block [0x82669FC8..0x8266A038)
	// 82669FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669FD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669FD8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669FDC: 38AACBE0  addi r5, r10, -0x3420
	ctx.r[5].s64 = ctx.r[10].s64 + -13344;
	// 82669FE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669FE4: 390B9F00  addi r8, r11, -0x6100
	ctx.r[8].s64 = ctx.r[11].s64 + -24832;
	// 82669FE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82669FEC: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 82669FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669FF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669FF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A000: 386AEEF0  addi r3, r10, -0x1110
	ctx.r[3].s64 = ctx.r[10].s64 + -4368;
	// 8266A004: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A00C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A014: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266A018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A01C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A024: 4BDFCDFD  bl 0x82466e20
	ctx.lr = 0x8266A028;
	sub_82466E20(ctx, base);
	// 8266A028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A02C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A038 size=112
    let mut pc: u32 = 0x8266A038;
    'dispatch: loop {
        match pc {
            0x8266A038 => {
    //   block [0x8266A038..0x8266A0A8)
	// 8266A038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A044: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A048: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A04C: 38AACBE0  addi r5, r10, -0x3420
	ctx.r[5].s64 = ctx.r[10].s64 + -13344;
	// 8266A050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A054: 390B9F18  addi r8, r11, -0x60e8
	ctx.r[8].s64 = ctx.r[11].s64 + -24808;
	// 8266A058: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266A05C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 8266A060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A064: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A06C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A070: 386AEF20  addi r3, r10, -0x10e0
	ctx.r[3].s64 = ctx.r[10].s64 + -4320;
	// 8266A074: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A084: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266A088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A08C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A094: 4BDFCD8D  bl 0x82466e20
	ctx.lr = 0x8266A098;
	sub_82466E20(ctx, base);
	// 8266A098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A09C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A0A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A0A8 size=112
    let mut pc: u32 = 0x8266A0A8;
    'dispatch: loop {
        match pc {
            0x8266A0A8 => {
    //   block [0x8266A0A8..0x8266A118)
	// 8266A0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A0B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A0B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A0B8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A0BC: 38AACBE0  addi r5, r10, -0x3420
	ctx.r[5].s64 = ctx.r[10].s64 + -13344;
	// 8266A0C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A0C4: 390B9F30  addi r8, r11, -0x60d0
	ctx.r[8].s64 = ctx.r[11].s64 + -24784;
	// 8266A0C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266A0CC: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 8266A0D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A0D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A0D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A0DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A0E0: 386AEF50  addi r3, r10, -0x10b0
	ctx.r[3].s64 = ctx.r[10].s64 + -4272;
	// 8266A0E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A0E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A0EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A0F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A0F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A0F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A0FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A104: 4BDFCD1D  bl 0x82466e20
	ctx.lr = 0x8266A108;
	sub_82466E20(ctx, base);
	// 8266A108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A10C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A118 size=108
    let mut pc: u32 = 0x8266A118;
    'dispatch: loop {
        match pc {
            0x8266A118 => {
    //   block [0x8266A118..0x8266A184)
	// 8266A118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A124: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A12C: 38EB9F60  addi r7, r11, -0x60a0
	ctx.r[7].s64 = ctx.r[11].s64 + -24736;
	// 8266A130: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266A134: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 8266A138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A13C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A140: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A148: 386AEF80  addi r3, r10, -0x1080
	ctx.r[3].s64 = ctx.r[10].s64 + -4224;
	// 8266A14C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A15C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A16C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A170: 4BDFCCB1  bl 0x82466e20
	ctx.lr = 0x8266A174;
	sub_82466E20(ctx, base);
	// 8266A174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A17C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A188 size=112
    let mut pc: u32 = 0x8266A188;
    'dispatch: loop {
        match pc {
            0x8266A188 => {
    //   block [0x8266A188..0x8266A1F8)
	// 8266A188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A194: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A198: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A19C: 38AACBE0  addi r5, r10, -0x3420
	ctx.r[5].s64 = ctx.r[10].s64 + -13344;
	// 8266A1A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A1A4: 390B9F90  addi r8, r11, -0x6070
	ctx.r[8].s64 = ctx.r[11].s64 + -24688;
	// 8266A1A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266A1AC: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 8266A1B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A1B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A1B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A1BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A1C0: 386AEFB0  addi r3, r10, -0x1050
	ctx.r[3].s64 = ctx.r[10].s64 + -4176;
	// 8266A1C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A1C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A1CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A1D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A1D4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266A1D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A1DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A1E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A1E4: 4BDFCC3D  bl 0x82466e20
	ctx.lr = 0x8266A1E8;
	sub_82466E20(ctx, base);
	// 8266A1E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A1EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A1F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A1F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A1F8 size=108
    let mut pc: u32 = 0x8266A1F8;
    'dispatch: loop {
        match pc {
            0x8266A1F8 => {
    //   block [0x8266A1F8..0x8266A264)
	// 8266A1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A204: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A20C: 38EB9FA8  addi r7, r11, -0x6058
	ctx.r[7].s64 = ctx.r[11].s64 + -24664;
	// 8266A210: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266A214: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 8266A218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A21C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A220: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A228: 386AEFE0  addi r3, r10, -0x1020
	ctx.r[3].s64 = ctx.r[10].s64 + -4128;
	// 8266A22C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A23C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A24C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A250: 4BDFCBD1  bl 0x82466e20
	ctx.lr = 0x8266A254;
	sub_82466E20(ctx, base);
	// 8266A254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A25C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A268 size=108
    let mut pc: u32 = 0x8266A268;
    'dispatch: loop {
        match pc {
            0x8266A268 => {
    //   block [0x8266A268..0x8266A2D4)
	// 8266A268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A26C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A274: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A27C: 38EB9FD8  addi r7, r11, -0x6028
	ctx.r[7].s64 = ctx.r[11].s64 + -24616;
	// 8266A280: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266A284: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 8266A288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A28C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A298: 386AF010  addi r3, r10, -0xff0
	ctx.r[3].s64 = ctx.r[10].s64 + -4080;
	// 8266A29C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A2A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A2A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A2A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A2AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A2B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A2B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A2B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A2BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A2C0: 4BDFCB61  bl 0x82466e20
	ctx.lr = 0x8266A2C4;
	sub_82466E20(ctx, base);
	// 8266A2C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A2C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A2CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A2D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A2D8 size=112
    let mut pc: u32 = 0x8266A2D8;
    'dispatch: loop {
        match pc {
            0x8266A2D8 => {
    //   block [0x8266A2D8..0x8266A348)
	// 8266A2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A2E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A2E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A2E8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A2EC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266A2F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A2F4: 390BA020  addi r8, r11, -0x5fe0
	ctx.r[8].s64 = ctx.r[11].s64 + -24544;
	// 8266A2F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266A2FC: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 8266A300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A304: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A308: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A30C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A310: 386AF040  addi r3, r10, -0xfc0
	ctx.r[3].s64 = ctx.r[10].s64 + -4032;
	// 8266A314: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A31C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A32C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A334: 4BDFCAED  bl 0x82466e20
	ctx.lr = 0x8266A338;
	sub_82466E20(ctx, base);
	// 8266A338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A33C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A348 size=112
    let mut pc: u32 = 0x8266A348;
    'dispatch: loop {
        match pc {
            0x8266A348 => {
    //   block [0x8266A348..0x8266A3B8)
	// 8266A348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A354: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A358: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A35C: 38AAEBC0  addi r5, r10, -0x1440
	ctx.r[5].s64 = ctx.r[10].s64 + -5184;
	// 8266A360: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266A364: 390BA068  addi r8, r11, -0x5f98
	ctx.r[8].s64 = ctx.r[11].s64 + -24472;
	// 8266A368: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266A36C: 388A0E58  addi r4, r10, 0xe58
	ctx.r[4].s64 = ctx.r[10].s64 + 3672;
	// 8266A370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A374: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A37C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A380: 386AF070  addi r3, r10, -0xf90
	ctx.r[3].s64 = ctx.r[10].s64 + -3984;
	// 8266A384: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A388: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A38C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A39C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A3A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A3A4: 4BDFCA7D  bl 0x82466e20
	ctx.lr = 0x8266A3A8;
	sub_82466E20(ctx, base);
	// 8266A3A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A3AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A3B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A3B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A3B8 size=108
    let mut pc: u32 = 0x8266A3B8;
    'dispatch: loop {
        match pc {
            0x8266A3B8 => {
    //   block [0x8266A3B8..0x8266A424)
	// 8266A3B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A3BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A3C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A3C4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A3C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266A3CC: 38EBA080  addi r7, r11, -0x5f80
	ctx.r[7].s64 = ctx.r[11].s64 + -24448;
	// 8266A3D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266A3D4: 388A0E74  addi r4, r10, 0xe74
	ctx.r[4].s64 = ctx.r[10].s64 + 3700;
	// 8266A3D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A3DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A3E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A3E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A3E8: 386AF0A0  addi r3, r10, -0xf60
	ctx.r[3].s64 = ctx.r[10].s64 + -3936;
	// 8266A3EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A3F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A3F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A3F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A3FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A40C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A410: 4BDFCA11  bl 0x82466e20
	ctx.lr = 0x8266A414;
	sub_82466E20(ctx, base);
	// 8266A414: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A41C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A428 size=108
    let mut pc: u32 = 0x8266A428;
    'dispatch: loop {
        match pc {
            0x8266A428 => {
    //   block [0x8266A428..0x8266A494)
	// 8266A428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A42C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A434: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A438: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266A43C: 38EBA0C8  addi r7, r11, -0x5f38
	ctx.r[7].s64 = ctx.r[11].s64 + -24376;
	// 8266A440: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266A444: 388A0E9C  addi r4, r10, 0xe9c
	ctx.r[4].s64 = ctx.r[10].s64 + 3740;
	// 8266A448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A44C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A450: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A458: 386AF0D0  addi r3, r10, -0xf30
	ctx.r[3].s64 = ctx.r[10].s64 + -3888;
	// 8266A45C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A46C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A47C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A480: 4BDFC9A1  bl 0x82466e20
	ctx.lr = 0x8266A484;
	sub_82466E20(ctx, base);
	// 8266A484: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A48C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A498 size=112
    let mut pc: u32 = 0x8266A498;
    'dispatch: loop {
        match pc {
            0x8266A498 => {
    //   block [0x8266A498..0x8266A508)
	// 8266A498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A4A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A4A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A4A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A4AC: 38AAF0D0  addi r5, r10, -0xf30
	ctx.r[5].s64 = ctx.r[10].s64 + -3888;
	// 8266A4B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266A4B4: 390BA0F8  addi r8, r11, -0x5f08
	ctx.r[8].s64 = ctx.r[11].s64 + -24328;
	// 8266A4B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266A4BC: 388A0EB4  addi r4, r10, 0xeb4
	ctx.r[4].s64 = ctx.r[10].s64 + 3764;
	// 8266A4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A4C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A4C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A4CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A4D0: 386AF100  addi r3, r10, -0xf00
	ctx.r[3].s64 = ctx.r[10].s64 + -3840;
	// 8266A4D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A4D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A4DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A4E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A4E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A4E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A4EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A4F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A4F4: 4BDFC92D  bl 0x82466e20
	ctx.lr = 0x8266A4F8;
	sub_82466E20(ctx, base);
	// 8266A4F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A4FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8266A508 size=24
    let mut pc: u32 = 0x8266A508;
    'dispatch: loop {
        match pc {
            0x8266A508 => {
    //   block [0x8266A508..0x8266A520)
	// 8266A508: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A50C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266A510: 394ACEA8  addi r10, r10, -0x3158
	ctx.r[10].s64 = ctx.r[10].s64 + -12632;
	// 8266A514: 816BA128  lwz r11, -0x5ed8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24280 as u32) ) } as u64;
	// 8266A518: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8266A51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A520 size=116
    let mut pc: u32 = 0x8266A520;
    'dispatch: loop {
        match pc {
            0x8266A520 => {
    //   block [0x8266A520..0x8266A594)
	// 8266A520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A52C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A530: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266A534: 390BCEA8  addi r8, r11, -0x3158
	ctx.r[8].s64 = ctx.r[11].s64 + -12632;
	// 8266A538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A53C: 392AFFB0  addi r9, r10, -0x50
	ctx.r[9].s64 = ctx.r[10].s64 + -80;
	// 8266A540: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A544: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 8266A548: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266A54C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A554: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266A558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A55C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A564: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8266A568: 388A0EDC  addi r4, r10, 0xedc
	ctx.r[4].s64 = ctx.r[10].s64 + 3804;
	// 8266A56C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266A570: 386BF130  addi r3, r11, -0xed0
	ctx.r[3].s64 = ctx.r[11].s64 + -3792;
	// 8266A574: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266A578: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A57C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A580: 4BDFC8A1  bl 0x82466e20
	ctx.lr = 0x8266A584;
	sub_82466E20(ctx, base);
	// 8266A584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A598 size=112
    let mut pc: u32 = 0x8266A598;
    'dispatch: loop {
        match pc {
            0x8266A598 => {
    //   block [0x8266A598..0x8266A608)
	// 8266A598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A5A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A5A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A5AC: 38AADDE0  addi r5, r10, -0x2220
	ctx.r[5].s64 = ctx.r[10].s64 + -8736;
	// 8266A5B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A5B4: 390BA130  addi r8, r11, -0x5ed0
	ctx.r[8].s64 = ctx.r[11].s64 + -24272;
	// 8266A5B8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8266A5BC: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 8266A5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A5C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A5C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A5CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A5D0: 386AF160  addi r3, r10, -0xea0
	ctx.r[3].s64 = ctx.r[10].s64 + -3744;
	// 8266A5D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A5D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A5DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A5E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A5E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A5E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A5EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A5F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A5F4: 4BDFC82D  bl 0x82466e20
	ctx.lr = 0x8266A5F8;
	sub_82466E20(ctx, base);
	// 8266A5F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A608 size=108
    let mut pc: u32 = 0x8266A608;
    'dispatch: loop {
        match pc {
            0x8266A608 => {
    //   block [0x8266A608..0x8266A674)
	// 8266A608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A614: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A61C: 38EBA1C0  addi r7, r11, -0x5e40
	ctx.r[7].s64 = ctx.r[11].s64 + -24128;
	// 8266A620: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266A624: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 8266A628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A62C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A638: 386AF190  addi r3, r10, -0xe70
	ctx.r[3].s64 = ctx.r[10].s64 + -3696;
	// 8266A63C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A64C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A65C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A660: 4BDFC7C1  bl 0x82466e20
	ctx.lr = 0x8266A664;
	sub_82466E20(ctx, base);
	// 8266A664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A66C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A678 size=108
    let mut pc: u32 = 0x8266A678;
    'dispatch: loop {
        match pc {
            0x8266A678 => {
    //   block [0x8266A678..0x8266A6E4)
	// 8266A678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A684: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A68C: 38EBA208  addi r7, r11, -0x5df8
	ctx.r[7].s64 = ctx.r[11].s64 + -24056;
	// 8266A690: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266A694: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 8266A698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A69C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A6A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A6A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A6A8: 386AF1C0  addi r3, r10, -0xe40
	ctx.r[3].s64 = ctx.r[10].s64 + -3648;
	// 8266A6AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A6B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A6B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A6B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A6C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A6C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A6C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A6CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A6D0: 4BDFC751  bl 0x82466e20
	ctx.lr = 0x8266A6D4;
	sub_82466E20(ctx, base);
	// 8266A6D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A6D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A6DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A6E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A6E8 size=108
    let mut pc: u32 = 0x8266A6E8;
    'dispatch: loop {
        match pc {
            0x8266A6E8 => {
    //   block [0x8266A6E8..0x8266A754)
	// 8266A6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A6F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A6F4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A6F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A6FC: 38EBA238  addi r7, r11, -0x5dc8
	ctx.r[7].s64 = ctx.r[11].s64 + -24008;
	// 8266A700: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266A704: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 8266A708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A70C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A710: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A718: 386AF1F0  addi r3, r10, -0xe10
	ctx.r[3].s64 = ctx.r[10].s64 + -3600;
	// 8266A71C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A73C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A740: 4BDFC6E1  bl 0x82466e20
	ctx.lr = 0x8266A744;
	sub_82466E20(ctx, base);
	// 8266A744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A74C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A758 size=112
    let mut pc: u32 = 0x8266A758;
    'dispatch: loop {
        match pc {
            0x8266A758 => {
    //   block [0x8266A758..0x8266A7C8)
	// 8266A758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A764: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A768: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A76C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266A770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A774: 390BA268  addi r8, r11, -0x5d98
	ctx.r[8].s64 = ctx.r[11].s64 + -23960;
	// 8266A778: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266A77C: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 8266A780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A784: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A78C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A790: 386AF220  addi r3, r10, -0xde0
	ctx.r[3].s64 = ctx.r[10].s64 + -3552;
	// 8266A794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A7A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A7A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A7A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A7AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A7B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A7B4: 4BDFC66D  bl 0x82466e20
	ctx.lr = 0x8266A7B8;
	sub_82466E20(ctx, base);
	// 8266A7B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A7C8 size=112
    let mut pc: u32 = 0x8266A7C8;
    'dispatch: loop {
        match pc {
            0x8266A7C8 => {
    //   block [0x8266A7C8..0x8266A838)
	// 8266A7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A7D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A7D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A7D8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A7DC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266A7E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A7E4: 390BA298  addi r8, r11, -0x5d68
	ctx.r[8].s64 = ctx.r[11].s64 + -23912;
	// 8266A7E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266A7EC: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 8266A7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A7F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A7F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A7FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A800: 386AF250  addi r3, r10, -0xdb0
	ctx.r[3].s64 = ctx.r[10].s64 + -3504;
	// 8266A804: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A80C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A81C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A824: 4BDFC5FD  bl 0x82466e20
	ctx.lr = 0x8266A828;
	sub_82466E20(ctx, base);
	// 8266A828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A82C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A838 size=112
    let mut pc: u32 = 0x8266A838;
    'dispatch: loop {
        match pc {
            0x8266A838 => {
    //   block [0x8266A838..0x8266A8A8)
	// 8266A838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A844: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A848: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A84C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266A850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A854: 390BA2B0  addi r8, r11, -0x5d50
	ctx.r[8].s64 = ctx.r[11].s64 + -23888;
	// 8266A858: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266A85C: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 8266A860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A864: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A868: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A86C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A870: 386AF280  addi r3, r10, -0xd80
	ctx.r[3].s64 = ctx.r[10].s64 + -3456;
	// 8266A874: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A87C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A88C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A894: 4BDFC58D  bl 0x82466e20
	ctx.lr = 0x8266A898;
	sub_82466E20(ctx, base);
	// 8266A898: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A8A8 size=108
    let mut pc: u32 = 0x8266A8A8;
    'dispatch: loop {
        match pc {
            0x8266A8A8 => {
    //   block [0x8266A8A8..0x8266A914)
	// 8266A8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A8B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A8B4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A8B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A8BC: 38EBA2C8  addi r7, r11, -0x5d38
	ctx.r[7].s64 = ctx.r[11].s64 + -23864;
	// 8266A8C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266A8C4: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 8266A8C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A8CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A8D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A8D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A8D8: 386AF2B0  addi r3, r10, -0xd50
	ctx.r[3].s64 = ctx.r[10].s64 + -3408;
	// 8266A8DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A8E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A8E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A8EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A8F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A8F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A8F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A8FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A900: 4BDFC521  bl 0x82466e20
	ctx.lr = 0x8266A904;
	sub_82466E20(ctx, base);
	// 8266A904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A90C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A918 size=112
    let mut pc: u32 = 0x8266A918;
    'dispatch: loop {
        match pc {
            0x8266A918 => {
    //   block [0x8266A918..0x8266A988)
	// 8266A918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A924: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A928: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A92C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266A930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A934: 390BA2F8  addi r8, r11, -0x5d08
	ctx.r[8].s64 = ctx.r[11].s64 + -23816;
	// 8266A938: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266A93C: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 8266A940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A944: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A94C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A950: 386AF2E0  addi r3, r10, -0xd20
	ctx.r[3].s64 = ctx.r[10].s64 + -3360;
	// 8266A954: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A96C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A974: 4BDFC4AD  bl 0x82466e20
	ctx.lr = 0x8266A978;
	sub_82466E20(ctx, base);
	// 8266A978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A97C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A988 size=108
    let mut pc: u32 = 0x8266A988;
    'dispatch: loop {
        match pc {
            0x8266A988 => {
    //   block [0x8266A988..0x8266A9F4)
	// 8266A988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A994: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A998: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A99C: 38EBA310  addi r7, r11, -0x5cf0
	ctx.r[7].s64 = ctx.r[11].s64 + -23792;
	// 8266A9A0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8266A9A4: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 8266A9A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A9AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A9B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A9B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A9B8: 386AF310  addi r3, r10, -0xcf0
	ctx.r[3].s64 = ctx.r[10].s64 + -3312;
	// 8266A9BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A9C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A9C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A9C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A9CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A9D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A9D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A9D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A9DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A9E0: 4BDFC441  bl 0x82466e20
	ctx.lr = 0x8266A9E4;
	sub_82466E20(ctx, base);
	// 8266A9E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A9E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A9EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A9F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A9F8 size=116
    let mut pc: u32 = 0x8266A9F8;
    'dispatch: loop {
        match pc {
            0x8266A9F8 => {
    //   block [0x8266A9F8..0x8266AA6C)
	// 8266A9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AA00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AA04: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266AA08: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 8266AA0C: 390AA400  addi r8, r10, -0x5c00
	ctx.r[8].s64 = ctx.r[10].s64 + -23552;
	// 8266AA10: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AA14: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8266AA18: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266AA1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AA20: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266AA24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AA28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266AA2C: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 8266AA30: 396BFFC8  addi r11, r11, -0x38
	ctx.r[11].s64 = ctx.r[11].s64 + -56;
	// 8266AA34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AA38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AA3C: 386AF340  addi r3, r10, -0xcc0
	ctx.r[3].s64 = ctx.r[10].s64 + -3264;
	// 8266AA40: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8266AA44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AA48: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8266AA4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AA54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AA58: 4BDFC3C9  bl 0x82466e20
	ctx.lr = 0x8266AA5C;
	sub_82466E20(ctx, base);
	// 8266AA5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AA60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AA64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AA68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AA70 size=108
    let mut pc: u32 = 0x8266AA70;
    'dispatch: loop {
        match pc {
            0x8266AA70 => {
    //   block [0x8266AA70..0x8266AADC)
	// 8266AA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AA78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AA7C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266AA80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AA84: 38EBA5C8  addi r7, r11, -0x5a38
	ctx.r[7].s64 = ctx.r[11].s64 + -23096;
	// 8266AA88: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 8266AA8C: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 8266AA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AA94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AA98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266AA9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AAA0: 386AF370  addi r3, r10, -0xc90
	ctx.r[3].s64 = ctx.r[10].s64 + -3216;
	// 8266AAA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266AAA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AAAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AAB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AAB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AAB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AAC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AAC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266AAC8: 4BDFC359  bl 0x82466e20
	ctx.lr = 0x8266AACC;
	sub_82466E20(ctx, base);
	// 8266AACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AAD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AAD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AAE0 size=112
    let mut pc: u32 = 0x8266AAE0;
    'dispatch: loop {
        match pc {
            0x8266AAE0 => {
    //   block [0x8266AAE0..0x8266AB50)
	// 8266AAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AAE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AAEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AAF0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266AAF4: 38AADDE0  addi r5, r10, -0x2220
	ctx.r[5].s64 = ctx.r[10].s64 + -8736;
	// 8266AAF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AAFC: 390BA760  addi r8, r11, -0x58a0
	ctx.r[8].s64 = ctx.r[11].s64 + -22688;
	// 8266AB00: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 8266AB04: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 8266AB08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AB0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AB10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266AB14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AB18: 386AF3A0  addi r3, r10, -0xc60
	ctx.r[3].s64 = ctx.r[10].s64 + -3168;
	// 8266AB1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266AB20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AB24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AB28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AB2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AB30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AB34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AB38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AB3C: 4BDFC2E5  bl 0x82466e20
	ctx.lr = 0x8266AB40;
	sub_82466E20(ctx, base);
	// 8266AB40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AB44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AB48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AB4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AB50 size=100
    let mut pc: u32 = 0x8266AB50;
    'dispatch: loop {
        match pc {
            0x8266AB50 => {
    //   block [0x8266AB50..0x8266ABB4)
	// 8266AB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AB5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AB60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AB64: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266AB68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AB6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AB70: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 8266AB74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AB78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AB7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AB80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AB84: 386AF3D0  addi r3, r10, -0xc30
	ctx.r[3].s64 = ctx.r[10].s64 + -3120;
	// 8266AB88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AB8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AB90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266AB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AB98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266AB9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266ABA0: 4BDFC281  bl 0x82466e20
	ctx.lr = 0x8266ABA4;
	sub_82466E20(ctx, base);
	// 8266ABA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266ABA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266ABAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266ABB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266ABB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266ABB8 size=112
    let mut pc: u32 = 0x8266ABB8;
    'dispatch: loop {
        match pc {
            0x8266ABB8 => {
    //   block [0x8266ABB8..0x8266AC28)
	// 8266ABB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266ABBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266ABC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266ABC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266ABC8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266ABCC: 38AAF3D0  addi r5, r10, -0xc30
	ctx.r[5].s64 = ctx.r[10].s64 + -3120;
	// 8266ABD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266ABD4: 390BA9B8  addi r8, r11, -0x5648
	ctx.r[8].s64 = ctx.r[11].s64 + -22088;
	// 8266ABD8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8266ABDC: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 8266ABE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266ABE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266ABE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266ABEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266ABF0: 386AF400  addi r3, r10, -0xc00
	ctx.r[3].s64 = ctx.r[10].s64 + -3072;
	// 8266ABF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266ABF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266ABFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AC00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AC04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AC08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AC0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AC10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AC14: 4BDFC20D  bl 0x82466e20
	ctx.lr = 0x8266AC18;
	sub_82466E20(ctx, base);
	// 8266AC18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AC1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AC20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AC28 size=100
    let mut pc: u32 = 0x8266AC28;
    'dispatch: loop {
        match pc {
            0x8266AC28 => {
    //   block [0x8266AC28..0x8266AC8C)
	// 8266AC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AC30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AC34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AC38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AC3C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266AC40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AC44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AC48: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 8266AC4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AC54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AC5C: 386AF430  addi r3, r10, -0xbd0
	ctx.r[3].s64 = ctx.r[10].s64 + -3024;
	// 8266AC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AC64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AC68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266AC6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AC70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266AC74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AC78: 4BDFC1A9  bl 0x82466e20
	ctx.lr = 0x8266AC7C;
	sub_82466E20(ctx, base);
	// 8266AC7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AC88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AC90 size=108
    let mut pc: u32 = 0x8266AC90;
    'dispatch: loop {
        match pc {
            0x8266AC90 => {
    //   block [0x8266AC90..0x8266ACFC)
	// 8266AC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AC98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AC9C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266ACA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266ACA4: 38EBAA30  addi r7, r11, -0x55d0
	ctx.r[7].s64 = ctx.r[11].s64 + -21968;
	// 8266ACA8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266ACAC: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 8266ACB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266ACB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266ACB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266ACBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266ACC0: 386AF460  addi r3, r10, -0xba0
	ctx.r[3].s64 = ctx.r[10].s64 + -2976;
	// 8266ACC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266ACC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266ACCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266ACD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266ACD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266ACD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266ACDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266ACE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266ACE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266ACE8: 4BDFC139  bl 0x82466e20
	ctx.lr = 0x8266ACEC;
	sub_82466E20(ctx, base);
	// 8266ACEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266ACF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266ACF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266ACF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AD00 size=112
    let mut pc: u32 = 0x8266AD00;
    'dispatch: loop {
        match pc {
            0x8266AD00 => {
    //   block [0x8266AD00..0x8266AD70)
	// 8266AD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AD08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AD0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AD10: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266AD14: 38AAF430  addi r5, r10, -0xbd0
	ctx.r[5].s64 = ctx.r[10].s64 + -3024;
	// 8266AD18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AD1C: 390BAA78  addi r8, r11, -0x5588
	ctx.r[8].s64 = ctx.r[11].s64 + -21896;
	// 8266AD20: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266AD24: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 8266AD28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AD2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AD30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266AD34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AD38: 386AF490  addi r3, r10, -0xb70
	ctx.r[3].s64 = ctx.r[10].s64 + -2928;
	// 8266AD3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266AD40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AD44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AD48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AD4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AD50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AD54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AD58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AD5C: 4BDFC0C5  bl 0x82466e20
	ctx.lr = 0x8266AD60;
	sub_82466E20(ctx, base);
	// 8266AD60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AD70 size=100
    let mut pc: u32 = 0x8266AD70;
    'dispatch: loop {
        match pc {
            0x8266AD70 => {
    //   block [0x8266AD70..0x8266ADD4)
	// 8266AD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AD78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AD7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AD84: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266AD88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AD8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AD90: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 8266AD94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AD98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266ADA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266ADA4: 386AF4C0  addi r3, r10, -0xb40
	ctx.r[3].s64 = ctx.r[10].s64 + -2880;
	// 8266ADA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266ADAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266ADB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266ADB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266ADB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266ADBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266ADC0: 4BDFC061  bl 0x82466e20
	ctx.lr = 0x8266ADC4;
	sub_82466E20(ctx, base);
	// 8266ADC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266ADC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266ADCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266ADD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266ADD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266ADD8 size=100
    let mut pc: u32 = 0x8266ADD8;
    'dispatch: loop {
        match pc {
            0x8266ADD8 => {
    //   block [0x8266ADD8..0x8266AE3C)
	// 8266ADD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266ADDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266ADE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266ADE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266ADE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266ADEC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266ADF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266ADF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266ADF8: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 8266ADFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AE00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AE08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AE0C: 386AF4F0  addi r3, r10, -0xb10
	ctx.r[3].s64 = ctx.r[10].s64 + -2832;
	// 8266AE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AE14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AE18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266AE1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AE20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266AE24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AE28: 4BDFBFF9  bl 0x82466e20
	ctx.lr = 0x8266AE2C;
	sub_82466E20(ctx, base);
	// 8266AE2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AE30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AE34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AE40 size=112
    let mut pc: u32 = 0x8266AE40;
    'dispatch: loop {
        match pc {
            0x8266AE40 => {
    //   block [0x8266AE40..0x8266AEB0)
	// 8266AE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AE48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AE4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AE50: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266AE54: 38AAF4C0  addi r5, r10, -0xb40
	ctx.r[5].s64 = ctx.r[10].s64 + -2880;
	// 8266AE58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AE5C: 390BAAA8  addi r8, r11, -0x5558
	ctx.r[8].s64 = ctx.r[11].s64 + -21848;
	// 8266AE60: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8266AE64: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 8266AE68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AE6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AE70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266AE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AE78: 386AF520  addi r3, r10, -0xae0
	ctx.r[3].s64 = ctx.r[10].s64 + -2784;
	// 8266AE7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266AE80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AE84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AE88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AE8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AE90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AE94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AE98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AE9C: 4BDFBF85  bl 0x82466e20
	ctx.lr = 0x8266AEA0;
	sub_82466E20(ctx, base);
	// 8266AEA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AEB0 size=112
    let mut pc: u32 = 0x8266AEB0;
    'dispatch: loop {
        match pc {
            0x8266AEB0 => {
    //   block [0x8266AEB0..0x8266AF20)
	// 8266AEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AEB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AEBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AEC0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266AEC4: 38AAF4F0  addi r5, r10, -0xb10
	ctx.r[5].s64 = ctx.r[10].s64 + -2832;
	// 8266AEC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AECC: 390BAB08  addi r8, r11, -0x54f8
	ctx.r[8].s64 = ctx.r[11].s64 + -21752;
	// 8266AED0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8266AED4: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 8266AED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AEDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AEE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266AEE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AEE8: 386AF550  addi r3, r10, -0xab0
	ctx.r[3].s64 = ctx.r[10].s64 + -2736;
	// 8266AEEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266AEF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AEF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AEF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AEFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AF00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AF04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AF0C: 4BDFBF15  bl 0x82466e20
	ctx.lr = 0x8266AF10;
	sub_82466E20(ctx, base);
	// 8266AF10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AF20 size=100
    let mut pc: u32 = 0x8266AF20;
    'dispatch: loop {
        match pc {
            0x8266AF20 => {
    //   block [0x8266AF20..0x8266AF84)
	// 8266AF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AF2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AF30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AF34: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266AF38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AF3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AF40: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 8266AF44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AF4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AF54: 386AF580  addi r3, r10, -0xa80
	ctx.r[3].s64 = ctx.r[10].s64 + -2688;
	// 8266AF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AF5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AF60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266AF64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AF68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266AF6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AF70: 4BDFBEB1  bl 0x82466e20
	ctx.lr = 0x8266AF74;
	sub_82466E20(ctx, base);
	// 8266AF74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AF78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AF7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AF80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AF88 size=112
    let mut pc: u32 = 0x8266AF88;
    'dispatch: loop {
        match pc {
            0x8266AF88 => {
    //   block [0x8266AF88..0x8266AFF8)
	// 8266AF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AF90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AF94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AF98: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266AF9C: 38AAF580  addi r5, r10, -0xa80
	ctx.r[5].s64 = ctx.r[10].s64 + -2688;
	// 8266AFA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AFA4: 390BAB68  addi r8, r11, -0x5498
	ctx.r[8].s64 = ctx.r[11].s64 + -21656;
	// 8266AFA8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8266AFAC: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 8266AFB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AFB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AFB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266AFBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AFC0: 386AF5B0  addi r3, r10, -0xa50
	ctx.r[3].s64 = ctx.r[10].s64 + -2640;
	// 8266AFC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266AFC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AFCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AFD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AFD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AFD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AFDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AFE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AFE4: 4BDFBE3D  bl 0x82466e20
	ctx.lr = 0x8266AFE8;
	sub_82466E20(ctx, base);
	// 8266AFE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AFEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AFF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AFF8 size=108
    let mut pc: u32 = 0x8266AFF8;
    'dispatch: loop {
        match pc {
            0x8266AFF8 => {
    //   block [0x8266AFF8..0x8266B064)
	// 8266AFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B004: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B00C: 38EBAC58  addi r7, r11, -0x53a8
	ctx.r[7].s64 = ctx.r[11].s64 + -21416;
	// 8266B010: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8266B014: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 8266B018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B01C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B020: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266B024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B028: 386AF5E0  addi r3, r10, -0xa20
	ctx.r[3].s64 = ctx.r[10].s64 + -2592;
	// 8266B02C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266B030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B03C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B04C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266B050: 4BDFBDD1  bl 0x82466e20
	ctx.lr = 0x8266B054;
	sub_82466E20(ctx, base);
	// 8266B054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B05C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B068 size=108
    let mut pc: u32 = 0x8266B068;
    'dispatch: loop {
        match pc {
            0x8266B068 => {
    //   block [0x8266B068..0x8266B0D4)
	// 8266B068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B074: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B07C: 38EBAD48  addi r7, r11, -0x52b8
	ctx.r[7].s64 = ctx.r[11].s64 + -21176;
	// 8266B080: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266B084: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 8266B088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B08C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266B094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B098: 386AF610  addi r3, r10, -0x9f0
	ctx.r[3].s64 = ctx.r[10].s64 + -2544;
	// 8266B09C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266B0A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B0A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B0A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B0AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B0B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B0B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B0B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B0BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266B0C0: 4BDFBD61  bl 0x82466e20
	ctx.lr = 0x8266B0C4;
	sub_82466E20(ctx, base);
	// 8266B0C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B0C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B0CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B0D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B0D8 size=108
    let mut pc: u32 = 0x8266B0D8;
    'dispatch: loop {
        match pc {
            0x8266B0D8 => {
    //   block [0x8266B0D8..0x8266B144)
	// 8266B0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B0E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B0E4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B0E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B0EC: 38EBAD90  addi r7, r11, -0x5270
	ctx.r[7].s64 = ctx.r[11].s64 + -21104;
	// 8266B0F0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8266B0F4: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 8266B0F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B0FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266B104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B108: 386AF640  addi r3, r10, -0x9c0
	ctx.r[3].s64 = ctx.r[10].s64 + -2496;
	// 8266B10C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266B110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B11C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B12C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266B130: 4BDFBCF1  bl 0x82466e20
	ctx.lr = 0x8266B134;
	sub_82466E20(ctx, base);
	// 8266B134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B13C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B148 size=108
    let mut pc: u32 = 0x8266B148;
    'dispatch: loop {
        match pc {
            0x8266B148 => {
    //   block [0x8266B148..0x8266B1B4)
	// 8266B148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B154: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B15C: 38EBAE68  addi r7, r11, -0x5198
	ctx.r[7].s64 = ctx.r[11].s64 + -20888;
	// 8266B160: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266B164: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 8266B168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B16C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B170: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266B174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B178: 386AF670  addi r3, r10, -0x990
	ctx.r[3].s64 = ctx.r[10].s64 + -2448;
	// 8266B17C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266B180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B18C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B19C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266B1A0: 4BDFBC81  bl 0x82466e20
	ctx.lr = 0x8266B1A4;
	sub_82466E20(ctx, base);
	// 8266B1A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B1A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B1AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B1B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B1B8 size=100
    let mut pc: u32 = 0x8266B1B8;
    'dispatch: loop {
        match pc {
            0x8266B1B8 => {
    //   block [0x8266B1B8..0x8266B21C)
	// 8266B1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B1C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B1C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B1C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B1CC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266B1D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B1D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B1D8: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 8266B1DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B1E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B1E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B1E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B1EC: 386AF6A0  addi r3, r10, -0x960
	ctx.r[3].s64 = ctx.r[10].s64 + -2400;
	// 8266B1F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B1F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B1F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266B1FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B200: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266B204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B208: 4BDFBC19  bl 0x82466e20
	ctx.lr = 0x8266B20C;
	sub_82466E20(ctx, base);
	// 8266B20C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B220 size=112
    let mut pc: u32 = 0x8266B220;
    'dispatch: loop {
        match pc {
            0x8266B220 => {
    //   block [0x8266B220..0x8266B290)
	// 8266B220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B22C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B230: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B234: 38AAF6A0  addi r5, r10, -0x960
	ctx.r[5].s64 = ctx.r[10].s64 + -2400;
	// 8266B238: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B23C: 390BAE80  addi r8, r11, -0x5180
	ctx.r[8].s64 = ctx.r[11].s64 + -20864;
	// 8266B240: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266B244: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 8266B248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B24C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B250: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B258: 386AF6D0  addi r3, r10, -0x930
	ctx.r[3].s64 = ctx.r[10].s64 + -2352;
	// 8266B25C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B26C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B27C: 4BDFBBA5  bl 0x82466e20
	ctx.lr = 0x8266B280;
	sub_82466E20(ctx, base);
	// 8266B280: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B290 size=108
    let mut pc: u32 = 0x8266B290;
    'dispatch: loop {
        match pc {
            0x8266B290 => {
    //   block [0x8266B290..0x8266B2FC)
	// 8266B290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B29C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B2A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B2A4: 38EBAEC8  addi r7, r11, -0x5138
	ctx.r[7].s64 = ctx.r[11].s64 + -20792;
	// 8266B2A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266B2AC: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 8266B2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B2B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B2B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266B2BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B2C0: 386AF700  addi r3, r10, -0x900
	ctx.r[3].s64 = ctx.r[10].s64 + -2304;
	// 8266B2C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266B2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B2CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B2D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B2E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266B2E8: 4BDFBB39  bl 0x82466e20
	ctx.lr = 0x8266B2EC;
	sub_82466E20(ctx, base);
	// 8266B2EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B2F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B2F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B2F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B300 size=112
    let mut pc: u32 = 0x8266B300;
    'dispatch: loop {
        match pc {
            0x8266B300 => {
    //   block [0x8266B300..0x8266B370)
	// 8266B300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B30C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B310: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B314: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266B318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B31C: 390BAF10  addi r8, r11, -0x50f0
	ctx.r[8].s64 = ctx.r[11].s64 + -20720;
	// 8266B320: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266B324: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 8266B328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B32C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B330: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B338: 386AF730  addi r3, r10, -0x8d0
	ctx.r[3].s64 = ctx.r[10].s64 + -2256;
	// 8266B33C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B34C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B35C: 4BDFBAC5  bl 0x82466e20
	ctx.lr = 0x8266B360;
	sub_82466E20(ctx, base);
	// 8266B360: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B370 size=108
    let mut pc: u32 = 0x8266B370;
    'dispatch: loop {
        match pc {
            0x8266B370 => {
    //   block [0x8266B370..0x8266B3DC)
	// 8266B370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B37C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B384: 38EBAF28  addi r7, r11, -0x50d8
	ctx.r[7].s64 = ctx.r[11].s64 + -20696;
	// 8266B388: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266B38C: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 8266B390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B394: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B398: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266B39C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B3A0: 386AF760  addi r3, r10, -0x8a0
	ctx.r[3].s64 = ctx.r[10].s64 + -2208;
	// 8266B3A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266B3A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B3AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B3B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B3B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B3B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B3BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B3C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B3C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266B3C8: 4BDFBA59  bl 0x82466e20
	ctx.lr = 0x8266B3CC;
	sub_82466E20(ctx, base);
	// 8266B3CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B3D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B3D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B3D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B3E0 size=112
    let mut pc: u32 = 0x8266B3E0;
    'dispatch: loop {
        match pc {
            0x8266B3E0 => {
    //   block [0x8266B3E0..0x8266B450)
	// 8266B3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B3E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B3EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B3F0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B3F4: 38AAF730  addi r5, r10, -0x8d0
	ctx.r[5].s64 = ctx.r[10].s64 + -2256;
	// 8266B3F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B3FC: 390BAF70  addi r8, r11, -0x5090
	ctx.r[8].s64 = ctx.r[11].s64 + -20624;
	// 8266B400: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266B404: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 8266B408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B40C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B410: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B418: 386AF790  addi r3, r10, -0x870
	ctx.r[3].s64 = ctx.r[10].s64 + -2160;
	// 8266B41C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B42C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B43C: 4BDFB9E5  bl 0x82466e20
	ctx.lr = 0x8266B440;
	sub_82466E20(ctx, base);
	// 8266B440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B44C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B450 size=100
    let mut pc: u32 = 0x8266B450;
    'dispatch: loop {
        match pc {
            0x8266B450 => {
    //   block [0x8266B450..0x8266B4B4)
	// 8266B450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B45C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B464: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266B468: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B46C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B470: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 8266B474: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B478: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B47C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B480: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B484: 386AF7C0  addi r3, r10, -0x840
	ctx.r[3].s64 = ctx.r[10].s64 + -2112;
	// 8266B488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B48C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B490: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266B494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B498: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266B49C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B4A0: 4BDFB981  bl 0x82466e20
	ctx.lr = 0x8266B4A4;
	sub_82466E20(ctx, base);
	// 8266B4A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B4A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B4AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B4B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B4B8 size=112
    let mut pc: u32 = 0x8266B4B8;
    'dispatch: loop {
        match pc {
            0x8266B4B8 => {
    //   block [0x8266B4B8..0x8266B528)
	// 8266B4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B4C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B4C8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B4CC: 38AAF7C0  addi r5, r10, -0x840
	ctx.r[5].s64 = ctx.r[10].s64 + -2112;
	// 8266B4D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B4D4: 390BAF88  addi r8, r11, -0x5078
	ctx.r[8].s64 = ctx.r[11].s64 + -20600;
	// 8266B4D8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8266B4DC: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 8266B4E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B4E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B4E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B4EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B4F0: 386AF7F0  addi r3, r10, -0x810
	ctx.r[3].s64 = ctx.r[10].s64 + -2064;
	// 8266B4F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B4F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B4FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B50C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B514: 4BDFB90D  bl 0x82466e20
	ctx.lr = 0x8266B518;
	sub_82466E20(ctx, base);
	// 8266B518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B51C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B528 size=108
    let mut pc: u32 = 0x8266B528;
    'dispatch: loop {
        match pc {
            0x8266B528 => {
    //   block [0x8266B528..0x8266B594)
	// 8266B528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B534: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B53C: 38EBB030  addi r7, r11, -0x4fd0
	ctx.r[7].s64 = ctx.r[11].s64 + -20432;
	// 8266B540: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266B544: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 8266B548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B54C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B550: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266B554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B558: 386AF820  addi r3, r10, -0x7e0
	ctx.r[3].s64 = ctx.r[10].s64 + -2016;
	// 8266B55C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266B560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B56C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B57C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266B580: 4BDFB8A1  bl 0x82466e20
	ctx.lr = 0x8266B584;
	sub_82466E20(ctx, base);
	// 8266B584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B598 size=112
    let mut pc: u32 = 0x8266B598;
    'dispatch: loop {
        match pc {
            0x8266B598 => {
    //   block [0x8266B598..0x8266B608)
	// 8266B598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B5A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B5A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B5AC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266B5B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B5B4: 390BB060  addi r8, r11, -0x4fa0
	ctx.r[8].s64 = ctx.r[11].s64 + -20384;
	// 8266B5B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266B5BC: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 8266B5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B5C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B5C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B5CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B5D0: 386AF850  addi r3, r10, -0x7b0
	ctx.r[3].s64 = ctx.r[10].s64 + -1968;
	// 8266B5D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B5D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B5DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B5E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B5E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B5E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B5EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B5F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B5F4: 4BDFB82D  bl 0x82466e20
	ctx.lr = 0x8266B5F8;
	sub_82466E20(ctx, base);
	// 8266B5F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B608 size=112
    let mut pc: u32 = 0x8266B608;
    'dispatch: loop {
        match pc {
            0x8266B608 => {
    //   block [0x8266B608..0x8266B678)
	// 8266B608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B614: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B618: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B61C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266B620: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B624: 390BB0A8  addi r8, r11, -0x4f58
	ctx.r[8].s64 = ctx.r[11].s64 + -20312;
	// 8266B628: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266B62C: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 8266B630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B634: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B638: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B63C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B640: 386AF880  addi r3, r10, -0x780
	ctx.r[3].s64 = ctx.r[10].s64 + -1920;
	// 8266B644: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B648: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B65C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B664: 4BDFB7BD  bl 0x82466e20
	ctx.lr = 0x8266B668;
	sub_82466E20(ctx, base);
	// 8266B668: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B678 size=100
    let mut pc: u32 = 0x8266B678;
    'dispatch: loop {
        match pc {
            0x8266B678 => {
    //   block [0x8266B678..0x8266B6DC)
	// 8266B678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B684: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B68C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266B690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B698: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 8266B69C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B6A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B6A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B6A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B6AC: 386AF8B0  addi r3, r10, -0x750
	ctx.r[3].s64 = ctx.r[10].s64 + -1872;
	// 8266B6B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B6B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B6B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266B6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B6C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266B6C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B6C8: 4BDFB759  bl 0x82466e20
	ctx.lr = 0x8266B6CC;
	sub_82466E20(ctx, base);
	// 8266B6CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B6D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B6D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B6D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B6E0 size=112
    let mut pc: u32 = 0x8266B6E0;
    'dispatch: loop {
        match pc {
            0x8266B6E0 => {
    //   block [0x8266B6E0..0x8266B750)
	// 8266B6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B6E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B6EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B6F0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B6F4: 38AAF8B0  addi r5, r10, -0x750
	ctx.r[5].s64 = ctx.r[10].s64 + -1872;
	// 8266B6F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B6FC: 390BB0F0  addi r8, r11, -0x4f10
	ctx.r[8].s64 = ctx.r[11].s64 + -20240;
	// 8266B700: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266B704: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 8266B708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B70C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B710: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B718: 386AF8E0  addi r3, r10, -0x720
	ctx.r[3].s64 = ctx.r[10].s64 + -1824;
	// 8266B71C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B72C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B73C: 4BDFB6E5  bl 0x82466e20
	ctx.lr = 0x8266B740;
	sub_82466E20(ctx, base);
	// 8266B740: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B750 size=112
    let mut pc: u32 = 0x8266B750;
    'dispatch: loop {
        match pc {
            0x8266B750 => {
    //   block [0x8266B750..0x8266B7C0)
	// 8266B750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B75C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B760: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B764: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266B768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B76C: 390BB138  addi r8, r11, -0x4ec8
	ctx.r[8].s64 = ctx.r[11].s64 + -20168;
	// 8266B770: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266B774: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 8266B778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B77C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B780: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B788: 386AF910  addi r3, r10, -0x6f0
	ctx.r[3].s64 = ctx.r[10].s64 + -1776;
	// 8266B78C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B7A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B7A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B7A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B7AC: 4BDFB675  bl 0x82466e20
	ctx.lr = 0x8266B7B0;
	sub_82466E20(ctx, base);
	// 8266B7B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B7C0 size=112
    let mut pc: u32 = 0x8266B7C0;
    'dispatch: loop {
        match pc {
            0x8266B7C0 => {
    //   block [0x8266B7C0..0x8266B830)
	// 8266B7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B7C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B7CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B7D0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B7D4: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266B7D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B7DC: 390BB150  addi r8, r11, -0x4eb0
	ctx.r[8].s64 = ctx.r[11].s64 + -20144;
	// 8266B7E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266B7E4: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 8266B7E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B7EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B7F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B7F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B7F8: 386AF940  addi r3, r10, -0x6c0
	ctx.r[3].s64 = ctx.r[10].s64 + -1728;
	// 8266B7FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B80C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266B810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B81C: 4BDFB605  bl 0x82466e20
	ctx.lr = 0x8266B820;
	sub_82466E20(ctx, base);
	// 8266B820: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B830 size=112
    let mut pc: u32 = 0x8266B830;
    'dispatch: loop {
        match pc {
            0x8266B830 => {
    //   block [0x8266B830..0x8266B8A0)
	// 8266B830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B83C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B840: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B844: 38AAF910  addi r5, r10, -0x6f0
	ctx.r[5].s64 = ctx.r[10].s64 + -1776;
	// 8266B848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B84C: 390BB168  addi r8, r11, -0x4e98
	ctx.r[8].s64 = ctx.r[11].s64 + -20120;
	// 8266B850: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266B854: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 8266B858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B85C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B860: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B868: 386AF970  addi r3, r10, -0x690
	ctx.r[3].s64 = ctx.r[10].s64 + -1680;
	// 8266B86C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B87C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B88C: 4BDFB595  bl 0x82466e20
	ctx.lr = 0x8266B890;
	sub_82466E20(ctx, base);
	// 8266B890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B8A0 size=72
    let mut pc: u32 = 0x8266B8A0;
    'dispatch: loop {
        match pc {
            0x8266B8A0 => {
    //   block [0x8266B8A0..0x8266B8E8)
	// 8266B8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B8A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B8AC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8266B8B0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8266B8B4: 38CBE750  addi r6, r11, -0x18b0
	ctx.r[6].s64 = ctx.r[11].s64 + -6320;
	// 8266B8B8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8266B8BC: 388B0018  addi r4, r11, 0x18
	ctx.r[4].s64 = ctx.r[11].s64 + 24;
	// 8266B8C0: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8266B8C4: 386BF9A0  addi r3, r11, -0x660
	ctx.r[3].s64 = ctx.r[11].s64 + -1632;
	// 8266B8C8: 4BE101C1  bl 0x8247ba88
	ctx.lr = 0x8266B8CC;
	sub_8247BA88(ctx, base);
	// 8266B8CC: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 8266B8D0: 386BCDF8  addi r3, r11, -0x3208
	ctx.r[3].s64 = ctx.r[11].s64 + -12808;
	// 8266B8D4: 4BEC7265  bl 0x82532b38
	ctx.lr = 0x8266B8D8;
	sub_82532B38(ctx, base);
	// 8266B8D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8266B8DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B8E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B8E8 size=108
    let mut pc: u32 = 0x8266B8E8;
    'dispatch: loop {
        match pc {
            0x8266B8E8 => {
    //   block [0x8266B8E8..0x8266B954)
	// 8266B8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B8F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B8F4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B8F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B8FC: 38EBD040  addi r7, r11, -0x2fc0
	ctx.r[7].s64 = ctx.r[11].s64 + -12224;
	// 8266B900: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8266B904: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 8266B908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B90C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B910: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266B914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B918: 386AF9B8  addi r3, r10, -0x648
	ctx.r[3].s64 = ctx.r[10].s64 + -1608;
	// 8266B91C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266B920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B92C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B93C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266B940: 4BDFB4E1  bl 0x82466e20
	ctx.lr = 0x8266B944;
	sub_82466E20(ctx, base);
	// 8266B944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B94C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8266B958 size=24
    let mut pc: u32 = 0x8266B958;
    'dispatch: loop {
        match pc {
            0x8266B958 => {
    //   block [0x8266B958..0x8266B970)
	// 8266B958: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B95C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266B960: 394A1FA8  addi r10, r10, 0x1fa8
	ctx.r[10].s64 = ctx.r[10].s64 + 8104;
	// 8266B964: 816BD0B8  lwz r11, -0x2f48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12104 as u32) ) } as u64;
	// 8266B968: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8266B96C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B970 size=112
    let mut pc: u32 = 0x8266B970;
    'dispatch: loop {
        match pc {
            0x8266B970 => {
    //   block [0x8266B970..0x8266B9E0)
	// 8266B970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B97C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8266B980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B984: 392B1274  addi r9, r11, 0x1274
	ctx.r[9].s64 = ctx.r[11].s64 + 4724;
	// 8266B988: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 8266B98C: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8266B990: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B994: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 8266B998: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B99C: 396B1FA8  addi r11, r11, 0x1fa8
	ctx.r[11].s64 = ctx.r[11].s64 + 8104;
	// 8266B9A0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8266B9A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B9A8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8266B9AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B9B0: 386AF9E8  addi r3, r10, -0x618
	ctx.r[3].s64 = ctx.r[10].s64 + -1560;
	// 8266B9B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266B9B8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8266B9BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B9C0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8266B9C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266B9C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266B9CC: 4BDFB455  bl 0x82466e20
	ctx.lr = 0x8266B9D0;
	sub_82466E20(ctx, base);
	// 8266B9D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B9D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B9D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B9DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B9E0 size=108
    let mut pc: u32 = 0x8266B9E0;
    'dispatch: loop {
        match pc {
            0x8266B9E0 => {
    //   block [0x8266B9E0..0x8266BA4C)
	// 8266B9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B9E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B9EC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B9F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B9F4: 38EBD0BC  addi r7, r11, -0x2f44
	ctx.r[7].s64 = ctx.r[11].s64 + -12100;
	// 8266B9F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266B9FC: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 8266BA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BA04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BA08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266BA0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BA10: 386AFA18  addi r3, r10, -0x5e8
	ctx.r[3].s64 = ctx.r[10].s64 + -1512;
	// 8266BA14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266BA18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BA1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BA20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BA24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BA28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BA2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BA30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BA34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266BA38: 4BDFB3E9  bl 0x82466e20
	ctx.lr = 0x8266BA3C;
	sub_82466E20(ctx, base);
	// 8266BA3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BA40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BA44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BA48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BA50 size=108
    let mut pc: u32 = 0x8266BA50;
    'dispatch: loop {
        match pc {
            0x8266BA50 => {
    //   block [0x8266BA50..0x8266BABC)
	// 8266BA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BA58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BA5C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BA60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BA64: 38EBD0EC  addi r7, r11, -0x2f14
	ctx.r[7].s64 = ctx.r[11].s64 + -12052;
	// 8266BA68: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266BA6C: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 8266BA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BA74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BA78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266BA7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BA80: 386AFA48  addi r3, r10, -0x5b8
	ctx.r[3].s64 = ctx.r[10].s64 + -1464;
	// 8266BA84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266BA88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BA8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BA90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BA94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BA98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BA9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BAA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266BAA8: 4BDFB379  bl 0x82466e20
	ctx.lr = 0x8266BAAC;
	sub_82466E20(ctx, base);
	// 8266BAAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BAB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BAB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BAC0 size=112
    let mut pc: u32 = 0x8266BAC0;
    'dispatch: loop {
        match pc {
            0x8266BAC0 => {
    //   block [0x8266BAC0..0x8266BB30)
	// 8266BAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BAC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BACC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BAD0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BAD4: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 8266BAD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BADC: 390BD120  addi r8, r11, -0x2ee0
	ctx.r[8].s64 = ctx.r[11].s64 + -12000;
	// 8266BAE0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8266BAE4: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 8266BAE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BAEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BAF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266BAF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BAF8: 386AFA78  addi r3, r10, -0x588
	ctx.r[3].s64 = ctx.r[10].s64 + -1416;
	// 8266BAFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266BB00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BB04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BB08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BB0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BB10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BB14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BB18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BB1C: 4BDFB305  bl 0x82466e20
	ctx.lr = 0x8266BB20;
	sub_82466E20(ctx, base);
	// 8266BB20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BB24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BB28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BB2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BB30 size=108
    let mut pc: u32 = 0x8266BB30;
    'dispatch: loop {
        match pc {
            0x8266BB30 => {
    //   block [0x8266BB30..0x8266BB9C)
	// 8266BB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BB38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BB3C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BB40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BB44: 38EBD180  addi r7, r11, -0x2e80
	ctx.r[7].s64 = ctx.r[11].s64 + -11904;
	// 8266BB48: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8266BB4C: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 8266BB50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BB54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BB58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266BB5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BB60: 386AFAA8  addi r3, r10, -0x558
	ctx.r[3].s64 = ctx.r[10].s64 + -1368;
	// 8266BB64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266BB68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BB6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BB70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BB74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BB78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BB7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BB80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BB84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266BB88: 4BDFB299  bl 0x82466e20
	ctx.lr = 0x8266BB8C;
	sub_82466E20(ctx, base);
	// 8266BB8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BB90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BB94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BB98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BBA0 size=112
    let mut pc: u32 = 0x8266BBA0;
    'dispatch: loop {
        match pc {
            0x8266BBA0 => {
    //   block [0x8266BBA0..0x8266BC10)
	// 8266BBA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BBA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BBA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BBAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BBB0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BBB4: 38AAFA78  addi r5, r10, -0x588
	ctx.r[5].s64 = ctx.r[10].s64 + -1416;
	// 8266BBB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BBBC: 390BD1E0  addi r8, r11, -0x2e20
	ctx.r[8].s64 = ctx.r[11].s64 + -11808;
	// 8266BBC0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8266BBC4: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 8266BBC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BBCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BBD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266BBD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BBD8: 386AFAD8  addi r3, r10, -0x528
	ctx.r[3].s64 = ctx.r[10].s64 + -1320;
	// 8266BBDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266BBE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BBE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BBE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BBEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BBF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BBF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BBF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BBFC: 4BDFB225  bl 0x82466e20
	ctx.lr = 0x8266BC00;
	sub_82466E20(ctx, base);
	// 8266BC00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BC04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BC08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BC0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BC10 size=112
    let mut pc: u32 = 0x8266BC10;
    'dispatch: loop {
        match pc {
            0x8266BC10 => {
    //   block [0x8266BC10..0x8266BC80)
	// 8266BC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BC18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BC1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BC20: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BC24: 38AAFA78  addi r5, r10, -0x588
	ctx.r[5].s64 = ctx.r[10].s64 + -1416;
	// 8266BC28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BC2C: 390BD270  addi r8, r11, -0x2d90
	ctx.r[8].s64 = ctx.r[11].s64 + -11664;
	// 8266BC30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266BC34: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 8266BC38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BC3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BC40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266BC44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BC48: 386AFB08  addi r3, r10, -0x4f8
	ctx.r[3].s64 = ctx.r[10].s64 + -1272;
	// 8266BC4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266BC50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BC54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BC58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BC5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BC60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BC64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BC68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BC6C: 4BDFB1B5  bl 0x82466e20
	ctx.lr = 0x8266BC70;
	sub_82466E20(ctx, base);
	// 8266BC70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BC74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BC78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BC7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BC80 size=108
    let mut pc: u32 = 0x8266BC80;
    'dispatch: loop {
        match pc {
            0x8266BC80 => {
    //   block [0x8266BC80..0x8266BCEC)
	// 8266BC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BC88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BC8C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BC90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BC94: 38EBD288  addi r7, r11, -0x2d78
	ctx.r[7].s64 = ctx.r[11].s64 + -11640;
	// 8266BC98: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8266BC9C: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 8266BCA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BCA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BCA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266BCAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BCB0: 386AFB38  addi r3, r10, -0x4c8
	ctx.r[3].s64 = ctx.r[10].s64 + -1224;
	// 8266BCB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266BCB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BCBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BCC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BCC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BCC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BCCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BCD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BCD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266BCD8: 4BDFB149  bl 0x82466e20
	ctx.lr = 0x8266BCDC;
	sub_82466E20(ctx, base);
	// 8266BCDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BCE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BCE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BCE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BCF0 size=112
    let mut pc: u32 = 0x8266BCF0;
    'dispatch: loop {
        match pc {
            0x8266BCF0 => {
    //   block [0x8266BCF0..0x8266BD60)
	// 8266BCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BCF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BCFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BD00: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BD04: 38AAFA78  addi r5, r10, -0x588
	ctx.r[5].s64 = ctx.r[10].s64 + -1416;
	// 8266BD08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BD0C: 390BD2E8  addi r8, r11, -0x2d18
	ctx.r[8].s64 = ctx.r[11].s64 + -11544;
	// 8266BD10: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8266BD14: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 8266BD18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BD1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BD20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266BD24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BD28: 386AFB68  addi r3, r10, -0x498
	ctx.r[3].s64 = ctx.r[10].s64 + -1176;
	// 8266BD2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266BD30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BD34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BD38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BD3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BD40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BD44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BD48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BD4C: 4BDFB0D5  bl 0x82466e20
	ctx.lr = 0x8266BD50;
	sub_82466E20(ctx, base);
	// 8266BD50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BD54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BD58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BD5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BD60 size=108
    let mut pc: u32 = 0x8266BD60;
    'dispatch: loop {
        match pc {
            0x8266BD60 => {
    //   block [0x8266BD60..0x8266BDCC)
	// 8266BD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BD68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BD6C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BD70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BD74: 38EBD390  addi r7, r11, -0x2c70
	ctx.r[7].s64 = ctx.r[11].s64 + -11376;
	// 8266BD78: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266BD7C: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 8266BD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BD84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BD88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266BD8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BD90: 386AFB98  addi r3, r10, -0x468
	ctx.r[3].s64 = ctx.r[10].s64 + -1128;
	// 8266BD94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266BD98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BDA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BDA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BDA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BDAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BDB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BDB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266BDB8: 4BDFB069  bl 0x82466e20
	ctx.lr = 0x8266BDBC;
	sub_82466E20(ctx, base);
	// 8266BDBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BDC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BDC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BDD0 size=108
    let mut pc: u32 = 0x8266BDD0;
    'dispatch: loop {
        match pc {
            0x8266BDD0 => {
    //   block [0x8266BDD0..0x8266BE3C)
	// 8266BDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BDD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BDDC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BDE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BDE4: 38EBD3A8  addi r7, r11, -0x2c58
	ctx.r[7].s64 = ctx.r[11].s64 + -11352;
	// 8266BDE8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8266BDEC: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 8266BDF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BDF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BDF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266BDFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BE00: 386AFBC8  addi r3, r10, -0x438
	ctx.r[3].s64 = ctx.r[10].s64 + -1080;
	// 8266BE04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266BE08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BE0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BE10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BE18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BE20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BE24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266BE28: 4BDFAFF9  bl 0x82466e20
	ctx.lr = 0x8266BE2C;
	sub_82466E20(ctx, base);
	// 8266BE2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BE30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BE34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BE40 size=112
    let mut pc: u32 = 0x8266BE40;
    'dispatch: loop {
        match pc {
            0x8266BE40 => {
    //   block [0x8266BE40..0x8266BEB0)
	// 8266BE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BE48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BE4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BE50: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BE54: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 8266BE58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BE5C: 390BD408  addi r8, r11, -0x2bf8
	ctx.r[8].s64 = ctx.r[11].s64 + -11256;
	// 8266BE60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266BE64: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 8266BE68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BE6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BE70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266BE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BE78: 386AFBF8  addi r3, r10, -0x408
	ctx.r[3].s64 = ctx.r[10].s64 + -1032;
	// 8266BE7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266BE80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BE84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BE88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BE8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BE90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BE94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BE98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BE9C: 4BDFAF85  bl 0x82466e20
	ctx.lr = 0x8266BEA0;
	sub_82466E20(ctx, base);
	// 8266BEA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BEB0 size=108
    let mut pc: u32 = 0x8266BEB0;
    'dispatch: loop {
        match pc {
            0x8266BEB0 => {
    //   block [0x8266BEB0..0x8266BF1C)
	// 8266BEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BEB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BEBC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BEC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BEC4: 38EBD420  addi r7, r11, -0x2be0
	ctx.r[7].s64 = ctx.r[11].s64 + -11232;
	// 8266BEC8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266BECC: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 8266BED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BED4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BED8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266BEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BEE0: 386AFC28  addi r3, r10, -0x3d8
	ctx.r[3].s64 = ctx.r[10].s64 + -984;
	// 8266BEE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266BEE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BEEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BEF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BEF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BEF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BEFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BF00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BF04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266BF08: 4BDFAF19  bl 0x82466e20
	ctx.lr = 0x8266BF0C;
	sub_82466E20(ctx, base);
	// 8266BF0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BF10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BF14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BF18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BF20 size=108
    let mut pc: u32 = 0x8266BF20;
    'dispatch: loop {
        match pc {
            0x8266BF20 => {
    //   block [0x8266BF20..0x8266BF8C)
	// 8266BF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BF2C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BF30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BF34: 38EBD468  addi r7, r11, -0x2b98
	ctx.r[7].s64 = ctx.r[11].s64 + -11160;
	// 8266BF38: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8266BF3C: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 8266BF40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BF44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BF48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266BF4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BF50: 386AFC58  addi r3, r10, -0x3a8
	ctx.r[3].s64 = ctx.r[10].s64 + -936;
	// 8266BF54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266BF58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BF5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BF60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BF64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BF68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BF6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BF70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BF74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266BF78: 4BDFAEA9  bl 0x82466e20
	ctx.lr = 0x8266BF7C;
	sub_82466E20(ctx, base);
	// 8266BF7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BF80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BF84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BF88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BF90 size=108
    let mut pc: u32 = 0x8266BF90;
    'dispatch: loop {
        match pc {
            0x8266BF90 => {
    //   block [0x8266BF90..0x8266BFFC)
	// 8266BF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BF98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BF9C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BFA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BFA4: 38EBD4F8  addi r7, r11, -0x2b08
	ctx.r[7].s64 = ctx.r[11].s64 + -11016;
	// 8266BFA8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8266BFAC: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 8266BFB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BFB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BFB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266BFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BFC0: 386AFC88  addi r3, r10, -0x378
	ctx.r[3].s64 = ctx.r[10].s64 + -888;
	// 8266BFC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266BFC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BFCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BFD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BFD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BFD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BFDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BFE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BFE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266BFE8: 4BDFAE39  bl 0x82466e20
	ctx.lr = 0x8266BFEC;
	sub_82466E20(ctx, base);
	// 8266BFEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BFF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BFF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BFF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C000 size=100
    let mut pc: u32 = 0x8266C000;
    'dispatch: loop {
        match pc {
            0x8266C000 => {
    //   block [0x8266C000..0x8266C064)
	// 8266C000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C00C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C014: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 8266C018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266C01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C020: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 8266C024: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C02C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C034: 386AFCB8  addi r3, r10, -0x348
	ctx.r[3].s64 = ctx.r[10].s64 + -840;
	// 8266C038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C03C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C040: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266C044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C048: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266C04C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C050: 4BDFADD1  bl 0x82466e20
	ctx.lr = 0x8266C054;
	sub_82466E20(ctx, base);
	// 8266C054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C05C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C068 size=112
    let mut pc: u32 = 0x8266C068;
    'dispatch: loop {
        match pc {
            0x8266C068 => {
    //   block [0x8266C068..0x8266C0D8)
	// 8266C068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C074: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C078: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C07C: 38AAFCB8  addi r5, r10, -0x348
	ctx.r[5].s64 = ctx.r[10].s64 + -840;
	// 8266C080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266C084: 390BD588  addi r8, r11, -0x2a78
	ctx.r[8].s64 = ctx.r[11].s64 + -10872;
	// 8266C088: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8266C08C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 8266C090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C094: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C098: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266C09C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C0A0: 386AFCE8  addi r3, r10, -0x318
	ctx.r[3].s64 = ctx.r[10].s64 + -792;
	// 8266C0A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266C0A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C0AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C0B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C0B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C0B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C0BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C0C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C0C4: 4BDFAD5D  bl 0x82466e20
	ctx.lr = 0x8266C0C8;
	sub_82466E20(ctx, base);
	// 8266C0C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C0D8 size=108
    let mut pc: u32 = 0x8266C0D8;
    'dispatch: loop {
        match pc {
            0x8266C0D8 => {
    //   block [0x8266C0D8..0x8266C144)
	// 8266C0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C0E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C0E4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C0E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266C0EC: 38EBD5E8  addi r7, r11, -0x2a18
	ctx.r[7].s64 = ctx.r[11].s64 + -10776;
	// 8266C0F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266C0F4: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 8266C0F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C0FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C108: 386AFD18  addi r3, r10, -0x2e8
	ctx.r[3].s64 = ctx.r[10].s64 + -744;
	// 8266C10C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C11C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C12C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C130: 4BDFACF1  bl 0x82466e20
	ctx.lr = 0x8266C134;
	sub_82466E20(ctx, base);
	// 8266C134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C13C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C148 size=108
    let mut pc: u32 = 0x8266C148;
    'dispatch: loop {
        match pc {
            0x8266C148 => {
    //   block [0x8266C148..0x8266C1B4)
	// 8266C148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C154: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266C15C: 38EBD618  addi r7, r11, -0x29e8
	ctx.r[7].s64 = ctx.r[11].s64 + -10728;
	// 8266C160: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266C164: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 8266C168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C16C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C170: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C178: 386AFD48  addi r3, r10, -0x2b8
	ctx.r[3].s64 = ctx.r[10].s64 + -696;
	// 8266C17C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C18C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C19C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C1A0: 4BDFAC81  bl 0x82466e20
	ctx.lr = 0x8266C1A4;
	sub_82466E20(ctx, base);
	// 8266C1A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C1A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C1AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C1B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C1B8 size=108
    let mut pc: u32 = 0x8266C1B8;
    'dispatch: loop {
        match pc {
            0x8266C1B8 => {
    //   block [0x8266C1B8..0x8266C224)
	// 8266C1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C1C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C1C4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C1C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266C1CC: 38EBD660  addi r7, r11, -0x29a0
	ctx.r[7].s64 = ctx.r[11].s64 + -10656;
	// 8266C1D0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8266C1D4: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 8266C1D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C1DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C1E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C1E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C1E8: 386AFD78  addi r3, r10, -0x288
	ctx.r[3].s64 = ctx.r[10].s64 + -648;
	// 8266C1EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C1F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C1F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C1FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C20C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C210: 4BDFAC11  bl 0x82466e20
	ctx.lr = 0x8266C214;
	sub_82466E20(ctx, base);
	// 8266C214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C21C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C228 size=96
    let mut pc: u32 = 0x8266C228;
    'dispatch: loop {
        match pc {
            0x8266C228 => {
    //   block [0x8266C228..0x8266C288)
	// 8266C228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C234: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C23C: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 8266C240: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C248: 386AFDA8  addi r3, r10, -0x258
	ctx.r[3].s64 = ctx.r[10].s64 + -600;
	// 8266C24C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C254: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266C258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C25C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C268: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266C26C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C270: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266C274: 4BDFABAD  bl 0x82466e20
	ctx.lr = 0x8266C278;
	sub_82466E20(ctx, base);
	// 8266C278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C288 size=112
    let mut pc: u32 = 0x8266C288;
    'dispatch: loop {
        match pc {
            0x8266C288 => {
    //   block [0x8266C288..0x8266C2F8)
	// 8266C288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C294: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C298: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C29C: 38AAFDA8  addi r5, r10, -0x258
	ctx.r[5].s64 = ctx.r[10].s64 + -600;
	// 8266C2A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C2A4: 390BD6C0  addi r8, r11, -0x2940
	ctx.r[8].s64 = ctx.r[11].s64 + -10560;
	// 8266C2A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266C2AC: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 8266C2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C2B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266C2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C2C0: 386AFDD8  addi r3, r10, -0x228
	ctx.r[3].s64 = ctx.r[10].s64 + -552;
	// 8266C2C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266C2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C2E4: 4BDFAB3D  bl 0x82466e20
	ctx.lr = 0x8266C2E8;
	sub_82466E20(ctx, base);
	// 8266C2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C2F8 size=112
    let mut pc: u32 = 0x8266C2F8;
    'dispatch: loop {
        match pc {
            0x8266C2F8 => {
    //   block [0x8266C2F8..0x8266C368)
	// 8266C2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C304: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266C308: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C30C: 392A12A0  addi r9, r10, 0x12a0
	ctx.r[9].s64 = ctx.r[10].s64 + 4768;
	// 8266C310: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C314: 390BD6F0  addi r8, r11, -0x2910
	ctx.r[8].s64 = ctx.r[11].s64 + -10512;
	// 8266C318: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8266C31C: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 8266C320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C324: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266C32C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C330: 386AFE08  addi r3, r10, -0x1f8
	ctx.r[3].s64 = ctx.r[10].s64 + -504;
	// 8266C334: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266C338: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266C33C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C34C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C354: 4BDFAACD  bl 0x82466e20
	ctx.lr = 0x8266C358;
	sub_82466E20(ctx, base);
	// 8266C358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C368 size=108
    let mut pc: u32 = 0x8266C368;
    'dispatch: loop {
        match pc {
            0x8266C368 => {
    //   block [0x8266C368..0x8266C3D4)
	// 8266C368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C374: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C378: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C37C: 38EBD798  addi r7, r11, -0x2868
	ctx.r[7].s64 = ctx.r[11].s64 + -10344;
	// 8266C380: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266C384: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 8266C388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C38C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C390: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C398: 386AFE38  addi r3, r10, -0x1c8
	ctx.r[3].s64 = ctx.r[10].s64 + -456;
	// 8266C39C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C3A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C3A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C3A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C3AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C3B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C3B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C3B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C3BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C3C0: 4BDFAA61  bl 0x82466e20
	ctx.lr = 0x8266C3C4;
	sub_82466E20(ctx, base);
	// 8266C3C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C3C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C3CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C3D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C3D8 size=108
    let mut pc: u32 = 0x8266C3D8;
    'dispatch: loop {
        match pc {
            0x8266C3D8 => {
    //   block [0x8266C3D8..0x8266C444)
	// 8266C3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C3E4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C3E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C3EC: 38EBD7C8  addi r7, r11, -0x2838
	ctx.r[7].s64 = ctx.r[11].s64 + -10296;
	// 8266C3F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266C3F4: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 8266C3F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C3FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C400: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C408: 386AFE68  addi r3, r10, -0x198
	ctx.r[3].s64 = ctx.r[10].s64 + -408;
	// 8266C40C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C41C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C42C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C430: 4BDFA9F1  bl 0x82466e20
	ctx.lr = 0x8266C434;
	sub_82466E20(ctx, base);
	// 8266C434: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C43C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8266C448 size=28
    let mut pc: u32 = 0x8266C448;
    'dispatch: loop {
        match pc {
            0x8266C448 => {
    //   block [0x8266C448..0x8266C464)
	// 8266C448: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C44C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266C450: 394A1FF0  addi r10, r10, 0x1ff0
	ctx.r[10].s64 = ctx.r[10].s64 + 8176;
	// 8266C454: 816BD7F8  lwz r11, -0x2808(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10248 as u32) ) } as u64;
	// 8266C458: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8266C45C: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8266C460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C468 size=112
    let mut pc: u32 = 0x8266C468;
    'dispatch: loop {
        match pc {
            0x8266C468 => {
    //   block [0x8266C468..0x8266C4D8)
	// 8266C468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C474: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266C478: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C47C: 392A1410  addi r9, r10, 0x1410
	ctx.r[9].s64 = ctx.r[10].s64 + 5136;
	// 8266C480: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C484: 390B1FF0  addi r8, r11, 0x1ff0
	ctx.r[8].s64 = ctx.r[11].s64 + 8176;
	// 8266C488: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8266C48C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 8266C490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C494: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266C49C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C4A0: 386AFE98  addi r3, r10, -0x168
	ctx.r[3].s64 = ctx.r[10].s64 + -360;
	// 8266C4A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266C4A8: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8266C4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C4BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C4C4: 4BDFA95D  bl 0x82466e20
	ctx.lr = 0x8266C4C8;
	sub_82466E20(ctx, base);
	// 8266C4C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C4D8 size=108
    let mut pc: u32 = 0x8266C4D8;
    'dispatch: loop {
        match pc {
            0x8266C4D8 => {
    //   block [0x8266C4D8..0x8266C544)
	// 8266C4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C4E4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C4E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C4EC: 38EBD804  addi r7, r11, -0x27fc
	ctx.r[7].s64 = ctx.r[11].s64 + -10236;
	// 8266C4F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266C4F4: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 8266C4F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C4FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C500: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C508: 386AFEC8  addi r3, r10, -0x138
	ctx.r[3].s64 = ctx.r[10].s64 + -312;
	// 8266C50C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C51C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C52C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C530: 4BDFA8F1  bl 0x82466e20
	ctx.lr = 0x8266C534;
	sub_82466E20(ctx, base);
	// 8266C534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C53C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C548 size=108
    let mut pc: u32 = 0x8266C548;
    'dispatch: loop {
        match pc {
            0x8266C548 => {
    //   block [0x8266C548..0x8266C5B4)
	// 8266C548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C554: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C558: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C55C: 38EBD834  addi r7, r11, -0x27cc
	ctx.r[7].s64 = ctx.r[11].s64 + -10188;
	// 8266C560: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266C564: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 8266C568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C56C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C578: 386AFEF8  addi r3, r10, -0x108
	ctx.r[3].s64 = ctx.r[10].s64 + -264;
	// 8266C57C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C58C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C59C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C5A0: 4BDFA881  bl 0x82466e20
	ctx.lr = 0x8266C5A4;
	sub_82466E20(ctx, base);
	// 8266C5A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C5A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C5AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8266C5B8 size=24
    let mut pc: u32 = 0x8266C5B8;
    'dispatch: loop {
        match pc {
            0x8266C5B8 => {
    //   block [0x8266C5B8..0x8266C5D0)
	// 8266C5B8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C5BC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266C5C0: 394A20B0  addi r10, r10, 0x20b0
	ctx.r[10].s64 = ctx.r[10].s64 + 8368;
	// 8266C5C4: 816BD84C  lwz r11, -0x27b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10164 as u32) ) } as u64;
	// 8266C5C8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8266C5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C5D0 size=112
    let mut pc: u32 = 0x8266C5D0;
    'dispatch: loop {
        match pc {
            0x8266C5D0 => {
    //   block [0x8266C5D0..0x8266C640)
	// 8266C5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C5D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C5DC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266C5E0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C5E4: 392A1464  addi r9, r10, 0x1464
	ctx.r[9].s64 = ctx.r[10].s64 + 5220;
	// 8266C5E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C5EC: 390B20B0  addi r8, r11, 0x20b0
	ctx.r[8].s64 = ctx.r[11].s64 + 8368;
	// 8266C5F0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8266C5F4: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 8266C5F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C5FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C600: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266C604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C608: 386AFF28  addi r3, r10, -0xd8
	ctx.r[3].s64 = ctx.r[10].s64 + -216;
	// 8266C60C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266C610: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266C614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C61C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C624: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C62C: 4BDFA7F5  bl 0x82466e20
	ctx.lr = 0x8266C630;
	sub_82466E20(ctx, base);
	// 8266C630: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C640 size=108
    let mut pc: u32 = 0x8266C640;
    'dispatch: loop {
        match pc {
            0x8266C640 => {
    //   block [0x8266C640..0x8266C6AC)
	// 8266C640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C64C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C650: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C654: 38EBD850  addi r7, r11, -0x27b0
	ctx.r[7].s64 = ctx.r[11].s64 + -10160;
	// 8266C658: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266C65C: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 8266C660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C664: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C668: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C66C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C670: 386AFF58  addi r3, r10, -0xa8
	ctx.r[3].s64 = ctx.r[10].s64 + -168;
	// 8266C674: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C67C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C68C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C698: 4BDFA789  bl 0x82466e20
	ctx.lr = 0x8266C69C;
	sub_82466E20(ctx, base);
	// 8266C69C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C6A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C6A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C6A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C6B0 size=108
    let mut pc: u32 = 0x8266C6B0;
    'dispatch: loop {
        match pc {
            0x8266C6B0 => {
    //   block [0x8266C6B0..0x8266C71C)
	// 8266C6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C6B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C6BC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C6C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C6C4: 38EBD880  addi r7, r11, -0x2780
	ctx.r[7].s64 = ctx.r[11].s64 + -10112;
	// 8266C6C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266C6CC: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 8266C6D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C6D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C6D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C6E0: 386AFF88  addi r3, r10, -0x78
	ctx.r[3].s64 = ctx.r[10].s64 + -120;
	// 8266C6E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C6E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C6EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C6F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C6F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C6F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C6FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C704: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C708: 4BDFA719  bl 0x82466e20
	ctx.lr = 0x8266C70C;
	sub_82466E20(ctx, base);
	// 8266C70C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C720 size=112
    let mut pc: u32 = 0x8266C720;
    'dispatch: loop {
        match pc {
            0x8266C720 => {
    //   block [0x8266C720..0x8266C790)
	// 8266C720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C72C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266C730: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C734: 392A1488  addi r9, r10, 0x1488
	ctx.r[9].s64 = ctx.r[10].s64 + 5256;
	// 8266C738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266C73C: 390BD8B8  addi r8, r11, -0x2748
	ctx.r[8].s64 = ctx.r[11].s64 + -10056;
	// 8266C740: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8266C744: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 8266C748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C74C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C750: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266C754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C758: 386AFFB8  addi r3, r10, -0x48
	ctx.r[3].s64 = ctx.r[10].s64 + -72;
	// 8266C75C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266C760: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266C764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C76C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C77C: 4BDFA6A5  bl 0x82466e20
	ctx.lr = 0x8266C780;
	sub_82466E20(ctx, base);
	// 8266C780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


