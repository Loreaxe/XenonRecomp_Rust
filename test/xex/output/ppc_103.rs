pub fn sub_826A0160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0160 size=108
    let mut pc: u32 = 0x826A0160;
    'dispatch: loop {
        match pc {
            0x826A0160 => {
    //   block [0x826A0160..0x826A01CC)
	// 826A0160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A016C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0170: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0174: 38EB36F0  addi r7, r11, 0x36f0
	ctx.r[7].s64 = ctx.r[11].s64 + 14064;
	// 826A0178: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A017C: 388AC910  addi r4, r10, -0x36f0
	ctx.r[4].s64 = ctx.r[10].s64 + -14064;
	// 826A0180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0184: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A018C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0190: 386A5C90  addi r3, r10, 0x5c90
	ctx.r[3].s64 = ctx.r[10].s64 + 23696;
	// 826A0194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A0198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A019C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A01A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A01A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A01A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A01AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A01B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A01B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A01B8: 4BDC6C69  bl 0x82466e20
	ctx.lr = 0x826A01BC;
	sub_82466E20(ctx, base);
	// 826A01BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A01C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A01C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A01C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A01D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A01D0 size=112
    let mut pc: u32 = 0x826A01D0;
    'dispatch: loop {
        match pc {
            0x826A01D0 => {
    //   block [0x826A01D0..0x826A0240)
	// 826A01D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A01D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A01D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A01DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A01E0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A01E4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A01E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A01EC: 390B3738  addi r8, r11, 0x3738
	ctx.r[8].s64 = ctx.r[11].s64 + 14136;
	// 826A01F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A01F4: 388AC930  addi r4, r10, -0x36d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14032;
	// 826A01F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A01FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0200: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0208: 386A5CC0  addi r3, r10, 0x5cc0
	ctx.r[3].s64 = ctx.r[10].s64 + 23744;
	// 826A020C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A021C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A022C: 4BDC6BF5  bl 0x82466e20
	ctx.lr = 0x826A0230;
	sub_82466E20(ctx, base);
	// 826A0230: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A023C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0240 size=112
    let mut pc: u32 = 0x826A0240;
    'dispatch: loop {
        match pc {
            0x826A0240 => {
    //   block [0x826A0240..0x826A02B0)
	// 826A0240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A024C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0250: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0254: 38AA3740  addi r5, r10, 0x3740
	ctx.r[5].s64 = ctx.r[10].s64 + 14144;
	// 826A0258: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A025C: 390B3780  addi r8, r11, 0x3780
	ctx.r[8].s64 = ctx.r[11].s64 + 14208;
	// 826A0260: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A0264: 388AC948  addi r4, r10, -0x36b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14008;
	// 826A0268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A026C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0278: 386A5CF0  addi r3, r10, 0x5cf0
	ctx.r[3].s64 = ctx.r[10].s64 + 23792;
	// 826A027C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A028C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A0290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A029C: 4BDC6B85  bl 0x82466e20
	ctx.lr = 0x826A02A0;
	sub_82466E20(ctx, base);
	// 826A02A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A02A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A02A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A02AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A02B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A02B0 size=112
    let mut pc: u32 = 0x826A02B0;
    'dispatch: loop {
        match pc {
            0x826A02B0 => {
    //   block [0x826A02B0..0x826A0320)
	// 826A02B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A02B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A02B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A02BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A02C0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A02C4: 38AA3740  addi r5, r10, 0x3740
	ctx.r[5].s64 = ctx.r[10].s64 + 14144;
	// 826A02C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A02CC: 390B3798  addi r8, r11, 0x3798
	ctx.r[8].s64 = ctx.r[11].s64 + 14232;
	// 826A02D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A02D4: 388AC968  addi r4, r10, -0x3698
	ctx.r[4].s64 = ctx.r[10].s64 + -13976;
	// 826A02D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A02DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A02E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A02E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A02E8: 386A5D20  addi r3, r10, 0x5d20
	ctx.r[3].s64 = ctx.r[10].s64 + 23840;
	// 826A02EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A02F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A02F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A02F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A02FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A030C: 4BDC6B15  bl 0x82466e20
	ctx.lr = 0x826A0310;
	sub_82466E20(ctx, base);
	// 826A0310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A031C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0320 size=112
    let mut pc: u32 = 0x826A0320;
    'dispatch: loop {
        match pc {
            0x826A0320 => {
    //   block [0x826A0320..0x826A0390)
	// 826A0320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A032C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0330: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0334: 38AA5810  addi r5, r10, 0x5810
	ctx.r[5].s64 = ctx.r[10].s64 + 22544;
	// 826A0338: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A033C: 390B37C8  addi r8, r11, 0x37c8
	ctx.r[8].s64 = ctx.r[11].s64 + 14280;
	// 826A0340: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A0344: 388AC980  addi r4, r10, -0x3680
	ctx.r[4].s64 = ctx.r[10].s64 + -13952;
	// 826A0348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A034C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0350: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0358: 386A5D50  addi r3, r10, 0x5d50
	ctx.r[3].s64 = ctx.r[10].s64 + 23888;
	// 826A035C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A036C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A037C: 4BDC6AA5  bl 0x82466e20
	ctx.lr = 0x826A0380;
	sub_82466E20(ctx, base);
	// 826A0380: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A038C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0390 size=108
    let mut pc: u32 = 0x826A0390;
    'dispatch: loop {
        match pc {
            0x826A0390 => {
    //   block [0x826A0390..0x826A03FC)
	// 826A0390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A039C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A03A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A03A4: 38EB37E0  addi r7, r11, 0x37e0
	ctx.r[7].s64 = ctx.r[11].s64 + 14304;
	// 826A03A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A03AC: 388AC9A0  addi r4, r10, -0x3660
	ctx.r[4].s64 = ctx.r[10].s64 + -13920;
	// 826A03B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A03B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A03B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A03BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A03C0: 386A5D80  addi r3, r10, 0x5d80
	ctx.r[3].s64 = ctx.r[10].s64 + 23936;
	// 826A03C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A03C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A03CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A03D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A03D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A03D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A03DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A03E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A03E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A03E8: 4BDC6A39  bl 0x82466e20
	ctx.lr = 0x826A03EC;
	sub_82466E20(ctx, base);
	// 826A03EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A03F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A03F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A03F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0400 size=112
    let mut pc: u32 = 0x826A0400;
    'dispatch: loop {
        match pc {
            0x826A0400 => {
    //   block [0x826A0400..0x826A0470)
	// 826A0400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A040C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0410: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0414: 38AA5D80  addi r5, r10, 0x5d80
	ctx.r[5].s64 = ctx.r[10].s64 + 23936;
	// 826A0418: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A041C: 390B3810  addi r8, r11, 0x3810
	ctx.r[8].s64 = ctx.r[11].s64 + 14352;
	// 826A0420: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A0424: 388AC9BC  addi r4, r10, -0x3644
	ctx.r[4].s64 = ctx.r[10].s64 + -13892;
	// 826A0428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A042C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0438: 386A5DB0  addi r3, r10, 0x5db0
	ctx.r[3].s64 = ctx.r[10].s64 + 23984;
	// 826A043C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A044C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A045C: 4BDC69C5  bl 0x82466e20
	ctx.lr = 0x826A0460;
	sub_82466E20(ctx, base);
	// 826A0460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A046C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A0470 size=24
    let mut pc: u32 = 0x826A0470;
    'dispatch: loop {
        match pc {
            0x826A0470 => {
    //   block [0x826A0470..0x826A0488)
	// 826A0470: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0474: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826A0478: 394A7A70  addi r10, r10, 0x7a70
	ctx.r[10].s64 = ctx.r[10].s64 + 31344;
	// 826A047C: 816B31AC  lwz r11, 0x31ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12716 as u32) ) } as u64;
	// 826A0480: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826A0484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0488 size=116
    let mut pc: u32 = 0x826A0488;
    'dispatch: loop {
        match pc {
            0x826A0488 => {
    //   block [0x826A0488..0x826A04FC)
	// 826A0488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A048C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0494: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0498: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A049C: 390B7A70  addi r8, r11, 0x7a70
	ctx.r[8].s64 = ctx.r[11].s64 + 31344;
	// 826A04A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A04A4: 392AA900  addi r9, r10, -0x5700
	ctx.r[9].s64 = ctx.r[10].s64 + -22272;
	// 826A04A8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A04AC: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 826A04B0: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A04B4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A04B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A04BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A04C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A04C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A04C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A04CC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A04D0: 388ACA40  addi r4, r10, -0x35c0
	ctx.r[4].s64 = ctx.r[10].s64 + -13760;
	// 826A04D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A04D8: 386B5DE0  addi r3, r11, 0x5de0
	ctx.r[3].s64 = ctx.r[11].s64 + 24032;
	// 826A04DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A04E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A04E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A04E8: 4BDC6939  bl 0x82466e20
	ctx.lr = 0x826A04EC;
	sub_82466E20(ctx, base);
	// 826A04EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A04F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A04F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A04F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0500 size=108
    let mut pc: u32 = 0x826A0500;
    'dispatch: loop {
        match pc {
            0x826A0500 => {
    //   block [0x826A0500..0x826A056C)
	// 826A0500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A050C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0510: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0514: 38EB3840  addi r7, r11, 0x3840
	ctx.r[7].s64 = ctx.r[11].s64 + 14400;
	// 826A0518: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A051C: 388ACA5C  addi r4, r10, -0x35a4
	ctx.r[4].s64 = ctx.r[10].s64 + -13732;
	// 826A0520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0524: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A052C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0530: 386A5E10  addi r3, r10, 0x5e10
	ctx.r[3].s64 = ctx.r[10].s64 + 24080;
	// 826A0534: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A0538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A053C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A054C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A0558: 4BDC68C9  bl 0x82466e20
	ctx.lr = 0x826A055C;
	sub_82466E20(ctx, base);
	// 826A055C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0570 size=108
    let mut pc: u32 = 0x826A0570;
    'dispatch: loop {
        match pc {
            0x826A0570 => {
    //   block [0x826A0570..0x826A05DC)
	// 826A0570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A057C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0580: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0584: 38EB3888  addi r7, r11, 0x3888
	ctx.r[7].s64 = ctx.r[11].s64 + 14472;
	// 826A0588: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A058C: 388ACA78  addi r4, r10, -0x3588
	ctx.r[4].s64 = ctx.r[10].s64 + -13704;
	// 826A0590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0594: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0598: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A059C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A05A0: 386A5E40  addi r3, r10, 0x5e40
	ctx.r[3].s64 = ctx.r[10].s64 + 24128;
	// 826A05A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A05A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A05AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A05B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A05B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A05B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A05BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A05C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A05C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A05C8: 4BDC6859  bl 0x82466e20
	ctx.lr = 0x826A05CC;
	sub_82466E20(ctx, base);
	// 826A05CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A05D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A05D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A05D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A05E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A05E0 size=108
    let mut pc: u32 = 0x826A05E0;
    'dispatch: loop {
        match pc {
            0x826A05E0 => {
    //   block [0x826A05E0..0x826A064C)
	// 826A05E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A05E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A05E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A05EC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A05F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A05F4: 38EB38B8  addi r7, r11, 0x38b8
	ctx.r[7].s64 = ctx.r[11].s64 + 14520;
	// 826A05F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A05FC: 388ACA98  addi r4, r10, -0x3568
	ctx.r[4].s64 = ctx.r[10].s64 + -13672;
	// 826A0600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0604: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A060C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0610: 386A5E70  addi r3, r10, 0x5e70
	ctx.r[3].s64 = ctx.r[10].s64 + 24176;
	// 826A0614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A0618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A061C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A062C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A0638: 4BDC67E9  bl 0x82466e20
	ctx.lr = 0x826A063C;
	sub_82466E20(ctx, base);
	// 826A063C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0650 size=112
    let mut pc: u32 = 0x826A0650;
    'dispatch: loop {
        match pc {
            0x826A0650 => {
    //   block [0x826A0650..0x826A06C0)
	// 826A0650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A065C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0660: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0664: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A0668: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A066C: 390B38E8  addi r8, r11, 0x38e8
	ctx.r[8].s64 = ctx.r[11].s64 + 14568;
	// 826A0670: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A0674: 388ACAB0  addi r4, r10, -0x3550
	ctx.r[4].s64 = ctx.r[10].s64 + -13648;
	// 826A0678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A067C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0688: 386A5EA0  addi r3, r10, 0x5ea0
	ctx.r[3].s64 = ctx.r[10].s64 + 24224;
	// 826A068C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A069C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A06A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A06A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A06A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A06AC: 4BDC6775  bl 0x82466e20
	ctx.lr = 0x826A06B0;
	sub_82466E20(ctx, base);
	// 826A06B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A06B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A06B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A06BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A06C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A06C0 size=112
    let mut pc: u32 = 0x826A06C0;
    'dispatch: loop {
        match pc {
            0x826A06C0 => {
    //   block [0x826A06C0..0x826A0730)
	// 826A06C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A06C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A06C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A06CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A06D0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A06D4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A06D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A06DC: 390B3918  addi r8, r11, 0x3918
	ctx.r[8].s64 = ctx.r[11].s64 + 14616;
	// 826A06E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A06E4: 388ACAC0  addi r4, r10, -0x3540
	ctx.r[4].s64 = ctx.r[10].s64 + -13632;
	// 826A06E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A06EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A06F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A06F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A06F8: 386A5ED0  addi r3, r10, 0x5ed0
	ctx.r[3].s64 = ctx.r[10].s64 + 24272;
	// 826A06FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A070C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A071C: 4BDC6705  bl 0x82466e20
	ctx.lr = 0x826A0720;
	sub_82466E20(ctx, base);
	// 826A0720: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A072C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0730 size=112
    let mut pc: u32 = 0x826A0730;
    'dispatch: loop {
        match pc {
            0x826A0730 => {
    //   block [0x826A0730..0x826A07A0)
	// 826A0730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A073C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0740: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0744: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A0748: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A074C: 390B3930  addi r8, r11, 0x3930
	ctx.r[8].s64 = ctx.r[11].s64 + 14640;
	// 826A0750: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A0754: 388ACADC  addi r4, r10, -0x3524
	ctx.r[4].s64 = ctx.r[10].s64 + -13604;
	// 826A0758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A075C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0768: 386A5F00  addi r3, r10, 0x5f00
	ctx.r[3].s64 = ctx.r[10].s64 + 24320;
	// 826A076C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A077C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A078C: 4BDC6695  bl 0x82466e20
	ctx.lr = 0x826A0790;
	sub_82466E20(ctx, base);
	// 826A0790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A079C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A07A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A07A0 size=108
    let mut pc: u32 = 0x826A07A0;
    'dispatch: loop {
        match pc {
            0x826A07A0 => {
    //   block [0x826A07A0..0x826A080C)
	// 826A07A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A07A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A07A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A07AC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A07B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A07B4: 38EB3948  addi r7, r11, 0x3948
	ctx.r[7].s64 = ctx.r[11].s64 + 14664;
	// 826A07B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A07BC: 388ACAFC  addi r4, r10, -0x3504
	ctx.r[4].s64 = ctx.r[10].s64 + -13572;
	// 826A07C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A07C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A07C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A07CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A07D0: 386A5F30  addi r3, r10, 0x5f30
	ctx.r[3].s64 = ctx.r[10].s64 + 24368;
	// 826A07D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A07D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A07DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A07E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A07E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A07E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A07EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A07F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A07F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A07F8: 4BDC6629  bl 0x82466e20
	ctx.lr = 0x826A07FC;
	sub_82466E20(ctx, base);
	// 826A07FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0810 size=112
    let mut pc: u32 = 0x826A0810;
    'dispatch: loop {
        match pc {
            0x826A0810 => {
    //   block [0x826A0810..0x826A0880)
	// 826A0810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A081C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0820: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0824: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A0828: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A082C: 390B3978  addi r8, r11, 0x3978
	ctx.r[8].s64 = ctx.r[11].s64 + 14712;
	// 826A0830: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A0834: 388ACB34  addi r4, r10, -0x34cc
	ctx.r[4].s64 = ctx.r[10].s64 + -13516;
	// 826A0838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A083C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0848: 386A5F60  addi r3, r10, 0x5f60
	ctx.r[3].s64 = ctx.r[10].s64 + 24416;
	// 826A084C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A085C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A086C: 4BDC65B5  bl 0x82466e20
	ctx.lr = 0x826A0870;
	sub_82466E20(ctx, base);
	// 826A0870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A087C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0880 size=108
    let mut pc: u32 = 0x826A0880;
    'dispatch: loop {
        match pc {
            0x826A0880 => {
    //   block [0x826A0880..0x826A08EC)
	// 826A0880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A088C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0890: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0894: 38EB3990  addi r7, r11, 0x3990
	ctx.r[7].s64 = ctx.r[11].s64 + 14736;
	// 826A0898: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826A089C: 388ACB58  addi r4, r10, -0x34a8
	ctx.r[4].s64 = ctx.r[10].s64 + -13480;
	// 826A08A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A08A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A08A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A08AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A08B0: 386A5F90  addi r3, r10, 0x5f90
	ctx.r[3].s64 = ctx.r[10].s64 + 24464;
	// 826A08B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A08B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A08BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A08C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A08C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A08C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A08CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A08D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A08D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A08D8: 4BDC6549  bl 0x82466e20
	ctx.lr = 0x826A08DC;
	sub_82466E20(ctx, base);
	// 826A08DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A08E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A08E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A08E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A08F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A08F0 size=116
    let mut pc: u32 = 0x826A08F0;
    'dispatch: loop {
        match pc {
            0x826A08F0 => {
    //   block [0x826A08F0..0x826A0964)
	// 826A08F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A08F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A08F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A08FC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826A0900: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 826A0904: 390A3A80  addi r8, r10, 0x3a80
	ctx.r[8].s64 = ctx.r[10].s64 + 14976;
	// 826A0908: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A090C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A0910: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A0914: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0918: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A091C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0920: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0924: 388ACB7C  addi r4, r10, -0x3484
	ctx.r[4].s64 = ctx.r[10].s64 + -13444;
	// 826A0928: 396BA918  addi r11, r11, -0x56e8
	ctx.r[11].s64 = ctx.r[11].s64 + -22248;
	// 826A092C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0930: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0934: 386A5FC0  addi r3, r10, 0x5fc0
	ctx.r[3].s64 = ctx.r[10].s64 + 24512;
	// 826A0938: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A093C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0940: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A0944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A094C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0950: 4BDC64D1  bl 0x82466e20
	ctx.lr = 0x826A0954;
	sub_82466E20(ctx, base);
	// 826A0954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A095C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0968 size=108
    let mut pc: u32 = 0x826A0968;
    'dispatch: loop {
        match pc {
            0x826A0968 => {
    //   block [0x826A0968..0x826A09D4)
	// 826A0968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A096C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0974: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0978: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A097C: 38EB3C48  addi r7, r11, 0x3c48
	ctx.r[7].s64 = ctx.r[11].s64 + 15432;
	// 826A0980: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826A0984: 388ACB8C  addi r4, r10, -0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + -13428;
	// 826A0988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A098C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A0994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0998: 386A5FF0  addi r3, r10, 0x5ff0
	ctx.r[3].s64 = ctx.r[10].s64 + 24560;
	// 826A099C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A09A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A09A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A09A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A09AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A09B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A09B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A09B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A09BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A09C0: 4BDC6461  bl 0x82466e20
	ctx.lr = 0x826A09C4;
	sub_82466E20(ctx, base);
	// 826A09C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A09C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A09CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A09D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A09D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A09D8 size=112
    let mut pc: u32 = 0x826A09D8;
    'dispatch: loop {
        match pc {
            0x826A09D8 => {
    //   block [0x826A09D8..0x826A0A48)
	// 826A09D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A09DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A09E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A09E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A09E8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A09EC: 38AA4940  addi r5, r10, 0x4940
	ctx.r[5].s64 = ctx.r[10].s64 + 18752;
	// 826A09F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A09F4: 390B3DE0  addi r8, r11, 0x3de0
	ctx.r[8].s64 = ctx.r[11].s64 + 15840;
	// 826A09F8: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 826A09FC: 388ACBA8  addi r4, r10, -0x3458
	ctx.r[4].s64 = ctx.r[10].s64 + -13400;
	// 826A0A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0A04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0A08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0A10: 386A6020  addi r3, r10, 0x6020
	ctx.r[3].s64 = ctx.r[10].s64 + 24608;
	// 826A0A14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0A34: 4BDC63ED  bl 0x82466e20
	ctx.lr = 0x826A0A38;
	sub_82466E20(ctx, base);
	// 826A0A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0A48 size=100
    let mut pc: u32 = 0x826A0A48;
    'dispatch: loop {
        match pc {
            0x826A0A48 => {
    //   block [0x826A0A48..0x826A0AAC)
	// 826A0A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0A54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0A5C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A0A60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0A64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0A68: 388ACBBC  addi r4, r10, -0x3444
	ctx.r[4].s64 = ctx.r[10].s64 + -13380;
	// 826A0A6C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0A70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0A74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0A78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0A7C: 386A6050  addi r3, r10, 0x6050
	ctx.r[3].s64 = ctx.r[10].s64 + 24656;
	// 826A0A80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0A84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0A88: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A0A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0A90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A0A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0A98: 4BDC6389  bl 0x82466e20
	ctx.lr = 0x826A0A9C;
	sub_82466E20(ctx, base);
	// 826A0A9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0AA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0AA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0AB0 size=112
    let mut pc: u32 = 0x826A0AB0;
    'dispatch: loop {
        match pc {
            0x826A0AB0 => {
    //   block [0x826A0AB0..0x826A0B20)
	// 826A0AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0ABC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0AC0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0AC4: 38AA6050  addi r5, r10, 0x6050
	ctx.r[5].s64 = ctx.r[10].s64 + 24656;
	// 826A0AC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0ACC: 390B4038  addi r8, r11, 0x4038
	ctx.r[8].s64 = ctx.r[11].s64 + 16440;
	// 826A0AD0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826A0AD4: 388ACBD4  addi r4, r10, -0x342c
	ctx.r[4].s64 = ctx.r[10].s64 + -13356;
	// 826A0AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0ADC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0AE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0AE8: 386A6080  addi r3, r10, 0x6080
	ctx.r[3].s64 = ctx.r[10].s64 + 24704;
	// 826A0AEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0B0C: 4BDC6315  bl 0x82466e20
	ctx.lr = 0x826A0B10;
	sub_82466E20(ctx, base);
	// 826A0B10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0B20 size=100
    let mut pc: u32 = 0x826A0B20;
    'dispatch: loop {
        match pc {
            0x826A0B20 => {
    //   block [0x826A0B20..0x826A0B84)
	// 826A0B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0B2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0B34: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A0B38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0B40: 388ACBF4  addi r4, r10, -0x340c
	ctx.r[4].s64 = ctx.r[10].s64 + -13324;
	// 826A0B44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0B54: 386A60B0  addi r3, r10, 0x60b0
	ctx.r[3].s64 = ctx.r[10].s64 + 24752;
	// 826A0B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0B5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0B60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A0B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0B68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A0B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0B70: 4BDC62B1  bl 0x82466e20
	ctx.lr = 0x826A0B74;
	sub_82466E20(ctx, base);
	// 826A0B74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0B88 size=108
    let mut pc: u32 = 0x826A0B88;
    'dispatch: loop {
        match pc {
            0x826A0B88 => {
    //   block [0x826A0B88..0x826A0BF4)
	// 826A0B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0B94: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0B98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0B9C: 38EB40B0  addi r7, r11, 0x40b0
	ctx.r[7].s64 = ctx.r[11].s64 + 16560;
	// 826A0BA0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A0BA4: 388ACC04  addi r4, r10, -0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + -13308;
	// 826A0BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0BAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0BB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A0BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0BB8: 386A60E0  addi r3, r10, 0x60e0
	ctx.r[3].s64 = ctx.r[10].s64 + 24800;
	// 826A0BBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A0BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0BDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A0BE0: 4BDC6241  bl 0x82466e20
	ctx.lr = 0x826A0BE4;
	sub_82466E20(ctx, base);
	// 826A0BE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0BF8 size=112
    let mut pc: u32 = 0x826A0BF8;
    'dispatch: loop {
        match pc {
            0x826A0BF8 => {
    //   block [0x826A0BF8..0x826A0C68)
	// 826A0BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0C04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0C08: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0C0C: 38AA60B0  addi r5, r10, 0x60b0
	ctx.r[5].s64 = ctx.r[10].s64 + 24752;
	// 826A0C10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0C14: 390B40F8  addi r8, r11, 0x40f8
	ctx.r[8].s64 = ctx.r[11].s64 + 16632;
	// 826A0C18: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A0C1C: 388ACC34  addi r4, r10, -0x33cc
	ctx.r[4].s64 = ctx.r[10].s64 + -13260;
	// 826A0C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0C24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0C30: 386A6110  addi r3, r10, 0x6110
	ctx.r[3].s64 = ctx.r[10].s64 + 24848;
	// 826A0C34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0C54: 4BDC61CD  bl 0x82466e20
	ctx.lr = 0x826A0C58;
	sub_82466E20(ctx, base);
	// 826A0C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0C68 size=100
    let mut pc: u32 = 0x826A0C68;
    'dispatch: loop {
        match pc {
            0x826A0C68 => {
    //   block [0x826A0C68..0x826A0CCC)
	// 826A0C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0C74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0C7C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A0C80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0C84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0C88: 388ACC4C  addi r4, r10, -0x33b4
	ctx.r[4].s64 = ctx.r[10].s64 + -13236;
	// 826A0C8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0C90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0C98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0C9C: 386A6140  addi r3, r10, 0x6140
	ctx.r[3].s64 = ctx.r[10].s64 + 24896;
	// 826A0CA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0CA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0CA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A0CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0CB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A0CB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0CB8: 4BDC6169  bl 0x82466e20
	ctx.lr = 0x826A0CBC;
	sub_82466E20(ctx, base);
	// 826A0CBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0CD0 size=100
    let mut pc: u32 = 0x826A0CD0;
    'dispatch: loop {
        match pc {
            0x826A0CD0 => {
    //   block [0x826A0CD0..0x826A0D34)
	// 826A0CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0CDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0CE4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A0CE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0CF0: 388ACC68  addi r4, r10, -0x3398
	ctx.r[4].s64 = ctx.r[10].s64 + -13208;
	// 826A0CF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0CF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0CFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0D00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0D04: 386A6170  addi r3, r10, 0x6170
	ctx.r[3].s64 = ctx.r[10].s64 + 24944;
	// 826A0D08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0D0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0D10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A0D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0D18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A0D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0D20: 4BDC6101  bl 0x82466e20
	ctx.lr = 0x826A0D24;
	sub_82466E20(ctx, base);
	// 826A0D24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0D38 size=112
    let mut pc: u32 = 0x826A0D38;
    'dispatch: loop {
        match pc {
            0x826A0D38 => {
    //   block [0x826A0D38..0x826A0DA8)
	// 826A0D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0D44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0D48: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0D4C: 38AA6140  addi r5, r10, 0x6140
	ctx.r[5].s64 = ctx.r[10].s64 + 24896;
	// 826A0D50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0D54: 390B4128  addi r8, r11, 0x4128
	ctx.r[8].s64 = ctx.r[11].s64 + 16680;
	// 826A0D58: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826A0D5C: 388ACC80  addi r4, r10, -0x3380
	ctx.r[4].s64 = ctx.r[10].s64 + -13184;
	// 826A0D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0D64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0D70: 386A61A0  addi r3, r10, 0x61a0
	ctx.r[3].s64 = ctx.r[10].s64 + 24992;
	// 826A0D74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0D94: 4BDC608D  bl 0x82466e20
	ctx.lr = 0x826A0D98;
	sub_82466E20(ctx, base);
	// 826A0D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0DA8 size=112
    let mut pc: u32 = 0x826A0DA8;
    'dispatch: loop {
        match pc {
            0x826A0DA8 => {
    //   block [0x826A0DA8..0x826A0E18)
	// 826A0DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0DB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0DB8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0DBC: 38AA6170  addi r5, r10, 0x6170
	ctx.r[5].s64 = ctx.r[10].s64 + 24944;
	// 826A0DC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0DC4: 390B4188  addi r8, r11, 0x4188
	ctx.r[8].s64 = ctx.r[11].s64 + 16776;
	// 826A0DC8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826A0DCC: 388ACCA4  addi r4, r10, -0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + -13148;
	// 826A0DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0DD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0DD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0DE0: 386A61D0  addi r3, r10, 0x61d0
	ctx.r[3].s64 = ctx.r[10].s64 + 25040;
	// 826A0DE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0DE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0DF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0DF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0E04: 4BDC601D  bl 0x82466e20
	ctx.lr = 0x826A0E08;
	sub_82466E20(ctx, base);
	// 826A0E08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0E18 size=100
    let mut pc: u32 = 0x826A0E18;
    'dispatch: loop {
        match pc {
            0x826A0E18 => {
    //   block [0x826A0E18..0x826A0E7C)
	// 826A0E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0E24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0E28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0E2C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A0E30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0E34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0E38: 388ACCC8  addi r4, r10, -0x3338
	ctx.r[4].s64 = ctx.r[10].s64 + -13112;
	// 826A0E3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0E40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0E44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0E48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0E4C: 386A6200  addi r3, r10, 0x6200
	ctx.r[3].s64 = ctx.r[10].s64 + 25088;
	// 826A0E50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0E54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0E58: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A0E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0E60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A0E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0E68: 4BDC5FB9  bl 0x82466e20
	ctx.lr = 0x826A0E6C;
	sub_82466E20(ctx, base);
	// 826A0E6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0E80 size=112
    let mut pc: u32 = 0x826A0E80;
    'dispatch: loop {
        match pc {
            0x826A0E80 => {
    //   block [0x826A0E80..0x826A0EF0)
	// 826A0E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0E8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0E90: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0E94: 38AA6200  addi r5, r10, 0x6200
	ctx.r[5].s64 = ctx.r[10].s64 + 25088;
	// 826A0E98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0E9C: 390B41E8  addi r8, r11, 0x41e8
	ctx.r[8].s64 = ctx.r[11].s64 + 16872;
	// 826A0EA0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826A0EA4: 388ACCDC  addi r4, r10, -0x3324
	ctx.r[4].s64 = ctx.r[10].s64 + -13092;
	// 826A0EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0EAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0EB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0EB8: 386A6230  addi r3, r10, 0x6230
	ctx.r[3].s64 = ctx.r[10].s64 + 25136;
	// 826A0EBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0EC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0EDC: 4BDC5F45  bl 0x82466e20
	ctx.lr = 0x826A0EE0;
	sub_82466E20(ctx, base);
	// 826A0EE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0EF0 size=108
    let mut pc: u32 = 0x826A0EF0;
    'dispatch: loop {
        match pc {
            0x826A0EF0 => {
    //   block [0x826A0EF0..0x826A0F5C)
	// 826A0EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0EFC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0F00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0F04: 38EB42D8  addi r7, r11, 0x42d8
	ctx.r[7].s64 = ctx.r[11].s64 + 17112;
	// 826A0F08: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826A0F0C: 388ACCF4  addi r4, r10, -0x330c
	ctx.r[4].s64 = ctx.r[10].s64 + -13068;
	// 826A0F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0F14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0F18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A0F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0F20: 386A6260  addi r3, r10, 0x6260
	ctx.r[3].s64 = ctx.r[10].s64 + 25184;
	// 826A0F24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A0F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0F2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0F44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A0F48: 4BDC5ED9  bl 0x82466e20
	ctx.lr = 0x826A0F4C;
	sub_82466E20(ctx, base);
	// 826A0F4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0F60 size=108
    let mut pc: u32 = 0x826A0F60;
    'dispatch: loop {
        match pc {
            0x826A0F60 => {
    //   block [0x826A0F60..0x826A0FCC)
	// 826A0F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0F6C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0F70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0F74: 38EB43C8  addi r7, r11, 0x43c8
	ctx.r[7].s64 = ctx.r[11].s64 + 17352;
	// 826A0F78: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A0F7C: 388ACD24  addi r4, r10, -0x32dc
	ctx.r[4].s64 = ctx.r[10].s64 + -13020;
	// 826A0F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0F84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0F88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A0F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0F90: 386A6290  addi r3, r10, 0x6290
	ctx.r[3].s64 = ctx.r[10].s64 + 25232;
	// 826A0F94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A0F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0FA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0FB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A0FB8: 4BDC5E69  bl 0x82466e20
	ctx.lr = 0x826A0FBC;
	sub_82466E20(ctx, base);
	// 826A0FBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0FD0 size=108
    let mut pc: u32 = 0x826A0FD0;
    'dispatch: loop {
        match pc {
            0x826A0FD0 => {
    //   block [0x826A0FD0..0x826A103C)
	// 826A0FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0FDC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0FE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0FE4: 38EB4410  addi r7, r11, 0x4410
	ctx.r[7].s64 = ctx.r[11].s64 + 17424;
	// 826A0FE8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826A0FEC: 388ACD44  addi r4, r10, -0x32bc
	ctx.r[4].s64 = ctx.r[10].s64 + -12988;
	// 826A0FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0FF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0FF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A0FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1000: 386A62C0  addi r3, r10, 0x62c0
	ctx.r[3].s64 = ctx.r[10].s64 + 25280;
	// 826A1004: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A100C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1014: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A101C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1024: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1028: 4BDC5DF9  bl 0x82466e20
	ctx.lr = 0x826A102C;
	sub_82466E20(ctx, base);
	// 826A102C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1040 size=108
    let mut pc: u32 = 0x826A1040;
    'dispatch: loop {
        match pc {
            0x826A1040 => {
    //   block [0x826A1040..0x826A10AC)
	// 826A1040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A104C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1050: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1054: 38EB44E8  addi r7, r11, 0x44e8
	ctx.r[7].s64 = ctx.r[11].s64 + 17640;
	// 826A1058: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A105C: 388ACD68  addi r4, r10, -0x3298
	ctx.r[4].s64 = ctx.r[10].s64 + -12952;
	// 826A1060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1064: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1068: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A106C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1070: 386A62F0  addi r3, r10, 0x62f0
	ctx.r[3].s64 = ctx.r[10].s64 + 25328;
	// 826A1074: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A107C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A108C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1094: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1098: 4BDC5D89  bl 0x82466e20
	ctx.lr = 0x826A109C;
	sub_82466E20(ctx, base);
	// 826A109C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A10A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A10A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A10A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A10B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A10B0 size=100
    let mut pc: u32 = 0x826A10B0;
    'dispatch: loop {
        match pc {
            0x826A10B0 => {
    //   block [0x826A10B0..0x826A1114)
	// 826A10B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A10B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A10B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A10BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A10C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A10C4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A10C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A10CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A10D0: 388ACD84  addi r4, r10, -0x327c
	ctx.r[4].s64 = ctx.r[10].s64 + -12924;
	// 826A10D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A10D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A10DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A10E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A10E4: 386A6320  addi r3, r10, 0x6320
	ctx.r[3].s64 = ctx.r[10].s64 + 25376;
	// 826A10E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A10EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A10F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A10F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A10F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A10FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1100: 4BDC5D21  bl 0x82466e20
	ctx.lr = 0x826A1104;
	sub_82466E20(ctx, base);
	// 826A1104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A110C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1118 size=112
    let mut pc: u32 = 0x826A1118;
    'dispatch: loop {
        match pc {
            0x826A1118 => {
    //   block [0x826A1118..0x826A1188)
	// 826A1118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A111C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1124: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1128: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A112C: 38AA6320  addi r5, r10, 0x6320
	ctx.r[5].s64 = ctx.r[10].s64 + 25376;
	// 826A1130: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1134: 390B4500  addi r8, r11, 0x4500
	ctx.r[8].s64 = ctx.r[11].s64 + 17664;
	// 826A1138: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A113C: 388ACD98  addi r4, r10, -0x3268
	ctx.r[4].s64 = ctx.r[10].s64 + -12904;
	// 826A1140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1144: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1148: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A114C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1150: 386A6350  addi r3, r10, 0x6350
	ctx.r[3].s64 = ctx.r[10].s64 + 25424;
	// 826A1154: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A115C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A116C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1174: 4BDC5CAD  bl 0x82466e20
	ctx.lr = 0x826A1178;
	sub_82466E20(ctx, base);
	// 826A1178: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A117C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1188 size=108
    let mut pc: u32 = 0x826A1188;
    'dispatch: loop {
        match pc {
            0x826A1188 => {
    //   block [0x826A1188..0x826A11F4)
	// 826A1188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A118C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1194: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1198: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A119C: 38EB4548  addi r7, r11, 0x4548
	ctx.r[7].s64 = ctx.r[11].s64 + 17736;
	// 826A11A0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A11A4: 388ACDB4  addi r4, r10, -0x324c
	ctx.r[4].s64 = ctx.r[10].s64 + -12876;
	// 826A11A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A11AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A11B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A11B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A11B8: 386A6380  addi r3, r10, 0x6380
	ctx.r[3].s64 = ctx.r[10].s64 + 25472;
	// 826A11BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A11C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A11C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A11C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A11CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A11D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A11D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A11D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A11DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A11E0: 4BDC5C41  bl 0x82466e20
	ctx.lr = 0x826A11E4;
	sub_82466E20(ctx, base);
	// 826A11E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A11E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A11EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A11F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A11F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A11F8 size=112
    let mut pc: u32 = 0x826A11F8;
    'dispatch: loop {
        match pc {
            0x826A11F8 => {
    //   block [0x826A11F8..0x826A1268)
	// 826A11F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A11FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1204: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1208: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A120C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A1210: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1214: 390B4590  addi r8, r11, 0x4590
	ctx.r[8].s64 = ctx.r[11].s64 + 17808;
	// 826A1218: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A121C: 388ACDE4  addi r4, r10, -0x321c
	ctx.r[4].s64 = ctx.r[10].s64 + -12828;
	// 826A1220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1224: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A122C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1230: 386A63B0  addi r3, r10, 0x63b0
	ctx.r[3].s64 = ctx.r[10].s64 + 25520;
	// 826A1234: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A123C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A124C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1254: 4BDC5BCD  bl 0x82466e20
	ctx.lr = 0x826A1258;
	sub_82466E20(ctx, base);
	// 826A1258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A125C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1268 size=108
    let mut pc: u32 = 0x826A1268;
    'dispatch: loop {
        match pc {
            0x826A1268 => {
    //   block [0x826A1268..0x826A12D4)
	// 826A1268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A126C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1274: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1278: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A127C: 38EB45A8  addi r7, r11, 0x45a8
	ctx.r[7].s64 = ctx.r[11].s64 + 17832;
	// 826A1280: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A1284: 388ACDFC  addi r4, r10, -0x3204
	ctx.r[4].s64 = ctx.r[10].s64 + -12804;
	// 826A1288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A128C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1298: 386A63E0  addi r3, r10, 0x63e0
	ctx.r[3].s64 = ctx.r[10].s64 + 25568;
	// 826A129C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A12A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A12A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A12A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A12AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A12B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A12B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A12B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A12BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A12C0: 4BDC5B61  bl 0x82466e20
	ctx.lr = 0x826A12C4;
	sub_82466E20(ctx, base);
	// 826A12C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A12C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A12CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A12D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A12D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A12D8 size=112
    let mut pc: u32 = 0x826A12D8;
    'dispatch: loop {
        match pc {
            0x826A12D8 => {
    //   block [0x826A12D8..0x826A1348)
	// 826A12D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A12DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A12E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A12E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A12E8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A12EC: 38AA63B0  addi r5, r10, 0x63b0
	ctx.r[5].s64 = ctx.r[10].s64 + 25520;
	// 826A12F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A12F4: 390B45F0  addi r8, r11, 0x45f0
	ctx.r[8].s64 = ctx.r[11].s64 + 17904;
	// 826A12F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A12FC: 388ACE38  addi r4, r10, -0x31c8
	ctx.r[4].s64 = ctx.r[10].s64 + -12744;
	// 826A1300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1304: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1308: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A130C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1310: 386A6410  addi r3, r10, 0x6410
	ctx.r[3].s64 = ctx.r[10].s64 + 25616;
	// 826A1314: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A131C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A132C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1334: 4BDC5AED  bl 0x82466e20
	ctx.lr = 0x826A1338;
	sub_82466E20(ctx, base);
	// 826A1338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A133C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1348 size=100
    let mut pc: u32 = 0x826A1348;
    'dispatch: loop {
        match pc {
            0x826A1348 => {
    //   block [0x826A1348..0x826A13AC)
	// 826A1348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A134C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1354: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A135C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A1360: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1368: 388ACE54  addi r4, r10, -0x31ac
	ctx.r[4].s64 = ctx.r[10].s64 + -12716;
	// 826A136C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A137C: 386A6440  addi r3, r10, 0x6440
	ctx.r[3].s64 = ctx.r[10].s64 + 25664;
	// 826A1380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1384: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1388: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A138C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1390: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A1394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1398: 4BDC5A89  bl 0x82466e20
	ctx.lr = 0x826A139C;
	sub_82466E20(ctx, base);
	// 826A139C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A13A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A13A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A13A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A13B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A13B0 size=112
    let mut pc: u32 = 0x826A13B0;
    'dispatch: loop {
        match pc {
            0x826A13B0 => {
    //   block [0x826A13B0..0x826A1420)
	// 826A13B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A13B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A13B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A13BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A13C0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A13C4: 38AA6440  addi r5, r10, 0x6440
	ctx.r[5].s64 = ctx.r[10].s64 + 25664;
	// 826A13C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A13CC: 390B4608  addi r8, r11, 0x4608
	ctx.r[8].s64 = ctx.r[11].s64 + 17928;
	// 826A13D0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826A13D4: 388ACE6C  addi r4, r10, -0x3194
	ctx.r[4].s64 = ctx.r[10].s64 + -12692;
	// 826A13D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A13DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A13E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A13E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A13E8: 386A6470  addi r3, r10, 0x6470
	ctx.r[3].s64 = ctx.r[10].s64 + 25712;
	// 826A13EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A13F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A13F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A13F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A13FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A140C: 4BDC5A15  bl 0x82466e20
	ctx.lr = 0x826A1410;
	sub_82466E20(ctx, base);
	// 826A1410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A141C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1420 size=108
    let mut pc: u32 = 0x826A1420;
    'dispatch: loop {
        match pc {
            0x826A1420 => {
    //   block [0x826A1420..0x826A148C)
	// 826A1420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A142C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1430: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1434: 38EB46B0  addi r7, r11, 0x46b0
	ctx.r[7].s64 = ctx.r[11].s64 + 18096;
	// 826A1438: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A143C: 388ACE8C  addi r4, r10, -0x3174
	ctx.r[4].s64 = ctx.r[10].s64 + -12660;
	// 826A1440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1444: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1448: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A144C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1450: 386A64A0  addi r3, r10, 0x64a0
	ctx.r[3].s64 = ctx.r[10].s64 + 25760;
	// 826A1454: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A145C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A146C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1478: 4BDC59A9  bl 0x82466e20
	ctx.lr = 0x826A147C;
	sub_82466E20(ctx, base);
	// 826A147C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1490 size=112
    let mut pc: u32 = 0x826A1490;
    'dispatch: loop {
        match pc {
            0x826A1490 => {
    //   block [0x826A1490..0x826A1500)
	// 826A1490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A149C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A14A0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A14A4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A14A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A14AC: 390B46E0  addi r8, r11, 0x46e0
	ctx.r[8].s64 = ctx.r[11].s64 + 18144;
	// 826A14B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A14B4: 388ACEA0  addi r4, r10, -0x3160
	ctx.r[4].s64 = ctx.r[10].s64 + -12640;
	// 826A14B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A14BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A14C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A14C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A14C8: 386A64D0  addi r3, r10, 0x64d0
	ctx.r[3].s64 = ctx.r[10].s64 + 25808;
	// 826A14CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A14D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A14D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A14D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A14DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A14E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A14E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A14E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A14EC: 4BDC5935  bl 0x82466e20
	ctx.lr = 0x826A14F0;
	sub_82466E20(ctx, base);
	// 826A14F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A14F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A14F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A14FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1500 size=112
    let mut pc: u32 = 0x826A1500;
    'dispatch: loop {
        match pc {
            0x826A1500 => {
    //   block [0x826A1500..0x826A1570)
	// 826A1500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A150C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1510: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1514: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A1518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A151C: 390B4728  addi r8, r11, 0x4728
	ctx.r[8].s64 = ctx.r[11].s64 + 18216;
	// 826A1520: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A1524: 388A88D0  addi r4, r10, -0x7730
	ctx.r[4].s64 = ctx.r[10].s64 + -30512;
	// 826A1528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A152C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A1534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1538: 386A6500  addi r3, r10, 0x6500
	ctx.r[3].s64 = ctx.r[10].s64 + 25856;
	// 826A153C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A154C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A155C: 4BDC58C5  bl 0x82466e20
	ctx.lr = 0x826A1560;
	sub_82466E20(ctx, base);
	// 826A1560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A156C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1570 size=100
    let mut pc: u32 = 0x826A1570;
    'dispatch: loop {
        match pc {
            0x826A1570 => {
    //   block [0x826A1570..0x826A15D4)
	// 826A1570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A157C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1584: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A1588: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A158C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1590: 388ACEB4  addi r4, r10, -0x314c
	ctx.r[4].s64 = ctx.r[10].s64 + -12620;
	// 826A1594: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A159C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A15A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A15A4: 386A6530  addi r3, r10, 0x6530
	ctx.r[3].s64 = ctx.r[10].s64 + 25904;
	// 826A15A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A15AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A15B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A15B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A15B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A15BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A15C0: 4BDC5861  bl 0x82466e20
	ctx.lr = 0x826A15C4;
	sub_82466E20(ctx, base);
	// 826A15C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A15C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A15CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A15D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A15D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A15D8 size=112
    let mut pc: u32 = 0x826A15D8;
    'dispatch: loop {
        match pc {
            0x826A15D8 => {
    //   block [0x826A15D8..0x826A1648)
	// 826A15D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A15DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A15E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A15E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A15E8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A15EC: 38AA6530  addi r5, r10, 0x6530
	ctx.r[5].s64 = ctx.r[10].s64 + 25904;
	// 826A15F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A15F4: 390B4770  addi r8, r11, 0x4770
	ctx.r[8].s64 = ctx.r[11].s64 + 18288;
	// 826A15F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A15FC: 388ACED0  addi r4, r10, -0x3130
	ctx.r[4].s64 = ctx.r[10].s64 + -12592;
	// 826A1600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1604: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A160C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1610: 386A6560  addi r3, r10, 0x6560
	ctx.r[3].s64 = ctx.r[10].s64 + 25952;
	// 826A1614: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A161C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A162C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1634: 4BDC57ED  bl 0x82466e20
	ctx.lr = 0x826A1638;
	sub_82466E20(ctx, base);
	// 826A1638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A163C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1648 size=112
    let mut pc: u32 = 0x826A1648;
    'dispatch: loop {
        match pc {
            0x826A1648 => {
    //   block [0x826A1648..0x826A16B8)
	// 826A1648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A164C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1654: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1658: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A165C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A1660: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1664: 390B47B8  addi r8, r11, 0x47b8
	ctx.r[8].s64 = ctx.r[11].s64 + 18360;
	// 826A1668: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A166C: 388ACEF0  addi r4, r10, -0x3110
	ctx.r[4].s64 = ctx.r[10].s64 + -12560;
	// 826A1670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1674: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A167C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1680: 386A6590  addi r3, r10, 0x6590
	ctx.r[3].s64 = ctx.r[10].s64 + 26000;
	// 826A1684: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A168C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A169C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A16A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A16A4: 4BDC577D  bl 0x82466e20
	ctx.lr = 0x826A16A8;
	sub_82466E20(ctx, base);
	// 826A16A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A16AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A16B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A16B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A16B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A16B8 size=112
    let mut pc: u32 = 0x826A16B8;
    'dispatch: loop {
        match pc {
            0x826A16B8 => {
    //   block [0x826A16B8..0x826A1728)
	// 826A16B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A16BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A16C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A16C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A16C8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A16CC: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A16D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A16D4: 390B47D0  addi r8, r11, 0x47d0
	ctx.r[8].s64 = ctx.r[11].s64 + 18384;
	// 826A16D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A16DC: 388ACF08  addi r4, r10, -0x30f8
	ctx.r[4].s64 = ctx.r[10].s64 + -12536;
	// 826A16E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A16E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A16E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A16EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A16F0: 386A65C0  addi r3, r10, 0x65c0
	ctx.r[3].s64 = ctx.r[10].s64 + 26048;
	// 826A16F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A16F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A16FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1704: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A1708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A170C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1714: 4BDC570D  bl 0x82466e20
	ctx.lr = 0x826A1718;
	sub_82466E20(ctx, base);
	// 826A1718: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A171C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1728 size=112
    let mut pc: u32 = 0x826A1728;
    'dispatch: loop {
        match pc {
            0x826A1728 => {
    //   block [0x826A1728..0x826A1798)
	// 826A1728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A172C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1734: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1738: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A173C: 38AA6590  addi r5, r10, 0x6590
	ctx.r[5].s64 = ctx.r[10].s64 + 26000;
	// 826A1740: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1744: 390B47E8  addi r8, r11, 0x47e8
	ctx.r[8].s64 = ctx.r[11].s64 + 18408;
	// 826A1748: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A174C: 388ACF24  addi r4, r10, -0x30dc
	ctx.r[4].s64 = ctx.r[10].s64 + -12508;
	// 826A1750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1754: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A175C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1760: 386A65F0  addi r3, r10, 0x65f0
	ctx.r[3].s64 = ctx.r[10].s64 + 26096;
	// 826A1764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A176C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A177C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1784: 4BDC569D  bl 0x82466e20
	ctx.lr = 0x826A1788;
	sub_82466E20(ctx, base);
	// 826A1788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A178C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1798 size=72
    let mut pc: u32 = 0x826A1798;
    'dispatch: loop {
        match pc {
            0x826A1798 => {
    //   block [0x826A1798..0x826A17E0)
	// 826A1798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A179C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A17A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A17A4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A17A8: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 826A17AC: 38CB8180  addi r6, r11, -0x7e80
	ctx.r[6].s64 = ctx.r[11].s64 + -32384;
	// 826A17B0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A17B4: 388BA968  addi r4, r11, -0x5698
	ctx.r[4].s64 = ctx.r[11].s64 + -22168;
	// 826A17B8: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A17BC: 386B6620  addi r3, r11, 0x6620
	ctx.r[3].s64 = ctx.r[11].s64 + 26144;
	// 826A17C0: 4BDDA2C9  bl 0x8247ba88
	ctx.lr = 0x826A17C4;
	sub_8247BA88(ctx, base);
	// 826A17C4: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 826A17C8: 386BCE88  addi r3, r11, -0x3178
	ctx.r[3].s64 = ctx.r[11].s64 + -12664;
	// 826A17CC: 4BE9136D  bl 0x82532b38
	ctx.lr = 0x826A17D0;
	sub_82532B38(ctx, base);
	// 826A17D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826A17D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A17D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A17DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A17E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A17E0 size=108
    let mut pc: u32 = 0x826A17E0;
    'dispatch: loop {
        match pc {
            0x826A17E0 => {
    //   block [0x826A17E0..0x826A184C)
	// 826A17E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A17E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A17E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A17EC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A17F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A17F4: 38EB7BC0  addi r7, r11, 0x7bc0
	ctx.r[7].s64 = ctx.r[11].s64 + 31680;
	// 826A17F8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826A17FC: 388AA590  addi r4, r10, -0x5a70
	ctx.r[4].s64 = ctx.r[10].s64 + -23152;
	// 826A1800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1804: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1808: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A180C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1810: 386A6638  addi r3, r10, 0x6638
	ctx.r[3].s64 = ctx.r[10].s64 + 26168;
	// 826A1814: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A181C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A182C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1834: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1838: 4BDC55E9  bl 0x82466e20
	ctx.lr = 0x826A183C;
	sub_82466E20(ctx, base);
	// 826A183C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A1850 size=24
    let mut pc: u32 = 0x826A1850;
    'dispatch: loop {
        match pc {
            0x826A1850 => {
    //   block [0x826A1850..0x826A1868)
	// 826A1850: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1854: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A1858: 394A17E0  addi r10, r10, 0x17e0
	ctx.r[10].s64 = ctx.r[10].s64 + 6112;
	// 826A185C: 816B7C38  lwz r11, 0x7c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31800 as u32) ) } as u64;
	// 826A1860: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826A1864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1868 size=112
    let mut pc: u32 = 0x826A1868;
    'dispatch: loop {
        match pc {
            0x826A1868 => {
    //   block [0x826A1868..0x826A18D8)
	// 826A1868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A186C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1874: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A1878: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A187C: 392AB434  addi r9, r10, -0x4bcc
	ctx.r[9].s64 = ctx.r[10].s64 + -19404;
	// 826A1880: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1884: 390B17E0  addi r8, r11, 0x17e0
	ctx.r[8].s64 = ctx.r[11].s64 + 6112;
	// 826A1888: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826A188C: 388AA5A8  addi r4, r10, -0x5a58
	ctx.r[4].s64 = ctx.r[10].s64 + -23128;
	// 826A1890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1894: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1898: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A189C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A18A0: 386A6668  addi r3, r10, 0x6668
	ctx.r[3].s64 = ctx.r[10].s64 + 26216;
	// 826A18A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A18A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A18AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A18B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A18B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A18B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A18BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A18C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A18C4: 4BDC555D  bl 0x82466e20
	ctx.lr = 0x826A18C8;
	sub_82466E20(ctx, base);
	// 826A18C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A18CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A18D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A18D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A18D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A18D8 size=108
    let mut pc: u32 = 0x826A18D8;
    'dispatch: loop {
        match pc {
            0x826A18D8 => {
    //   block [0x826A18D8..0x826A1944)
	// 826A18D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A18DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A18E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A18E4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A18E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A18EC: 38EB7C3C  addi r7, r11, 0x7c3c
	ctx.r[7].s64 = ctx.r[11].s64 + 31804;
	// 826A18F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A18F4: 388AA5BC  addi r4, r10, -0x5a44
	ctx.r[4].s64 = ctx.r[10].s64 + -23108;
	// 826A18F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A18FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1900: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1908: 386A6698  addi r3, r10, 0x6698
	ctx.r[3].s64 = ctx.r[10].s64 + 26264;
	// 826A190C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A191C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A192C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1930: 4BDC54F1  bl 0x82466e20
	ctx.lr = 0x826A1934;
	sub_82466E20(ctx, base);
	// 826A1934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A193C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1948 size=108
    let mut pc: u32 = 0x826A1948;
    'dispatch: loop {
        match pc {
            0x826A1948 => {
    //   block [0x826A1948..0x826A19B4)
	// 826A1948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A194C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1954: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1958: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A195C: 38EB7C6C  addi r7, r11, 0x7c6c
	ctx.r[7].s64 = ctx.r[11].s64 + 31852;
	// 826A1960: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A1964: 388AA5DC  addi r4, r10, -0x5a24
	ctx.r[4].s64 = ctx.r[10].s64 + -23076;
	// 826A1968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A196C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1970: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1978: 386A66C8  addi r3, r10, 0x66c8
	ctx.r[3].s64 = ctx.r[10].s64 + 26312;
	// 826A197C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A198C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A199C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A19A0: 4BDC5481  bl 0x82466e20
	ctx.lr = 0x826A19A4;
	sub_82466E20(ctx, base);
	// 826A19A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A19A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A19AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A19B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A19B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A19B8 size=24
    let mut pc: u32 = 0x826A19B8;
    'dispatch: loop {
        match pc {
            0x826A19B8 => {
    //   block [0x826A19B8..0x826A19D0)
	// 826A19B8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A19BC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A19C0: 394A1840  addi r10, r10, 0x1840
	ctx.r[10].s64 = ctx.r[10].s64 + 6208;
	// 826A19C4: 816B7C9C  lwz r11, 0x7c9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31900 as u32) ) } as u64;
	// 826A19C8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826A19CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A19D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A19D0 size=116
    let mut pc: u32 = 0x826A19D0;
    'dispatch: loop {
        match pc {
            0x826A19D0 => {
    //   block [0x826A19D0..0x826A1A44)
	// 826A19D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A19D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A19D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A19DC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A19E0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A19E4: 390B1840  addi r8, r11, 0x1840
	ctx.r[8].s64 = ctx.r[11].s64 + 6208;
	// 826A19E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A19EC: 392AB478  addi r9, r10, -0x4b88
	ctx.r[9].s64 = ctx.r[10].s64 + -19336;
	// 826A19F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A19F4: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826A19F8: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A19FC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A1A00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1A04: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1A08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1A0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1A10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1A14: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A1A18: 388AA5F0  addi r4, r10, -0x5a10
	ctx.r[4].s64 = ctx.r[10].s64 + -23056;
	// 826A1A1C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A1A20: 386B66F8  addi r3, r11, 0x66f8
	ctx.r[3].s64 = ctx.r[11].s64 + 26360;
	// 826A1A24: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A1A28: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1A30: 4BDC53F1  bl 0x82466e20
	ctx.lr = 0x826A1A34;
	sub_82466E20(ctx, base);
	// 826A1A34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1A48 size=108
    let mut pc: u32 = 0x826A1A48;
    'dispatch: loop {
        match pc {
            0x826A1A48 => {
    //   block [0x826A1A48..0x826A1AB4)
	// 826A1A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1A54: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1A58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1A5C: 38EB7CA0  addi r7, r11, 0x7ca0
	ctx.r[7].s64 = ctx.r[11].s64 + 31904;
	// 826A1A60: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826A1A64: 388AA608  addi r4, r10, -0x59f8
	ctx.r[4].s64 = ctx.r[10].s64 + -23032;
	// 826A1A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1A6C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1A70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1A78: 386A6728  addi r3, r10, 0x6728
	ctx.r[3].s64 = ctx.r[10].s64 + 26408;
	// 826A1A7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1A80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1A84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1A88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1A90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1A98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1A9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1AA0: 4BDC5381  bl 0x82466e20
	ctx.lr = 0x826A1AA4;
	sub_82466E20(ctx, base);
	// 826A1AA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1AA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1AAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1AB8 size=112
    let mut pc: u32 = 0x826A1AB8;
    'dispatch: loop {
        match pc {
            0x826A1AB8 => {
    //   block [0x826A1AB8..0x826A1B28)
	// 826A1AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1AC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1AC8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1ACC: 38AA66F8  addi r5, r10, 0x66f8
	ctx.r[5].s64 = ctx.r[10].s64 + 26360;
	// 826A1AD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1AD4: 390B7D30  addi r8, r11, 0x7d30
	ctx.r[8].s64 = ctx.r[11].s64 + 32048;
	// 826A1AD8: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 826A1ADC: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826A1AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1AE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1AE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A1AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1AF0: 386A6758  addi r3, r10, 0x6758
	ctx.r[3].s64 = ctx.r[10].s64 + 26456;
	// 826A1AF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1B14: 4BDC530D  bl 0x82466e20
	ctx.lr = 0x826A1B18;
	sub_82466E20(ctx, base);
	// 826A1B18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1B28 size=112
    let mut pc: u32 = 0x826A1B28;
    'dispatch: loop {
        match pc {
            0x826A1B28 => {
    //   block [0x826A1B28..0x826A1B98)
	// 826A1B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1B34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1B38: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1B3C: 38AA66F8  addi r5, r10, 0x66f8
	ctx.r[5].s64 = ctx.r[10].s64 + 26360;
	// 826A1B40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1B44: 390B7E50  addi r8, r11, 0x7e50
	ctx.r[8].s64 = ctx.r[11].s64 + 32336;
	// 826A1B48: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A1B4C: 388AA664  addi r4, r10, -0x599c
	ctx.r[4].s64 = ctx.r[10].s64 + -22940;
	// 826A1B50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1B54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1B58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A1B5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1B60: 386A6788  addi r3, r10, 0x6788
	ctx.r[3].s64 = ctx.r[10].s64 + 26504;
	// 826A1B64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1B68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1B6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1B70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1B74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1B78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1B80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1B84: 4BDC529D  bl 0x82466e20
	ctx.lr = 0x826A1B88;
	sub_82466E20(ctx, base);
	// 826A1B88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1B8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1B90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1B94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A1B98 size=44
    let mut pc: u32 = 0x826A1B98;
    'dispatch: loop {
        match pc {
            0x826A1B98 => {
    //   block [0x826A1B98..0x826A1BC4)
	// 826A1B98: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1B9C: 814B7E80  lwz r10, 0x7e80(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32384 as u32) ) } as u64;
	// 826A1BA0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A1BA4: 396B18D0  addi r11, r11, 0x18d0
	ctx.r[11].s64 = ctx.r[11].s64 + 6352;
	// 826A1BA8: 914B00C8  stw r10, 0xc8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(200 as u32), ctx.r[10].u32 ) };
	// 826A1BAC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826A1BB0: 814A7E84  lwz r10, 0x7e84(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32388 as u32) ) } as u64;
	// 826A1BB4: 914B00E0  stw r10, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[10].u32 ) };
	// 826A1BB8: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 826A1BBC: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826A1BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1BC8 size=112
    let mut pc: u32 = 0x826A1BC8;
    'dispatch: loop {
        match pc {
            0x826A1BC8 => {
    //   block [0x826A1BC8..0x826A1C38)
	// 826A1BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1BD4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A1BD8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A1BDC: 392AB4CC  addi r9, r10, -0x4b34
	ctx.r[9].s64 = ctx.r[10].s64 + -19252;
	// 826A1BE0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826A1BE4: 390B18D0  addi r8, r11, 0x18d0
	ctx.r[8].s64 = ctx.r[11].s64 + 6352;
	// 826A1BE8: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 826A1BEC: 388A8F0C  addi r4, r10, -0x70f4
	ctx.r[4].s64 = ctx.r[10].s64 + -28916;
	// 826A1BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1BF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A1BFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1C00: 386A67B8  addi r3, r10, 0x67b8
	ctx.r[3].s64 = ctx.r[10].s64 + 26552;
	// 826A1C04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A1C08: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826A1C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1C1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1C24: 4BDC51FD  bl 0x82466e20
	ctx.lr = 0x826A1C28;
	sub_82466E20(ctx, base);
	// 826A1C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1C38 size=112
    let mut pc: u32 = 0x826A1C38;
    'dispatch: loop {
        match pc {
            0x826A1C38 => {
    //   block [0x826A1C38..0x826A1CA8)
	// 826A1C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1C44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1C48: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1C4C: 38AA66F8  addi r5, r10, 0x66f8
	ctx.r[5].s64 = ctx.r[10].s64 + 26360;
	// 826A1C50: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A1C54: 390B7E88  addi r8, r11, 0x7e88
	ctx.r[8].s64 = ctx.r[11].s64 + 32392;
	// 826A1C58: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 826A1C5C: 388AB1CC  addi r4, r10, -0x4e34
	ctx.r[4].s64 = ctx.r[10].s64 + -20020;
	// 826A1C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1C64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A1C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1C70: 386A67E8  addi r3, r10, 0x67e8
	ctx.r[3].s64 = ctx.r[10].s64 + 26600;
	// 826A1C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1C94: 4BDC518D  bl 0x82466e20
	ctx.lr = 0x826A1C98;
	sub_82466E20(ctx, base);
	// 826A1C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1CA8 size=108
    let mut pc: u32 = 0x826A1CA8;
    'dispatch: loop {
        match pc {
            0x826A1CA8 => {
    //   block [0x826A1CA8..0x826A1D14)
	// 826A1CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1CB4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1CB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1CBC: 38EB7FC0  addi r7, r11, 0x7fc0
	ctx.r[7].s64 = ctx.r[11].s64 + 32704;
	// 826A1CC0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826A1CC4: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 826A1CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1CCC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1CD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1CD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1CD8: 386A6818  addi r3, r10, 0x6818
	ctx.r[3].s64 = ctx.r[10].s64 + 26648;
	// 826A1CDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1CE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1CFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1D00: 4BDC5121  bl 0x82466e20
	ctx.lr = 0x826A1D04;
	sub_82466E20(ctx, base);
	// 826A1D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1D18 size=108
    let mut pc: u32 = 0x826A1D18;
    'dispatch: loop {
        match pc {
            0x826A1D18 => {
    //   block [0x826A1D18..0x826A1D84)
	// 826A1D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1D24: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A1D28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1D2C: 38EB80B0  addi r7, r11, -0x7f50
	ctx.r[7].s64 = ctx.r[11].s64 + -32592;
	// 826A1D30: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826A1D34: 388AA6B4  addi r4, r10, -0x594c
	ctx.r[4].s64 = ctx.r[10].s64 + -22860;
	// 826A1D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1D3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1D40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1D48: 386A6848  addi r3, r10, 0x6848
	ctx.r[3].s64 = ctx.r[10].s64 + 26696;
	// 826A1D4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1D6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1D70: 4BDC50B1  bl 0x82466e20
	ctx.lr = 0x826A1D74;
	sub_82466E20(ctx, base);
	// 826A1D74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1D88 size=112
    let mut pc: u32 = 0x826A1D88;
    'dispatch: loop {
        match pc {
            0x826A1D88 => {
    //   block [0x826A1D88..0x826A1DF8)
	// 826A1D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1D94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1D98: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A1D9C: 38AA66F8  addi r5, r10, 0x66f8
	ctx.r[5].s64 = ctx.r[10].s64 + 26360;
	// 826A1DA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1DA4: 390B8140  addi r8, r11, -0x7ec0
	ctx.r[8].s64 = ctx.r[11].s64 + -32448;
	// 826A1DA8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826A1DAC: 388AA6E4  addi r4, r10, -0x591c
	ctx.r[4].s64 = ctx.r[10].s64 + -22812;
	// 826A1DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1DB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1DB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A1DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1DC0: 386A6878  addi r3, r10, 0x6878
	ctx.r[3].s64 = ctx.r[10].s64 + 26744;
	// 826A1DC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1DE4: 4BDC503D  bl 0x82466e20
	ctx.lr = 0x826A1DE8;
	sub_82466E20(ctx, base);
	// 826A1DE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1DF8 size=108
    let mut pc: u32 = 0x826A1DF8;
    'dispatch: loop {
        match pc {
            0x826A1DF8 => {
    //   block [0x826A1DF8..0x826A1E64)
	// 826A1DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1E04: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A1E08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1E0C: 38EB8230  addi r7, r11, -0x7dd0
	ctx.r[7].s64 = ctx.r[11].s64 + -32208;
	// 826A1E10: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A1E14: 388AA700  addi r4, r10, -0x5900
	ctx.r[4].s64 = ctx.r[10].s64 + -22784;
	// 826A1E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1E1C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1E20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1E28: 386A68A8  addi r3, r10, 0x68a8
	ctx.r[3].s64 = ctx.r[10].s64 + 26792;
	// 826A1E2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1E34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1E4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1E50: 4BDC4FD1  bl 0x82466e20
	ctx.lr = 0x826A1E54;
	sub_82466E20(ctx, base);
	// 826A1E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1E68 size=108
    let mut pc: u32 = 0x826A1E68;
    'dispatch: loop {
        match pc {
            0x826A1E68 => {
    //   block [0x826A1E68..0x826A1ED4)
	// 826A1E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1E74: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A1E78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1E7C: 38EB8248  addi r7, r11, -0x7db8
	ctx.r[7].s64 = ctx.r[11].s64 + -32184;
	// 826A1E80: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A1E84: 388AA718  addi r4, r10, -0x58e8
	ctx.r[4].s64 = ctx.r[10].s64 + -22760;
	// 826A1E88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1E8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1E90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1E94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1E98: 386A68D8  addi r3, r10, 0x68d8
	ctx.r[3].s64 = ctx.r[10].s64 + 26840;
	// 826A1E9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1EA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1EA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1EA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1EB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1EB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1EB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1EBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1EC0: 4BDC4F61  bl 0x82466e20
	ctx.lr = 0x826A1EC4;
	sub_82466E20(ctx, base);
	// 826A1EC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1ED8 size=116
    let mut pc: u32 = 0x826A1ED8;
    'dispatch: loop {
        match pc {
            0x826A1ED8 => {
    //   block [0x826A1ED8..0x826A1F4C)
	// 826A1ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1EE4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A1EE8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A1EEC: 390B82AC  addi r8, r11, -0x7d54
	ctx.r[8].s64 = ctx.r[11].s64 + -32084;
	// 826A1EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1EF4: 392AB50C  addi r9, r10, -0x4af4
	ctx.r[9].s64 = ctx.r[10].s64 + -19188;
	// 826A1EF8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A1EFC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826A1F00: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A1F04: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A1F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1F0C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1F1C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A1F20: 388AA728  addi r4, r10, -0x58d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22744;
	// 826A1F24: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A1F28: 386B6908  addi r3, r11, 0x6908
	ctx.r[3].s64 = ctx.r[11].s64 + 26888;
	// 826A1F2C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A1F30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1F34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1F38: 4BDC4EE9  bl 0x82466e20
	ctx.lr = 0x826A1F3C;
	sub_82466E20(ctx, base);
	// 826A1F3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1F50 size=108
    let mut pc: u32 = 0x826A1F50;
    'dispatch: loop {
        match pc {
            0x826A1F50 => {
    //   block [0x826A1F50..0x826A1FBC)
	// 826A1F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1F58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1F5C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A1F60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1F64: 38EB82C8  addi r7, r11, -0x7d38
	ctx.r[7].s64 = ctx.r[11].s64 + -32056;
	// 826A1F68: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A1F6C: 388AA73C  addi r4, r10, -0x58c4
	ctx.r[4].s64 = ctx.r[10].s64 + -22724;
	// 826A1F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1F74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1F78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1F7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1F80: 386A6938  addi r3, r10, 0x6938
	ctx.r[3].s64 = ctx.r[10].s64 + 26936;
	// 826A1F84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1F88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1F8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1F90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1F94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1F98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1F9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1FA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1FA8: 4BDC4E79  bl 0x82466e20
	ctx.lr = 0x826A1FAC;
	sub_82466E20(ctx, base);
	// 826A1FAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1FC0 size=108
    let mut pc: u32 = 0x826A1FC0;
    'dispatch: loop {
        match pc {
            0x826A1FC0 => {
    //   block [0x826A1FC0..0x826A202C)
	// 826A1FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1FCC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A1FD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1FD4: 38EB8310  addi r7, r11, -0x7cf0
	ctx.r[7].s64 = ctx.r[11].s64 + -31984;
	// 826A1FD8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826A1FDC: 388AA760  addi r4, r10, -0x58a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22688;
	// 826A1FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1FE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1FE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1FEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1FF0: 386A6968  addi r3, r10, 0x6968
	ctx.r[3].s64 = ctx.r[10].s64 + 26984;
	// 826A1FF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1FF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1FFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A200C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2018: 4BDC4E09  bl 0x82466e20
	ctx.lr = 0x826A201C;
	sub_82466E20(ctx, base);
	// 826A201C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2030 size=108
    let mut pc: u32 = 0x826A2030;
    'dispatch: loop {
        match pc {
            0x826A2030 => {
    //   block [0x826A2030..0x826A209C)
	// 826A2030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A203C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2040: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A2044: 38EB83A0  addi r7, r11, -0x7c60
	ctx.r[7].s64 = ctx.r[11].s64 + -31840;
	// 826A2048: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826A204C: 388AA784  addi r4, r10, -0x587c
	ctx.r[4].s64 = ctx.r[10].s64 + -22652;
	// 826A2050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2054: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2058: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A205C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2060: 386A6998  addi r3, r10, 0x6998
	ctx.r[3].s64 = ctx.r[10].s64 + 27032;
	// 826A2064: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A2068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A206C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A207C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2088: 4BDC4D99  bl 0x82466e20
	ctx.lr = 0x826A208C;
	sub_82466E20(ctx, base);
	// 826A208C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A20A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A20A0 size=100
    let mut pc: u32 = 0x826A20A0;
    'dispatch: loop {
        match pc {
            0x826A20A0 => {
    //   block [0x826A20A0..0x826A2104)
	// 826A20A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A20A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A20A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A20AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A20B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A20B4: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A20B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A20BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A20C0: 388AA79C  addi r4, r10, -0x5864
	ctx.r[4].s64 = ctx.r[10].s64 + -22628;
	// 826A20C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A20C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A20CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A20D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A20D4: 386A69C8  addi r3, r10, 0x69c8
	ctx.r[3].s64 = ctx.r[10].s64 + 27080;
	// 826A20D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A20DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A20E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A20E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A20E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A20EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A20F0: 4BDC4D31  bl 0x82466e20
	ctx.lr = 0x826A20F4;
	sub_82466E20(ctx, base);
	// 826A20F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A20F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A20FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2108 size=112
    let mut pc: u32 = 0x826A2108;
    'dispatch: loop {
        match pc {
            0x826A2108 => {
    //   block [0x826A2108..0x826A2178)
	// 826A2108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A210C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2114: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2118: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A211C: 38AA69C8  addi r5, r10, 0x69c8
	ctx.r[5].s64 = ctx.r[10].s64 + 27080;
	// 826A2120: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A2124: 390B8430  addi r8, r11, -0x7bd0
	ctx.r[8].s64 = ctx.r[11].s64 + -31696;
	// 826A2128: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826A212C: 388AA7B8  addi r4, r10, -0x5848
	ctx.r[4].s64 = ctx.r[10].s64 + -22600;
	// 826A2130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2134: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2138: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A213C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2140: 386A69F8  addi r3, r10, 0x69f8
	ctx.r[3].s64 = ctx.r[10].s64 + 27128;
	// 826A2144: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A2148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A214C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A215C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2164: 4BDC4CBD  bl 0x82466e20
	ctx.lr = 0x826A2168;
	sub_82466E20(ctx, base);
	// 826A2168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A216C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2178 size=108
    let mut pc: u32 = 0x826A2178;
    'dispatch: loop {
        match pc {
            0x826A2178 => {
    //   block [0x826A2178..0x826A21E4)
	// 826A2178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A217C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2184: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2188: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A218C: 38EB8490  addi r7, r11, -0x7b70
	ctx.r[7].s64 = ctx.r[11].s64 + -31600;
	// 826A2190: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A2194: 388AA7DC  addi r4, r10, -0x5824
	ctx.r[4].s64 = ctx.r[10].s64 + -22564;
	// 826A2198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A219C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A21A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A21A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A21A8: 386A6A28  addi r3, r10, 0x6a28
	ctx.r[3].s64 = ctx.r[10].s64 + 27176;
	// 826A21AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A21B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A21B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A21B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A21BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A21C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A21C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A21C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A21CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A21D0: 4BDC4C51  bl 0x82466e20
	ctx.lr = 0x826A21D4;
	sub_82466E20(ctx, base);
	// 826A21D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A21D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A21DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A21E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A21E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A21E8 size=108
    let mut pc: u32 = 0x826A21E8;
    'dispatch: loop {
        match pc {
            0x826A21E8 => {
    //   block [0x826A21E8..0x826A2254)
	// 826A21E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A21EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A21F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A21F4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A21F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A21FC: 38EB84C0  addi r7, r11, -0x7b40
	ctx.r[7].s64 = ctx.r[11].s64 + -31552;
	// 826A2200: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A2204: 388AA7E4  addi r4, r10, -0x581c
	ctx.r[4].s64 = ctx.r[10].s64 + -22556;
	// 826A2208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A220C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2210: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A2214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2218: 386A6A58  addi r3, r10, 0x6a58
	ctx.r[3].s64 = ctx.r[10].s64 + 27224;
	// 826A221C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A2220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A222C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A223C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2240: 4BDC4BE1  bl 0x82466e20
	ctx.lr = 0x826A2244;
	sub_82466E20(ctx, base);
	// 826A2244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A224C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2258 size=108
    let mut pc: u32 = 0x826A2258;
    'dispatch: loop {
        match pc {
            0x826A2258 => {
    //   block [0x826A2258..0x826A22C4)
	// 826A2258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A225C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2264: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2268: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A226C: 38EB8520  addi r7, r11, -0x7ae0
	ctx.r[7].s64 = ctx.r[11].s64 + -31456;
	// 826A2270: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826A2274: 388AA7F8  addi r4, r10, -0x5808
	ctx.r[4].s64 = ctx.r[10].s64 + -22536;
	// 826A2278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A227C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2280: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A2284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2288: 386A6A88  addi r3, r10, 0x6a88
	ctx.r[3].s64 = ctx.r[10].s64 + 27272;
	// 826A228C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A2290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A229C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A22A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A22A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A22A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A22AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A22B0: 4BDC4B71  bl 0x82466e20
	ctx.lr = 0x826A22B4;
	sub_82466E20(ctx, base);
	// 826A22B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A22B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A22BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A22C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A22C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A22C8 size=112
    let mut pc: u32 = 0x826A22C8;
    'dispatch: loop {
        match pc {
            0x826A22C8 => {
    //   block [0x826A22C8..0x826A2338)
	// 826A22C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A22CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A22D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A22D4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A22D8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826A22DC: 38EA8598  addi r7, r10, -0x7a68
	ctx.r[7].s64 = ctx.r[10].s64 + -31336;
	// 826A22E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A22E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A22E8: 388AA804  addi r4, r10, -0x57fc
	ctx.r[4].s64 = ctx.r[10].s64 + -22524;
	// 826A22EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A22F0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A22F4: 396BB520  addi r11, r11, -0x4ae0
	ctx.r[11].s64 = ctx.r[11].s64 + -19168;
	// 826A22F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A22FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2300: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2304: 386A6AB8  addi r3, r10, 0x6ab8
	ctx.r[3].s64 = ctx.r[10].s64 + 27320;
	// 826A2308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A230C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A2310: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2314: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A2318: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A231C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2320: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2324: 4BDC4AFD  bl 0x82466e20
	ctx.lr = 0x826A2328;
	sub_82466E20(ctx, base);
	// 826A2328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A232C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2338 size=96
    let mut pc: u32 = 0x826A2338;
    'dispatch: loop {
        match pc {
            0x826A2338 => {
    //   block [0x826A2338..0x826A2398)
	// 826A2338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A233C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2344: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A2348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A234C: 388AA82C  addi r4, r10, -0x57d4
	ctx.r[4].s64 = ctx.r[10].s64 + -22484;
	// 826A2350: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2358: 386A6AE8  addi r3, r10, 0x6ae8
	ctx.r[3].s64 = ctx.r[10].s64 + 27368;
	// 826A235C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A236C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2378: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A237C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2380: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A2384: 4BDC4A9D  bl 0x82466e20
	ctx.lr = 0x826A2388;
	sub_82466E20(ctx, base);
	// 826A2388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A238C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2398 size=112
    let mut pc: u32 = 0x826A2398;
    'dispatch: loop {
        match pc {
            0x826A2398 => {
    //   block [0x826A2398..0x826A2408)
	// 826A2398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A239C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A23A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A23A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A23A8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A23AC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A23B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A23B4: 390B86B8  addi r8, r11, -0x7948
	ctx.r[8].s64 = ctx.r[11].s64 + -31048;
	// 826A23B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826A23BC: 388AA848  addi r4, r10, -0x57b8
	ctx.r[4].s64 = ctx.r[10].s64 + -22456;
	// 826A23C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A23C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A23C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A23CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A23D0: 386A6B18  addi r3, r10, 0x6b18
	ctx.r[3].s64 = ctx.r[10].s64 + 27416;
	// 826A23D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A23D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A23DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A23E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A23E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A23E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A23EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A23F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A23F4: 4BDC4A2D  bl 0x82466e20
	ctx.lr = 0x826A23F8;
	sub_82466E20(ctx, base);
	// 826A23F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A23FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A2408 size=24
    let mut pc: u32 = 0x826A2408;
    'dispatch: loop {
        match pc {
            0x826A2408 => {
    //   block [0x826A2408..0x826A2420)
	// 826A2408: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A240C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A2410: 394A1A20  addi r10, r10, 0x1a20
	ctx.r[10].s64 = ctx.r[10].s64 + 6688;
	// 826A2414: 816B82C4  lwz r11, -0x7d3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32060 as u32) ) } as u64;
	// 826A2418: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 826A241C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2420 size=116
    let mut pc: u32 = 0x826A2420;
    'dispatch: loop {
        match pc {
            0x826A2420 => {
    //   block [0x826A2420..0x826A2494)
	// 826A2420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A242C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2430: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A2434: 390B1A20  addi r8, r11, 0x1a20
	ctx.r[8].s64 = ctx.r[11].s64 + 6688;
	// 826A2438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A243C: 392AB59C  addi r9, r10, -0x4a64
	ctx.r[9].s64 = ctx.r[10].s64 + -19044;
	// 826A2440: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A2444: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826A2448: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A244C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A2450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2454: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A245C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2464: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A2468: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 826A246C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A2470: 386B6B48  addi r3, r11, 0x6b48
	ctx.r[3].s64 = ctx.r[11].s64 + 27464;
	// 826A2474: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A2478: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A247C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2480: 4BDC49A1  bl 0x82466e20
	ctx.lr = 0x826A2484;
	sub_82466E20(ctx, base);
	// 826A2484: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A248C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A2498 size=24
    let mut pc: u32 = 0x826A2498;
    'dispatch: loop {
        match pc {
            0x826A2498 => {
    //   block [0x826A2498..0x826A24B0)
	// 826A2498: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A249C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A24A0: 394A1AF8  addi r10, r10, 0x1af8
	ctx.r[10].s64 = ctx.r[10].s64 + 6904;
	// 826A24A4: 816B8720  lwz r11, -0x78e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30944 as u32) ) } as u64;
	// 826A24A8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826A24AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A24B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A24B0 size=116
    let mut pc: u32 = 0x826A24B0;
    'dispatch: loop {
        match pc {
            0x826A24B0 => {
    //   block [0x826A24B0..0x826A2524)
	// 826A24B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A24B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A24B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A24BC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A24C0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A24C4: 390B1AF8  addi r8, r11, 0x1af8
	ctx.r[8].s64 = ctx.r[11].s64 + 6904;
	// 826A24C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A24CC: 392AB650  addi r9, r10, -0x49b0
	ctx.r[9].s64 = ctx.r[10].s64 + -18864;
	// 826A24D0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A24D4: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 826A24D8: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A24DC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A24E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A24E4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A24E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A24EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A24F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A24F4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A24F8: 388AA870  addi r4, r10, -0x5790
	ctx.r[4].s64 = ctx.r[10].s64 + -22416;
	// 826A24FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A2500: 386B6B78  addi r3, r11, 0x6b78
	ctx.r[3].s64 = ctx.r[11].s64 + 27512;
	// 826A2504: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A2508: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A250C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2510: 4BDC4911  bl 0x82466e20
	ctx.lr = 0x826A2514;
	sub_82466E20(ctx, base);
	// 826A2514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A251C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2528 size=112
    let mut pc: u32 = 0x826A2528;
    'dispatch: loop {
        match pc {
            0x826A2528 => {
    //   block [0x826A2528..0x826A2598)
	// 826A2528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A252C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2534: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A2538: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A253C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A2540: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A2544: 390B8728  addi r8, r11, -0x78d8
	ctx.r[8].s64 = ctx.r[11].s64 + -30936;
	// 826A2548: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A254C: 388AA884  addi r4, r10, -0x577c
	ctx.r[4].s64 = ctx.r[10].s64 + -22396;
	// 826A2550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2554: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2558: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A255C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2560: 386A6BA8  addi r3, r10, 0x6ba8
	ctx.r[3].s64 = ctx.r[10].s64 + 27560;
	// 826A2564: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A2568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A256C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A257C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2584: 4BDC489D  bl 0x82466e20
	ctx.lr = 0x826A2588;
	sub_82466E20(ctx, base);
	// 826A2588: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A258C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2598 size=112
    let mut pc: u32 = 0x826A2598;
    'dispatch: loop {
        match pc {
            0x826A2598 => {
    //   block [0x826A2598..0x826A2608)
	// 826A2598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A259C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A25A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A25A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A25A8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A25AC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A25B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A25B4: 390B8770  addi r8, r11, -0x7890
	ctx.r[8].s64 = ctx.r[11].s64 + -30864;
	// 826A25B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A25BC: 388AA89C  addi r4, r10, -0x5764
	ctx.r[4].s64 = ctx.r[10].s64 + -22372;
	// 826A25C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A25C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A25C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A25CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A25D0: 386A6BD8  addi r3, r10, 0x6bd8
	ctx.r[3].s64 = ctx.r[10].s64 + 27608;
	// 826A25D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A25D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A25DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A25E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A25E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A25E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A25EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A25F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A25F4: 4BDC482D  bl 0x82466e20
	ctx.lr = 0x826A25F8;
	sub_82466E20(ctx, base);
	// 826A25F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A25FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2608 size=108
    let mut pc: u32 = 0x826A2608;
    'dispatch: loop {
        match pc {
            0x826A2608 => {
    //   block [0x826A2608..0x826A2674)
	// 826A2608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A260C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2614: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A261C: 38EB87B8  addi r7, r11, -0x7848
	ctx.r[7].s64 = ctx.r[11].s64 + -30792;
	// 826A2620: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826A2624: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 826A2628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A262C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A2634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2638: 386A6C08  addi r3, r10, 0x6c08
	ctx.r[3].s64 = ctx.r[10].s64 + 27656;
	// 826A263C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A2640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A264C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A265C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2660: 4BDC47C1  bl 0x82466e20
	ctx.lr = 0x826A2664;
	sub_82466E20(ctx, base);
	// 826A2664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A266C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2678 size=112
    let mut pc: u32 = 0x826A2678;
    'dispatch: loop {
        match pc {
            0x826A2678 => {
    //   block [0x826A2678..0x826A26E8)
	// 826A2678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A267C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2684: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A2688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A268C: 392BB684  addi r9, r11, -0x497c
	ctx.r[9].s64 = ctx.r[11].s64 + -18812;
	// 826A2690: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826A2694: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826A2698: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A269C: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 826A26A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A26A4: 396B8860  addi r11, r11, -0x77a0
	ctx.r[11].s64 = ctx.r[11].s64 + -30624;
	// 826A26A8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A26AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A26B0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A26B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A26B8: 386A6C38  addi r3, r10, 0x6c38
	ctx.r[3].s64 = ctx.r[10].s64 + 27704;
	// 826A26BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A26C0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A26C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A26C8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A26CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A26D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A26D4: 4BDC474D  bl 0x82466e20
	ctx.lr = 0x826A26D8;
	sub_82466E20(ctx, base);
	// 826A26D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A26DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A26E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A26E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A26E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A26E8 size=112
    let mut pc: u32 = 0x826A26E8;
    'dispatch: loop {
        match pc {
            0x826A26E8 => {
    //   block [0x826A26E8..0x826A2758)
	// 826A26E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A26EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A26F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A26F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A26F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A26FC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A2700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2704: 390B89B0  addi r8, r11, -0x7650
	ctx.r[8].s64 = ctx.r[11].s64 + -30288;
	// 826A2708: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826A270C: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 826A2710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2714: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A271C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2720: 386A6C68  addi r3, r10, 0x6c68
	ctx.r[3].s64 = ctx.r[10].s64 + 27752;
	// 826A2724: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A2728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A272C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A273C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2744: 4BDC46DD  bl 0x82466e20
	ctx.lr = 0x826A2748;
	sub_82466E20(ctx, base);
	// 826A2748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A274C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2758 size=112
    let mut pc: u32 = 0x826A2758;
    'dispatch: loop {
        match pc {
            0x826A2758 => {
    //   block [0x826A2758..0x826A27C8)
	// 826A2758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A275C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2764: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A2768: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A276C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A2770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2774: 390B8A58  addi r8, r11, -0x75a8
	ctx.r[8].s64 = ctx.r[11].s64 + -30120;
	// 826A2778: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826A277C: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 826A2780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2784: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A278C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2790: 386A6C98  addi r3, r10, 0x6c98
	ctx.r[3].s64 = ctx.r[10].s64 + 27800;
	// 826A2794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A2798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A279C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A27A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A27A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A27A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A27AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A27B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A27B4: 4BDC466D  bl 0x82466e20
	ctx.lr = 0x826A27B8;
	sub_82466E20(ctx, base);
	// 826A27B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A27BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A27C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A27C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A27C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A27C8 size=112
    let mut pc: u32 = 0x826A27C8;
    'dispatch: loop {
        match pc {
            0x826A27C8 => {
    //   block [0x826A27C8..0x826A2838)
	// 826A27C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A27CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A27D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A27D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A27D8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A27DC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A27E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A27E4: 390B8AE8  addi r8, r11, -0x7518
	ctx.r[8].s64 = ctx.r[11].s64 + -29976;
	// 826A27E8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826A27EC: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 826A27F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A27F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A27F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A27FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2800: 386A6CC8  addi r3, r10, 0x6cc8
	ctx.r[3].s64 = ctx.r[10].s64 + 27848;
	// 826A2804: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A2808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A280C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A281C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2824: 4BDC45FD  bl 0x82466e20
	ctx.lr = 0x826A2828;
	sub_82466E20(ctx, base);
	// 826A2828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A282C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2838 size=116
    let mut pc: u32 = 0x826A2838;
    'dispatch: loop {
        match pc {
            0x826A2838 => {
    //   block [0x826A2838..0x826A28AC)
	// 826A2838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A283C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2844: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A2848: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826A284C: 390A8B90  addi r8, r10, -0x7470
	ctx.r[8].s64 = ctx.r[10].s64 + -29808;
	// 826A2850: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A2854: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A2858: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A285C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A2860: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A2864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2868: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A286C: 388AA8B8  addi r4, r10, -0x5748
	ctx.r[4].s64 = ctx.r[10].s64 + -22344;
	// 826A2870: 396BB6D8  addi r11, r11, -0x4928
	ctx.r[11].s64 = ctx.r[11].s64 + -18728;
	// 826A2874: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2878: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A287C: 386A6CF8  addi r3, r10, 0x6cf8
	ctx.r[3].s64 = ctx.r[10].s64 + 27896;
	// 826A2880: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A2884: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2888: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A288C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2898: 4BDC4589  bl 0x82466e20
	ctx.lr = 0x826A289C;
	sub_82466E20(ctx, base);
	// 826A289C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A28A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A28A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A28A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A28B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A28B0 size=108
    let mut pc: u32 = 0x826A28B0;
    'dispatch: loop {
        match pc {
            0x826A28B0 => {
    //   block [0x826A28B0..0x826A291C)
	// 826A28B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A28B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A28B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A28BC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A28C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A28C4: 38EB8C20  addi r7, r11, -0x73e0
	ctx.r[7].s64 = ctx.r[11].s64 + -29664;
	// 826A28C8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826A28CC: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 826A28D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A28D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A28D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A28DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A28E0: 386A6D28  addi r3, r10, 0x6d28
	ctx.r[3].s64 = ctx.r[10].s64 + 27944;
	// 826A28E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A28E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A28EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A28F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A28F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A28F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A28FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2904: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2908: 4BDC4519  bl 0x82466e20
	ctx.lr = 0x826A290C;
	sub_82466E20(ctx, base);
	// 826A290C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2920 size=112
    let mut pc: u32 = 0x826A2920;
    'dispatch: loop {
        match pc {
            0x826A2920 => {
    //   block [0x826A2920..0x826A2990)
	// 826A2920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A292C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A2930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2934: 392BB798  addi r9, r11, -0x4868
	ctx.r[9].s64 = ctx.r[11].s64 + -18536;
	// 826A2938: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 826A293C: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826A2940: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2944: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 826A2948: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A294C: 396B8CE4  addi r11, r11, -0x731c
	ctx.r[11].s64 = ctx.r[11].s64 + -29468;
	// 826A2950: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A2954: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2958: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A295C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2960: 386A6D58  addi r3, r10, 0x6d58
	ctx.r[3].s64 = ctx.r[10].s64 + 27992;
	// 826A2964: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A2968: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A296C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2970: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A2974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2978: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A297C: 4BDC44A5  bl 0x82466e20
	ctx.lr = 0x826A2980;
	sub_82466E20(ctx, base);
	// 826A2980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A298C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2990 size=100
    let mut pc: u32 = 0x826A2990;
    'dispatch: loop {
        match pc {
            0x826A2990 => {
    //   block [0x826A2990..0x826A29F4)
	// 826A2990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A299C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A29A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A29A4: 38AA7748  addi r5, r10, 0x7748
	ctx.r[5].s64 = ctx.r[10].s64 + 30536;
	// 826A29A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A29AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A29B0: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 826A29B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A29B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A29BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A29C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A29C4: 386A6D88  addi r3, r10, 0x6d88
	ctx.r[3].s64 = ctx.r[10].s64 + 28040;
	// 826A29C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A29CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A29D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A29D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A29D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A29DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A29E0: 4BDC4441  bl 0x82466e20
	ctx.lr = 0x826A29E4;
	sub_82466E20(ctx, base);
	// 826A29E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A29E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A29EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A29F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A29F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A29F8 size=24
    let mut pc: u32 = 0x826A29F8;
    'dispatch: loop {
        match pc {
            0x826A29F8 => {
    //   block [0x826A29F8..0x826A2A10)
	// 826A29F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A29FC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A2A00: 394A1C30  addi r10, r10, 0x1c30
	ctx.r[10].s64 = ctx.r[10].s64 + 7216;
	// 826A2A04: 816B8D18  lwz r11, -0x72e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29416 as u32) ) } as u64;
	// 826A2A08: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826A2A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2A10 size=108
    let mut pc: u32 = 0x826A2A10;
    'dispatch: loop {
        match pc {
            0x826A2A10 => {
    //   block [0x826A2A10..0x826A2A7C)
	// 826A2A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2A1C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2A20: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A2A24: 38EB1C30  addi r7, r11, 0x1c30
	ctx.r[7].s64 = ctx.r[11].s64 + 7216;
	// 826A2A28: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826A2A2C: 388AB204  addi r4, r10, -0x4dfc
	ctx.r[4].s64 = ctx.r[10].s64 + -19964;
	// 826A2A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2A34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2A38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A2A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2A40: 386A6DB8  addi r3, r10, 0x6db8
	ctx.r[3].s64 = ctx.r[10].s64 + 28088;
	// 826A2A44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A2A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2A64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2A68: 4BDC43B9  bl 0x82466e20
	ctx.lr = 0x826A2A6C;
	sub_82466E20(ctx, base);
	// 826A2A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2A80 size=108
    let mut pc: u32 = 0x826A2A80;
    'dispatch: loop {
        match pc {
            0x826A2A80 => {
    //   block [0x826A2A80..0x826A2AEC)
	// 826A2A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2A8C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2A90: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A2A94: 38EB8D20  addi r7, r11, -0x72e0
	ctx.r[7].s64 = ctx.r[11].s64 + -29408;
	// 826A2A98: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A2A9C: 388AB224  addi r4, r10, -0x4ddc
	ctx.r[4].s64 = ctx.r[10].s64 + -19932;
	// 826A2AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2AA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2AA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A2AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2AB0: 386A6DE8  addi r3, r10, 0x6de8
	ctx.r[3].s64 = ctx.r[10].s64 + 28136;
	// 826A2AB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A2AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2AD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2AD8: 4BDC4349  bl 0x82466e20
	ctx.lr = 0x826A2ADC;
	sub_82466E20(ctx, base);
	// 826A2ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2AF0 size=108
    let mut pc: u32 = 0x826A2AF0;
    'dispatch: loop {
        match pc {
            0x826A2AF0 => {
    //   block [0x826A2AF0..0x826A2B5C)
	// 826A2AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2AFC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2B00: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A2B04: 38EB8D68  addi r7, r11, -0x7298
	ctx.r[7].s64 = ctx.r[11].s64 + -29336;
	// 826A2B08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A2B0C: 388AB248  addi r4, r10, -0x4db8
	ctx.r[4].s64 = ctx.r[10].s64 + -19896;
	// 826A2B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2B14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2B18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A2B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2B20: 386A6E18  addi r3, r10, 0x6e18
	ctx.r[3].s64 = ctx.r[10].s64 + 28184;
	// 826A2B24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A2B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2B2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2B44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2B48: 4BDC42D9  bl 0x82466e20
	ctx.lr = 0x826A2B4C;
	sub_82466E20(ctx, base);
	// 826A2B4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2B60 size=108
    let mut pc: u32 = 0x826A2B60;
    'dispatch: loop {
        match pc {
            0x826A2B60 => {
    //   block [0x826A2B60..0x826A2BCC)
	// 826A2B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2B6C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2B70: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A2B74: 38EB8D98  addi r7, r11, -0x7268
	ctx.r[7].s64 = ctx.r[11].s64 + -29288;
	// 826A2B78: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A2B7C: 388AB264  addi r4, r10, -0x4d9c
	ctx.r[4].s64 = ctx.r[10].s64 + -19868;
	// 826A2B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2B84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2B88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A2B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2B90: 386A6E48  addi r3, r10, 0x6e48
	ctx.r[3].s64 = ctx.r[10].s64 + 28232;
	// 826A2B94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A2B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2BA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2BB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2BB8: 4BDC4269  bl 0x82466e20
	ctx.lr = 0x826A2BBC;
	sub_82466E20(ctx, base);
	// 826A2BBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2BC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2BC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2BC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2BD0 size=100
    let mut pc: u32 = 0x826A2BD0;
    'dispatch: loop {
        match pc {
            0x826A2BD0 => {
    //   block [0x826A2BD0..0x826A2C34)
	// 826A2BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2BDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2BE4: 38AA6E48  addi r5, r10, 0x6e48
	ctx.r[5].s64 = ctx.r[10].s64 + 28232;
	// 826A2BE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A2BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2BF0: 388AA8D0  addi r4, r10, -0x5730
	ctx.r[4].s64 = ctx.r[10].s64 + -22320;
	// 826A2BF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2BF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2BFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2C00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2C04: 386A6E78  addi r3, r10, 0x6e78
	ctx.r[3].s64 = ctx.r[10].s64 + 28280;
	// 826A2C08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2C0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2C10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A2C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2C18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A2C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2C20: 4BDC4201  bl 0x82466e20
	ctx.lr = 0x826A2C24;
	sub_82466E20(ctx, base);
	// 826A2C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2C38 size=112
    let mut pc: u32 = 0x826A2C38;
    'dispatch: loop {
        match pc {
            0x826A2C38 => {
    //   block [0x826A2C38..0x826A2CA8)
	// 826A2C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2C44: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A2C48: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2C4C: 392AB820  addi r9, r10, -0x47e0
	ctx.r[9].s64 = ctx.r[10].s64 + -18400;
	// 826A2C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2C54: 390B8DC8  addi r8, r11, -0x7238
	ctx.r[8].s64 = ctx.r[11].s64 + -29240;
	// 826A2C58: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826A2C5C: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 826A2C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2C64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A2C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2C70: 386A6EA8  addi r3, r10, 0x6ea8
	ctx.r[3].s64 = ctx.r[10].s64 + 28328;
	// 826A2C74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A2C78: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826A2C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2C8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2C94: 4BDC418D  bl 0x82466e20
	ctx.lr = 0x826A2C98;
	sub_82466E20(ctx, base);
	// 826A2C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2CA8 size=112
    let mut pc: u32 = 0x826A2CA8;
    'dispatch: loop {
        match pc {
            0x826A2CA8 => {
    //   block [0x826A2CA8..0x826A2D18)
	// 826A2CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2CB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2CB8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2CBC: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A2CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2CC4: 390B8E10  addi r8, r11, -0x71f0
	ctx.r[8].s64 = ctx.r[11].s64 + -29168;
	// 826A2CC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A2CCC: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 826A2CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2CD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2CD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A2CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2CE0: 386A6ED8  addi r3, r10, 0x6ed8
	ctx.r[3].s64 = ctx.r[10].s64 + 28376;
	// 826A2CE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A2CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2D04: 4BDC411D  bl 0x82466e20
	ctx.lr = 0x826A2D08;
	sub_82466E20(ctx, base);
	// 826A2D08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2D18 size=116
    let mut pc: u32 = 0x826A2D18;
    'dispatch: loop {
        match pc {
            0x826A2D18 => {
    //   block [0x826A2D18..0x826A2D8C)
	// 826A2D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2D24: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A2D28: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826A2D2C: 390A8E40  addi r8, r10, -0x71c0
	ctx.r[8].s64 = ctx.r[10].s64 + -29120;
	// 826A2D30: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2D34: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A2D38: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A2D3C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2D40: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A2D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A2D4C: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 826A2D50: 396BB848  addi r11, r11, -0x47b8
	ctx.r[11].s64 = ctx.r[11].s64 + -18360;
	// 826A2D54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2D58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2D5C: 386A6F08  addi r3, r10, 0x6f08
	ctx.r[3].s64 = ctx.r[10].s64 + 28424;
	// 826A2D60: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A2D64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2D68: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A2D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2D74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2D78: 4BDC40A9  bl 0x82466e20
	ctx.lr = 0x826A2D7C;
	sub_82466E20(ctx, base);
	// 826A2D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2D90 size=100
    let mut pc: u32 = 0x826A2D90;
    'dispatch: loop {
        match pc {
            0x826A2D90 => {
    //   block [0x826A2D90..0x826A2DF4)
	// 826A2D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2D9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2DA4: 38AA6F08  addi r5, r10, 0x6f08
	ctx.r[5].s64 = ctx.r[10].s64 + 28424;
	// 826A2DA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2DB0: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826A2DB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2DB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2DBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2DC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2DC4: 386A6F38  addi r3, r10, 0x6f38
	ctx.r[3].s64 = ctx.r[10].s64 + 28472;
	// 826A2DC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2DCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2DD0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A2DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2DD8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A2DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2DE0: 4BDC4041  bl 0x82466e20
	ctx.lr = 0x826A2DE4;
	sub_82466E20(ctx, base);
	// 826A2DE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2DF8 size=116
    let mut pc: u32 = 0x826A2DF8;
    'dispatch: loop {
        match pc {
            0x826A2DF8 => {
    //   block [0x826A2DF8..0x826A2E6C)
	// 826A2DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2E04: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A2E08: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2E0C: 392BB894  addi r9, r11, -0x476c
	ctx.r[9].s64 = ctx.r[11].s64 + -18284;
	// 826A2E10: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A2E14: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2E18: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826A2E1C: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826A2E20: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2E24: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 826A2E28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2E2C: 396B8EE8  addi r11, r11, -0x7118
	ctx.r[11].s64 = ctx.r[11].s64 + -28952;
	// 826A2E30: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A2E34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2E38: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A2E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2E40: 386A6F68  addi r3, r10, 0x6f68
	ctx.r[3].s64 = ctx.r[10].s64 + 28520;
	// 826A2E44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A2E48: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A2E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2E50: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A2E54: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A2E58: 4BDC3FC9  bl 0x82466e20
	ctx.lr = 0x826A2E5C;
	sub_82466E20(ctx, base);
	// 826A2E5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A2E70 size=24
    let mut pc: u32 = 0x826A2E70;
    'dispatch: loop {
        match pc {
            0x826A2E70 => {
    //   block [0x826A2E70..0x826A2E88)
	// 826A2E70: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2E74: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A2E78: 394A1CC0  addi r10, r10, 0x1cc0
	ctx.r[10].s64 = ctx.r[10].s64 + 7360;
	// 826A2E7C: 816B9038  lwz r11, -0x6fc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28616 as u32) ) } as u64;
	// 826A2E80: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826A2E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2E88 size=116
    let mut pc: u32 = 0x826A2E88;
    'dispatch: loop {
        match pc {
            0x826A2E88 => {
    //   block [0x826A2E88..0x826A2EFC)
	// 826A2E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2E94: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A2E98: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A2E9C: 392BB900  addi r9, r11, -0x4700
	ctx.r[9].s64 = ctx.r[11].s64 + -18176;
	// 826A2EA0: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A2EA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2EA8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826A2EAC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 826A2EB0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2EB4: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 826A2EB8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2EBC: 396B1CC0  addi r11, r11, 0x1cc0
	ctx.r[11].s64 = ctx.r[11].s64 + 7360;
	// 826A2EC0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A2EC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2EC8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A2ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2ED0: 386A6F98  addi r3, r10, 0x6f98
	ctx.r[3].s64 = ctx.r[10].s64 + 28568;
	// 826A2ED4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A2ED8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A2EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2EE0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A2EE4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A2EE8: 4BDC3F39  bl 0x82466e20
	ctx.lr = 0x826A2EEC;
	sub_82466E20(ctx, base);
	// 826A2EEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2F00 size=108
    let mut pc: u32 = 0x826A2F00;
    'dispatch: loop {
        match pc {
            0x826A2F00 => {
    //   block [0x826A2F00..0x826A2F6C)
	// 826A2F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2F0C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2F10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2F14: 38EB9040  addi r7, r11, -0x6fc0
	ctx.r[7].s64 = ctx.r[11].s64 + -28608;
	// 826A2F18: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826A2F1C: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 826A2F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2F24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2F28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A2F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2F30: 386A6FC8  addi r3, r10, 0x6fc8
	ctx.r[3].s64 = ctx.r[10].s64 + 28616;
	// 826A2F34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A2F38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2F3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2F40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2F48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2F50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2F54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2F58: 4BDC3EC9  bl 0x82466e20
	ctx.lr = 0x826A2F5C;
	sub_82466E20(ctx, base);
	// 826A2F5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A2F70 size=24
    let mut pc: u32 = 0x826A2F70;
    'dispatch: loop {
        match pc {
            0x826A2F70 => {
    //   block [0x826A2F70..0x826A2F88)
	// 826A2F70: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2F74: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A2F78: 394A1D80  addi r10, r10, 0x1d80
	ctx.r[10].s64 = ctx.r[10].s64 + 7552;
	// 826A2F7C: 816B903C  lwz r11, -0x6fc4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28612 as u32) ) } as u64;
	// 826A2F80: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826A2F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2F88 size=116
    let mut pc: u32 = 0x826A2F88;
    'dispatch: loop {
        match pc {
            0x826A2F88 => {
    //   block [0x826A2F88..0x826A2FFC)
	// 826A2F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2F94: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2F98: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A2F9C: 390B1D80  addi r8, r11, 0x1d80
	ctx.r[8].s64 = ctx.r[11].s64 + 7552;
	// 826A2FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2FA4: 392AB978  addi r9, r10, -0x4688
	ctx.r[9].s64 = ctx.r[10].s64 + -18056;
	// 826A2FA8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2FAC: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 826A2FB0: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A2FB4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A2FB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2FBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2FC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2FC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2FCC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A2FD0: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826A2FD4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A2FD8: 386B6FF8  addi r3, r11, 0x6ff8
	ctx.r[3].s64 = ctx.r[11].s64 + 28664;
	// 826A2FDC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826A2FE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2FE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2FE8: 4BDC3E39  bl 0x82466e20
	ctx.lr = 0x826A2FEC;
	sub_82466E20(ctx, base);
	// 826A2FEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3000 size=112
    let mut pc: u32 = 0x826A3000;
    'dispatch: loop {
        match pc {
            0x826A3000 => {
    //   block [0x826A3000..0x826A3070)
	// 826A3000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A300C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3010: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3014: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A3018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A301C: 390B90BC  addi r8, r11, -0x6f44
	ctx.r[8].s64 = ctx.r[11].s64 + -28484;
	// 826A3020: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A3024: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 826A3028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A302C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3038: 386A7028  addi r3, r10, 0x7028
	ctx.r[3].s64 = ctx.r[10].s64 + 28712;
	// 826A303C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A304C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A305C: 4BDC3DC5  bl 0x82466e20
	ctx.lr = 0x826A3060;
	sub_82466E20(ctx, base);
	// 826A3060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A306C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A3070 size=24
    let mut pc: u32 = 0x826A3070;
    'dispatch: loop {
        match pc {
            0x826A3070 => {
    //   block [0x826A3070..0x826A3088)
	// 826A3070: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3074: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3078: 394A1F18  addi r10, r10, 0x1f18
	ctx.r[10].s64 = ctx.r[10].s64 + 7960;
	// 826A307C: 816B90EC  lwz r11, -0x6f14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28436 as u32) ) } as u64;
	// 826A3080: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 826A3084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3088 size=116
    let mut pc: u32 = 0x826A3088;
    'dispatch: loop {
        match pc {
            0x826A3088 => {
    //   block [0x826A3088..0x826A30FC)
	// 826A3088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A308C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3094: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3098: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A309C: 392BB9B0  addi r9, r11, -0x4650
	ctx.r[9].s64 = ctx.r[11].s64 + -18000;
	// 826A30A0: 38AA6F68  addi r5, r10, 0x6f68
	ctx.r[5].s64 = ctx.r[10].s64 + 28520;
	// 826A30A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A30A8: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826A30AC: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 826A30B0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A30B4: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 826A30B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A30BC: 396B1F18  addi r11, r11, 0x1f18
	ctx.r[11].s64 = ctx.r[11].s64 + 7960;
	// 826A30C0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A30C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A30C8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A30CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A30D0: 386A7058  addi r3, r10, 0x7058
	ctx.r[3].s64 = ctx.r[10].s64 + 28760;
	// 826A30D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A30D8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A30DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A30E0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A30E4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A30E8: 4BDC3D39  bl 0x82466e20
	ctx.lr = 0x826A30EC;
	sub_82466E20(ctx, base);
	// 826A30EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A30F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A30F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A30F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3100 size=116
    let mut pc: u32 = 0x826A3100;
    'dispatch: loop {
        match pc {
            0x826A3100 => {
    //   block [0x826A3100..0x826A3174)
	// 826A3100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A310C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3110: 38E0000F  li r7, 0xf
	ctx.r[7].s64 = 15;
	// 826A3114: 390A90F0  addi r8, r10, -0x6f10
	ctx.r[8].s64 = ctx.r[10].s64 + -28432;
	// 826A3118: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A311C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3120: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A3124: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A3128: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A312C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3130: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3134: 388AB294  addi r4, r10, -0x4d6c
	ctx.r[4].s64 = ctx.r[10].s64 + -19820;
	// 826A3138: 396BBA20  addi r11, r11, -0x45e0
	ctx.r[11].s64 = ctx.r[11].s64 + -17888;
	// 826A313C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3140: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3144: 386A7088  addi r3, r10, 0x7088
	ctx.r[3].s64 = ctx.r[10].s64 + 28808;
	// 826A3148: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A314C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3150: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A315C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3160: 4BDC3CC1  bl 0x82466e20
	ctx.lr = 0x826A3164;
	sub_82466E20(ctx, base);
	// 826A3164: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A316C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3178 size=116
    let mut pc: u32 = 0x826A3178;
    'dispatch: loop {
        match pc {
            0x826A3178 => {
    //   block [0x826A3178..0x826A31EC)
	// 826A3178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A317C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3184: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3188: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826A318C: 390A9258  addi r8, r10, -0x6da8
	ctx.r[8].s64 = ctx.r[10].s64 + -28072;
	// 826A3190: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A3194: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3198: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A319C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A31A0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A31A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A31A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A31AC: 388AB2BC  addi r4, r10, -0x4d44
	ctx.r[4].s64 = ctx.r[10].s64 + -19780;
	// 826A31B0: 396BBA9C  addi r11, r11, -0x4564
	ctx.r[11].s64 = ctx.r[11].s64 + -17764;
	// 826A31B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A31B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A31BC: 386A70B8  addi r3, r10, 0x70b8
	ctx.r[3].s64 = ctx.r[10].s64 + 28856;
	// 826A31C0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A31C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A31C8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A31CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A31D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A31D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A31D8: 4BDC3C49  bl 0x82466e20
	ctx.lr = 0x826A31DC;
	sub_82466E20(ctx, base);
	// 826A31DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A31E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A31E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A31E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A31F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A31F0 size=112
    let mut pc: u32 = 0x826A31F0;
    'dispatch: loop {
        match pc {
            0x826A31F0 => {
    //   block [0x826A31F0..0x826A3260)
	// 826A31F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A31F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A31F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A31FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3200: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3204: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A3208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A320C: 390B92A0  addi r8, r11, -0x6d60
	ctx.r[8].s64 = ctx.r[11].s64 + -28000;
	// 826A3210: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A3214: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 826A3218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A321C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3228: 386A70E8  addi r3, r10, 0x70e8
	ctx.r[3].s64 = ctx.r[10].s64 + 28904;
	// 826A322C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A323C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A324C: 4BDC3BD5  bl 0x82466e20
	ctx.lr = 0x826A3250;
	sub_82466E20(ctx, base);
	// 826A3250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A325C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3260 size=100
    let mut pc: u32 = 0x826A3260;
    'dispatch: loop {
        match pc {
            0x826A3260 => {
    //   block [0x826A3260..0x826A32C4)
	// 826A3260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A326C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3274: 38AA7748  addi r5, r10, 0x7748
	ctx.r[5].s64 = ctx.r[10].s64 + 30536;
	// 826A3278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A327C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3280: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 826A3284: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A328C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3294: 386A7118  addi r3, r10, 0x7118
	ctx.r[3].s64 = ctx.r[10].s64 + 28952;
	// 826A3298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A329C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A32A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A32A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A32A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A32AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A32B0: 4BDC3B71  bl 0x82466e20
	ctx.lr = 0x826A32B4;
	sub_82466E20(ctx, base);
	// 826A32B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A32B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A32BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A32C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A32C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A32C8 size=112
    let mut pc: u32 = 0x826A32C8;
    'dispatch: loop {
        match pc {
            0x826A32C8 => {
    //   block [0x826A32C8..0x826A3338)
	// 826A32C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A32CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A32D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A32D4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A32D8: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826A32DC: 38EA92B8  addi r7, r10, -0x6d48
	ctx.r[7].s64 = ctx.r[10].s64 + -27976;
	// 826A32E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A32E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A32E8: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 826A32EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A32F0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A32F4: 396BBAC8  addi r11, r11, -0x4538
	ctx.r[11].s64 = ctx.r[11].s64 + -17720;
	// 826A32F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A32FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3300: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3304: 386A7148  addi r3, r10, 0x7148
	ctx.r[3].s64 = ctx.r[10].s64 + 29000;
	// 826A3308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A330C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A3310: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3314: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3318: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A331C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3320: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A3324: 4BDC3AFD  bl 0x82466e20
	ctx.lr = 0x826A3328;
	sub_82466E20(ctx, base);
	// 826A3328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A332C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3338 size=112
    let mut pc: u32 = 0x826A3338;
    'dispatch: loop {
        match pc {
            0x826A3338 => {
    //   block [0x826A3338..0x826A33A8)
	// 826A3338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A333C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3344: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3348: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A334C: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3354: 390B93F0  addi r8, r11, -0x6c10
	ctx.r[8].s64 = ctx.r[11].s64 + -27664;
	// 826A3358: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A335C: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 826A3360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3364: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A336C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3370: 386A7178  addi r3, r10, 0x7178
	ctx.r[3].s64 = ctx.r[10].s64 + 29048;
	// 826A3374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A337C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A338C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3394: 4BDC3A8D  bl 0x82466e20
	ctx.lr = 0x826A3398;
	sub_82466E20(ctx, base);
	// 826A3398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A339C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A33A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A33A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A33A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A33A8 size=112
    let mut pc: u32 = 0x826A33A8;
    'dispatch: loop {
        match pc {
            0x826A33A8 => {
    //   block [0x826A33A8..0x826A3418)
	// 826A33A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A33AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A33B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A33B4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A33B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A33BC: 38EA9420  addi r7, r10, -0x6be0
	ctx.r[7].s64 = ctx.r[10].s64 + -27616;
	// 826A33C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A33C4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A33C8: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 826A33CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A33D0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A33D4: 396BBB1C  addi r11, r11, -0x44e4
	ctx.r[11].s64 = ctx.r[11].s64 + -17636;
	// 826A33D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A33DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A33E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A33E4: 386A71A8  addi r3, r10, 0x71a8
	ctx.r[3].s64 = ctx.r[10].s64 + 29096;
	// 826A33E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A33EC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A33F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A33F4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A33F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A33FC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3400: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A3404: 4BDC3A1D  bl 0x82466e20
	ctx.lr = 0x826A3408;
	sub_82466E20(ctx, base);
	// 826A3408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A340C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3418 size=112
    let mut pc: u32 = 0x826A3418;
    'dispatch: loop {
        match pc {
            0x826A3418 => {
    //   block [0x826A3418..0x826A3488)
	// 826A3418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A341C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3424: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3428: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A342C: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3434: 390B9450  addi r8, r11, -0x6bb0
	ctx.r[8].s64 = ctx.r[11].s64 + -27568;
	// 826A3438: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A343C: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 826A3440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3444: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A344C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3450: 386A71D8  addi r3, r10, 0x71d8
	ctx.r[3].s64 = ctx.r[10].s64 + 29144;
	// 826A3454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A345C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A346C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3474: 4BDC39AD  bl 0x82466e20
	ctx.lr = 0x826A3478;
	sub_82466E20(ctx, base);
	// 826A3478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A347C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3488 size=108
    let mut pc: u32 = 0x826A3488;
    'dispatch: loop {
        match pc {
            0x826A3488 => {
    //   block [0x826A3488..0x826A34F4)
	// 826A3488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A348C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3494: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A349C: 38EB9468  addi r7, r11, -0x6b98
	ctx.r[7].s64 = ctx.r[11].s64 + -27544;
	// 826A34A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A34A4: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 826A34A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A34AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A34B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A34B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A34B8: 386A7208  addi r3, r10, 0x7208
	ctx.r[3].s64 = ctx.r[10].s64 + 29192;
	// 826A34BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A34C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A34C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A34C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A34CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A34D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A34D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A34D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A34DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A34E0: 4BDC3941  bl 0x82466e20
	ctx.lr = 0x826A34E4;
	sub_82466E20(ctx, base);
	// 826A34E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A34E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A34EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A34F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A34F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A34F8 size=112
    let mut pc: u32 = 0x826A34F8;
    'dispatch: loop {
        match pc {
            0x826A34F8 => {
    //   block [0x826A34F8..0x826A3568)
	// 826A34F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A34FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3504: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3508: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A350C: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3514: 390B9480  addi r8, r11, -0x6b80
	ctx.r[8].s64 = ctx.r[11].s64 + -27520;
	// 826A3518: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A351C: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 826A3520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3524: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A352C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3530: 386A7238  addi r3, r10, 0x7238
	ctx.r[3].s64 = ctx.r[10].s64 + 29240;
	// 826A3534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A353C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A354C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3554: 4BDC38CD  bl 0x82466e20
	ctx.lr = 0x826A3558;
	sub_82466E20(ctx, base);
	// 826A3558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A355C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3568 size=112
    let mut pc: u32 = 0x826A3568;
    'dispatch: loop {
        match pc {
            0x826A3568 => {
    //   block [0x826A3568..0x826A35D8)
	// 826A3568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A356C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3574: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3578: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826A357C: 38EA9498  addi r7, r10, -0x6b68
	ctx.r[7].s64 = ctx.r[10].s64 + -27496;
	// 826A3580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3584: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3588: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 826A358C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3590: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A3594: 396BBB28  addi r11, r11, -0x44d8
	ctx.r[11].s64 = ctx.r[11].s64 + -17624;
	// 826A3598: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A359C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A35A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A35A4: 386A7268  addi r3, r10, 0x7268
	ctx.r[3].s64 = ctx.r[10].s64 + 29288;
	// 826A35A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A35AC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A35B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A35B4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A35B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A35BC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A35C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A35C4: 4BDC385D  bl 0x82466e20
	ctx.lr = 0x826A35C8;
	sub_82466E20(ctx, base);
	// 826A35C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A35CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A35D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A35D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A35D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A35D8 size=112
    let mut pc: u32 = 0x826A35D8;
    'dispatch: loop {
        match pc {
            0x826A35D8 => {
    //   block [0x826A35D8..0x826A3648)
	// 826A35D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A35DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A35E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A35E4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A35E8: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 826A35EC: 38EA9570  addi r7, r10, -0x6a90
	ctx.r[7].s64 = ctx.r[10].s64 + -27280;
	// 826A35F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A35F4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A35F8: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 826A35FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3600: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A3604: 396BBB68  addi r11, r11, -0x4498
	ctx.r[11].s64 = ctx.r[11].s64 + -17560;
	// 826A3608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A360C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3610: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3614: 386A7298  addi r3, r10, 0x7298
	ctx.r[3].s64 = ctx.r[10].s64 + 29336;
	// 826A3618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A361C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A3620: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3624: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3628: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A362C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3630: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A3634: 4BDC37ED  bl 0x82466e20
	ctx.lr = 0x826A3638;
	sub_82466E20(ctx, base);
	// 826A3638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A363C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3648 size=108
    let mut pc: u32 = 0x826A3648;
    'dispatch: loop {
        match pc {
            0x826A3648 => {
    //   block [0x826A3648..0x826A36B4)
	// 826A3648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A364C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3654: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3658: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A365C: 38EB96C0  addi r7, r11, -0x6940
	ctx.r[7].s64 = ctx.r[11].s64 + -26944;
	// 826A3660: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A3664: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 826A3668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A366C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3670: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A3674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3678: 386A72C8  addi r3, r10, 0x72c8
	ctx.r[3].s64 = ctx.r[10].s64 + 29384;
	// 826A367C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A3680: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A368C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3694: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A369C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A36A0: 4BDC3781  bl 0x82466e20
	ctx.lr = 0x826A36A4;
	sub_82466E20(ctx, base);
	// 826A36A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A36A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A36AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A36B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A36B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A36B8 size=116
    let mut pc: u32 = 0x826A36B8;
    'dispatch: loop {
        match pc {
            0x826A36B8 => {
    //   block [0x826A36B8..0x826A372C)
	// 826A36B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A36BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A36C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A36C4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A36C8: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 826A36CC: 390A9720  addi r8, r10, -0x68e0
	ctx.r[8].s64 = ctx.r[10].s64 + -26848;
	// 826A36D0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A36D4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A36D8: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A36DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A36E0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A36E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A36E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A36EC: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 826A36F0: 396BBC08  addi r11, r11, -0x43f8
	ctx.r[11].s64 = ctx.r[11].s64 + -17400;
	// 826A36F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A36F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A36FC: 386A72F8  addi r3, r10, 0x72f8
	ctx.r[3].s64 = ctx.r[10].s64 + 29432;
	// 826A3700: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A3704: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3708: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A370C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3718: 4BDC3709  bl 0x82466e20
	ctx.lr = 0x826A371C;
	sub_82466E20(ctx, base);
	// 826A371C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3730 size=116
    let mut pc: u32 = 0x826A3730;
    'dispatch: loop {
        match pc {
            0x826A3730 => {
    //   block [0x826A3730..0x826A37A4)
	// 826A3730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A373C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3740: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826A3744: 390A9840  addi r8, r10, -0x67c0
	ctx.r[8].s64 = ctx.r[10].s64 + -26560;
	// 826A3748: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A374C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3750: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3754: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3758: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A375C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3764: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 826A3768: 396BBC40  addi r11, r11, -0x43c0
	ctx.r[11].s64 = ctx.r[11].s64 + -17344;
	// 826A376C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3770: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3774: 386A7328  addi r3, r10, 0x7328
	ctx.r[3].s64 = ctx.r[10].s64 + 29480;
	// 826A3778: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A377C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3780: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A378C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3790: 4BDC3691  bl 0x82466e20
	ctx.lr = 0x826A3794;
	sub_82466E20(ctx, base);
	// 826A3794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A379C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A37A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A37A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A37A8 size=108
    let mut pc: u32 = 0x826A37A8;
    'dispatch: loop {
        match pc {
            0x826A37A8 => {
    //   block [0x826A37A8..0x826A3814)
	// 826A37A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A37AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A37B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A37B4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A37B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A37BC: 38EB98A0  addi r7, r11, -0x6760
	ctx.r[7].s64 = ctx.r[11].s64 + -26464;
	// 826A37C0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826A37C4: 388AA8F8  addi r4, r10, -0x5708
	ctx.r[4].s64 = ctx.r[10].s64 + -22280;
	// 826A37C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A37CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A37D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A37D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A37D8: 386A7358  addi r3, r10, 0x7358
	ctx.r[3].s64 = ctx.r[10].s64 + 29528;
	// 826A37DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A37E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A37E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A37E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A37EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A37F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A37F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A37F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A37FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A3800: 4BDC3621  bl 0x82466e20
	ctx.lr = 0x826A3804;
	sub_82466E20(ctx, base);
	// 826A3804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A380C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3818 size=112
    let mut pc: u32 = 0x826A3818;
    'dispatch: loop {
        match pc {
            0x826A3818 => {
    //   block [0x826A3818..0x826A3888)
	// 826A3818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A381C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3824: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3828: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826A382C: 38EA9948  addi r7, r10, -0x66b8
	ctx.r[7].s64 = ctx.r[10].s64 + -26296;
	// 826A3830: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A3834: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3838: 388AA910  addi r4, r10, -0x56f0
	ctx.r[4].s64 = ctx.r[10].s64 + -22256;
	// 826A383C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3840: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A3844: 396BBC58  addi r11, r11, -0x43a8
	ctx.r[11].s64 = ctx.r[11].s64 + -17320;
	// 826A3848: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A384C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3850: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3854: 386A7388  addi r3, r10, 0x7388
	ctx.r[3].s64 = ctx.r[10].s64 + 29576;
	// 826A3858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A385C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A3860: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3864: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3868: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A386C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3870: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A3874: 4BDC35AD  bl 0x82466e20
	ctx.lr = 0x826A3878;
	sub_82466E20(ctx, base);
	// 826A3878: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A387C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3888 size=112
    let mut pc: u32 = 0x826A3888;
    'dispatch: loop {
        match pc {
            0x826A3888 => {
    //   block [0x826A3888..0x826A38F8)
	// 826A3888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A388C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3894: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3898: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A389C: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A38A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A38A4: 390B9A20  addi r8, r11, -0x65e0
	ctx.r[8].s64 = ctx.r[11].s64 + -26080;
	// 826A38A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A38AC: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 826A38B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A38B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A38B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A38BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A38C0: 386A73B8  addi r3, r10, 0x73b8
	ctx.r[3].s64 = ctx.r[10].s64 + 29624;
	// 826A38C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A38C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A38CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A38D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A38D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A38D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A38DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A38E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A38E4: 4BDC353D  bl 0x82466e20
	ctx.lr = 0x826A38E8;
	sub_82466E20(ctx, base);
	// 826A38E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A38EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A38F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A38F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A38F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A38F8 size=24
    let mut pc: u32 = 0x826A38F8;
    'dispatch: loop {
        match pc {
            0x826A38F8 => {
    //   block [0x826A38F8..0x826A3910)
	// 826A38F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A38FC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3900: 394A2080  addi r10, r10, 0x2080
	ctx.r[10].s64 = ctx.r[10].s64 + 8320;
	// 826A3904: 816B9A68  lwz r11, -0x6598(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26008 as u32) ) } as u64;
	// 826A3908: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 826A390C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3910 size=116
    let mut pc: u32 = 0x826A3910;
    'dispatch: loop {
        match pc {
            0x826A3910 => {
    //   block [0x826A3910..0x826A3984)
	// 826A3910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A391C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3920: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A3924: 392BBCC8  addi r9, r11, -0x4338
	ctx.r[9].s64 = ctx.r[11].s64 + -17208;
	// 826A3928: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A392C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A3930: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826A3934: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 826A3938: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A393C: 388A7B20  addi r4, r10, 0x7b20
	ctx.r[4].s64 = ctx.r[10].s64 + 31520;
	// 826A3940: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3944: 396B2080  addi r11, r11, 0x2080
	ctx.r[11].s64 = ctx.r[11].s64 + 8320;
	// 826A3948: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A394C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3950: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A3954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3958: 386A73E8  addi r3, r10, 0x73e8
	ctx.r[3].s64 = ctx.r[10].s64 + 29672;
	// 826A395C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A3960: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A3964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3968: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A396C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A3970: 4BDC34B1  bl 0x82466e20
	ctx.lr = 0x826A3974;
	sub_82466E20(ctx, base);
	// 826A3974: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A397C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3988 size=116
    let mut pc: u32 = 0x826A3988;
    'dispatch: loop {
        match pc {
            0x826A3988 => {
    //   block [0x826A3988..0x826A39FC)
	// 826A3988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A398C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3994: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3998: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826A399C: 390A9A70  addi r8, r10, -0x6590
	ctx.r[8].s64 = ctx.r[10].s64 + -26000;
	// 826A39A0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A39A4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A39A8: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A39AC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A39B0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A39B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A39B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A39BC: 388AB2FC  addi r4, r10, -0x4d04
	ctx.r[4].s64 = ctx.r[10].s64 + -19716;
	// 826A39C0: 396BBD24  addi r11, r11, -0x42dc
	ctx.r[11].s64 = ctx.r[11].s64 + -17116;
	// 826A39C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A39C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A39CC: 386A7418  addi r3, r10, 0x7418
	ctx.r[3].s64 = ctx.r[10].s64 + 29720;
	// 826A39D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A39D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A39D8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A39DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A39E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A39E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A39E8: 4BDC3439  bl 0x82466e20
	ctx.lr = 0x826A39EC;
	sub_82466E20(ctx, base);
	// 826A39EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A39F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A39F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A39F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3A00 size=116
    let mut pc: u32 = 0x826A3A00;
    'dispatch: loop {
        match pc {
            0x826A3A00 => {
    //   block [0x826A3A00..0x826A3A74)
	// 826A3A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3A0C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3A10: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 826A3A14: 390A9AB8  addi r8, r10, -0x6548
	ctx.r[8].s64 = ctx.r[10].s64 + -25928;
	// 826A3A18: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3A1C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3A20: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3A24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3A28: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A3A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3A30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3A34: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 826A3A38: 396BBD38  addi r11, r11, -0x42c8
	ctx.r[11].s64 = ctx.r[11].s64 + -17096;
	// 826A3A3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3A40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3A44: 386A7448  addi r3, r10, 0x7448
	ctx.r[3].s64 = ctx.r[10].s64 + 29768;
	// 826A3A48: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A3A4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3A50: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3A60: 4BDC33C1  bl 0x82466e20
	ctx.lr = 0x826A3A64;
	sub_82466E20(ctx, base);
	// 826A3A64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3A78 size=112
    let mut pc: u32 = 0x826A3A78;
    'dispatch: loop {
        match pc {
            0x826A3A78 => {
    //   block [0x826A3A78..0x826A3AE8)
	// 826A3A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3A84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3A88: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3A8C: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3A90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A3A94: 390B9BF0  addi r8, r11, -0x6410
	ctx.r[8].s64 = ctx.r[11].s64 + -25616;
	// 826A3A98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A3A9C: 388AA928  addi r4, r10, -0x56d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22232;
	// 826A3AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3AA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3AB0: 386A7478  addi r3, r10, 0x7478
	ctx.r[3].s64 = ctx.r[10].s64 + 29816;
	// 826A3AB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3AD4: 4BDC334D  bl 0x82466e20
	ctx.lr = 0x826A3AD8;
	sub_82466E20(ctx, base);
	// 826A3AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3AE8 size=112
    let mut pc: u32 = 0x826A3AE8;
    'dispatch: loop {
        match pc {
            0x826A3AE8 => {
    //   block [0x826A3AE8..0x826A3B58)
	// 826A3AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3AF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3AF8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3AFC: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3B04: 390B9C08  addi r8, r11, -0x63f8
	ctx.r[8].s64 = ctx.r[11].s64 + -25592;
	// 826A3B08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A3B0C: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 826A3B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3B14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3B18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3B20: 386A74A8  addi r3, r10, 0x74a8
	ctx.r[3].s64 = ctx.r[10].s64 + 29864;
	// 826A3B24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3B44: 4BDC32DD  bl 0x82466e20
	ctx.lr = 0x826A3B48;
	sub_82466E20(ctx, base);
	// 826A3B48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3B58 size=112
    let mut pc: u32 = 0x826A3B58;
    'dispatch: loop {
        match pc {
            0x826A3B58 => {
    //   block [0x826A3B58..0x826A3BC8)
	// 826A3B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3B64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A3B68: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3B6C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A3B70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3B74: 390B9C20  addi r8, r11, -0x63e0
	ctx.r[8].s64 = ctx.r[11].s64 + -25568;
	// 826A3B78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A3B7C: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 826A3B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3B84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3B88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3B90: 386A74D8  addi r3, r10, 0x74d8
	ctx.r[3].s64 = ctx.r[10].s64 + 29912;
	// 826A3B94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3BB4: 4BDC326D  bl 0x82466e20
	ctx.lr = 0x826A3BB8;
	sub_82466E20(ctx, base);
	// 826A3BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3BC8 size=112
    let mut pc: u32 = 0x826A3BC8;
    'dispatch: loop {
        match pc {
            0x826A3BC8 => {
    //   block [0x826A3BC8..0x826A3C38)
	// 826A3BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3BD4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3BD8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826A3BDC: 38EA9C50  addi r7, r10, -0x63b0
	ctx.r[7].s64 = ctx.r[10].s64 + -25520;
	// 826A3BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3BE4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3BE8: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 826A3BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3BF0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A3BF4: 396BBDB0  addi r11, r11, -0x4250
	ctx.r[11].s64 = ctx.r[11].s64 + -16976;
	// 826A3BF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A3BFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3C00: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3C04: 386A7508  addi r3, r10, 0x7508
	ctx.r[3].s64 = ctx.r[10].s64 + 29960;
	// 826A3C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3C0C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A3C10: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3C14: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3C18: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3C1C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3C20: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A3C24: 4BDC31FD  bl 0x82466e20
	ctx.lr = 0x826A3C28;
	sub_82466E20(ctx, base);
	// 826A3C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3C38 size=112
    let mut pc: u32 = 0x826A3C38;
    'dispatch: loop {
        match pc {
            0x826A3C38 => {
    //   block [0x826A3C38..0x826A3CA8)
	// 826A3C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3C44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3C48: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3C4C: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3C54: 390B9CC8  addi r8, r11, -0x6338
	ctx.r[8].s64 = ctx.r[11].s64 + -25400;
	// 826A3C58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A3C5C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 826A3C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3C64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3C70: 386A7538  addi r3, r10, 0x7538
	ctx.r[3].s64 = ctx.r[10].s64 + 30008;
	// 826A3C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3C94: 4BDC318D  bl 0x82466e20
	ctx.lr = 0x826A3C98;
	sub_82466E20(ctx, base);
	// 826A3C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A3CA8 size=24
    let mut pc: u32 = 0x826A3CA8;
    'dispatch: loop {
        match pc {
            0x826A3CA8 => {
    //   block [0x826A3CA8..0x826A3CC0)
	// 826A3CA8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3CAC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3CB0: 394A2188  addi r10, r10, 0x2188
	ctx.r[10].s64 = ctx.r[10].s64 + 8584;
	// 826A3CB4: 816B9A6C  lwz r11, -0x6594(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26004 as u32) ) } as u64;
	// 826A3CB8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826A3CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3CC0 size=116
    let mut pc: u32 = 0x826A3CC0;
    'dispatch: loop {
        match pc {
            0x826A3CC0 => {
    //   block [0x826A3CC0..0x826A3D34)
	// 826A3CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3CCC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3CD0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A3CD4: 390B2188  addi r8, r11, 0x2188
	ctx.r[8].s64 = ctx.r[11].s64 + 8584;
	// 826A3CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3CDC: 392ABDF0  addi r9, r10, -0x4210
	ctx.r[9].s64 = ctx.r[10].s64 + -16912;
	// 826A3CE0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3CE4: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826A3CE8: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3CEC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3CF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3CFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3D04: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A3D08: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 826A3D0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A3D10: 386B7568  addi r3, r11, 0x7568
	ctx.r[3].s64 = ctx.r[11].s64 + 30056;
	// 826A3D14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A3D18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3D20: 4BDC3101  bl 0x82466e20
	ctx.lr = 0x826A3D24;
	sub_82466E20(ctx, base);
	// 826A3D24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3D38 size=116
    let mut pc: u32 = 0x826A3D38;
    'dispatch: loop {
        match pc {
            0x826A3D38 => {
    //   block [0x826A3D38..0x826A3DAC)
	// 826A3D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3D44: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3D48: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826A3D4C: 390A9CF8  addi r8, r10, -0x6308
	ctx.r[8].s64 = ctx.r[10].s64 + -25352;
	// 826A3D50: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3D54: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3D58: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3D5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3D60: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A3D64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3D6C: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 826A3D70: 396BBE04  addi r11, r11, -0x41fc
	ctx.r[11].s64 = ctx.r[11].s64 + -16892;
	// 826A3D74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3D78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3D7C: 386A7598  addi r3, r10, 0x7598
	ctx.r[3].s64 = ctx.r[10].s64 + 30104;
	// 826A3D80: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A3D84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3D88: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3D8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3D94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3D98: 4BDC3089  bl 0x82466e20
	ctx.lr = 0x826A3D9C;
	sub_82466E20(ctx, base);
	// 826A3D9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3DB0 size=112
    let mut pc: u32 = 0x826A3DB0;
    'dispatch: loop {
        match pc {
            0x826A3DB0 => {
    //   block [0x826A3DB0..0x826A3E20)
	// 826A3DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3DB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3DBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3DC0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3DC4: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3DC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3DCC: 390B9DB8  addi r8, r11, -0x6248
	ctx.r[8].s64 = ctx.r[11].s64 + -25160;
	// 826A3DD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A3DD4: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 826A3DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3DDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3DE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3DE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3DE8: 386A75C8  addi r3, r10, 0x75c8
	ctx.r[3].s64 = ctx.r[10].s64 + 30152;
	// 826A3DEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3DF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3E0C: 4BDC3015  bl 0x82466e20
	ctx.lr = 0x826A3E10;
	sub_82466E20(ctx, base);
	// 826A3E10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3E20 size=112
    let mut pc: u32 = 0x826A3E20;
    'dispatch: loop {
        match pc {
            0x826A3E20 => {
    //   block [0x826A3E20..0x826A3E90)
	// 826A3E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3E2C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3E30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A3E34: 38EA9DD0  addi r7, r10, -0x6230
	ctx.r[7].s64 = ctx.r[10].s64 + -25136;
	// 826A3E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3E3C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3E40: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826A3E44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3E48: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A3E4C: 396BBE28  addi r11, r11, -0x41d8
	ctx.r[11].s64 = ctx.r[11].s64 + -16856;
	// 826A3E50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A3E54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3E58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3E5C: 386A75F8  addi r3, r10, 0x75f8
	ctx.r[3].s64 = ctx.r[10].s64 + 30200;
	// 826A3E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3E64: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A3E68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3E6C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3E70: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3E74: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3E78: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A3E7C: 4BDC2FA5  bl 0x82466e20
	ctx.lr = 0x826A3E80;
	sub_82466E20(ctx, base);
	// 826A3E80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3E90 size=112
    let mut pc: u32 = 0x826A3E90;
    'dispatch: loop {
        match pc {
            0x826A3E90 => {
    //   block [0x826A3E90..0x826A3F00)
	// 826A3E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3E9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3EA0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3EA4: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3EA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3EAC: 390B9E00  addi r8, r11, -0x6200
	ctx.r[8].s64 = ctx.r[11].s64 + -25088;
	// 826A3EB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A3EB4: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826A3EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3EBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3EC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3EC8: 386A7628  addi r3, r10, 0x7628
	ctx.r[3].s64 = ctx.r[10].s64 + 30248;
	// 826A3ECC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3ED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3ED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3EE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3EEC: 4BDC2F35  bl 0x82466e20
	ctx.lr = 0x826A3EF0;
	sub_82466E20(ctx, base);
	// 826A3EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3F00 size=116
    let mut pc: u32 = 0x826A3F00;
    'dispatch: loop {
        match pc {
            0x826A3F00 => {
    //   block [0x826A3F00..0x826A3F74)
	// 826A3F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3F0C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3F10: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826A3F14: 390A9E18  addi r8, r10, -0x61e8
	ctx.r[8].s64 = ctx.r[10].s64 + -25064;
	// 826A3F18: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3F1C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3F20: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3F24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3F28: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A3F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3F30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3F34: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 826A3F38: 396BBE34  addi r11, r11, -0x41cc
	ctx.r[11].s64 = ctx.r[11].s64 + -16844;
	// 826A3F3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3F40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3F44: 386A7658  addi r3, r10, 0x7658
	ctx.r[3].s64 = ctx.r[10].s64 + 30296;
	// 826A3F48: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A3F4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3F50: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3F60: 4BDC2EC1  bl 0x82466e20
	ctx.lr = 0x826A3F64;
	sub_82466E20(ctx, base);
	// 826A3F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3F78 size=112
    let mut pc: u32 = 0x826A3F78;
    'dispatch: loop {
        match pc {
            0x826A3F78 => {
    //   block [0x826A3F78..0x826A3FE8)
	// 826A3F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3F84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3F88: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3F8C: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3F94: 390B9EC0  addi r8, r11, -0x6140
	ctx.r[8].s64 = ctx.r[11].s64 + -24896;
	// 826A3F98: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 826A3F9C: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 826A3FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3FA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3FB0: 386A7688  addi r3, r10, 0x7688
	ctx.r[3].s64 = ctx.r[10].s64 + 30344;
	// 826A3FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3FD4: 4BDC2E4D  bl 0x82466e20
	ctx.lr = 0x826A3FD8;
	sub_82466E20(ctx, base);
	// 826A3FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A3FE8 size=24
    let mut pc: u32 = 0x826A3FE8;
    'dispatch: loop {
        match pc {
            0x826A3FE8 => {
    //   block [0x826A3FE8..0x826A4000)
	// 826A3FE8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3FEC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3FF0: 394A2290  addi r10, r10, 0x2290
	ctx.r[10].s64 = ctx.r[10].s64 + 8848;
	// 826A3FF4: 816B9FF8  lwz r11, -0x6008(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24584 as u32) ) } as u64;
	// 826A3FF8: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826A3FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4000 size=116
    let mut pc: u32 = 0x826A4000;
    'dispatch: loop {
        match pc {
            0x826A4000 => {
    //   block [0x826A4000..0x826A4074)
	// 826A4000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A4004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A400C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A4010: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4014: 392BBE64  addi r9, r11, -0x419c
	ctx.r[9].s64 = ctx.r[11].s64 + -16796;
	// 826A4018: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A401C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A4020: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826A4024: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 826A4028: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A402C: 388A7B04  addi r4, r10, 0x7b04
	ctx.r[4].s64 = ctx.r[10].s64 + 31492;
	// 826A4030: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4034: 396B2290  addi r11, r11, 0x2290
	ctx.r[11].s64 = ctx.r[11].s64 + 8848;
	// 826A4038: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A403C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4040: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A4044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4048: 386A76B8  addi r3, r10, 0x76b8
	ctx.r[3].s64 = ctx.r[10].s64 + 30392;
	// 826A404C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A4050: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A4054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4058: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A405C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A4060: 4BDC2DC1  bl 0x82466e20
	ctx.lr = 0x826A4064;
	sub_82466E20(ctx, base);
	// 826A4064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A406C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4078 size=112
    let mut pc: u32 = 0x826A4078;
    'dispatch: loop {
        match pc {
            0x826A4078 => {
    //   block [0x826A4078..0x826A40E8)
	// 826A4078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A407C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4084: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4088: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A408C: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A4090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A4094: 390B9FFC  addi r8, r11, -0x6004
	ctx.r[8].s64 = ctx.r[11].s64 + -24580;
	// 826A4098: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A409C: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 826A40A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A40A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A40A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A40AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A40B0: 386A76E8  addi r3, r10, 0x76e8
	ctx.r[3].s64 = ctx.r[10].s64 + 30440;
	// 826A40B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A40B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A40BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A40C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A40C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A40C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A40CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A40D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A40D4: 4BDC2D4D  bl 0x82466e20
	ctx.lr = 0x826A40D8;
	sub_82466E20(ctx, base);
	// 826A40D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A40DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A40E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A40E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A40E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A40E8 size=24
    let mut pc: u32 = 0x826A40E8;
    'dispatch: loop {
        match pc {
            0x826A40E8 => {
    //   block [0x826A40E8..0x826A4100)
	// 826A40E8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A40EC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A40F0: 394A2368  addi r10, r10, 0x2368
	ctx.r[10].s64 = ctx.r[10].s64 + 9064;
	// 826A40F4: 816BA014  lwz r11, -0x5fec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24556 as u32) ) } as u64;
	// 826A40F8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826A40FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4100 size=116
    let mut pc: u32 = 0x826A4100;
    'dispatch: loop {
        match pc {
            0x826A4100 => {
    //   block [0x826A4100..0x826A4174)
	// 826A4100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A4104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A410C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A4110: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4114: 392BBF00  addi r9, r11, -0x4100
	ctx.r[9].s64 = ctx.r[11].s64 + -16640;
	// 826A4118: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A411C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A4120: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826A4124: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 826A4128: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A412C: 388AB34C  addi r4, r10, -0x4cb4
	ctx.r[4].s64 = ctx.r[10].s64 + -19636;
	// 826A4130: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4134: 396B2368  addi r11, r11, 0x2368
	ctx.r[11].s64 = ctx.r[11].s64 + 9064;
	// 826A4138: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A413C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4140: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A4144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4148: 386A7718  addi r3, r10, 0x7718
	ctx.r[3].s64 = ctx.r[10].s64 + 30488;
	// 826A414C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A4150: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A4154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4158: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A415C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A4160: 4BDC2CC1  bl 0x82466e20
	ctx.lr = 0x826A4164;
	sub_82466E20(ctx, base);
	// 826A4164: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A416C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4178 size=116
    let mut pc: u32 = 0x826A4178;
    'dispatch: loop {
        match pc {
            0x826A4178 => {
    //   block [0x826A4178..0x826A41EC)
	// 826A4178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A417C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4184: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4188: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A418C: 390BA01C  addi r8, r11, -0x5fe4
	ctx.r[8].s64 = ctx.r[11].s64 + -24548;
	// 826A4190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A4194: 392ABF5C  addi r9, r10, -0x40a4
	ctx.r[9].s64 = ctx.r[10].s64 + -16548;
	// 826A4198: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A419C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826A41A0: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A41A4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A41A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A41AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A41B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A41B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A41B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A41BC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A41C0: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 826A41C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A41C8: 386B7748  addi r3, r11, 0x7748
	ctx.r[3].s64 = ctx.r[11].s64 + 30536;
	// 826A41CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A41D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A41D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A41D8: 4BDC2C49  bl 0x82466e20
	ctx.lr = 0x826A41DC;
	sub_82466E20(ctx, base);
	// 826A41DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A41E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A41E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A41E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A41F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A41F0 size=100
    let mut pc: u32 = 0x826A41F0;
    'dispatch: loop {
        match pc {
            0x826A41F0 => {
    //   block [0x826A41F0..0x826A4254)
	// 826A41F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A41F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A41F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A41FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A4200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A4204: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A4208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A420C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A4210: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 826A4214: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A421C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A4224: 386A7778  addi r3, r10, 0x7778
	ctx.r[3].s64 = ctx.r[10].s64 + 30584;
	// 826A4228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A422C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A4230: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A4234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4238: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A423C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4240: 4BDC2BE1  bl 0x82466e20
	ctx.lr = 0x826A4244;
	sub_82466E20(ctx, base);
	// 826A4244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A424C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4258 size=112
    let mut pc: u32 = 0x826A4258;
    'dispatch: loop {
        match pc {
            0x826A4258 => {
    //   block [0x826A4258..0x826A42C8)
	// 826A4258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A425C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4264: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4268: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A426C: 38AA7778  addi r5, r10, 0x7778
	ctx.r[5].s64 = ctx.r[10].s64 + 30584;
	// 826A4270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A4274: 390BA04C  addi r8, r11, -0x5fb4
	ctx.r[8].s64 = ctx.r[11].s64 + -24500;
	// 826A4278: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A427C: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 826A4280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A4284: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A428C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4290: 386A77A8  addi r3, r10, 0x77a8
	ctx.r[3].s64 = ctx.r[10].s64 + 30632;
	// 826A4294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A4298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A429C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A42A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A42A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A42A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A42AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A42B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A42B4: 4BDC2B6D  bl 0x82466e20
	ctx.lr = 0x826A42B8;
	sub_82466E20(ctx, base);
	// 826A42B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A42BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A42C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A42C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A42C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A42C8 size=112
    let mut pc: u32 = 0x826A42C8;
    'dispatch: loop {
        match pc {
            0x826A42C8 => {
    //   block [0x826A42C8..0x826A4338)
	// 826A42C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A42CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A42D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A42D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A42D8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A42DC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A42E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A42E4: 390BA068  addi r8, r11, -0x5f98
	ctx.r[8].s64 = ctx.r[11].s64 + -24472;
	// 826A42E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A42EC: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 826A42F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A42F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A42F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A42FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4300: 386A77D8  addi r3, r10, 0x77d8
	ctx.r[3].s64 = ctx.r[10].s64 + 30680;
	// 826A4304: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A4308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A430C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A4310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A4314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A431C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A4324: 4BDC2AFD  bl 0x82466e20
	ctx.lr = 0x826A4328;
	sub_82466E20(ctx, base);
	// 826A4328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A432C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A4330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4338 size=112
    let mut pc: u32 = 0x826A4338;
    'dispatch: loop {
        match pc {
            0x826A4338 => {
    //   block [0x826A4338..0x826A43A8)
	// 826A4338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A433C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4344: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A4348: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A434C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A4350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A4354: 390BA0B0  addi r8, r11, -0x5f50
	ctx.r[8].s64 = ctx.r[11].s64 + -24400;
	// 826A4358: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826A435C: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 826A4360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A4364: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A436C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4370: 386A7808  addi r3, r10, 0x7808
	ctx.r[3].s64 = ctx.r[10].s64 + 30728;
	// 826A4374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A4378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A437C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A4380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A4384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A438C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A4394: 4BDC2A8D  bl 0x82466e20
	ctx.lr = 0x826A4398;
	sub_82466E20(ctx, base);
	// 826A4398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A439C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A43A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A43A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A43A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A43A8 size=116
    let mut pc: u32 = 0x826A43A8;
    'dispatch: loop {
        match pc {
            0x826A43A8 => {
    //   block [0x826A43A8..0x826A441C)
	// 826A43A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A43AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A43B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A43B4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A43B8: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826A43BC: 390AA158  addi r8, r10, -0x5ea8
	ctx.r[8].s64 = ctx.r[10].s64 + -24232;
	// 826A43C0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A43C4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A43C8: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A43CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A43D0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A43D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A43D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A43DC: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 826A43E0: 396BBF70  addi r11, r11, -0x4090
	ctx.r[11].s64 = ctx.r[11].s64 + -16528;
	// 826A43E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A43E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A43EC: 386A7838  addi r3, r10, 0x7838
	ctx.r[3].s64 = ctx.r[10].s64 + 30776;
	// 826A43F0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A43F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A43F8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A43FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A4404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4408: 4BDC2A19  bl 0x82466e20
	ctx.lr = 0x826A440C;
	sub_82466E20(ctx, base);
	// 826A440C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A4414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4420 size=112
    let mut pc: u32 = 0x826A4420;
    'dispatch: loop {
        match pc {
            0x826A4420 => {
    //   block [0x826A4420..0x826A4490)
	// 826A4420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A4424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A442C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A4430: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4434: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A4438: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A443C: 390BA260  addi r8, r11, -0x5da0
	ctx.r[8].s64 = ctx.r[11].s64 + -23968;
	// 826A4440: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A4444: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 826A4448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A444C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A4454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4458: 386A7868  addi r3, r10, 0x7868
	ctx.r[3].s64 = ctx.r[10].s64 + 30824;
	// 826A445C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A4460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A4464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A4468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A446C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A4474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A447C: 4BDC29A5  bl 0x82466e20
	ctx.lr = 0x826A4480;
	sub_82466E20(ctx, base);
	// 826A4480: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A4488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A448C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4490 size=100
    let mut pc: u32 = 0x826A4490;
    'dispatch: loop {
        match pc {
            0x826A4490 => {
    //   block [0x826A4490..0x826A44F4)
	// 826A4490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A4494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A449C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A44A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A44A4: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A44A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A44AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A44B0: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 826A44B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A44B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A44BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A44C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A44C4: 386A7898  addi r3, r10, 0x7898
	ctx.r[3].s64 = ctx.r[10].s64 + 30872;
	// 826A44C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A44CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A44D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A44D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A44D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A44DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A44E0: 4BDC2941  bl 0x82466e20
	ctx.lr = 0x826A44E4;
	sub_82466E20(ctx, base);
	// 826A44E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A44E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A44EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A44F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A44F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A44F8 size=108
    let mut pc: u32 = 0x826A44F8;
    'dispatch: loop {
        match pc {
            0x826A44F8 => {
    //   block [0x826A44F8..0x826A4564)
	// 826A44F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A44FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4504: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4508: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A450C: 38EBA290  addi r7, r11, -0x5d70
	ctx.r[7].s64 = ctx.r[11].s64 + -23920;
	// 826A4510: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A4514: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 826A4518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A451C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4520: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A4524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A4528: 386A78C8  addi r3, r10, 0x78c8
	ctx.r[3].s64 = ctx.r[10].s64 + 30920;
	// 826A452C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A4530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A4534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A453C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A4544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A454C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A4550: 4BDC28D1  bl 0x82466e20
	ctx.lr = 0x826A4554;
	sub_82466E20(ctx, base);
	// 826A4554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A455C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4568 size=112
    let mut pc: u32 = 0x826A4568;
    'dispatch: loop {
        match pc {
            0x826A4568 => {
    //   block [0x826A4568..0x826A45D8)
	// 826A4568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A456C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4574: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4578: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A457C: 38AA7898  addi r5, r10, 0x7898
	ctx.r[5].s64 = ctx.r[10].s64 + 30872;
	// 826A4580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A4584: 390BA2C0  addi r8, r11, -0x5d40
	ctx.r[8].s64 = ctx.r[11].s64 + -23872;
	// 826A4588: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A458C: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 826A4590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A4594: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A459C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A45A0: 386A78F8  addi r3, r10, 0x78f8
	ctx.r[3].s64 = ctx.r[10].s64 + 30968;
	// 826A45A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A45A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A45AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A45B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A45B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A45B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A45BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A45C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A45C4: 4BDC285D  bl 0x82466e20
	ctx.lr = 0x826A45C8;
	sub_82466E20(ctx, base);
	// 826A45C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A45CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A45D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A45D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A45D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A45D8 size=108
    let mut pc: u32 = 0x826A45D8;
    'dispatch: loop {
        match pc {
            0x826A45D8 => {
    //   block [0x826A45D8..0x826A4644)
	// 826A45D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A45DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A45E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A45E4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A45E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A45EC: 38EBA2F0  addi r7, r11, -0x5d10
	ctx.r[7].s64 = ctx.r[11].s64 + -23824;
	// 826A45F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A45F4: 388AA93C  addi r4, r10, -0x56c4
	ctx.r[4].s64 = ctx.r[10].s64 + -22212;
	// 826A45F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A45FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4600: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A4604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A4608: 386A7928  addi r3, r10, 0x7928
	ctx.r[3].s64 = ctx.r[10].s64 + 31016;
	// 826A460C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A4610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A4614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A461C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A4624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A462C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A4630: 4BDC27F1  bl 0x82466e20
	ctx.lr = 0x826A4634;
	sub_82466E20(ctx, base);
	// 826A4634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A463C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4648 size=116
    let mut pc: u32 = 0x826A4648;
    'dispatch: loop {
        match pc {
            0x826A4648 => {
    //   block [0x826A4648..0x826A46BC)
	// 826A4648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A464C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4654: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A4658: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826A465C: 390AA320  addi r8, r10, -0x5ce0
	ctx.r[8].s64 = ctx.r[10].s64 + -23776;
	// 826A4660: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4664: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A4668: 38AA7898  addi r5, r10, 0x7898
	ctx.r[5].s64 = ctx.r[10].s64 + 30872;
	// 826A466C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A4670: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A4674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A4678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A467C: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 826A4680: 396BBFA4  addi r11, r11, -0x405c
	ctx.r[11].s64 = ctx.r[11].s64 + -16476;
	// 826A4684: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4688: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A468C: 386A7958  addi r3, r10, 0x7958
	ctx.r[3].s64 = ctx.r[10].s64 + 31064;
	// 826A4690: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A4694: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A4698: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A469C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A46A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A46A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A46A8: 4BDC2779  bl 0x82466e20
	ctx.lr = 0x826A46AC;
	sub_82466E20(ctx, base);
	// 826A46AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A46B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A46B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A46B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A46C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A46C0 size=108
    let mut pc: u32 = 0x826A46C0;
    'dispatch: loop {
        match pc {
            0x826A46C0 => {
    //   block [0x826A46C0..0x826A472C)
	// 826A46C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A46C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A46C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A46CC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A46D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A46D4: 38EBA368  addi r7, r11, -0x5c98
	ctx.r[7].s64 = ctx.r[11].s64 + -23704;
	// 826A46D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A46DC: 388AA960  addi r4, r10, -0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22176;
	// 826A46E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A46E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A46E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A46EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A46F0: 386A7988  addi r3, r10, 0x7988
	ctx.r[3].s64 = ctx.r[10].s64 + 31112;
	// 826A46F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A46F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A46FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A4704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A470C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A4714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A4718: 4BDC2709  bl 0x82466e20
	ctx.lr = 0x826A471C;
	sub_82466E20(ctx, base);
	// 826A471C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A4724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4730 size=116
    let mut pc: u32 = 0x826A4730;
    'dispatch: loop {
        match pc {
            0x826A4730 => {
    //   block [0x826A4730..0x826A47A4)
	// 826A4730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A4734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A473C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A4740: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826A4744: 390AA398  addi r8, r10, -0x5c68
	ctx.r[8].s64 = ctx.r[10].s64 + -23656;
	// 826A4748: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A474C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A4750: 38AA7898  addi r5, r10, 0x7898
	ctx.r[5].s64 = ctx.r[10].s64 + 30872;
	// 826A4754: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A4758: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A475C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A4760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A4764: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 826A4768: 396BBFB4  addi r11, r11, -0x404c
	ctx.r[11].s64 = ctx.r[11].s64 + -16460;
	// 826A476C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4770: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4774: 386A79B8  addi r3, r10, 0x79b8
	ctx.r[3].s64 = ctx.r[10].s64 + 31160;
	// 826A4778: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A477C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A4780: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A4784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A478C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4790: 4BDC2691  bl 0x82466e20
	ctx.lr = 0x826A4794;
	sub_82466E20(ctx, base);
	// 826A4794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A479C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A47A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A47A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A47A8 size=108
    let mut pc: u32 = 0x826A47A8;
    'dispatch: loop {
        match pc {
            0x826A47A8 => {
    //   block [0x826A47A8..0x826A4814)
	// 826A47A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A47AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A47B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A47B4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A47B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A47BC: 38EBA3E0  addi r7, r11, -0x5c20
	ctx.r[7].s64 = ctx.r[11].s64 + -23584;
	// 826A47C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A47C4: 388AA984  addi r4, r10, -0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + -22140;
	// 826A47C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A47CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A47D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A47D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A47D8: 386A79E8  addi r3, r10, 0x79e8
	ctx.r[3].s64 = ctx.r[10].s64 + 31208;
	// 826A47DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A47E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A47E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A47E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A47EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A47F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A47F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A47F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A47FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A4800: 4BDC2621  bl 0x82466e20
	ctx.lr = 0x826A4804;
	sub_82466E20(ctx, base);
	// 826A4804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A480C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4818 size=116
    let mut pc: u32 = 0x826A4818;
    'dispatch: loop {
        match pc {
            0x826A4818 => {
    //   block [0x826A4818..0x826A488C)
	// 826A4818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A481C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4824: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A4828: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826A482C: 390AA410  addi r8, r10, -0x5bf0
	ctx.r[8].s64 = ctx.r[10].s64 + -23536;
	// 826A4830: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4834: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A4838: 38AA7898  addi r5, r10, 0x7898
	ctx.r[5].s64 = ctx.r[10].s64 + 30872;
	// 826A483C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A4840: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A4844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A4848: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A484C: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 826A4850: 396BBFC4  addi r11, r11, -0x403c
	ctx.r[11].s64 = ctx.r[11].s64 + -16444;
	// 826A4854: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4858: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A485C: 386A7A18  addi r3, r10, 0x7a18
	ctx.r[3].s64 = ctx.r[10].s64 + 31256;
	// 826A4860: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A4864: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A4868: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A486C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A4874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4878: 4BDC25A9  bl 0x82466e20
	ctx.lr = 0x826A487C;
	sub_82466E20(ctx, base);
	// 826A487C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A4884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4890 size=108
    let mut pc: u32 = 0x826A4890;
    'dispatch: loop {
        match pc {
            0x826A4890 => {
    //   block [0x826A4890..0x826A48FC)
	// 826A4890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A4894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A489C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A48A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A48A4: 38EBA458  addi r7, r11, -0x5ba8
	ctx.r[7].s64 = ctx.r[11].s64 + -23464;
	// 826A48A8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A48AC: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 826A48B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A48B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A48B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A48BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A48C0: 386A7A48  addi r3, r10, 0x7a48
	ctx.r[3].s64 = ctx.r[10].s64 + 31304;
	// 826A48C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A48C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A48CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A48D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A48D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A48D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A48DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A48E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A48E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A48E8: 4BDC2539  bl 0x82466e20
	ctx.lr = 0x826A48EC;
	sub_82466E20(ctx, base);
	// 826A48EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A48F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A48F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A48F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A4900 size=24
    let mut pc: u32 = 0x826A4900;
    'dispatch: loop {
        match pc {
            0x826A4900 => {
    //   block [0x826A4900..0x826A4918)
	// 826A4900: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4904: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A4908: 394A23F8  addi r10, r10, 0x23f8
	ctx.r[10].s64 = ctx.r[10].s64 + 9208;
	// 826A490C: 816BA4B8  lwz r11, -0x5b48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23368 as u32) ) } as u64;
	// 826A4910: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 826A4914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4918 size=112
    let mut pc: u32 = 0x826A4918;
    'dispatch: loop {
        match pc {
            0x826A4918 => {
    //   block [0x826A4918..0x826A4988)
	// 826A4918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A491C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4924: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A4928: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A492C: 392AC090  addi r9, r10, -0x3f70
	ctx.r[9].s64 = ctx.r[10].s64 + -16240;
	// 826A4930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A4934: 390B23F8  addi r8, r11, 0x23f8
	ctx.r[8].s64 = ctx.r[11].s64 + 9208;
	// 826A4938: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826A493C: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 826A4940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A4944: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A494C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4950: 386A7A78  addi r3, r10, 0x7a78
	ctx.r[3].s64 = ctx.r[10].s64 + 31352;
	// 826A4954: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A4958: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826A495C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A4964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A496C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A4970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A4974: 4BDC24AD  bl 0x82466e20
	ctx.lr = 0x826A4978;
	sub_82466E20(ctx, base);
	// 826A4978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A497C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A4980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4988 size=108
    let mut pc: u32 = 0x826A4988;
    'dispatch: loop {
        match pc {
            0x826A4988 => {
    //   block [0x826A4988..0x826A49F4)
	// 826A4988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A498C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4994: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4998: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A499C: 38EBA4C0  addi r7, r11, -0x5b40
	ctx.r[7].s64 = ctx.r[11].s64 + -23360;
	// 826A49A0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826A49A4: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 826A49A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A49AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A49B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A49B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A49B8: 386A7AA8  addi r3, r10, 0x7aa8
	ctx.r[3].s64 = ctx.r[10].s64 + 31400;
	// 826A49BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A49C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A49C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A49C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A49CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A49D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A49D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A49D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A49DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A49E0: 4BDC2441  bl 0x82466e20
	ctx.lr = 0x826A49E4;
	sub_82466E20(ctx, base);
	// 826A49E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A49E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A49EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A49F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A49F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A49F8 size=108
    let mut pc: u32 = 0x826A49F8;
    'dispatch: loop {
        match pc {
            0x826A49F8 => {
    //   block [0x826A49F8..0x826A4A64)
	// 826A49F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A49FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4A04: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4A08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A4A0C: 38EBA538  addi r7, r11, -0x5ac8
	ctx.r[7].s64 = ctx.r[11].s64 + -23240;
	// 826A4A10: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A4A14: 388AA9B8  addi r4, r10, -0x5648
	ctx.r[4].s64 = ctx.r[10].s64 + -22088;
	// 826A4A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A4A1C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4A20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A4A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A4A28: 386A7AD8  addi r3, r10, 0x7ad8
	ctx.r[3].s64 = ctx.r[10].s64 + 31448;
	// 826A4A2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A4A30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A4A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4A38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A4A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4A40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A4A44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4A48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A4A4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A4A50: 4BDC23D1  bl 0x82466e20
	ctx.lr = 0x826A4A54;
	sub_82466E20(ctx, base);
	// 826A4A54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A4A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4A68 size=108
    let mut pc: u32 = 0x826A4A68;
    'dispatch: loop {
        match pc {
            0x826A4A68 => {
    //   block [0x826A4A68..0x826A4AD4)
	// 826A4A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A4A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4A74: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4A78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A4A7C: 38EBA598  addi r7, r11, -0x5a68
	ctx.r[7].s64 = ctx.r[11].s64 + -23144;
	// 826A4A80: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826A4A84: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 826A4A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A4A8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4A90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A4A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A4A98: 386A7B08  addi r3, r10, 0x7b08
	ctx.r[3].s64 = ctx.r[10].s64 + 31496;
	// 826A4A9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A4AA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A4AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4AA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A4AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4AB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A4AB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4AB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A4ABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A4AC0: 4BDC2361  bl 0x82466e20
	ctx.lr = 0x826A4AC4;
	sub_82466E20(ctx, base);
	// 826A4AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A4ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A4AD8 size=24
    let mut pc: u32 = 0x826A4AD8;
    'dispatch: loop {
        match pc {
            0x826A4AD8 => {
    //   block [0x826A4AD8..0x826A4AF0)
	// 826A4AD8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4ADC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A4AE0: 394A2500  addi r10, r10, 0x2500
	ctx.r[10].s64 = ctx.r[10].s64 + 9472;
	// 826A4AE4: 816BA064  lwz r11, -0x5f9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24476 as u32) ) } as u64;
	// 826A4AE8: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 826A4AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4AF0 size=116
    let mut pc: u32 = 0x826A4AF0;
    'dispatch: loop {
        match pc {
            0x826A4AF0 => {
    //   block [0x826A4AF0..0x826A4B64)
	// 826A4AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A4AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4AFC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A4B00: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4B04: 392BBFF4  addi r9, r11, -0x400c
	ctx.r[9].s64 = ctx.r[11].s64 + -16396;
	// 826A4B08: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A4B0C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A4B10: 38E900C4  addi r7, r9, 0xc4
	ctx.r[7].s64 = ctx.r[9].s64 + 196;
	// 826A4B14: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 826A4B18: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4B1C: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 826A4B20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4B24: 396B2500  addi r11, r11, 0x2500
	ctx.r[11].s64 = ctx.r[11].s64 + 9472;
	// 826A4B28: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A4B2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4B30: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A4B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4B38: 386A7B38  addi r3, r10, 0x7b38
	ctx.r[3].s64 = ctx.r[10].s64 + 31544;
	// 826A4B3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A4B40: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A4B44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4B48: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A4B4C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A4B50: 4BDC22D1  bl 0x82466e20
	ctx.lr = 0x826A4B54;
	sub_82466E20(ctx, base);
	// 826A4B54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A4B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A4B68 size=36
    let mut pc: u32 = 0x826A4B68;
    'dispatch: loop {
        match pc {
            0x826A4B68 => {
    //   block [0x826A4B68..0x826A4B8C)
	// 826A4B68: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4B6C: 814BA640  lwz r10, -0x59c0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22976 as u32) ) } as u64;
	// 826A4B70: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4B74: 396B27D0  addi r11, r11, 0x27d0
	ctx.r[11].s64 = ctx.r[11].s64 + 10192;
	// 826A4B78: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826A4B7C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A4B80: 814AA644  lwz r10, -0x59bc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-22972 as u32) ) } as u64;
	// 826A4B84: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826A4B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4B90 size=116
    let mut pc: u32 = 0x826A4B90;
    'dispatch: loop {
        match pc {
            0x826A4B90 => {
    //   block [0x826A4B90..0x826A4C04)
	// 826A4B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A4B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4B9C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4BA0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A4BA4: 390B27D0  addi r8, r11, 0x27d0
	ctx.r[8].s64 = ctx.r[11].s64 + 10192;
	// 826A4BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A4BAC: 392AC178  addi r9, r10, -0x3e88
	ctx.r[9].s64 = ctx.r[10].s64 + -16008;
	// 826A4BB0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4BB4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826A4BB8: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A4BBC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A4BC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A4BC4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A4BC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A4BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4BD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A4BD4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A4BD8: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 826A4BDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A4BE0: 386B7B68  addi r3, r11, 0x7b68
	ctx.r[3].s64 = ctx.r[11].s64 + 31592;
	// 826A4BE4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826A4BE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4BF0: 4BDC2231  bl 0x82466e20
	ctx.lr = 0x826A4BF4;
	sub_82466E20(ctx, base);
	// 826A4BF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4BF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A4BFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A4C08 size=24
    let mut pc: u32 = 0x826A4C08;
    'dispatch: loop {
        match pc {
            0x826A4C08 => {
    //   block [0x826A4C08..0x826A4C20)
	// 826A4C08: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4C0C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A4C10: 394A2800  addi r10, r10, 0x2800
	ctx.r[10].s64 = ctx.r[10].s64 + 10240;
	// 826A4C14: 816BA64C  lwz r11, -0x59b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22964 as u32) ) } as u64;
	// 826A4C18: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826A4C1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4C20 size=116
    let mut pc: u32 = 0x826A4C20;
    'dispatch: loop {
        match pc {
            0x826A4C20 => {
    //   block [0x826A4C20..0x826A4C94)
	// 826A4C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A4C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4C28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4C2C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4C30: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A4C34: 390B2800  addi r8, r11, 0x2800
	ctx.r[8].s64 = ctx.r[11].s64 + 10240;
	// 826A4C38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A4C3C: 392AC1D0  addi r9, r10, -0x3e30
	ctx.r[9].s64 = ctx.r[10].s64 + -15920;
	// 826A4C40: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4C44: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 826A4C48: 38AA7B68  addi r5, r10, 0x7b68
	ctx.r[5].s64 = ctx.r[10].s64 + 31592;
	// 826A4C4C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A4C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A4C54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A4C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A4C5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A4C64: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A4C68: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 826A4C6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A4C70: 386B7B98  addi r3, r11, 0x7b98
	ctx.r[3].s64 = ctx.r[11].s64 + 31640;
	// 826A4C74: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826A4C78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4C7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4C80: 4BDC21A1  bl 0x82466e20
	ctx.lr = 0x826A4C84;
	sub_82466E20(ctx, base);
	// 826A4C84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A4C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4C98 size=116
    let mut pc: u32 = 0x826A4C98;
    'dispatch: loop {
        match pc {
            0x826A4C98 => {
    //   block [0x826A4C98..0x826A4D0C)
	// 826A4C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A4C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4CA4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4CA8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A4CAC: 390BA658  addi r8, r11, -0x59a8
	ctx.r[8].s64 = ctx.r[11].s64 + -22952;
	// 826A4CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A4CB4: 392AC218  addi r9, r10, -0x3de8
	ctx.r[9].s64 = ctx.r[10].s64 + -15848;
	// 826A4CB8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4CBC: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826A4CC0: 38AA7B68  addi r5, r10, 0x7b68
	ctx.r[5].s64 = ctx.r[10].s64 + 31592;
	// 826A4CC4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A4CC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A4CCC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A4CD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A4CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4CD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A4CDC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A4CE0: 388AAAE8  addi r4, r10, -0x5518
	ctx.r[4].s64 = ctx.r[10].s64 + -21784;
	// 826A4CE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A4CE8: 386B7BC8  addi r3, r11, 0x7bc8
	ctx.r[3].s64 = ctx.r[11].s64 + 31688;
	// 826A4CEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A4CF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4CF8: 4BDC2129  bl 0x82466e20
	ctx.lr = 0x826A4CFC;
	sub_82466E20(ctx, base);
	// 826A4CFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4D00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A4D04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4D10 size=112
    let mut pc: u32 = 0x826A4D10;
    'dispatch: loop {
        match pc {
            0x826A4D10 => {
    //   block [0x826A4D10..0x826A4D80)
	// 826A4D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A4D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4D1C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A4D20: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826A4D24: 38EAA760  addi r7, r10, -0x58a0
	ctx.r[7].s64 = ctx.r[10].s64 + -22688;
	// 826A4D28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A4D2C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A4D30: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 826A4D34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A4D38: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A4D3C: 396BC22C  addi r11, r11, -0x3dd4
	ctx.r[11].s64 = ctx.r[11].s64 + -15828;
	// 826A4D40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A4D44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4D48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4D4C: 386A7BF8  addi r3, r10, 0x7bf8
	ctx.r[3].s64 = ctx.r[10].s64 + 31736;
	// 826A4D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A4D54: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A4D58: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4D5C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A4D60: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4D64: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A4D68: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A4D6C: 4BDC20B5  bl 0x82466e20
	ctx.lr = 0x826A4D70;
	sub_82466E20(ctx, base);
	// 826A4D70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A4D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4D80 size=112
    let mut pc: u32 = 0x826A4D80;
    'dispatch: loop {
        match pc {
            0x826A4D80 => {
    //   block [0x826A4D80..0x826A4DF0)
	// 826A4D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A4D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4D8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A4D90: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4D94: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A4D98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A4D9C: 390BA7F0  addi r8, r11, -0x5810
	ctx.r[8].s64 = ctx.r[11].s64 + -22544;
	// 826A4DA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A4DA4: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 826A4DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A4DAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4DB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A4DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4DB8: 386A7C28  addi r3, r10, 0x7c28
	ctx.r[3].s64 = ctx.r[10].s64 + 31784;
	// 826A4DBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A4DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A4DC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A4DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A4DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A4DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A4DDC: 4BDC2045  bl 0x82466e20
	ctx.lr = 0x826A4DE0;
	sub_82466E20(ctx, base);
	// 826A4DE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A4DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A4DF0 size=24
    let mut pc: u32 = 0x826A4DF0;
    'dispatch: loop {
        match pc {
            0x826A4DF0 => {
    //   block [0x826A4DF0..0x826A4E08)
	// 826A4DF0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4DF4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A4DF8: 394A2938  addi r10, r10, 0x2938
	ctx.r[10].s64 = ctx.r[10].s64 + 10552;
	// 826A4DFC: 816BA654  lwz r11, -0x59ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22956 as u32) ) } as u64;
	// 826A4E00: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826A4E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4E08 size=112
    let mut pc: u32 = 0x826A4E08;
    'dispatch: loop {
        match pc {
            0x826A4E08 => {
    //   block [0x826A4E08..0x826A4E78)
	// 826A4E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A4E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4E14: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A4E18: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4E1C: 392AC278  addi r9, r10, -0x3d88
	ctx.r[9].s64 = ctx.r[10].s64 + -15752;
	// 826A4E20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A4E24: 390B2938  addi r8, r11, 0x2938
	ctx.r[8].s64 = ctx.r[11].s64 + 10552;
	// 826A4E28: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826A4E2C: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 826A4E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A4E34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4E38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A4E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4E40: 386A7C58  addi r3, r10, 0x7c58
	ctx.r[3].s64 = ctx.r[10].s64 + 31832;
	// 826A4E44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A4E48: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A4E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4E50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A4E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4E58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A4E5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A4E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A4E64: 4BDC1FBD  bl 0x82466e20
	ctx.lr = 0x826A4E68;
	sub_82466E20(ctx, base);
	// 826A4E68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A4E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4E78 size=108
    let mut pc: u32 = 0x826A4E78;
    'dispatch: loop {
        match pc {
            0x826A4E78 => {
    //   block [0x826A4E78..0x826A4EE4)
	// 826A4E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A4E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4E84: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4E88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A4E8C: 38EBA810  addi r7, r11, -0x57f0
	ctx.r[7].s64 = ctx.r[11].s64 + -22512;
	// 826A4E90: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A4E94: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 826A4E98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A4E9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4EA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A4EA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A4EA8: 386A7C88  addi r3, r10, 0x7c88
	ctx.r[3].s64 = ctx.r[10].s64 + 31880;
	// 826A4EAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A4EB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A4EB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4EB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A4EBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4EC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A4EC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4EC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A4ECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A4ED0: 4BDC1F51  bl 0x82466e20
	ctx.lr = 0x826A4ED4;
	sub_82466E20(ctx, base);
	// 826A4ED4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4ED8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A4EDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4EE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4EE8 size=108
    let mut pc: u32 = 0x826A4EE8;
    'dispatch: loop {
        match pc {
            0x826A4EE8 => {
    //   block [0x826A4EE8..0x826A4F54)
	// 826A4EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A4EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4EF4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4EF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A4EFC: 38EBA870  addi r7, r11, -0x5790
	ctx.r[7].s64 = ctx.r[11].s64 + -22416;
	// 826A4F00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A4F04: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 826A4F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A4F0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4F10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A4F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A4F18: 386A7CB8  addi r3, r10, 0x7cb8
	ctx.r[3].s64 = ctx.r[10].s64 + 31928;
	// 826A4F1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A4F20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A4F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4F28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A4F2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4F30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A4F34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4F38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A4F3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A4F40: 4BDC1EE1  bl 0x82466e20
	ctx.lr = 0x826A4F44;
	sub_82466E20(ctx, base);
	// 826A4F44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A4F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4F58 size=116
    let mut pc: u32 = 0x826A4F58;
    'dispatch: loop {
        match pc {
            0x826A4F58 => {
    //   block [0x826A4F58..0x826A4FCC)
	// 826A4F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A4F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4F64: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4F68: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A4F6C: 390BA8A0  addi r8, r11, -0x5760
	ctx.r[8].s64 = ctx.r[11].s64 + -22368;
	// 826A4F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A4F74: 392AC2A4  addi r9, r10, -0x3d5c
	ctx.r[9].s64 = ctx.r[10].s64 + -15708;
	// 826A4F78: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A4F7C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826A4F80: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A4F84: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A4F88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A4F8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A4F90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A4F94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A4F98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A4F9C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A4FA0: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 826A4FA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A4FA8: 386B7CE8  addi r3, r11, 0x7ce8
	ctx.r[3].s64 = ctx.r[11].s64 + 31976;
	// 826A4FAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A4FB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A4FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A4FB8: 4BDC1E69  bl 0x82466e20
	ctx.lr = 0x826A4FBC;
	sub_82466E20(ctx, base);
	// 826A4FBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A4FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A4FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A4FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A4FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A4FD0 size=108
    let mut pc: u32 = 0x826A4FD0;
    'dispatch: loop {
        match pc {
            0x826A4FD0 => {
    //   block [0x826A4FD0..0x826A503C)
	// 826A4FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A4FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A4FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A4FDC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A4FE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A4FE4: 38EBA8B8  addi r7, r11, -0x5748
	ctx.r[7].s64 = ctx.r[11].s64 + -22344;
	// 826A4FE8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A4FEC: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 826A4FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A4FF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A4FF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A4FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A5000: 386A7D18  addi r3, r10, 0x7d18
	ctx.r[3].s64 = ctx.r[10].s64 + 32024;
	// 826A5004: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A5008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A500C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A5014: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A501C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A5024: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A5028: 4BDC1DF9  bl 0x82466e20
	ctx.lr = 0x826A502C;
	sub_82466E20(ctx, base);
	// 826A502C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5040 size=112
    let mut pc: u32 = 0x826A5040;
    'dispatch: loop {
        match pc {
            0x826A5040 => {
    //   block [0x826A5040..0x826A50B0)
	// 826A5040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A504C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A5050: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5054: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A5058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A505C: 390BA8D0  addi r8, r11, -0x5730
	ctx.r[8].s64 = ctx.r[11].s64 + -22320;
	// 826A5060: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826A5064: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 826A5068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A506C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5070: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A5074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5078: 386A7D48  addi r3, r10, 0x7d48
	ctx.r[3].s64 = ctx.r[10].s64 + 32072;
	// 826A507C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A5080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A5084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A5088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A508C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A5094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A509C: 4BDC1D85  bl 0x82466e20
	ctx.lr = 0x826A50A0;
	sub_82466E20(ctx, base);
	// 826A50A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A50A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A50A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A50AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A50B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A50B0 size=108
    let mut pc: u32 = 0x826A50B0;
    'dispatch: loop {
        match pc {
            0x826A50B0 => {
    //   block [0x826A50B0..0x826A511C)
	// 826A50B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A50B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A50B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A50BC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A50C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A50C4: 38EBA9A8  addi r7, r11, -0x5658
	ctx.r[7].s64 = ctx.r[11].s64 + -22104;
	// 826A50C8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826A50CC: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 826A50D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A50D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A50D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A50DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A50E0: 386A7D78  addi r3, r10, 0x7d78
	ctx.r[3].s64 = ctx.r[10].s64 + 32120;
	// 826A50E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A50E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A50EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A50F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A50F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A50F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A50FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A5104: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A5108: 4BDC1D19  bl 0x82466e20
	ctx.lr = 0x826A510C;
	sub_82466E20(ctx, base);
	// 826A510C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5120 size=108
    let mut pc: u32 = 0x826A5120;
    'dispatch: loop {
        match pc {
            0x826A5120 => {
    //   block [0x826A5120..0x826A518C)
	// 826A5120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A512C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A5134: 38EBAA20  addi r7, r11, -0x55e0
	ctx.r[7].s64 = ctx.r[11].s64 + -21984;
	// 826A5138: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A513C: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 826A5140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A5144: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5148: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A514C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A5150: 386A7DA8  addi r3, r10, 0x7da8
	ctx.r[3].s64 = ctx.r[10].s64 + 32168;
	// 826A5154: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A5158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A515C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A5164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A516C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A5174: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A5178: 4BDC1CA9  bl 0x82466e20
	ctx.lr = 0x826A517C;
	sub_82466E20(ctx, base);
	// 826A517C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5190 size=116
    let mut pc: u32 = 0x826A5190;
    'dispatch: loop {
        match pc {
            0x826A5190 => {
    //   block [0x826A5190..0x826A5204)
	// 826A5190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A519C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A51A0: 38E00017  li r7, 0x17
	ctx.r[7].s64 = 23;
	// 826A51A4: 390AAA68  addi r8, r10, -0x5598
	ctx.r[8].s64 = ctx.r[10].s64 + -21912;
	// 826A51A8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A51AC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A51B0: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A51B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A51B8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A51BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A51C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A51C4: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 826A51C8: 396BC2B8  addi r11, r11, -0x3d48
	ctx.r[11].s64 = ctx.r[11].s64 + -15688;
	// 826A51CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A51D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A51D4: 386A7DD8  addi r3, r10, 0x7dd8
	ctx.r[3].s64 = ctx.r[10].s64 + 32216;
	// 826A51D8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A51DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A51E0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A51E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A51E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A51EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A51F0: 4BDC1C31  bl 0x82466e20
	ctx.lr = 0x826A51F4;
	sub_82466E20(ctx, base);
	// 826A51F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A51F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A51FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5208 size=112
    let mut pc: u32 = 0x826A5208;
    'dispatch: loop {
        match pc {
            0x826A5208 => {
    //   block [0x826A5208..0x826A5278)
	// 826A5208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A520C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A5214: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5218: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A521C: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A5220: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A5224: 390BAC90  addi r8, r11, -0x5370
	ctx.r[8].s64 = ctx.r[11].s64 + -21360;
	// 826A5228: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A522C: 388AB374  addi r4, r10, -0x4c8c
	ctx.r[4].s64 = ctx.r[10].s64 + -19596;
	// 826A5230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A5234: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5238: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A523C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5240: 386A7E08  addi r3, r10, 0x7e08
	ctx.r[3].s64 = ctx.r[10].s64 + 32264;
	// 826A5244: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A5248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A524C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A5250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A5254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A525C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A5264: 4BDC1BBD  bl 0x82466e20
	ctx.lr = 0x826A5268;
	sub_82466E20(ctx, base);
	// 826A5268: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A526C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A5278 size=76
    let mut pc: u32 = 0x826A5278;
    'dispatch: loop {
        match pc {
            0x826A5278 => {
    //   block [0x826A5278..0x826A52C4)
	// 826A5278: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A527C: 814BA80C  lwz r10, -0x57f4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22516 as u32) ) } as u64;
	// 826A5280: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5284: 396B2968  addi r11, r11, 0x2968
	ctx.r[11].s64 = ctx.r[11].s64 + 10600;
	// 826A5288: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 826A528C: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826A5290: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 826A5294: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826A5298: 914B0158  stw r10, 0x158(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 826A529C: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 826A52A0: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A52A4: 814AACA8  lwz r10, -0x5358(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21336 as u32) ) } as u64;
	// 826A52A8: 914B0188  stw r10, 0x188(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 826A52AC: 914B01A0  stw r10, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[10].u32 ) };
	// 826A52B0: 914B01B8  stw r10, 0x1b8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 826A52B4: 914B01D0  stw r10, 0x1d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 826A52B8: 914B01E8  stw r10, 0x1e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), ctx.r[10].u32 ) };
	// 826A52BC: 914B0200  stw r10, 0x200(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(512 as u32), ctx.r[10].u32 ) };
	// 826A52C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A52C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A52C8 size=108
    let mut pc: u32 = 0x826A52C8;
    'dispatch: loop {
        match pc {
            0x826A52C8 => {
    //   block [0x826A52C8..0x826A5334)
	// 826A52C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A52CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A52D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A52D4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A52D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A52DC: 38EB2968  addi r7, r11, 0x2968
	ctx.r[7].s64 = ctx.r[11].s64 + 10600;
	// 826A52E0: 3900001A  li r8, 0x1a
	ctx.r[8].s64 = 26;
	// 826A52E4: 388AABE8  addi r4, r10, -0x5418
	ctx.r[4].s64 = ctx.r[10].s64 + -21528;
	// 826A52E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A52EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A52F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A52F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A52F8: 386A7E38  addi r3, r10, 0x7e38
	ctx.r[3].s64 = ctx.r[10].s64 + 32312;
	// 826A52FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A5300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A5304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A530C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A5314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A531C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A5320: 4BDC1B01  bl 0x82466e20
	ctx.lr = 0x826A5324;
	sub_82466E20(ctx, base);
	// 826A5324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A532C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A5338 size=76
    let mut pc: u32 = 0x826A5338;
    'dispatch: loop {
        match pc {
            0x826A5338 => {
    //   block [0x826A5338..0x826A5384)
	// 826A5338: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A533C: 814BA80C  lwz r10, -0x57f4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22516 as u32) ) } as u64;
	// 826A5340: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5344: 396B2BD8  addi r11, r11, 0x2bd8
	ctx.r[11].s64 = ctx.r[11].s64 + 11224;
	// 826A5348: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 826A534C: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826A5350: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 826A5354: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826A5358: 914B0158  stw r10, 0x158(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 826A535C: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 826A5360: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A5364: 814AACA8  lwz r10, -0x5358(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21336 as u32) ) } as u64;
	// 826A5368: 914B0188  stw r10, 0x188(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 826A536C: 914B01A0  stw r10, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[10].u32 ) };
	// 826A5370: 914B01B8  stw r10, 0x1b8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 826A5374: 914B01D0  stw r10, 0x1d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 826A5378: 914B01E8  stw r10, 0x1e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), ctx.r[10].u32 ) };
	// 826A537C: 914B0200  stw r10, 0x200(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(512 as u32), ctx.r[10].u32 ) };
	// 826A5380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5388 size=116
    let mut pc: u32 = 0x826A5388;
    'dispatch: loop {
        match pc {
            0x826A5388 => {
    //   block [0x826A5388..0x826A53FC)
	// 826A5388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A538C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A5394: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5398: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A539C: 390B2BD8  addi r8, r11, 0x2bd8
	ctx.r[8].s64 = ctx.r[11].s64 + 11224;
	// 826A53A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A53A4: 392AC354  addi r9, r10, -0x3cac
	ctx.r[9].s64 = ctx.r[10].s64 + -15532;
	// 826A53A8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A53AC: 38E00028  li r7, 0x28
	ctx.r[7].s64 = 40;
	// 826A53B0: 38AA70E8  addi r5, r10, 0x70e8
	ctx.r[5].s64 = ctx.r[10].s64 + 28904;
	// 826A53B4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A53B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A53BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A53C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A53C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A53C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A53CC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A53D0: 388AACDC  addi r4, r10, -0x5324
	ctx.r[4].s64 = ctx.r[10].s64 + -21284;
	// 826A53D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A53D8: 386B7E68  addi r3, r11, 0x7e68
	ctx.r[3].s64 = ctx.r[11].s64 + 32360;
	// 826A53DC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826A53E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A53E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A53E8: 4BDC1A39  bl 0x82466e20
	ctx.lr = 0x826A53EC;
	sub_82466E20(ctx, base);
	// 826A53EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A53F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A53F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A53F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5400 size=112
    let mut pc: u32 = 0x826A5400;
    'dispatch: loop {
        match pc {
            0x826A5400 => {
    //   block [0x826A5400..0x826A5470)
	// 826A5400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A540C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5410: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5414: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A5418: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A541C: 390BACB0  addi r8, r11, -0x5350
	ctx.r[8].s64 = ctx.r[11].s64 + -21328;
	// 826A5420: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A5424: 388AACF4  addi r4, r10, -0x530c
	ctx.r[4].s64 = ctx.r[10].s64 + -21260;
	// 826A5428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A542C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A5434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5438: 386A7E98  addi r3, r10, 0x7e98
	ctx.r[3].s64 = ctx.r[10].s64 + 32408;
	// 826A543C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A5440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A5444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A5448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A544C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A5454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A545C: 4BDC19C5  bl 0x82466e20
	ctx.lr = 0x826A5460;
	sub_82466E20(ctx, base);
	// 826A5460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A546C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5470 size=108
    let mut pc: u32 = 0x826A5470;
    'dispatch: loop {
        match pc {
            0x826A5470 => {
    //   block [0x826A5470..0x826A54DC)
	// 826A5470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A547C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5480: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A5484: 38EBACF8  addi r7, r11, -0x5308
	ctx.r[7].s64 = ctx.r[11].s64 + -21256;
	// 826A5488: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A548C: 388AAD10  addi r4, r10, -0x52f0
	ctx.r[4].s64 = ctx.r[10].s64 + -21232;
	// 826A5490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A5494: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A549C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A54A0: 386A7EC8  addi r3, r10, 0x7ec8
	ctx.r[3].s64 = ctx.r[10].s64 + 32456;
	// 826A54A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A54A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A54AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A54B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A54B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A54B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A54BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A54C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A54C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A54C8: 4BDC1959  bl 0x82466e20
	ctx.lr = 0x826A54CC;
	sub_82466E20(ctx, base);
	// 826A54CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A54D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A54D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A54D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A54E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A54E0 size=108
    let mut pc: u32 = 0x826A54E0;
    'dispatch: loop {
        match pc {
            0x826A54E0 => {
    //   block [0x826A54E0..0x826A554C)
	// 826A54E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A54E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A54E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A54EC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A54F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A54F4: 38EBAD40  addi r7, r11, -0x52c0
	ctx.r[7].s64 = ctx.r[11].s64 + -21184;
	// 826A54F8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A54FC: 388AAD38  addi r4, r10, -0x52c8
	ctx.r[4].s64 = ctx.r[10].s64 + -21192;
	// 826A5500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A5504: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5508: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A550C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A5510: 386A7EF8  addi r3, r10, 0x7ef8
	ctx.r[3].s64 = ctx.r[10].s64 + 32504;
	// 826A5514: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A5518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A551C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A5524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A552C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A5534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A5538: 4BDC18E9  bl 0x82466e20
	ctx.lr = 0x826A553C;
	sub_82466E20(ctx, base);
	// 826A553C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5550 size=116
    let mut pc: u32 = 0x826A5550;
    'dispatch: loop {
        match pc {
            0x826A5550 => {
    //   block [0x826A5550..0x826A55C4)
	// 826A5550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A555C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A5560: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826A5564: 390AAD88  addi r8, r10, -0x5278
	ctx.r[8].s64 = ctx.r[10].s64 + -21112;
	// 826A5568: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A556C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A5570: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A5574: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A5578: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A557C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A5580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A5584: 388AAD60  addi r4, r10, -0x52a0
	ctx.r[4].s64 = ctx.r[10].s64 + -21152;
	// 826A5588: 396BC37C  addi r11, r11, -0x3c84
	ctx.r[11].s64 = ctx.r[11].s64 + -15492;
	// 826A558C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5590: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5594: 386A7F28  addi r3, r10, 0x7f28
	ctx.r[3].s64 = ctx.r[10].s64 + 32552;
	// 826A5598: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A559C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A55A0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A55A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A55A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A55AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A55B0: 4BDC1871  bl 0x82466e20
	ctx.lr = 0x826A55B4;
	sub_82466E20(ctx, base);
	// 826A55B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A55B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A55BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A55C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A55C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A55C8 size=116
    let mut pc: u32 = 0x826A55C8;
    'dispatch: loop {
        match pc {
            0x826A55C8 => {
    //   block [0x826A55C8..0x826A563C)
	// 826A55C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A55CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A55D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A55D4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A55D8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826A55DC: 390AAE30  addi r8, r10, -0x51d0
	ctx.r[8].s64 = ctx.r[10].s64 + -20944;
	// 826A55E0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A55E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A55E8: 38AA7F28  addi r5, r10, 0x7f28
	ctx.r[5].s64 = ctx.r[10].s64 + 32552;
	// 826A55EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A55F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A55F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A55F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A55FC: 388AAD7C  addi r4, r10, -0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + -21124;
	// 826A5600: 396BC3A0  addi r11, r11, -0x3c60
	ctx.r[11].s64 = ctx.r[11].s64 + -15456;
	// 826A5604: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5608: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A560C: 386A7F58  addi r3, r10, 0x7f58
	ctx.r[3].s64 = ctx.r[10].s64 + 32600;
	// 826A5610: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A5614: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A5618: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A561C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A5624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5628: 4BDC17F9  bl 0x82466e20
	ctx.lr = 0x826A562C;
	sub_82466E20(ctx, base);
	// 826A562C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A5640 size=36
    let mut pc: u32 = 0x826A5640;
    'dispatch: loop {
        match pc {
            0x826A5640 => {
    //   block [0x826A5640..0x826A5664)
	// 826A5640: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5644: 814BACAC  lwz r10, -0x5354(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21332 as u32) ) } as u64;
	// 826A5648: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A564C: 396B2F98  addi r11, r11, 0x2f98
	ctx.r[11].s64 = ctx.r[11].s64 + 12184;
	// 826A5650: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826A5654: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A5658: 814AAE78  lwz r10, -0x5188(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20872 as u32) ) } as u64;
	// 826A565C: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826A5660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5668 size=116
    let mut pc: u32 = 0x826A5668;
    'dispatch: loop {
        match pc {
            0x826A5668 => {
    //   block [0x826A5668..0x826A56DC)
	// 826A5668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A566C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A5674: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5678: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A567C: 390B2F98  addi r8, r11, 0x2f98
	ctx.r[8].s64 = ctx.r[11].s64 + 12184;
	// 826A5680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A5684: 392AC3E0  addi r9, r10, -0x3c20
	ctx.r[9].s64 = ctx.r[10].s64 + -15392;
	// 826A5688: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A568C: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826A5690: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A5694: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A5698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A569C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A56A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A56A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A56A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A56AC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A56B0: 388AAE18  addi r4, r10, -0x51e8
	ctx.r[4].s64 = ctx.r[10].s64 + -20968;
	// 826A56B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A56B8: 386B7F88  addi r3, r11, 0x7f88
	ctx.r[3].s64 = ctx.r[11].s64 + 32648;
	// 826A56BC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826A56C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A56C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A56C8: 4BDC1759  bl 0x82466e20
	ctx.lr = 0x826A56CC;
	sub_82466E20(ctx, base);
	// 826A56CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A56D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A56D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A56D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A56E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A56E0 size=112
    let mut pc: u32 = 0x826A56E0;
    'dispatch: loop {
        match pc {
            0x826A56E0 => {
    //   block [0x826A56E0..0x826A5750)
	// 826A56E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A56E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A56E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A56EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A56F0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A56F4: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A56F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A56FC: 390BAE80  addi r8, r11, -0x5180
	ctx.r[8].s64 = ctx.r[11].s64 + -20864;
	// 826A5700: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826A5704: 388AAE30  addi r4, r10, -0x51d0
	ctx.r[4].s64 = ctx.r[10].s64 + -20944;
	// 826A5708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A570C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5710: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A5714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5718: 386A7FB8  addi r3, r10, 0x7fb8
	ctx.r[3].s64 = ctx.r[10].s64 + 32696;
	// 826A571C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A5720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A5724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A5728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A572C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A5734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A573C: 4BDC16E5  bl 0x82466e20
	ctx.lr = 0x826A5740;
	sub_82466E20(ctx, base);
	// 826A5740: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A574C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5750 size=108
    let mut pc: u32 = 0x826A5750;
    'dispatch: loop {
        match pc {
            0x826A5750 => {
    //   block [0x826A5750..0x826A57BC)
	// 826A5750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A575C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5760: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A5764: 38EBAF40  addi r7, r11, -0x50c0
	ctx.r[7].s64 = ctx.r[11].s64 + -20672;
	// 826A5768: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A576C: 388AAE50  addi r4, r10, -0x51b0
	ctx.r[4].s64 = ctx.r[10].s64 + -20912;
	// 826A5770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A5774: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5778: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A577C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A5780: 386A7FE8  addi r3, r10, 0x7fe8
	ctx.r[3].s64 = ctx.r[10].s64 + 32744;
	// 826A5784: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A5788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A578C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A5794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A579C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A57A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A57A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A57A8: 4BDC1679  bl 0x82466e20
	ctx.lr = 0x826A57AC;
	sub_82466E20(ctx, base);
	// 826A57AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A57B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A57B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A57B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A57C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A57C0 size=112
    let mut pc: u32 = 0x826A57C0;
    'dispatch: loop {
        match pc {
            0x826A57C0 => {
    //   block [0x826A57C0..0x826A5830)
	// 826A57C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A57C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A57C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A57CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A57D0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A57D4: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A57D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A57DC: 390BAF70  addi r8, r11, -0x5090
	ctx.r[8].s64 = ctx.r[11].s64 + -20624;
	// 826A57E0: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 826A57E4: 388AAE6C  addi r4, r10, -0x5194
	ctx.r[4].s64 = ctx.r[10].s64 + -20884;
	// 826A57E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A57EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A57F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A57F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A57F8: 386A8018  addi r3, r10, -0x7fe8
	ctx.r[3].s64 = ctx.r[10].s64 + -32744;
	// 826A57FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A5800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A5804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A5808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A580C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A5814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A581C: 4BDC1605  bl 0x82466e20
	ctx.lr = 0x826A5820;
	sub_82466E20(ctx, base);
	// 826A5820: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A582C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5830 size=112
    let mut pc: u32 = 0x826A5830;
    'dispatch: loop {
        match pc {
            0x826A5830 => {
    //   block [0x826A5830..0x826A58A0)
	// 826A5830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A583C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5840: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5844: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A5848: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A584C: 390BB078  addi r8, r11, -0x4f88
	ctx.r[8].s64 = ctx.r[11].s64 + -20360;
	// 826A5850: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 826A5854: 388AAE80  addi r4, r10, -0x5180
	ctx.r[4].s64 = ctx.r[10].s64 + -20864;
	// 826A5858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A585C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A5860: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A5864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5868: 386A8048  addi r3, r10, -0x7fb8
	ctx.r[3].s64 = ctx.r[10].s64 + -32696;
	// 826A586C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A5870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A5874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A5878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A587C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A5884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A588C: 4BDC1595  bl 0x82466e20
	ctx.lr = 0x826A5890;
	sub_82466E20(ctx, base);
	// 826A5890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A589C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A58A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A58A0 size=112
    let mut pc: u32 = 0x826A58A0;
    'dispatch: loop {
        match pc {
            0x826A58A0 => {
    //   block [0x826A58A0..0x826A5910)
	// 826A58A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A58A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A58A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A58AC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A58B0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826A58B4: 38EAB1B0  addi r7, r10, -0x4e50
	ctx.r[7].s64 = ctx.r[10].s64 + -20048;
	// 826A58B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A58BC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A58C0: 388A7AE8  addi r4, r10, 0x7ae8
	ctx.r[4].s64 = ctx.r[10].s64 + 31464;
	// 826A58C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A58C8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A58CC: 396BC42C  addi r11, r11, -0x3bd4
	ctx.r[11].s64 = ctx.r[11].s64 + -15316;
	// 826A58D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A58D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A58D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A58DC: 386A8078  addi r3, r10, -0x7f88
	ctx.r[3].s64 = ctx.r[10].s64 + -32648;
	// 826A58E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A58E4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A58E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A58EC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A58F0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A58F4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A58F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A58FC: 4BDC1525  bl 0x82466e20
	ctx.lr = 0x826A5900;
	sub_82466E20(ctx, base);
	// 826A5900: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A590C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5910 size=116
    let mut pc: u32 = 0x826A5910;
    'dispatch: loop {
        match pc {
            0x826A5910 => {
    //   block [0x826A5910..0x826A5984)
	// 826A5910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A591C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A5920: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5924: 392BC418  addi r9, r11, -0x3be8
	ctx.r[9].s64 = ctx.r[11].s64 + -15336;
	// 826A5928: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A592C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A5930: 38E90038  addi r7, r9, 0x38
	ctx.r[7].s64 = ctx.r[9].s64 + 56;
	// 826A5934: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 826A5938: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A593C: 388AAE98  addi r4, r10, -0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + -20840;
	// 826A5940: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5944: 396BB228  addi r11, r11, -0x4dd8
	ctx.r[11].s64 = ctx.r[11].s64 + -19928;
	// 826A5948: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A594C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A5950: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A5954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5958: 386A80A8  addi r3, r10, -0x7f58
	ctx.r[3].s64 = ctx.r[10].s64 + -32600;
	// 826A595C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A5960: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A5964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5968: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A596C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A5970: 4BDC14B1  bl 0x82466e20
	ctx.lr = 0x826A5974;
	sub_82466E20(ctx, base);
	// 826A5974: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A597C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5988 size=112
    let mut pc: u32 = 0x826A5988;
    'dispatch: loop {
        match pc {
            0x826A5988 => {
    //   block [0x826A5988..0x826A59F8)
	// 826A5988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A598C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A5994: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5998: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A599C: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A59A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A59A4: 390BB510  addi r8, r11, -0x4af0
	ctx.r[8].s64 = ctx.r[11].s64 + -19184;
	// 826A59A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A59AC: 388AAEB0  addi r4, r10, -0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + -20816;
	// 826A59B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A59B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A59B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A59BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A59C0: 386A80D8  addi r3, r10, -0x7f28
	ctx.r[3].s64 = ctx.r[10].s64 + -32552;
	// 826A59C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A59C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A59CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A59D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A59D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A59D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A59DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A59E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A59E4: 4BDC143D  bl 0x82466e20
	ctx.lr = 0x826A59E8;
	sub_82466E20(ctx, base);
	// 826A59E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A59EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A59F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A59F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A59F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A59F8 size=116
    let mut pc: u32 = 0x826A59F8;
    'dispatch: loop {
        match pc {
            0x826A59F8 => {
    //   block [0x826A59F8..0x826A5A6C)
	// 826A59F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A59FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A5A04: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A5A08: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826A5A0C: 390AB558  addi r8, r10, -0x4aa8
	ctx.r[8].s64 = ctx.r[10].s64 + -19112;
	// 826A5A10: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5A14: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A5A18: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A5A1C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A5A20: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A5A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A5A28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A5A2C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826A5A30: 396BC514  addi r11, r11, -0x3aec
	ctx.r[11].s64 = ctx.r[11].s64 + -15084;
	// 826A5A34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A5A38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5A3C: 386A8108  addi r3, r10, -0x7ef8
	ctx.r[3].s64 = ctx.r[10].s64 + -32504;
	// 826A5A40: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A5A44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A5A48: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A5A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A5A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5A58: 4BDC13C9  bl 0x82466e20
	ctx.lr = 0x826A5A5C;
	sub_82466E20(ctx, base);
	// 826A5A5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5A70 size=112
    let mut pc: u32 = 0x826A5A70;
    'dispatch: loop {
        match pc {
            0x826A5A70 => {
    //   block [0x826A5A70..0x826A5AE0)
	// 826A5A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A5A7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5A80: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5A84: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A5A88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A5A8C: 390BB5A0  addi r8, r11, -0x4a60
	ctx.r[8].s64 = ctx.r[11].s64 + -19040;
	// 826A5A90: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826A5A94: 388AAEF4  addi r4, r10, -0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + -20748;
	// 826A5A98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A5A9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A5AA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A5AA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5AA8: 386A8138  addi r3, r10, -0x7ec8
	ctx.r[3].s64 = ctx.r[10].s64 + -32456;
	// 826A5AAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A5AB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A5AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A5AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A5ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A5AC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A5ACC: 4BDC1355  bl 0x82466e20
	ctx.lr = 0x826A5AD0;
	sub_82466E20(ctx, base);
	// 826A5AD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5AE0 size=112
    let mut pc: u32 = 0x826A5AE0;
    'dispatch: loop {
        match pc {
            0x826A5AE0 => {
    //   block [0x826A5AE0..0x826A5B50)
	// 826A5AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A5AEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5AF0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5AF4: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A5AF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A5AFC: 390BB630  addi r8, r11, -0x49d0
	ctx.r[8].s64 = ctx.r[11].s64 + -18896;
	// 826A5B00: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826A5B04: 388AAF08  addi r4, r10, -0x50f8
	ctx.r[4].s64 = ctx.r[10].s64 + -20728;
	// 826A5B08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A5B0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A5B10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A5B14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5B18: 386A8168  addi r3, r10, -0x7e98
	ctx.r[3].s64 = ctx.r[10].s64 + -32408;
	// 826A5B1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A5B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A5B24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A5B28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A5B2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5B30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A5B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5B38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A5B3C: 4BDC12E5  bl 0x82466e20
	ctx.lr = 0x826A5B40;
	sub_82466E20(ctx, base);
	// 826A5B40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A5B50 size=28
    let mut pc: u32 = 0x826A5B50;
    'dispatch: loop {
        match pc {
            0x826A5B50 => {
    //   block [0x826A5B50..0x826A5B6C)
	// 826A5B50: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5B54: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A5B58: 394A30A0  addi r10, r10, 0x30a0
	ctx.r[10].s64 = ctx.r[10].s64 + 12448;
	// 826A5B5C: 816BB6A8  lwz r11, -0x4958(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18776 as u32) ) } as u64;
	// 826A5B60: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826A5B64: 916A0098  stw r11, 0x98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 826A5B68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5B70 size=116
    let mut pc: u32 = 0x826A5B70;
    'dispatch: loop {
        match pc {
            0x826A5B70 => {
    //   block [0x826A5B70..0x826A5BE4)
	// 826A5B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A5B7C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A5B80: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5B84: 392BC540  addi r9, r11, -0x3ac0
	ctx.r[9].s64 = ctx.r[11].s64 + -15040;
	// 826A5B88: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A5B8C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A5B90: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826A5B94: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 826A5B98: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5B9C: 388AAF24  addi r4, r10, -0x50dc
	ctx.r[4].s64 = ctx.r[10].s64 + -20700;
	// 826A5BA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5BA4: 396B30A0  addi r11, r11, 0x30a0
	ctx.r[11].s64 = ctx.r[11].s64 + 12448;
	// 826A5BA8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A5BAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A5BB0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A5BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5BB8: 386A8198  addi r3, r10, -0x7e68
	ctx.r[3].s64 = ctx.r[10].s64 + -32360;
	// 826A5BBC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A5BC0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A5BC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5BC8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A5BCC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A5BD0: 4BDC1251  bl 0x82466e20
	ctx.lr = 0x826A5BD4;
	sub_82466E20(ctx, base);
	// 826A5BD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5BE8 size=112
    let mut pc: u32 = 0x826A5BE8;
    'dispatch: loop {
        match pc {
            0x826A5BE8 => {
    //   block [0x826A5BE8..0x826A5C58)
	// 826A5BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A5BF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5BF8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5BFC: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A5C00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A5C04: 390BB6B0  addi r8, r11, -0x4950
	ctx.r[8].s64 = ctx.r[11].s64 + -18768;
	// 826A5C08: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826A5C0C: 388AAF40  addi r4, r10, -0x50c0
	ctx.r[4].s64 = ctx.r[10].s64 + -20672;
	// 826A5C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A5C14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A5C18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A5C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5C20: 386A81C8  addi r3, r10, -0x7e38
	ctx.r[3].s64 = ctx.r[10].s64 + -32312;
	// 826A5C24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A5C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A5C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A5C30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A5C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5C38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A5C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5C40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A5C44: 4BDC11DD  bl 0x82466e20
	ctx.lr = 0x826A5C48;
	sub_82466E20(ctx, base);
	// 826A5C48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A5C58 size=24
    let mut pc: u32 = 0x826A5C58;
    'dispatch: loop {
        match pc {
            0x826A5C58 => {
    //   block [0x826A5C58..0x826A5C70)
	// 826A5C58: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5C5C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A5C60: 394A3160  addi r10, r10, 0x3160
	ctx.r[10].s64 = ctx.r[10].s64 + 12640;
	// 826A5C64: 816BB6AC  lwz r11, -0x4954(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18772 as u32) ) } as u64;
	// 826A5C68: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826A5C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5C70 size=116
    let mut pc: u32 = 0x826A5C70;
    'dispatch: loop {
        match pc {
            0x826A5C70 => {
    //   block [0x826A5C70..0x826A5CE4)
	// 826A5C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A5C7C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5C80: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A5C84: 390B3160  addi r8, r11, 0x3160
	ctx.r[8].s64 = ctx.r[11].s64 + 12640;
	// 826A5C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A5C8C: 392AC5B0  addi r9, r10, -0x3a50
	ctx.r[9].s64 = ctx.r[10].s64 + -14928;
	// 826A5C90: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5C94: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826A5C98: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A5C9C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A5CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A5CA4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A5CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A5CAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A5CB4: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826A5CB8: 388AAF78  addi r4, r10, -0x5088
	ctx.r[4].s64 = ctx.r[10].s64 + -20616;
	// 826A5CBC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A5CC0: 386B81F8  addi r3, r11, -0x7e08
	ctx.r[3].s64 = ctx.r[11].s64 + -32264;
	// 826A5CC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A5CC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5CD0: 4BDC1151  bl 0x82466e20
	ctx.lr = 0x826A5CD4;
	sub_82466E20(ctx, base);
	// 826A5CD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5CE8 size=112
    let mut pc: u32 = 0x826A5CE8;
    'dispatch: loop {
        match pc {
            0x826A5CE8 => {
    //   block [0x826A5CE8..0x826A5D58)
	// 826A5CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A5CF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5CF8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5CFC: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A5D00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A5D04: 390BB728  addi r8, r11, -0x48d8
	ctx.r[8].s64 = ctx.r[11].s64 + -18648;
	// 826A5D08: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826A5D0C: 388AAF98  addi r4, r10, -0x5068
	ctx.r[4].s64 = ctx.r[10].s64 + -20584;
	// 826A5D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A5D14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A5D18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A5D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5D20: 386A8228  addi r3, r10, -0x7dd8
	ctx.r[3].s64 = ctx.r[10].s64 + -32216;
	// 826A5D24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A5D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A5D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A5D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A5D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A5D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A5D44: 4BDC10DD  bl 0x82466e20
	ctx.lr = 0x826A5D48;
	sub_82466E20(ctx, base);
	// 826A5D48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5D58 size=112
    let mut pc: u32 = 0x826A5D58;
    'dispatch: loop {
        match pc {
            0x826A5D58 => {
    //   block [0x826A5D58..0x826A5DC8)
	// 826A5D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A5D64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5D68: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5D6C: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A5D70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A5D74: 390BB818  addi r8, r11, -0x47e8
	ctx.r[8].s64 = ctx.r[11].s64 + -18408;
	// 826A5D78: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826A5D7C: 388AAFB4  addi r4, r10, -0x504c
	ctx.r[4].s64 = ctx.r[10].s64 + -20556;
	// 826A5D80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A5D84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A5D88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A5D8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5D90: 386A8258  addi r3, r10, -0x7da8
	ctx.r[3].s64 = ctx.r[10].s64 + -32168;
	// 826A5D94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A5D98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A5D9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A5DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A5DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A5DAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A5DB4: 4BDC106D  bl 0x82466e20
	ctx.lr = 0x826A5DB8;
	sub_82466E20(ctx, base);
	// 826A5DB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A5DC8 size=24
    let mut pc: u32 = 0x826A5DC8;
    'dispatch: loop {
        match pc {
            0x826A5DC8 => {
    //   block [0x826A5DC8..0x826A5DE0)
	// 826A5DC8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5DCC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A5DD0: 394A3208  addi r10, r10, 0x3208
	ctx.r[10].s64 = ctx.r[10].s64 + 12808;
	// 826A5DD4: 816BB878  lwz r11, -0x4788(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18312 as u32) ) } as u64;
	// 826A5DD8: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826A5DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5DE0 size=116
    let mut pc: u32 = 0x826A5DE0;
    'dispatch: loop {
        match pc {
            0x826A5DE0 => {
    //   block [0x826A5DE0..0x826A5E54)
	// 826A5DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A5DEC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5DF0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A5DF4: 390B3208  addi r8, r11, 0x3208
	ctx.r[8].s64 = ctx.r[11].s64 + 12808;
	// 826A5DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A5DFC: 392AC5FC  addi r9, r10, -0x3a04
	ctx.r[9].s64 = ctx.r[10].s64 + -14852;
	// 826A5E00: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5E04: 38E00019  li r7, 0x19
	ctx.r[7].s64 = 25;
	// 826A5E08: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A5E0C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A5E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A5E14: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A5E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A5E1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A5E24: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826A5E28: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826A5E2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A5E30: 386B8288  addi r3, r11, -0x7d78
	ctx.r[3].s64 = ctx.r[11].s64 + -32120;
	// 826A5E34: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826A5E38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5E40: 4BDC0FE1  bl 0x82466e20
	ctx.lr = 0x826A5E44;
	sub_82466E20(ctx, base);
	// 826A5E44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5E58 size=116
    let mut pc: u32 = 0x826A5E58;
    'dispatch: loop {
        match pc {
            0x826A5E58 => {
    //   block [0x826A5E58..0x826A5ECC)
	// 826A5E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5E60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A5E64: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A5E68: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5E6C: 392BC634  addi r9, r11, -0x39cc
	ctx.r[9].s64 = ctx.r[11].s64 + -14796;
	// 826A5E70: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A5E74: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A5E78: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826A5E7C: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 826A5E80: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5E84: 388AB168  addi r4, r10, -0x4e98
	ctx.r[4].s64 = ctx.r[10].s64 + -20120;
	// 826A5E88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5E8C: 396BB888  addi r11, r11, -0x4778
	ctx.r[11].s64 = ctx.r[11].s64 + -18296;
	// 826A5E90: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A5E94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A5E98: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A5E9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5EA0: 386A82B8  addi r3, r10, -0x7d48
	ctx.r[3].s64 = ctx.r[10].s64 + -32072;
	// 826A5EA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A5EA8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A5EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5EB0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A5EB4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A5EB8: 4BDC0F69  bl 0x82466e20
	ctx.lr = 0x826A5EBC;
	sub_82466E20(ctx, base);
	// 826A5EBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5ED0 size=112
    let mut pc: u32 = 0x826A5ED0;
    'dispatch: loop {
        match pc {
            0x826A5ED0 => {
    //   block [0x826A5ED0..0x826A5F40)
	// 826A5ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5ED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A5EDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A5EE0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5EE4: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A5EE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A5EEC: 390BBA50  addi r8, r11, -0x45b0
	ctx.r[8].s64 = ctx.r[11].s64 + -17840;
	// 826A5EF0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826A5EF4: 388AB188  addi r4, r10, -0x4e78
	ctx.r[4].s64 = ctx.r[10].s64 + -20088;
	// 826A5EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A5EFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A5F00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A5F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5F08: 386A82E8  addi r3, r10, -0x7d18
	ctx.r[3].s64 = ctx.r[10].s64 + -32024;
	// 826A5F0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A5F10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A5F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A5F18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A5F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5F20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A5F24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5F28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A5F2C: 4BDC0EF5  bl 0x82466e20
	ctx.lr = 0x826A5F30;
	sub_82466E20(ctx, base);
	// 826A5F30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A5F40 size=48
    let mut pc: u32 = 0x826A5F40;
    'dispatch: loop {
        match pc {
            0x826A5F40 => {
    //   block [0x826A5F40..0x826A5F70)
	// 826A5F40: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5F44: 814BBAC8  lwz r10, -0x4538(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17720 as u32) ) } as u64;
	// 826A5F48: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5F4C: 396B3460  addi r11, r11, 0x3460
	ctx.r[11].s64 = ctx.r[11].s64 + 13408;
	// 826A5F50: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826A5F54: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A5F58: 814ABACC  lwz r10, -0x4534(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17716 as u32) ) } as u64;
	// 826A5F5C: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 826A5F60: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A5F64: 814AB884  lwz r10, -0x477c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-18300 as u32) ) } as u64;
	// 826A5F68: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 826A5F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5F70 size=116
    let mut pc: u32 = 0x826A5F70;
    'dispatch: loop {
        match pc {
            0x826A5F70 => {
    //   block [0x826A5F70..0x826A5FE4)
	// 826A5F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A5F7C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A5F80: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A5F84: 390B3460  addi r8, r11, 0x3460
	ctx.r[8].s64 = ctx.r[11].s64 + 13408;
	// 826A5F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A5F8C: 392AC6F0  addi r9, r10, -0x3910
	ctx.r[9].s64 = ctx.r[10].s64 + -14608;
	// 826A5F90: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A5F94: 38E00016  li r7, 0x16
	ctx.r[7].s64 = 22;
	// 826A5F98: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A5F9C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A5FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A5FA4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A5FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A5FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A5FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A5FB4: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826A5FB8: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826A5FBC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A5FC0: 386B8318  addi r3, r11, -0x7ce8
	ctx.r[3].s64 = ctx.r[11].s64 + -31976;
	// 826A5FC4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826A5FC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A5FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A5FD0: 4BDC0E51  bl 0x82466e20
	ctx.lr = 0x826A5FD4;
	sub_82466E20(ctx, base);
	// 826A5FD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A5FD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A5FDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A5FE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A5FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A5FE8 size=116
    let mut pc: u32 = 0x826A5FE8;
    'dispatch: loop {
        match pc {
            0x826A5FE8 => {
    //   block [0x826A5FE8..0x826A605C)
	// 826A5FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A5FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A5FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A5FF4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A5FF8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826A5FFC: 390ABAD0  addi r8, r10, -0x4530
	ctx.r[8].s64 = ctx.r[10].s64 + -17712;
	// 826A6000: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A6004: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A6008: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A600C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A6010: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A6014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A601C: 388AB2C4  addi r4, r10, -0x4d3c
	ctx.r[4].s64 = ctx.r[10].s64 + -19772;
	// 826A6020: 396BC72C  addi r11, r11, -0x38d4
	ctx.r[11].s64 = ctx.r[11].s64 + -14548;
	// 826A6024: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6028: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A602C: 386A8348  addi r3, r10, -0x7cb8
	ctx.r[3].s64 = ctx.r[10].s64 + -31928;
	// 826A6030: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A6034: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6038: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A603C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6048: 4BDC0DD9  bl 0x82466e20
	ctx.lr = 0x826A604C;
	sub_82466E20(ctx, base);
	// 826A604C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6060 size=116
    let mut pc: u32 = 0x826A6060;
    'dispatch: loop {
        match pc {
            0x826A6060 => {
    //   block [0x826A6060..0x826A60D4)
	// 826A6060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A606C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A6070: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826A6074: 390ABB18  addi r8, r10, -0x44e8
	ctx.r[8].s64 = ctx.r[10].s64 + -17640;
	// 826A6078: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A607C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A6080: 38AA7778  addi r5, r10, 0x7778
	ctx.r[5].s64 = ctx.r[10].s64 + 30584;
	// 826A6084: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A6088: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A608C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6090: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A6094: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 826A6098: 396BC73C  addi r11, r11, -0x38c4
	ctx.r[11].s64 = ctx.r[11].s64 + -14532;
	// 826A609C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A60A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A60A4: 386A8378  addi r3, r10, -0x7c88
	ctx.r[3].s64 = ctx.r[10].s64 + -31880;
	// 826A60A8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A60AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A60B0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A60B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A60B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A60BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A60C0: 4BDC0D61  bl 0x82466e20
	ctx.lr = 0x826A60C4;
	sub_82466E20(ctx, base);
	// 826A60C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A60C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A60CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A60D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A60D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A60D8 size=116
    let mut pc: u32 = 0x826A60D8;
    'dispatch: loop {
        match pc {
            0x826A60D8 => {
    //   block [0x826A60D8..0x826A614C)
	// 826A60D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A60DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A60E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A60E4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A60E8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826A60EC: 390ABB90  addi r8, r10, -0x4470
	ctx.r[8].s64 = ctx.r[10].s64 + -17520;
	// 826A60F0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A60F4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A60F8: 38AA7B98  addi r5, r10, 0x7b98
	ctx.r[5].s64 = ctx.r[10].s64 + 31640;
	// 826A60FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A6100: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A6104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6108: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A610C: 388AB2F4  addi r4, r10, -0x4d0c
	ctx.r[4].s64 = ctx.r[10].s64 + -19724;
	// 826A6110: 396BC754  addi r11, r11, -0x38ac
	ctx.r[11].s64 = ctx.r[11].s64 + -14508;
	// 826A6114: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6118: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A611C: 386A83A8  addi r3, r10, -0x7c58
	ctx.r[3].s64 = ctx.r[10].s64 + -31832;
	// 826A6120: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A6124: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6128: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A612C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6138: 4BDC0CE9  bl 0x82466e20
	ctx.lr = 0x826A613C;
	sub_82466E20(ctx, base);
	// 826A613C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6150 size=108
    let mut pc: u32 = 0x826A6150;
    'dispatch: loop {
        match pc {
            0x826A6150 => {
    //   block [0x826A6150..0x826A61BC)
	// 826A6150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A615C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6160: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A6164: 38EBBC08  addi r7, r11, -0x43f8
	ctx.r[7].s64 = ctx.r[11].s64 + -17400;
	// 826A6168: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A616C: 388AB318  addi r4, r10, -0x4ce8
	ctx.r[4].s64 = ctx.r[10].s64 + -19688;
	// 826A6170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6174: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6178: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A617C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6180: 386A83D8  addi r3, r10, -0x7c28
	ctx.r[3].s64 = ctx.r[10].s64 + -31784;
	// 826A6184: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A618C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A619C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A61A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A61A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A61A8: 4BDC0C79  bl 0x82466e20
	ctx.lr = 0x826A61AC;
	sub_82466E20(ctx, base);
	// 826A61AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A61B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A61B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A61B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A61C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A61C0 size=108
    let mut pc: u32 = 0x826A61C0;
    'dispatch: loop {
        match pc {
            0x826A61C0 => {
    //   block [0x826A61C0..0x826A622C)
	// 826A61C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A61C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A61C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A61CC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A61D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A61D4: 38EBBC50  addi r7, r11, -0x43b0
	ctx.r[7].s64 = ctx.r[11].s64 + -17328;
	// 826A61D8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A61DC: 388AB344  addi r4, r10, -0x4cbc
	ctx.r[4].s64 = ctx.r[10].s64 + -19644;
	// 826A61E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A61E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A61E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A61EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A61F0: 386A8408  addi r3, r10, -0x7bf8
	ctx.r[3].s64 = ctx.r[10].s64 + -31736;
	// 826A61F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A61F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A61FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A620C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6214: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6218: 4BDC0C09  bl 0x82466e20
	ctx.lr = 0x826A621C;
	sub_82466E20(ctx, base);
	// 826A621C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6230 size=108
    let mut pc: u32 = 0x826A6230;
    'dispatch: loop {
        match pc {
            0x826A6230 => {
    //   block [0x826A6230..0x826A629C)
	// 826A6230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A623C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6240: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A6244: 38EBBC98  addi r7, r11, -0x4368
	ctx.r[7].s64 = ctx.r[11].s64 + -17256;
	// 826A6248: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A624C: 388AB36C  addi r4, r10, -0x4c94
	ctx.r[4].s64 = ctx.r[10].s64 + -19604;
	// 826A6250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6254: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6258: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A625C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6260: 386A8438  addi r3, r10, -0x7bc8
	ctx.r[3].s64 = ctx.r[10].s64 + -31688;
	// 826A6264: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A626C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A627C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6284: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6288: 4BDC0B99  bl 0x82466e20
	ctx.lr = 0x826A628C;
	sub_82466E20(ctx, base);
	// 826A628C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A62A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A62A0 size=108
    let mut pc: u32 = 0x826A62A0;
    'dispatch: loop {
        match pc {
            0x826A62A0 => {
    //   block [0x826A62A0..0x826A630C)
	// 826A62A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A62A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A62A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A62AC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A62B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A62B4: 38EBBCE0  addi r7, r11, -0x4320
	ctx.r[7].s64 = ctx.r[11].s64 + -17184;
	// 826A62B8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826A62BC: 388AB398  addi r4, r10, -0x4c68
	ctx.r[4].s64 = ctx.r[10].s64 + -19560;
	// 826A62C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A62C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A62C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A62CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A62D0: 386A8468  addi r3, r10, -0x7b98
	ctx.r[3].s64 = ctx.r[10].s64 + -31640;
	// 826A62D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A62D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A62DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A62E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A62E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A62E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A62EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A62F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A62F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A62F8: 4BDC0B29  bl 0x82466e20
	ctx.lr = 0x826A62FC;
	sub_82466E20(ctx, base);
	// 826A62FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6310 size=108
    let mut pc: u32 = 0x826A6310;
    'dispatch: loop {
        match pc {
            0x826A6310 => {
    //   block [0x826A6310..0x826A637C)
	// 826A6310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A631C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6320: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A6324: 38EBBD88  addi r7, r11, -0x4278
	ctx.r[7].s64 = ctx.r[11].s64 + -17016;
	// 826A6328: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A632C: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 826A6330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6334: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6338: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A633C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6340: 386A8498  addi r3, r10, -0x7b68
	ctx.r[3].s64 = ctx.r[10].s64 + -31592;
	// 826A6344: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A634C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A635C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6368: 4BDC0AB9  bl 0x82466e20
	ctx.lr = 0x826A636C;
	sub_82466E20(ctx, base);
	// 826A636C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6380 size=112
    let mut pc: u32 = 0x826A6380;
    'dispatch: loop {
        match pc {
            0x826A6380 => {
    //   block [0x826A6380..0x826A63F0)
	// 826A6380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A638C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A6390: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6394: 392AC794  addi r9, r10, -0x386c
	ctx.r[9].s64 = ctx.r[10].s64 + -14444;
	// 826A6398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A639C: 390BBDC0  addi r8, r11, -0x4240
	ctx.r[8].s64 = ctx.r[11].s64 + -16960;
	// 826A63A0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826A63A4: 388A8198  addi r4, r10, -0x7e68
	ctx.r[4].s64 = ctx.r[10].s64 + -32360;
	// 826A63A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A63AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A63B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A63B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A63B8: 386A84C8  addi r3, r10, -0x7b38
	ctx.r[3].s64 = ctx.r[10].s64 + -31544;
	// 826A63BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A63C0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826A63C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A63C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A63CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A63D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A63D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A63D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A63DC: 4BDC0A45  bl 0x82466e20
	ctx.lr = 0x826A63E0;
	sub_82466E20(ctx, base);
	// 826A63E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A63E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A63E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A63EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A63F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A63F0 size=108
    let mut pc: u32 = 0x826A63F0;
    'dispatch: loop {
        match pc {
            0x826A63F0 => {
    //   block [0x826A63F0..0x826A645C)
	// 826A63F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A63F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A63F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A63FC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A6404: 38EBBE08  addi r7, r11, -0x41f8
	ctx.r[7].s64 = ctx.r[11].s64 + -16888;
	// 826A6408: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826A640C: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 826A6410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6414: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6418: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A641C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6420: 386A84F8  addi r3, r10, -0x7b08
	ctx.r[3].s64 = ctx.r[10].s64 + -31496;
	// 826A6424: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A642C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A643C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6448: 4BDC09D9  bl 0x82466e20
	ctx.lr = 0x826A644C;
	sub_82466E20(ctx, base);
	// 826A644C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6460 size=108
    let mut pc: u32 = 0x826A6460;
    'dispatch: loop {
        match pc {
            0x826A6460 => {
    //   block [0x826A6460..0x826A64CC)
	// 826A6460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A646C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6470: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A6474: 38EBBE80  addi r7, r11, -0x4180
	ctx.r[7].s64 = ctx.r[11].s64 + -16768;
	// 826A6478: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A647C: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 826A6480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6484: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6488: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A648C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6490: 386A8528  addi r3, r10, -0x7ad8
	ctx.r[3].s64 = ctx.r[10].s64 + -31448;
	// 826A6494: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A649C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A64A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A64A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A64A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A64AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A64B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A64B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A64B8: 4BDC0969  bl 0x82466e20
	ctx.lr = 0x826A64BC;
	sub_82466E20(ctx, base);
	// 826A64BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A64C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A64C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A64C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A64D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A64D0 size=108
    let mut pc: u32 = 0x826A64D0;
    'dispatch: loop {
        match pc {
            0x826A64D0 => {
    //   block [0x826A64D0..0x826A653C)
	// 826A64D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A64D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A64D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A64DC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A64E0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A64E4: 38EBBEB0  addi r7, r11, -0x4150
	ctx.r[7].s64 = ctx.r[11].s64 + -16720;
	// 826A64E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A64EC: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 826A64F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A64F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A64F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A64FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6500: 386A8558  addi r3, r10, -0x7aa8
	ctx.r[3].s64 = ctx.r[10].s64 + -31400;
	// 826A6504: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A650C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A651C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6524: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6528: 4BDC08F9  bl 0x82466e20
	ctx.lr = 0x826A652C;
	sub_82466E20(ctx, base);
	// 826A652C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A6540 size=24
    let mut pc: u32 = 0x826A6540;
    'dispatch: loop {
        match pc {
            0x826A6540 => {
    //   block [0x826A6540..0x826A6558)
	// 826A6540: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6544: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A6548: 394A3670  addi r10, r10, 0x3670
	ctx.r[10].s64 = ctx.r[10].s64 + 13936;
	// 826A654C: 816BBEC8  lwz r11, -0x4138(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16696 as u32) ) } as u64;
	// 826A6550: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826A6554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6558 size=112
    let mut pc: u32 = 0x826A6558;
    'dispatch: loop {
        match pc {
            0x826A6558 => {
    //   block [0x826A6558..0x826A65C8)
	// 826A6558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A655C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6564: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A6568: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A656C: 392AC7D4  addi r9, r10, -0x382c
	ctx.r[9].s64 = ctx.r[10].s64 + -14380;
	// 826A6570: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A6574: 390B3670  addi r8, r11, 0x3670
	ctx.r[8].s64 = ctx.r[11].s64 + 13936;
	// 826A6578: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826A657C: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 826A6580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6584: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6588: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A658C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6590: 386A8588  addi r3, r10, -0x7a78
	ctx.r[3].s64 = ctx.r[10].s64 + -31352;
	// 826A6594: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A6598: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A659C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A65A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A65A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A65A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A65AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A65B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A65B4: 4BDC086D  bl 0x82466e20
	ctx.lr = 0x826A65B8;
	sub_82466E20(ctx, base);
	// 826A65B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A65BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A65C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A65C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A65C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A65C8 size=96
    let mut pc: u32 = 0x826A65C8;
    'dispatch: loop {
        match pc {
            0x826A65C8 => {
    //   block [0x826A65C8..0x826A6628)
	// 826A65C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A65CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A65D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A65D4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A65D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A65DC: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826A65E0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A65E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A65E8: 386A85B8  addi r3, r10, -0x7a48
	ctx.r[3].s64 = ctx.r[10].s64 + -31304;
	// 826A65EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A65F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A65F4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A65F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A65FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6608: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A660C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6610: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A6614: 4BDC080D  bl 0x82466e20
	ctx.lr = 0x826A6618;
	sub_82466E20(ctx, base);
	// 826A6618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A661C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6628 size=112
    let mut pc: u32 = 0x826A6628;
    'dispatch: loop {
        match pc {
            0x826A6628 => {
    //   block [0x826A6628..0x826A6698)
	// 826A6628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A662C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6634: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6638: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A663C: 38AA85B8  addi r5, r10, -0x7a48
	ctx.r[5].s64 = ctx.r[10].s64 + -31304;
	// 826A6640: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A6644: 390BBECC  addi r8, r11, -0x4134
	ctx.r[8].s64 = ctx.r[11].s64 + -16692;
	// 826A6648: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A664C: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 826A6650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6654: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6658: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A665C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6660: 386A85E8  addi r3, r10, -0x7a18
	ctx.r[3].s64 = ctx.r[10].s64 + -31256;
	// 826A6664: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A6668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A666C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6674: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A667C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6684: 4BDC079D  bl 0x82466e20
	ctx.lr = 0x826A6688;
	sub_82466E20(ctx, base);
	// 826A6688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A668C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A6698 size=24
    let mut pc: u32 = 0x826A6698;
    'dispatch: loop {
        match pc {
            0x826A6698 => {
    //   block [0x826A6698..0x826A66B0)
	// 826A6698: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A669C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A66A0: 394A3748  addi r10, r10, 0x3748
	ctx.r[10].s64 = ctx.r[10].s64 + 14152;
	// 826A66A4: 816BBF00  lwz r11, -0x4100(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16640 as u32) ) } as u64;
	// 826A66A8: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 826A66AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


