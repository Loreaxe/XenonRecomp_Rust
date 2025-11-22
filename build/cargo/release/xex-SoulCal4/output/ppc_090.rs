pub fn sub_825F6190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6190 size=112
    let mut pc: u32 = 0x825F6190;
    'dispatch: loop {
        match pc {
            0x825F6190 => {
    //   block [0x825F6190..0x825F6200)
	// 825F6190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F619C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F61A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F61A4: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F61A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F61AC: 390BC5F0  addi r8, r11, -0x3a10
	ctx.r[8].s64 = ctx.r[11].s64 + -14864;
	// 825F61B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F61B4: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 825F61B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F61BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F61C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F61C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F61C8: 386AE154  addi r3, r10, -0x1eac
	ctx.r[3].s64 = ctx.r[10].s64 + -7852;
	// 825F61CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F61D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F61D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F61D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F61DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F61E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F61E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F61E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F61EC: 4BE70C35  bl 0x82466e20
	ctx.lr = 0x825F61F0;
	sub_82466E20(ctx, base);
	// 825F61F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F61F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F61F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F61FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6200 size=112
    let mut pc: u32 = 0x825F6200;
    'dispatch: loop {
        match pc {
            0x825F6200 => {
    //   block [0x825F6200..0x825F6270)
	// 825F6200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F620C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6210: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6214: 38AAD164  addi r5, r10, -0x2e9c
	ctx.r[5].s64 = ctx.r[10].s64 + -11932;
	// 825F6218: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F621C: 390BC608  addi r8, r11, -0x39f8
	ctx.r[8].s64 = ctx.r[11].s64 + -14840;
	// 825F6220: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F6224: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 825F6228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F622C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6230: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6238: 386AE184  addi r3, r10, -0x1e7c
	ctx.r[3].s64 = ctx.r[10].s64 + -7804;
	// 825F623C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F624C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F625C: 4BE70BC5  bl 0x82466e20
	ctx.lr = 0x825F6260;
	sub_82466E20(ctx, base);
	// 825F6260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F626C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6270 size=112
    let mut pc: u32 = 0x825F6270;
    'dispatch: loop {
        match pc {
            0x825F6270 => {
    //   block [0x825F6270..0x825F62E0)
	// 825F6270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F627C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6280: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6284: 38AAD164  addi r5, r10, -0x2e9c
	ctx.r[5].s64 = ctx.r[10].s64 + -11932;
	// 825F6288: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F628C: 390BC650  addi r8, r11, -0x39b0
	ctx.r[8].s64 = ctx.r[11].s64 + -14768;
	// 825F6290: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F6294: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 825F6298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F629C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F62A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F62A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F62A8: 386AE1B4  addi r3, r10, -0x1e4c
	ctx.r[3].s64 = ctx.r[10].s64 + -7756;
	// 825F62AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F62B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F62B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F62B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F62BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F62C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F62C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F62C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F62CC: 4BE70B55  bl 0x82466e20
	ctx.lr = 0x825F62D0;
	sub_82466E20(ctx, base);
	// 825F62D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F62D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F62D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F62DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F62E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F62E0 size=112
    let mut pc: u32 = 0x825F62E0;
    'dispatch: loop {
        match pc {
            0x825F62E0 => {
    //   block [0x825F62E0..0x825F6350)
	// 825F62E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F62E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F62E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F62EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F62F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F62F4: 38AAD194  addi r5, r10, -0x2e6c
	ctx.r[5].s64 = ctx.r[10].s64 + -11884;
	// 825F62F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F62FC: 390BC6B0  addi r8, r11, -0x3950
	ctx.r[8].s64 = ctx.r[11].s64 + -14672;
	// 825F6300: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F6304: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 825F6308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F630C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6310: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6318: 386AE1E4  addi r3, r10, -0x1e1c
	ctx.r[3].s64 = ctx.r[10].s64 + -7708;
	// 825F631C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F632C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F633C: 4BE70AE5  bl 0x82466e20
	ctx.lr = 0x825F6340;
	sub_82466E20(ctx, base);
	// 825F6340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F634C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6350 size=112
    let mut pc: u32 = 0x825F6350;
    'dispatch: loop {
        match pc {
            0x825F6350 => {
    //   block [0x825F6350..0x825F63C0)
	// 825F6350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F635C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6360: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6364: 38AAD194  addi r5, r10, -0x2e6c
	ctx.r[5].s64 = ctx.r[10].s64 + -11884;
	// 825F6368: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F636C: 390BC710  addi r8, r11, -0x38f0
	ctx.r[8].s64 = ctx.r[11].s64 + -14576;
	// 825F6370: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F6374: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 825F6378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F637C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6380: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6388: 386AE214  addi r3, r10, -0x1dec
	ctx.r[3].s64 = ctx.r[10].s64 + -7660;
	// 825F638C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F639C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F63A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F63A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F63A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F63AC: 4BE70A75  bl 0x82466e20
	ctx.lr = 0x825F63B0;
	sub_82466E20(ctx, base);
	// 825F63B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F63B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F63B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F63BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F63C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F63C0 size=112
    let mut pc: u32 = 0x825F63C0;
    'dispatch: loop {
        match pc {
            0x825F63C0 => {
    //   block [0x825F63C0..0x825F6430)
	// 825F63C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F63C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F63C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F63CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F63D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F63D4: 38AAD164  addi r5, r10, -0x2e9c
	ctx.r[5].s64 = ctx.r[10].s64 + -11932;
	// 825F63D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F63DC: 390BC770  addi r8, r11, -0x3890
	ctx.r[8].s64 = ctx.r[11].s64 + -14480;
	// 825F63E0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 825F63E4: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 825F63E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F63EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F63F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F63F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F63F8: 386AE244  addi r3, r10, -0x1dbc
	ctx.r[3].s64 = ctx.r[10].s64 + -7612;
	// 825F63FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F640C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F641C: 4BE70A05  bl 0x82466e20
	ctx.lr = 0x825F6420;
	sub_82466E20(ctx, base);
	// 825F6420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F642C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6430 size=112
    let mut pc: u32 = 0x825F6430;
    'dispatch: loop {
        match pc {
            0x825F6430 => {
    //   block [0x825F6430..0x825F64A0)
	// 825F6430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F643C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F6440: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 825F6444: 38EAC830  addi r7, r10, -0x37d0
	ctx.r[7].s64 = ctx.r[10].s64 + -14288;
	// 825F6448: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F644C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F6450: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 825F6454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6458: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F645C: 396B2400  addi r11, r11, 0x2400
	ctx.r[11].s64 = ctx.r[11].s64 + 9216;
	// 825F6460: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F6464: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6468: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F646C: 386AE274  addi r3, r10, -0x1d8c
	ctx.r[3].s64 = ctx.r[10].s64 + -7564;
	// 825F6470: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6474: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F6478: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F647C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F6480: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6484: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6488: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F648C: 4BE70995  bl 0x82466e20
	ctx.lr = 0x825F6490;
	sub_82466E20(ctx, base);
	// 825F6490: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F649C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F64A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F64A0 size=112
    let mut pc: u32 = 0x825F64A0;
    'dispatch: loop {
        match pc {
            0x825F64A0 => {
    //   block [0x825F64A0..0x825F6510)
	// 825F64A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F64A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F64A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F64AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F64B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F64B4: 38AAC024  addi r5, r10, -0x3fdc
	ctx.r[5].s64 = ctx.r[10].s64 + -16348;
	// 825F64B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F64BC: 390BC9F8  addi r8, r11, -0x3608
	ctx.r[8].s64 = ctx.r[11].s64 + -13832;
	// 825F64C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F64C4: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 825F64C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F64CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F64D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F64D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F64D8: 386AE2A4  addi r3, r10, -0x1d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -7516;
	// 825F64DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F64E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F64E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F64E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F64EC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F64F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F64F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F64F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F64FC: 4BE70925  bl 0x82466e20
	ctx.lr = 0x825F6500;
	sub_82466E20(ctx, base);
	// 825F6500: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F650C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6510 size=112
    let mut pc: u32 = 0x825F6510;
    'dispatch: loop {
        match pc {
            0x825F6510 => {
    //   block [0x825F6510..0x825F6580)
	// 825F6510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F651C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6520: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6524: 38AAC024  addi r5, r10, -0x3fdc
	ctx.r[5].s64 = ctx.r[10].s64 + -16348;
	// 825F6528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F652C: 390BCA10  addi r8, r11, -0x35f0
	ctx.r[8].s64 = ctx.r[11].s64 + -13808;
	// 825F6530: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F6534: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 825F6538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F653C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6540: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6548: 386AE2D4  addi r3, r10, -0x1d2c
	ctx.r[3].s64 = ctx.r[10].s64 + -7468;
	// 825F654C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F655C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F6560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F656C: 4BE708B5  bl 0x82466e20
	ctx.lr = 0x825F6570;
	sub_82466E20(ctx, base);
	// 825F6570: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F657C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6580 size=112
    let mut pc: u32 = 0x825F6580;
    'dispatch: loop {
        match pc {
            0x825F6580 => {
    //   block [0x825F6580..0x825F65F0)
	// 825F6580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F658C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6590: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6594: 38AAC024  addi r5, r10, -0x3fdc
	ctx.r[5].s64 = ctx.r[10].s64 + -16348;
	// 825F6598: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F659C: 390BCA28  addi r8, r11, -0x35d8
	ctx.r[8].s64 = ctx.r[11].s64 + -13784;
	// 825F65A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F65A4: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 825F65A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F65AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F65B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F65B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F65B8: 386AE304  addi r3, r10, -0x1cfc
	ctx.r[3].s64 = ctx.r[10].s64 + -7420;
	// 825F65BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F65C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F65C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F65C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F65CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F65D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F65D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F65D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F65DC: 4BE70845  bl 0x82466e20
	ctx.lr = 0x825F65E0;
	sub_82466E20(ctx, base);
	// 825F65E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F65E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F65E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F65EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F65F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F65F0 size=108
    let mut pc: u32 = 0x825F65F0;
    'dispatch: loop {
        match pc {
            0x825F65F0 => {
    //   block [0x825F65F0..0x825F665C)
	// 825F65F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F65F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F65F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F65FC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6604: 38EBCA58  addi r7, r11, -0x35a8
	ctx.r[7].s64 = ctx.r[11].s64 + -13736;
	// 825F6608: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F660C: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 825F6610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6614: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6618: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F661C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6620: 386AE334  addi r3, r10, -0x1ccc
	ctx.r[3].s64 = ctx.r[10].s64 + -7372;
	// 825F6624: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F6628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F662C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F663C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6648: 4BE707D9  bl 0x82466e20
	ctx.lr = 0x825F664C;
	sub_82466E20(ctx, base);
	// 825F664C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6660 size=112
    let mut pc: u32 = 0x825F6660;
    'dispatch: loop {
        match pc {
            0x825F6660 => {
    //   block [0x825F6660..0x825F66D0)
	// 825F6660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F666C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6670: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6674: 38AAC024  addi r5, r10, -0x3fdc
	ctx.r[5].s64 = ctx.r[10].s64 + -16348;
	// 825F6678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F667C: 390BCA88  addi r8, r11, -0x3578
	ctx.r[8].s64 = ctx.r[11].s64 + -13688;
	// 825F6680: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F6684: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 825F6688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F668C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6690: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6698: 386AE364  addi r3, r10, -0x1c9c
	ctx.r[3].s64 = ctx.r[10].s64 + -7324;
	// 825F669C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F66A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F66A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F66A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F66AC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F66B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F66B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F66B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F66BC: 4BE70765  bl 0x82466e20
	ctx.lr = 0x825F66C0;
	sub_82466E20(ctx, base);
	// 825F66C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F66C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F66C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F66CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F66D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F66D0 size=108
    let mut pc: u32 = 0x825F66D0;
    'dispatch: loop {
        match pc {
            0x825F66D0 => {
    //   block [0x825F66D0..0x825F673C)
	// 825F66D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F66D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F66D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F66DC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F66E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F66E4: 38EBCAA0  addi r7, r11, -0x3560
	ctx.r[7].s64 = ctx.r[11].s64 + -13664;
	// 825F66E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F66EC: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 825F66F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F66F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F66F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F66FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6700: 386AE394  addi r3, r10, -0x1c6c
	ctx.r[3].s64 = ctx.r[10].s64 + -7276;
	// 825F6704: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F6708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F670C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F671C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6724: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6728: 4BE706F9  bl 0x82466e20
	ctx.lr = 0x825F672C;
	sub_82466E20(ctx, base);
	// 825F672C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6740 size=108
    let mut pc: u32 = 0x825F6740;
    'dispatch: loop {
        match pc {
            0x825F6740 => {
    //   block [0x825F6740..0x825F67AC)
	// 825F6740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F674C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6754: 38EBCAD0  addi r7, r11, -0x3530
	ctx.r[7].s64 = ctx.r[11].s64 + -13616;
	// 825F6758: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F675C: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 825F6760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6764: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6768: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F676C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6770: 386AE3C4  addi r3, r10, -0x1c3c
	ctx.r[3].s64 = ctx.r[10].s64 + -7228;
	// 825F6774: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F6778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F677C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F678C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6798: 4BE70689  bl 0x82466e20
	ctx.lr = 0x825F679C;
	sub_82466E20(ctx, base);
	// 825F679C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F67A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F67A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F67A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F67B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F67B0 size=112
    let mut pc: u32 = 0x825F67B0;
    'dispatch: loop {
        match pc {
            0x825F67B0 => {
    //   block [0x825F67B0..0x825F6820)
	// 825F67B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F67B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F67B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F67BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F67C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F67C4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F67C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F67CC: 390BCB18  addi r8, r11, -0x34e8
	ctx.r[8].s64 = ctx.r[11].s64 + -13544;
	// 825F67D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F67D4: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 825F67D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F67DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F67E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F67E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F67E8: 386AE3F4  addi r3, r10, -0x1c0c
	ctx.r[3].s64 = ctx.r[10].s64 + -7180;
	// 825F67EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F67F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F67F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F67F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F67FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F680C: 4BE70615  bl 0x82466e20
	ctx.lr = 0x825F6810;
	sub_82466E20(ctx, base);
	// 825F6810: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F681C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6820 size=112
    let mut pc: u32 = 0x825F6820;
    'dispatch: loop {
        match pc {
            0x825F6820 => {
    //   block [0x825F6820..0x825F6890)
	// 825F6820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F682C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6830: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6834: 38AAD194  addi r5, r10, -0x2e6c
	ctx.r[5].s64 = ctx.r[10].s64 + -11884;
	// 825F6838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F683C: 390BCB60  addi r8, r11, -0x34a0
	ctx.r[8].s64 = ctx.r[11].s64 + -13472;
	// 825F6840: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825F6844: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 825F6848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F684C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6850: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6858: 386AE424  addi r3, r10, -0x1bdc
	ctx.r[3].s64 = ctx.r[10].s64 + -7132;
	// 825F685C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F686C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F687C: 4BE705A5  bl 0x82466e20
	ctx.lr = 0x825F6880;
	sub_82466E20(ctx, base);
	// 825F6880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F688C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6890 size=108
    let mut pc: u32 = 0x825F6890;
    'dispatch: loop {
        match pc {
            0x825F6890 => {
    //   block [0x825F6890..0x825F68FC)
	// 825F6890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F689C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F68A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F68A4: 38EBCBF0  addi r7, r11, -0x3410
	ctx.r[7].s64 = ctx.r[11].s64 + -13328;
	// 825F68A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F68AC: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 825F68B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F68B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F68B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F68BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F68C0: 386AE454  addi r3, r10, -0x1bac
	ctx.r[3].s64 = ctx.r[10].s64 + -7084;
	// 825F68C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F68C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F68CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F68D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F68D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F68D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F68DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F68E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F68E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F68E8: 4BE70539  bl 0x82466e20
	ctx.lr = 0x825F68EC;
	sub_82466E20(ctx, base);
	// 825F68EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F68F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F68F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F68F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6900 size=108
    let mut pc: u32 = 0x825F6900;
    'dispatch: loop {
        match pc {
            0x825F6900 => {
    //   block [0x825F6900..0x825F696C)
	// 825F6900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F690C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6914: 38EBCC38  addi r7, r11, -0x33c8
	ctx.r[7].s64 = ctx.r[11].s64 + -13256;
	// 825F6918: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F691C: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 825F6920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6924: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6928: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F692C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6930: 386AE484  addi r3, r10, -0x1b7c
	ctx.r[3].s64 = ctx.r[10].s64 + -7036;
	// 825F6934: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F6938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F693C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F694C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6958: 4BE704C9  bl 0x82466e20
	ctx.lr = 0x825F695C;
	sub_82466E20(ctx, base);
	// 825F695C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6970 size=108
    let mut pc: u32 = 0x825F6970;
    'dispatch: loop {
        match pc {
            0x825F6970 => {
    //   block [0x825F6970..0x825F69DC)
	// 825F6970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F697C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6984: 38EBCC68  addi r7, r11, -0x3398
	ctx.r[7].s64 = ctx.r[11].s64 + -13208;
	// 825F6988: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F698C: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 825F6990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6994: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F699C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F69A0: 386AE4B4  addi r3, r10, -0x1b4c
	ctx.r[3].s64 = ctx.r[10].s64 + -6988;
	// 825F69A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F69A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F69AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F69B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F69B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F69B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F69BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F69C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F69C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F69C8: 4BE70459  bl 0x82466e20
	ctx.lr = 0x825F69CC;
	sub_82466E20(ctx, base);
	// 825F69CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F69D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F69D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F69D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F69E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F69E0 size=112
    let mut pc: u32 = 0x825F69E0;
    'dispatch: loop {
        match pc {
            0x825F69E0 => {
    //   block [0x825F69E0..0x825F6A50)
	// 825F69E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F69E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F69E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F69EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F69F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F69F4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F69F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F69FC: 390BCC98  addi r8, r11, -0x3368
	ctx.r[8].s64 = ctx.r[11].s64 + -13160;
	// 825F6A00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F6A04: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 825F6A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6A0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6A10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6A18: 386AE4E4  addi r3, r10, -0x1b1c
	ctx.r[3].s64 = ctx.r[10].s64 + -6940;
	// 825F6A1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6A2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6A3C: 4BE703E5  bl 0x82466e20
	ctx.lr = 0x825F6A40;
	sub_82466E20(ctx, base);
	// 825F6A40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6A50 size=112
    let mut pc: u32 = 0x825F6A50;
    'dispatch: loop {
        match pc {
            0x825F6A50 => {
    //   block [0x825F6A50..0x825F6AC0)
	// 825F6A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6A5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6A60: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6A64: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F6A68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6A6C: 390BCCC8  addi r8, r11, -0x3338
	ctx.r[8].s64 = ctx.r[11].s64 + -13112;
	// 825F6A70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F6A74: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 825F6A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6A7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6A80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6A88: 386AE514  addi r3, r10, -0x1aec
	ctx.r[3].s64 = ctx.r[10].s64 + -6892;
	// 825F6A8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6A9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6AAC: 4BE70375  bl 0x82466e20
	ctx.lr = 0x825F6AB0;
	sub_82466E20(ctx, base);
	// 825F6AB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6AC0 size=112
    let mut pc: u32 = 0x825F6AC0;
    'dispatch: loop {
        match pc {
            0x825F6AC0 => {
    //   block [0x825F6AC0..0x825F6B30)
	// 825F6AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6ACC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6AD0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6AD4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F6AD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6ADC: 390BCCE0  addi r8, r11, -0x3320
	ctx.r[8].s64 = ctx.r[11].s64 + -13088;
	// 825F6AE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F6AE4: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 825F6AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6AEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6AF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6AF8: 386AE544  addi r3, r10, -0x1abc
	ctx.r[3].s64 = ctx.r[10].s64 + -6844;
	// 825F6AFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6B0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6B1C: 4BE70305  bl 0x82466e20
	ctx.lr = 0x825F6B20;
	sub_82466E20(ctx, base);
	// 825F6B20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6B30 size=108
    let mut pc: u32 = 0x825F6B30;
    'dispatch: loop {
        match pc {
            0x825F6B30 => {
    //   block [0x825F6B30..0x825F6B9C)
	// 825F6B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6B3C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6B40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6B44: 38EBCCF8  addi r7, r11, -0x3308
	ctx.r[7].s64 = ctx.r[11].s64 + -13064;
	// 825F6B48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F6B4C: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 825F6B50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6B54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6B58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F6B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6B60: 386AE574  addi r3, r10, -0x1a8c
	ctx.r[3].s64 = ctx.r[10].s64 + -6796;
	// 825F6B64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F6B68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6B70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6B78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6B80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6B84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6B88: 4BE70299  bl 0x82466e20
	ctx.lr = 0x825F6B8C;
	sub_82466E20(ctx, base);
	// 825F6B8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6BA0 size=112
    let mut pc: u32 = 0x825F6BA0;
    'dispatch: loop {
        match pc {
            0x825F6BA0 => {
    //   block [0x825F6BA0..0x825F6C10)
	// 825F6BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6BAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6BB0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6BB4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F6BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6BBC: 390BCD28  addi r8, r11, -0x32d8
	ctx.r[8].s64 = ctx.r[11].s64 + -13016;
	// 825F6BC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F6BC4: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 825F6BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6BCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6BD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6BD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6BD8: 386AE5A4  addi r3, r10, -0x1a5c
	ctx.r[3].s64 = ctx.r[10].s64 + -6748;
	// 825F6BDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6BEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6BFC: 4BE70225  bl 0x82466e20
	ctx.lr = 0x825F6C00;
	sub_82466E20(ctx, base);
	// 825F6C00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6C10 size=108
    let mut pc: u32 = 0x825F6C10;
    'dispatch: loop {
        match pc {
            0x825F6C10 => {
    //   block [0x825F6C10..0x825F6C7C)
	// 825F6C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6C1C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6C24: 38EBCD40  addi r7, r11, -0x32c0
	ctx.r[7].s64 = ctx.r[11].s64 + -12992;
	// 825F6C28: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 825F6C2C: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 825F6C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6C34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6C38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F6C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6C40: 386AE5D4  addi r3, r10, -0x1a2c
	ctx.r[3].s64 = ctx.r[10].s64 + -6700;
	// 825F6C44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F6C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6C64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6C68: 4BE701B9  bl 0x82466e20
	ctx.lr = 0x825F6C6C;
	sub_82466E20(ctx, base);
	// 825F6C6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6C80 size=112
    let mut pc: u32 = 0x825F6C80;
    'dispatch: loop {
        match pc {
            0x825F6C80 => {
    //   block [0x825F6C80..0x825F6CF0)
	// 825F6C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6C8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6C90: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6C94: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F6C98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6C9C: 390BCE30  addi r8, r11, -0x31d0
	ctx.r[8].s64 = ctx.r[11].s64 + -12752;
	// 825F6CA0: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 825F6CA4: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 825F6CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6CAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6CB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6CB8: 386AE604  addi r3, r10, -0x19fc
	ctx.r[3].s64 = ctx.r[10].s64 + -6652;
	// 825F6CBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6CC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6CC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6CC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6CCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6CD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6CD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6CDC: 4BE70145  bl 0x82466e20
	ctx.lr = 0x825F6CE0;
	sub_82466E20(ctx, base);
	// 825F6CE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6CF0 size=108
    let mut pc: u32 = 0x825F6CF0;
    'dispatch: loop {
        match pc {
            0x825F6CF0 => {
    //   block [0x825F6CF0..0x825F6D5C)
	// 825F6CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6CFC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6D04: 38EBCFE0  addi r7, r11, -0x3020
	ctx.r[7].s64 = ctx.r[11].s64 + -12320;
	// 825F6D08: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 825F6D0C: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 825F6D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6D14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6D18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F6D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6D20: 386AE634  addi r3, r10, -0x19cc
	ctx.r[3].s64 = ctx.r[10].s64 + -6604;
	// 825F6D24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F6D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6D2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6D34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6D44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6D48: 4BE700D9  bl 0x82466e20
	ctx.lr = 0x825F6D4C;
	sub_82466E20(ctx, base);
	// 825F6D4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6D60 size=112
    let mut pc: u32 = 0x825F6D60;
    'dispatch: loop {
        match pc {
            0x825F6D60 => {
    //   block [0x825F6D60..0x825F6DD0)
	// 825F6D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6D68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6D6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6D70: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6D74: 38AAD194  addi r5, r10, -0x2e6c
	ctx.r[5].s64 = ctx.r[10].s64 + -11884;
	// 825F6D78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6D7C: 390BD178  addi r8, r11, -0x2e88
	ctx.r[8].s64 = ctx.r[11].s64 + -11912;
	// 825F6D80: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 825F6D84: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 825F6D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6D8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6D90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6D98: 386AE664  addi r3, r10, -0x199c
	ctx.r[3].s64 = ctx.r[10].s64 + -6556;
	// 825F6D9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6DA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6DB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6DBC: 4BE70065  bl 0x82466e20
	ctx.lr = 0x825F6DC0;
	sub_82466E20(ctx, base);
	// 825F6DC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6DD0 size=100
    let mut pc: u32 = 0x825F6DD0;
    'dispatch: loop {
        match pc {
            0x825F6DD0 => {
    //   block [0x825F6DD0..0x825F6E34)
	// 825F6DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6DDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6DE4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F6DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6DF0: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 825F6DF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6E04: 386AE694  addi r3, r10, -0x196c
	ctx.r[3].s64 = ctx.r[10].s64 + -6508;
	// 825F6E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6E0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6E10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F6E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6E18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F6E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6E20: 4BE70001  bl 0x82466e20
	ctx.lr = 0x825F6E24;
	sub_82466E20(ctx, base);
	// 825F6E24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6E38 size=112
    let mut pc: u32 = 0x825F6E38;
    'dispatch: loop {
        match pc {
            0x825F6E38 => {
    //   block [0x825F6E38..0x825F6EA8)
	// 825F6E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6E44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6E48: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6E4C: 38AAE694  addi r5, r10, -0x196c
	ctx.r[5].s64 = ctx.r[10].s64 + -6508;
	// 825F6E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6E54: 390BD3D0  addi r8, r11, -0x2c30
	ctx.r[8].s64 = ctx.r[11].s64 + -11312;
	// 825F6E58: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825F6E5C: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 825F6E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6E64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6E68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6E70: 386AE6C4  addi r3, r10, -0x193c
	ctx.r[3].s64 = ctx.r[10].s64 + -6460;
	// 825F6E74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6E78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6E80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6E88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6E90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6E94: 4BE6FF8D  bl 0x82466e20
	ctx.lr = 0x825F6E98;
	sub_82466E20(ctx, base);
	// 825F6E98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6EA8 size=100
    let mut pc: u32 = 0x825F6EA8;
    'dispatch: loop {
        match pc {
            0x825F6EA8 => {
    //   block [0x825F6EA8..0x825F6F0C)
	// 825F6EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6EB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6EBC: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F6EC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6EC8: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 825F6ECC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6ED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6EDC: 386AE6F4  addi r3, r10, -0x190c
	ctx.r[3].s64 = ctx.r[10].s64 + -6412;
	// 825F6EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6EE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6EE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F6EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6EF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F6EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6EF8: 4BE6FF29  bl 0x82466e20
	ctx.lr = 0x825F6EFC;
	sub_82466E20(ctx, base);
	// 825F6EFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6F10 size=108
    let mut pc: u32 = 0x825F6F10;
    'dispatch: loop {
        match pc {
            0x825F6F10 => {
    //   block [0x825F6F10..0x825F6F7C)
	// 825F6F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6F1C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6F24: 38EBD448  addi r7, r11, -0x2bb8
	ctx.r[7].s64 = ctx.r[11].s64 + -11192;
	// 825F6F28: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F6F2C: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 825F6F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6F34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6F38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F6F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6F40: 386AE724  addi r3, r10, -0x18dc
	ctx.r[3].s64 = ctx.r[10].s64 + -6364;
	// 825F6F44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F6F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6F64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6F68: 4BE6FEB9  bl 0x82466e20
	ctx.lr = 0x825F6F6C;
	sub_82466E20(ctx, base);
	// 825F6F6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6F80 size=112
    let mut pc: u32 = 0x825F6F80;
    'dispatch: loop {
        match pc {
            0x825F6F80 => {
    //   block [0x825F6F80..0x825F6FF0)
	// 825F6F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6F8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6F90: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6F94: 38AAE6F4  addi r5, r10, -0x190c
	ctx.r[5].s64 = ctx.r[10].s64 + -6412;
	// 825F6F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6F9C: 390BD490  addi r8, r11, -0x2b70
	ctx.r[8].s64 = ctx.r[11].s64 + -11120;
	// 825F6FA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F6FA4: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 825F6FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6FAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6FB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6FB8: 386AE754  addi r3, r10, -0x18ac
	ctx.r[3].s64 = ctx.r[10].s64 + -6316;
	// 825F6FBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6FC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6FDC: 4BE6FE45  bl 0x82466e20
	ctx.lr = 0x825F6FE0;
	sub_82466E20(ctx, base);
	// 825F6FE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6FF0 size=100
    let mut pc: u32 = 0x825F6FF0;
    'dispatch: loop {
        match pc {
            0x825F6FF0 => {
    //   block [0x825F6FF0..0x825F7054)
	// 825F6FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6FFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7004: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F7008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F700C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7010: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 825F7014: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F701C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7024: 386AE784  addi r3, r10, -0x187c
	ctx.r[3].s64 = ctx.r[10].s64 + -6268;
	// 825F7028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F702C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7030: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F7034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7038: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F703C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7040: 4BE6FDE1  bl 0x82466e20
	ctx.lr = 0x825F7044;
	sub_82466E20(ctx, base);
	// 825F7044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F704C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7058 size=100
    let mut pc: u32 = 0x825F7058;
    'dispatch: loop {
        match pc {
            0x825F7058 => {
    //   block [0x825F7058..0x825F70BC)
	// 825F7058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F705C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7064: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F706C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F7070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7078: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 825F707C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F708C: 386AE7B4  addi r3, r10, -0x184c
	ctx.r[3].s64 = ctx.r[10].s64 + -6220;
	// 825F7090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7094: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7098: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F709C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F70A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F70A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F70A8: 4BE6FD79  bl 0x82466e20
	ctx.lr = 0x825F70AC;
	sub_82466E20(ctx, base);
	// 825F70AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F70B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F70B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F70B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F70C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F70C0 size=112
    let mut pc: u32 = 0x825F70C0;
    'dispatch: loop {
        match pc {
            0x825F70C0 => {
    //   block [0x825F70C0..0x825F7130)
	// 825F70C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F70C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F70C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F70CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F70D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F70D4: 38AAE784  addi r5, r10, -0x187c
	ctx.r[5].s64 = ctx.r[10].s64 + -6268;
	// 825F70D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F70DC: 390BD4C0  addi r8, r11, -0x2b40
	ctx.r[8].s64 = ctx.r[11].s64 + -11072;
	// 825F70E0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F70E4: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 825F70E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F70EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F70F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F70F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F70F8: 386AE7E4  addi r3, r10, -0x181c
	ctx.r[3].s64 = ctx.r[10].s64 + -6172;
	// 825F70FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F710C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F711C: 4BE6FD05  bl 0x82466e20
	ctx.lr = 0x825F7120;
	sub_82466E20(ctx, base);
	// 825F7120: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F712C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7130 size=112
    let mut pc: u32 = 0x825F7130;
    'dispatch: loop {
        match pc {
            0x825F7130 => {
    //   block [0x825F7130..0x825F71A0)
	// 825F7130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F713C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7140: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7144: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 825F7148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F714C: 390BD520  addi r8, r11, -0x2ae0
	ctx.r[8].s64 = ctx.r[11].s64 + -10976;
	// 825F7150: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F7154: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 825F7158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F715C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7168: 386AE814  addi r3, r10, -0x17ec
	ctx.r[3].s64 = ctx.r[10].s64 + -6124;
	// 825F716C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F717C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F718C: 4BE6FC95  bl 0x82466e20
	ctx.lr = 0x825F7190;
	sub_82466E20(ctx, base);
	// 825F7190: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F719C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F71A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F71A0 size=100
    let mut pc: u32 = 0x825F71A0;
    'dispatch: loop {
        match pc {
            0x825F71A0 => {
    //   block [0x825F71A0..0x825F7204)
	// 825F71A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F71A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F71A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F71AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F71B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F71B4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F71B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F71BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F71C0: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 825F71C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F71C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F71CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F71D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F71D4: 386AE844  addi r3, r10, -0x17bc
	ctx.r[3].s64 = ctx.r[10].s64 + -6076;
	// 825F71D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F71DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F71E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F71E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F71E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F71EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F71F0: 4BE6FC31  bl 0x82466e20
	ctx.lr = 0x825F71F4;
	sub_82466E20(ctx, base);
	// 825F71F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F71F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F71FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7208 size=112
    let mut pc: u32 = 0x825F7208;
    'dispatch: loop {
        match pc {
            0x825F7208 => {
    //   block [0x825F7208..0x825F7278)
	// 825F7208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F720C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7214: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7218: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F721C: 38AAE844  addi r5, r10, -0x17bc
	ctx.r[5].s64 = ctx.r[10].s64 + -6076;
	// 825F7220: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7224: 390BD580  addi r8, r11, -0x2a80
	ctx.r[8].s64 = ctx.r[11].s64 + -10880;
	// 825F7228: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 825F722C: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 825F7230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7234: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7238: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F723C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7240: 386AE874  addi r3, r10, -0x178c
	ctx.r[3].s64 = ctx.r[10].s64 + -6028;
	// 825F7244: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F724C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F725C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7264: 4BE6FBBD  bl 0x82466e20
	ctx.lr = 0x825F7268;
	sub_82466E20(ctx, base);
	// 825F7268: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F726C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7278 size=108
    let mut pc: u32 = 0x825F7278;
    'dispatch: loop {
        match pc {
            0x825F7278 => {
    //   block [0x825F7278..0x825F72E4)
	// 825F7278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F727C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7284: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7288: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F728C: 38EBD670  addi r7, r11, -0x2990
	ctx.r[7].s64 = ctx.r[11].s64 + -10640;
	// 825F7290: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 825F7294: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 825F7298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F729C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F72A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F72A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F72A8: 386AE8A4  addi r3, r10, -0x175c
	ctx.r[3].s64 = ctx.r[10].s64 + -5980;
	// 825F72AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F72B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F72B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F72B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F72BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F72C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F72C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F72C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F72CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F72D0: 4BE6FB51  bl 0x82466e20
	ctx.lr = 0x825F72D4;
	sub_82466E20(ctx, base);
	// 825F72D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F72D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F72DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F72E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F72E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F72E8 size=108
    let mut pc: u32 = 0x825F72E8;
    'dispatch: loop {
        match pc {
            0x825F72E8 => {
    //   block [0x825F72E8..0x825F7354)
	// 825F72E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F72EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F72F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F72F4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F72F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F72FC: 38EBD760  addi r7, r11, -0x28a0
	ctx.r[7].s64 = ctx.r[11].s64 + -10400;
	// 825F7300: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F7304: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 825F7308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F730C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7310: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F7314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7318: 386AE8D4  addi r3, r10, -0x172c
	ctx.r[3].s64 = ctx.r[10].s64 + -5932;
	// 825F731C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F732C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F733C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7340: 4BE6FAE1  bl 0x82466e20
	ctx.lr = 0x825F7344;
	sub_82466E20(ctx, base);
	// 825F7344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F734C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7358 size=108
    let mut pc: u32 = 0x825F7358;
    'dispatch: loop {
        match pc {
            0x825F7358 => {
    //   block [0x825F7358..0x825F73C4)
	// 825F7358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F735C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7364: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7368: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F736C: 38EBD7A8  addi r7, r11, -0x2858
	ctx.r[7].s64 = ctx.r[11].s64 + -10328;
	// 825F7370: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825F7374: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 825F7378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F737C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7380: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F7384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7388: 386AE904  addi r3, r10, -0x16fc
	ctx.r[3].s64 = ctx.r[10].s64 + -5884;
	// 825F738C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F739C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F73A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F73A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F73A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F73AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F73B0: 4BE6FA71  bl 0x82466e20
	ctx.lr = 0x825F73B4;
	sub_82466E20(ctx, base);
	// 825F73B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F73B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F73BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F73C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F73C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F73C8 size=108
    let mut pc: u32 = 0x825F73C8;
    'dispatch: loop {
        match pc {
            0x825F73C8 => {
    //   block [0x825F73C8..0x825F7434)
	// 825F73C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F73CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F73D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F73D4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F73D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F73DC: 38EBD880  addi r7, r11, -0x2780
	ctx.r[7].s64 = ctx.r[11].s64 + -10112;
	// 825F73E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F73E4: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 825F73E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F73EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F73F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F73F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F73F8: 386AE934  addi r3, r10, -0x16cc
	ctx.r[3].s64 = ctx.r[10].s64 + -5836;
	// 825F73FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F740C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F741C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7420: 4BE6FA01  bl 0x82466e20
	ctx.lr = 0x825F7424;
	sub_82466E20(ctx, base);
	// 825F7424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F742C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7438 size=100
    let mut pc: u32 = 0x825F7438;
    'dispatch: loop {
        match pc {
            0x825F7438 => {
    //   block [0x825F7438..0x825F749C)
	// 825F7438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F743C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7444: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F744C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F7450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7458: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 825F745C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F746C: 386AE964  addi r3, r10, -0x169c
	ctx.r[3].s64 = ctx.r[10].s64 + -5788;
	// 825F7470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7474: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7478: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F747C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7480: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F7484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7488: 4BE6F999  bl 0x82466e20
	ctx.lr = 0x825F748C;
	sub_82466E20(ctx, base);
	// 825F748C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F74A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F74A0 size=112
    let mut pc: u32 = 0x825F74A0;
    'dispatch: loop {
        match pc {
            0x825F74A0 => {
    //   block [0x825F74A0..0x825F7510)
	// 825F74A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F74A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F74A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F74AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F74B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F74B4: 38AAE964  addi r5, r10, -0x169c
	ctx.r[5].s64 = ctx.r[10].s64 + -5788;
	// 825F74B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F74BC: 390BD898  addi r8, r11, -0x2768
	ctx.r[8].s64 = ctx.r[11].s64 + -10088;
	// 825F74C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F74C4: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 825F74C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F74CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F74D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F74D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F74D8: 386AE994  addi r3, r10, -0x166c
	ctx.r[3].s64 = ctx.r[10].s64 + -5740;
	// 825F74DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F74E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F74E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F74E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F74EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F74F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F74F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F74F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F74FC: 4BE6F925  bl 0x82466e20
	ctx.lr = 0x825F7500;
	sub_82466E20(ctx, base);
	// 825F7500: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F750C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7510 size=108
    let mut pc: u32 = 0x825F7510;
    'dispatch: loop {
        match pc {
            0x825F7510 => {
    //   block [0x825F7510..0x825F757C)
	// 825F7510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F751C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7524: 38EBD8E0  addi r7, r11, -0x2720
	ctx.r[7].s64 = ctx.r[11].s64 + -10016;
	// 825F7528: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F752C: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 825F7530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7534: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F753C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7540: 386AE9C4  addi r3, r10, -0x163c
	ctx.r[3].s64 = ctx.r[10].s64 + -5692;
	// 825F7544: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F754C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F755C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7564: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7568: 4BE6F8B9  bl 0x82466e20
	ctx.lr = 0x825F756C;
	sub_82466E20(ctx, base);
	// 825F756C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7580 size=112
    let mut pc: u32 = 0x825F7580;
    'dispatch: loop {
        match pc {
            0x825F7580 => {
    //   block [0x825F7580..0x825F75F0)
	// 825F7580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F758C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7590: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7594: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F7598: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F759C: 390BD928  addi r8, r11, -0x26d8
	ctx.r[8].s64 = ctx.r[11].s64 + -9944;
	// 825F75A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F75A4: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 825F75A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F75AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F75B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F75B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F75B8: 386AE9F4  addi r3, r10, -0x160c
	ctx.r[3].s64 = ctx.r[10].s64 + -5644;
	// 825F75BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F75C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F75C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F75C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F75CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F75D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F75D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F75D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F75DC: 4BE6F845  bl 0x82466e20
	ctx.lr = 0x825F75E0;
	sub_82466E20(ctx, base);
	// 825F75E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F75E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F75E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F75EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F75F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F75F0 size=108
    let mut pc: u32 = 0x825F75F0;
    'dispatch: loop {
        match pc {
            0x825F75F0 => {
    //   block [0x825F75F0..0x825F765C)
	// 825F75F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F75F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F75F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F75FC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7604: 38EBD940  addi r7, r11, -0x26c0
	ctx.r[7].s64 = ctx.r[11].s64 + -9920;
	// 825F7608: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F760C: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 825F7610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7614: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7618: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F761C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7620: 386AEA24  addi r3, r10, -0x15dc
	ctx.r[3].s64 = ctx.r[10].s64 + -5596;
	// 825F7624: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F762C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F763C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7648: 4BE6F7D9  bl 0x82466e20
	ctx.lr = 0x825F764C;
	sub_82466E20(ctx, base);
	// 825F764C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7660 size=112
    let mut pc: u32 = 0x825F7660;
    'dispatch: loop {
        match pc {
            0x825F7660 => {
    //   block [0x825F7660..0x825F76D0)
	// 825F7660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F766C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7670: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7674: 38AAE9F4  addi r5, r10, -0x160c
	ctx.r[5].s64 = ctx.r[10].s64 + -5644;
	// 825F7678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F767C: 390BD988  addi r8, r11, -0x2678
	ctx.r[8].s64 = ctx.r[11].s64 + -9848;
	// 825F7680: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F7684: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 825F7688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F768C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7690: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7698: 386AEA54  addi r3, r10, -0x15ac
	ctx.r[3].s64 = ctx.r[10].s64 + -5548;
	// 825F769C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F76A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F76A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F76A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F76AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F76B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F76B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F76B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F76BC: 4BE6F765  bl 0x82466e20
	ctx.lr = 0x825F76C0;
	sub_82466E20(ctx, base);
	// 825F76C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F76C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F76C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F76CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F76D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F76D0 size=100
    let mut pc: u32 = 0x825F76D0;
    'dispatch: loop {
        match pc {
            0x825F76D0 => {
    //   block [0x825F76D0..0x825F7734)
	// 825F76D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F76D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F76D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F76DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F76E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F76E4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F76E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F76EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F76F0: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 825F76F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F76F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F76FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7704: 386AEA84  addi r3, r10, -0x157c
	ctx.r[3].s64 = ctx.r[10].s64 + -5500;
	// 825F7708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F770C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7710: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F7714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7718: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F771C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7720: 4BE6F701  bl 0x82466e20
	ctx.lr = 0x825F7724;
	sub_82466E20(ctx, base);
	// 825F7724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F772C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7738 size=112
    let mut pc: u32 = 0x825F7738;
    'dispatch: loop {
        match pc {
            0x825F7738 => {
    //   block [0x825F7738..0x825F77A8)
	// 825F7738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F773C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7744: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7748: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F774C: 38AAEA84  addi r5, r10, -0x157c
	ctx.r[5].s64 = ctx.r[10].s64 + -5500;
	// 825F7750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7754: 390BD9A0  addi r8, r11, -0x2660
	ctx.r[8].s64 = ctx.r[11].s64 + -9824;
	// 825F7758: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825F775C: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 825F7760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7764: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7768: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F776C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7770: 386AEAB4  addi r3, r10, -0x154c
	ctx.r[3].s64 = ctx.r[10].s64 + -5452;
	// 825F7774: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F777C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F778C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7794: 4BE6F68D  bl 0x82466e20
	ctx.lr = 0x825F7798;
	sub_82466E20(ctx, base);
	// 825F7798: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F779C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F77A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F77A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F77A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F77A8 size=108
    let mut pc: u32 = 0x825F77A8;
    'dispatch: loop {
        match pc {
            0x825F77A8 => {
    //   block [0x825F77A8..0x825F7814)
	// 825F77A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F77AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F77B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F77B4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F77B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F77BC: 38EBDA48  addi r7, r11, -0x25b8
	ctx.r[7].s64 = ctx.r[11].s64 + -9656;
	// 825F77C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F77C4: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 825F77C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F77CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F77D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F77D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F77D8: 386AEAE4  addi r3, r10, -0x151c
	ctx.r[3].s64 = ctx.r[10].s64 + -5404;
	// 825F77DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F77E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F77E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F77E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F77EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F77F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F77F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F77F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F77FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7800: 4BE6F621  bl 0x82466e20
	ctx.lr = 0x825F7804;
	sub_82466E20(ctx, base);
	// 825F7804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F780C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7818 size=112
    let mut pc: u32 = 0x825F7818;
    'dispatch: loop {
        match pc {
            0x825F7818 => {
    //   block [0x825F7818..0x825F7888)
	// 825F7818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F781C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7824: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7828: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F782C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F7830: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7834: 390BDA78  addi r8, r11, -0x2588
	ctx.r[8].s64 = ctx.r[11].s64 + -9608;
	// 825F7838: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F783C: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 825F7840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7844: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7848: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F784C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7850: 386AEB14  addi r3, r10, -0x14ec
	ctx.r[3].s64 = ctx.r[10].s64 + -5356;
	// 825F7854: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F785C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F786C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7874: 4BE6F5AD  bl 0x82466e20
	ctx.lr = 0x825F7878;
	sub_82466E20(ctx, base);
	// 825F7878: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F787C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7888 size=112
    let mut pc: u32 = 0x825F7888;
    'dispatch: loop {
        match pc {
            0x825F7888 => {
    //   block [0x825F7888..0x825F78F8)
	// 825F7888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F788C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7894: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7898: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F789C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F78A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F78A4: 390BDAC0  addi r8, r11, -0x2540
	ctx.r[8].s64 = ctx.r[11].s64 + -9536;
	// 825F78A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F78AC: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 825F78B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F78B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F78B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F78BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F78C0: 386AEB44  addi r3, r10, -0x14bc
	ctx.r[3].s64 = ctx.r[10].s64 + -5308;
	// 825F78C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F78C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F78CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F78D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F78D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F78D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F78DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F78E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F78E4: 4BE6F53D  bl 0x82466e20
	ctx.lr = 0x825F78E8;
	sub_82466E20(ctx, base);
	// 825F78E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F78EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F78F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F78F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F78F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F78F8 size=100
    let mut pc: u32 = 0x825F78F8;
    'dispatch: loop {
        match pc {
            0x825F78F8 => {
    //   block [0x825F78F8..0x825F795C)
	// 825F78F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F78FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7904: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F790C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F7910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7918: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 825F791C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F792C: 386AEB74  addi r3, r10, -0x148c
	ctx.r[3].s64 = ctx.r[10].s64 + -5260;
	// 825F7930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7934: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7938: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F793C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7940: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F7944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7948: 4BE6F4D9  bl 0x82466e20
	ctx.lr = 0x825F794C;
	sub_82466E20(ctx, base);
	// 825F794C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7960 size=112
    let mut pc: u32 = 0x825F7960;
    'dispatch: loop {
        match pc {
            0x825F7960 => {
    //   block [0x825F7960..0x825F79D0)
	// 825F7960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F796C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7970: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7974: 38AAEB74  addi r5, r10, -0x148c
	ctx.r[5].s64 = ctx.r[10].s64 + -5260;
	// 825F7978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F797C: 390BDB08  addi r8, r11, -0x24f8
	ctx.r[8].s64 = ctx.r[11].s64 + -9464;
	// 825F7980: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F7984: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 825F7988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F798C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7990: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7998: 386AEBA4  addi r3, r10, -0x145c
	ctx.r[3].s64 = ctx.r[10].s64 + -5212;
	// 825F799C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F79A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F79A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F79A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F79AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F79B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F79B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F79B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F79BC: 4BE6F465  bl 0x82466e20
	ctx.lr = 0x825F79C0;
	sub_82466E20(ctx, base);
	// 825F79C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F79C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F79C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F79CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F79D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F79D0 size=112
    let mut pc: u32 = 0x825F79D0;
    'dispatch: loop {
        match pc {
            0x825F79D0 => {
    //   block [0x825F79D0..0x825F7A40)
	// 825F79D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F79D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F79D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F79DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F79E0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F79E4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F79E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F79EC: 390BDB50  addi r8, r11, -0x24b0
	ctx.r[8].s64 = ctx.r[11].s64 + -9392;
	// 825F79F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F79F4: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 825F79F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F79FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7A00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7A08: 386AEBD4  addi r3, r10, -0x142c
	ctx.r[3].s64 = ctx.r[10].s64 + -5164;
	// 825F7A0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7A2C: 4BE6F3F5  bl 0x82466e20
	ctx.lr = 0x825F7A30;
	sub_82466E20(ctx, base);
	// 825F7A30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7A40 size=112
    let mut pc: u32 = 0x825F7A40;
    'dispatch: loop {
        match pc {
            0x825F7A40 => {
    //   block [0x825F7A40..0x825F7AB0)
	// 825F7A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7A4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7A50: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7A54: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F7A58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7A5C: 390BDB68  addi r8, r11, -0x2498
	ctx.r[8].s64 = ctx.r[11].s64 + -9368;
	// 825F7A60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F7A64: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 825F7A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7A6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7A70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7A78: 386AEC04  addi r3, r10, -0x13fc
	ctx.r[3].s64 = ctx.r[10].s64 + -5116;
	// 825F7A7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7A80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7A88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7A8C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F7A90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7A98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7A9C: 4BE6F385  bl 0x82466e20
	ctx.lr = 0x825F7AA0;
	sub_82466E20(ctx, base);
	// 825F7AA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7AB0 size=112
    let mut pc: u32 = 0x825F7AB0;
    'dispatch: loop {
        match pc {
            0x825F7AB0 => {
    //   block [0x825F7AB0..0x825F7B20)
	// 825F7AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7ABC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7AC0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7AC4: 38AAEBD4  addi r5, r10, -0x142c
	ctx.r[5].s64 = ctx.r[10].s64 + -5164;
	// 825F7AC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7ACC: 390BDB80  addi r8, r11, -0x2480
	ctx.r[8].s64 = ctx.r[11].s64 + -9344;
	// 825F7AD0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F7AD4: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 825F7AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7ADC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7AE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7AE8: 386AEC34  addi r3, r10, -0x13cc
	ctx.r[3].s64 = ctx.r[10].s64 + -5068;
	// 825F7AEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7B0C: 4BE6F315  bl 0x82466e20
	ctx.lr = 0x825F7B10;
	sub_82466E20(ctx, base);
	// 825F7B10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7B20 size=72
    let mut pc: u32 = 0x825F7B20;
    'dispatch: loop {
        match pc {
            0x825F7B20 => {
    //   block [0x825F7B20..0x825F7B68)
	// 825F7B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7B28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7B2C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F7B30: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 825F7B34: 38CB0DA8  addi r6, r11, 0xda8
	ctx.r[6].s64 = ctx.r[11].s64 + 3496;
	// 825F7B38: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F7B3C: 388B2458  addi r4, r11, 0x2458
	ctx.r[4].s64 = ctx.r[11].s64 + 9304;
	// 825F7B40: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F7B44: 386BEC64  addi r3, r11, -0x139c
	ctx.r[3].s64 = ctx.r[11].s64 + -5020;
	// 825F7B48: 4BE83F41  bl 0x8247ba88
	ctx.lr = 0x825F7B4C;
	sub_8247BA88(ctx, base);
	// 825F7B4C: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825F7B50: 386BCCF0  addi r3, r11, -0x3310
	ctx.r[3].s64 = ctx.r[11].s64 + -13072;
	// 825F7B54: 4BF3AFE5  bl 0x82532b38
	ctx.lr = 0x825F7B58;
	sub_82532B38(ctx, base);
	// 825F7B58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825F7B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7B68 size=108
    let mut pc: u32 = 0x825F7B68;
    'dispatch: loop {
        match pc {
            0x825F7B68 => {
    //   block [0x825F7B68..0x825F7BD4)
	// 825F7B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7B74: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7B78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7B7C: 38EBF518  addi r7, r11, -0xae8
	ctx.r[7].s64 = ctx.r[11].s64 + -2792;
	// 825F7B80: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825F7B84: 388AA590  addi r4, r10, -0x5a70
	ctx.r[4].s64 = ctx.r[10].s64 + -23152;
	// 825F7B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7B8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7B90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F7B94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7B98: 386AEC7C  addi r3, r10, -0x1384
	ctx.r[3].s64 = ctx.r[10].s64 + -4996;
	// 825F7B9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7BBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7BC0: 4BE6F261  bl 0x82466e20
	ctx.lr = 0x825F7BC4;
	sub_82466E20(ctx, base);
	// 825F7BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F7BD8 size=24
    let mut pc: u32 = 0x825F7BD8;
    'dispatch: loop {
        match pc {
            0x825F7BD8 => {
    //   block [0x825F7BD8..0x825F7BF0)
	// 825F7BD8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7BDC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825F7BE0: 394A8B68  addi r10, r10, -0x7498
	ctx.r[10].s64 = ctx.r[10].s64 + -29848;
	// 825F7BE4: 816BF590  lwz r11, -0xa70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2672 as u32) ) } as u64;
	// 825F7BE8: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 825F7BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7BF0 size=112
    let mut pc: u32 = 0x825F7BF0;
    'dispatch: loop {
        match pc {
            0x825F7BF0 => {
    //   block [0x825F7BF0..0x825F7C60)
	// 825F7BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7BFC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7C00: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825F7C04: 392A7C04  addi r9, r10, 0x7c04
	ctx.r[9].s64 = ctx.r[10].s64 + 31748;
	// 825F7C08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7C0C: 390B8B68  addi r8, r11, -0x7498
	ctx.r[8].s64 = ctx.r[11].s64 + -29848;
	// 825F7C10: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825F7C14: 388AA5A8  addi r4, r10, -0x5a58
	ctx.r[4].s64 = ctx.r[10].s64 + -23128;
	// 825F7C18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7C1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7C20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7C28: 386AECAC  addi r3, r10, -0x1354
	ctx.r[3].s64 = ctx.r[10].s64 + -4948;
	// 825F7C2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F7C30: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F7C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7C44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7C4C: 4BE6F1D5  bl 0x82466e20
	ctx.lr = 0x825F7C50;
	sub_82466E20(ctx, base);
	// 825F7C50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7C60 size=108
    let mut pc: u32 = 0x825F7C60;
    'dispatch: loop {
        match pc {
            0x825F7C60 => {
    //   block [0x825F7C60..0x825F7CCC)
	// 825F7C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7C68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7C6C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7C70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7C74: 38EBF594  addi r7, r11, -0xa6c
	ctx.r[7].s64 = ctx.r[11].s64 + -2668;
	// 825F7C78: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F7C7C: 388AA5BC  addi r4, r10, -0x5a44
	ctx.r[4].s64 = ctx.r[10].s64 + -23108;
	// 825F7C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7C84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7C88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F7C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7C90: 386AECDC  addi r3, r10, -0x1324
	ctx.r[3].s64 = ctx.r[10].s64 + -4900;
	// 825F7C94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7C98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7CB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7CB8: 4BE6F169  bl 0x82466e20
	ctx.lr = 0x825F7CBC;
	sub_82466E20(ctx, base);
	// 825F7CBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7CD0 size=108
    let mut pc: u32 = 0x825F7CD0;
    'dispatch: loop {
        match pc {
            0x825F7CD0 => {
    //   block [0x825F7CD0..0x825F7D3C)
	// 825F7CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7CDC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7CE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7CE4: 38EBF5C4  addi r7, r11, -0xa3c
	ctx.r[7].s64 = ctx.r[11].s64 + -2620;
	// 825F7CE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F7CEC: 388AA5DC  addi r4, r10, -0x5a24
	ctx.r[4].s64 = ctx.r[10].s64 + -23076;
	// 825F7CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7CF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7CF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F7CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7D00: 386AED0C  addi r3, r10, -0x12f4
	ctx.r[3].s64 = ctx.r[10].s64 + -4852;
	// 825F7D04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7D24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7D28: 4BE6F0F9  bl 0x82466e20
	ctx.lr = 0x825F7D2C;
	sub_82466E20(ctx, base);
	// 825F7D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F7D40 size=24
    let mut pc: u32 = 0x825F7D40;
    'dispatch: loop {
        match pc {
            0x825F7D40 => {
    //   block [0x825F7D40..0x825F7D58)
	// 825F7D40: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7D44: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825F7D48: 394A8BB0  addi r10, r10, -0x7450
	ctx.r[10].s64 = ctx.r[10].s64 + -29776;
	// 825F7D4C: 816BF5F4  lwz r11, -0xa0c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2572 as u32) ) } as u64;
	// 825F7D50: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825F7D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7D58 size=116
    let mut pc: u32 = 0x825F7D58;
    'dispatch: loop {
        match pc {
            0x825F7D58 => {
    //   block [0x825F7D58..0x825F7DCC)
	// 825F7D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7D64: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825F7D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7D6C: 390B8BB0  addi r8, r11, -0x7450
	ctx.r[8].s64 = ctx.r[11].s64 + -29776;
	// 825F7D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7D74: 392A7C40  addi r9, r10, 0x7c40
	ctx.r[9].s64 = ctx.r[10].s64 + 31808;
	// 825F7D78: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7D7C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825F7D80: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F7D84: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7D88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7D8C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7D90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7D98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7D9C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F7DA0: 388AA5F0  addi r4, r10, -0x5a10
	ctx.r[4].s64 = ctx.r[10].s64 + -23056;
	// 825F7DA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F7DA8: 386BED3C  addi r3, r11, -0x12c4
	ctx.r[3].s64 = ctx.r[11].s64 + -4804;
	// 825F7DAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F7DB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7DB8: 4BE6F069  bl 0x82466e20
	ctx.lr = 0x825F7DBC;
	sub_82466E20(ctx, base);
	// 825F7DBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7DD0 size=108
    let mut pc: u32 = 0x825F7DD0;
    'dispatch: loop {
        match pc {
            0x825F7DD0 => {
    //   block [0x825F7DD0..0x825F7E3C)
	// 825F7DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7DDC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7DE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7DE4: 38EBF5F8  addi r7, r11, -0xa08
	ctx.r[7].s64 = ctx.r[11].s64 + -2568;
	// 825F7DE8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825F7DEC: 388AA608  addi r4, r10, -0x59f8
	ctx.r[4].s64 = ctx.r[10].s64 + -23032;
	// 825F7DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7DF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7DF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F7DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7E00: 386AED6C  addi r3, r10, -0x1294
	ctx.r[3].s64 = ctx.r[10].s64 + -4756;
	// 825F7E04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7E24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7E28: 4BE6EFF9  bl 0x82466e20
	ctx.lr = 0x825F7E2C;
	sub_82466E20(ctx, base);
	// 825F7E2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7E40 size=112
    let mut pc: u32 = 0x825F7E40;
    'dispatch: loop {
        match pc {
            0x825F7E40 => {
    //   block [0x825F7E40..0x825F7EB0)
	// 825F7E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7E4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7E50: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7E54: 38AAED3C  addi r5, r10, -0x12c4
	ctx.r[5].s64 = ctx.r[10].s64 + -4804;
	// 825F7E58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7E5C: 390BF688  addi r8, r11, -0x978
	ctx.r[8].s64 = ctx.r[11].s64 + -2424;
	// 825F7E60: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 825F7E64: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 825F7E68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7E6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7E70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7E74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7E78: 386AED9C  addi r3, r10, -0x1264
	ctx.r[3].s64 = ctx.r[10].s64 + -4708;
	// 825F7E7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7E80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7E84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7E88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7E8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7E90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7E94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7E9C: 4BE6EF85  bl 0x82466e20
	ctx.lr = 0x825F7EA0;
	sub_82466E20(ctx, base);
	// 825F7EA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7EB0 size=112
    let mut pc: u32 = 0x825F7EB0;
    'dispatch: loop {
        match pc {
            0x825F7EB0 => {
    //   block [0x825F7EB0..0x825F7F20)
	// 825F7EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7EB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7EBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7EC0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7EC4: 38AAED3C  addi r5, r10, -0x12c4
	ctx.r[5].s64 = ctx.r[10].s64 + -4804;
	// 825F7EC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7ECC: 390BF7A8  addi r8, r11, -0x858
	ctx.r[8].s64 = ctx.r[11].s64 + -2136;
	// 825F7ED0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F7ED4: 388AA664  addi r4, r10, -0x599c
	ctx.r[4].s64 = ctx.r[10].s64 + -22940;
	// 825F7ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7EDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7EE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7EE8: 386AEDCC  addi r3, r10, -0x1234
	ctx.r[3].s64 = ctx.r[10].s64 + -4660;
	// 825F7EEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7EF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7EF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7EF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7EFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7F00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7F08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7F0C: 4BE6EF15  bl 0x82466e20
	ctx.lr = 0x825F7F10;
	sub_82466E20(ctx, base);
	// 825F7F10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7F20 size=108
    let mut pc: u32 = 0x825F7F20;
    'dispatch: loop {
        match pc {
            0x825F7F20 => {
    //   block [0x825F7F20..0x825F7F8C)
	// 825F7F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7F28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7F2C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7F30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7F34: 38EBF7C0  addi r7, r11, -0x840
	ctx.r[7].s64 = ctx.r[11].s64 + -2112;
	// 825F7F38: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825F7F3C: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 825F7F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7F44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7F48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F7F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7F50: 386AEDFC  addi r3, r10, -0x1204
	ctx.r[3].s64 = ctx.r[10].s64 + -4612;
	// 825F7F54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7F74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7F78: 4BE6EEA9  bl 0x82466e20
	ctx.lr = 0x825F7F7C;
	sub_82466E20(ctx, base);
	// 825F7F7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7F90 size=108
    let mut pc: u32 = 0x825F7F90;
    'dispatch: loop {
        match pc {
            0x825F7F90 => {
    //   block [0x825F7F90..0x825F7FFC)
	// 825F7F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7F98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7F9C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7FA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7FA4: 38EBF898  addi r7, r11, -0x768
	ctx.r[7].s64 = ctx.r[11].s64 + -1896;
	// 825F7FA8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825F7FAC: 388AA6B4  addi r4, r10, -0x594c
	ctx.r[4].s64 = ctx.r[10].s64 + -22860;
	// 825F7FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7FB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7FB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F7FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7FC0: 386AEE2C  addi r3, r10, -0x11d4
	ctx.r[3].s64 = ctx.r[10].s64 + -4564;
	// 825F7FC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7FE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7FE8: 4BE6EE39  bl 0x82466e20
	ctx.lr = 0x825F7FEC;
	sub_82466E20(ctx, base);
	// 825F7FEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8000 size=112
    let mut pc: u32 = 0x825F8000;
    'dispatch: loop {
        match pc {
            0x825F8000 => {
    //   block [0x825F8000..0x825F8070)
	// 825F8000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F800C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8010: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8014: 38AAED3C  addi r5, r10, -0x12c4
	ctx.r[5].s64 = ctx.r[10].s64 + -4804;
	// 825F8018: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F801C: 390BF928  addi r8, r11, -0x6d8
	ctx.r[8].s64 = ctx.r[11].s64 + -1752;
	// 825F8020: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 825F8024: 388AA6E4  addi r4, r10, -0x591c
	ctx.r[4].s64 = ctx.r[10].s64 + -22812;
	// 825F8028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F802C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8038: 386AEE5C  addi r3, r10, -0x11a4
	ctx.r[3].s64 = ctx.r[10].s64 + -4516;
	// 825F803C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F8040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F804C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F805C: 4BE6EDC5  bl 0x82466e20
	ctx.lr = 0x825F8060;
	sub_82466E20(ctx, base);
	// 825F8060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F806C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8070 size=108
    let mut pc: u32 = 0x825F8070;
    'dispatch: loop {
        match pc {
            0x825F8070 => {
    //   block [0x825F8070..0x825F80DC)
	// 825F8070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F807C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F8084: 38EBFA18  addi r7, r11, -0x5e8
	ctx.r[7].s64 = ctx.r[11].s64 + -1512;
	// 825F8088: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F808C: 388AA700  addi r4, r10, -0x5900
	ctx.r[4].s64 = ctx.r[10].s64 + -22784;
	// 825F8090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8094: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F809C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F80A0: 386AEE8C  addi r3, r10, -0x1174
	ctx.r[3].s64 = ctx.r[10].s64 + -4468;
	// 825F80A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F80A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F80AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F80B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F80B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F80B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F80BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F80C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F80C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F80C8: 4BE6ED59  bl 0x82466e20
	ctx.lr = 0x825F80CC;
	sub_82466E20(ctx, base);
	// 825F80CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F80D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F80D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F80D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F80E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F80E0 size=108
    let mut pc: u32 = 0x825F80E0;
    'dispatch: loop {
        match pc {
            0x825F80E0 => {
    //   block [0x825F80E0..0x825F814C)
	// 825F80E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F80E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F80E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F80EC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F80F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F80F4: 38EBFA30  addi r7, r11, -0x5d0
	ctx.r[7].s64 = ctx.r[11].s64 + -1488;
	// 825F80F8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F80FC: 388AA718  addi r4, r10, -0x58e8
	ctx.r[4].s64 = ctx.r[10].s64 + -22760;
	// 825F8100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8104: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8108: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F810C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8110: 386AEEBC  addi r3, r10, -0x1144
	ctx.r[3].s64 = ctx.r[10].s64 + -4420;
	// 825F8114: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F8118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F811C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F812C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8134: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8138: 4BE6ECE9  bl 0x82466e20
	ctx.lr = 0x825F813C;
	sub_82466E20(ctx, base);
	// 825F813C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8150 size=116
    let mut pc: u32 = 0x825F8150;
    'dispatch: loop {
        match pc {
            0x825F8150 => {
    //   block [0x825F8150..0x825F81C4)
	// 825F8150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F815C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8164: 390BFA94  addi r8, r11, -0x56c
	ctx.r[8].s64 = ctx.r[11].s64 + -1388;
	// 825F8168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F816C: 392A7C6C  addi r9, r10, 0x7c6c
	ctx.r[9].s64 = ctx.r[10].s64 + 31852;
	// 825F8170: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8174: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825F8178: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F817C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8184: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F8188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F818C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8194: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F8198: 388AA728  addi r4, r10, -0x58d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22744;
	// 825F819C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F81A0: 386BEEEC  addi r3, r11, -0x1114
	ctx.r[3].s64 = ctx.r[11].s64 + -4372;
	// 825F81A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F81A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F81AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F81B0: 4BE6EC71  bl 0x82466e20
	ctx.lr = 0x825F81B4;
	sub_82466E20(ctx, base);
	// 825F81B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F81B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F81BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F81C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F81C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F81C8 size=108
    let mut pc: u32 = 0x825F81C8;
    'dispatch: loop {
        match pc {
            0x825F81C8 => {
    //   block [0x825F81C8..0x825F8234)
	// 825F81C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F81CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F81D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F81D4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F81D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F81DC: 38EBFAB0  addi r7, r11, -0x550
	ctx.r[7].s64 = ctx.r[11].s64 + -1360;
	// 825F81E0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F81E4: 388AA73C  addi r4, r10, -0x58c4
	ctx.r[4].s64 = ctx.r[10].s64 + -22724;
	// 825F81E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F81EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F81F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F81F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F81F8: 386AEF1C  addi r3, r10, -0x10e4
	ctx.r[3].s64 = ctx.r[10].s64 + -4324;
	// 825F81FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F8200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F820C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F821C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8220: 4BE6EC01  bl 0x82466e20
	ctx.lr = 0x825F8224;
	sub_82466E20(ctx, base);
	// 825F8224: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F822C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8238 size=108
    let mut pc: u32 = 0x825F8238;
    'dispatch: loop {
        match pc {
            0x825F8238 => {
    //   block [0x825F8238..0x825F82A4)
	// 825F8238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F823C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8244: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8248: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F824C: 38EBFAF8  addi r7, r11, -0x508
	ctx.r[7].s64 = ctx.r[11].s64 + -1288;
	// 825F8250: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825F8254: 388AA760  addi r4, r10, -0x58a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22688;
	// 825F8258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F825C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8260: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F8264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8268: 386AEF4C  addi r3, r10, -0x10b4
	ctx.r[3].s64 = ctx.r[10].s64 + -4276;
	// 825F826C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F8270: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F827C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F828C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8290: 4BE6EB91  bl 0x82466e20
	ctx.lr = 0x825F8294;
	sub_82466E20(ctx, base);
	// 825F8294: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F829C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F82A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F82A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F82A8 size=108
    let mut pc: u32 = 0x825F82A8;
    'dispatch: loop {
        match pc {
            0x825F82A8 => {
    //   block [0x825F82A8..0x825F8314)
	// 825F82A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F82AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F82B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F82B4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F82B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F82BC: 38EBFB88  addi r7, r11, -0x478
	ctx.r[7].s64 = ctx.r[11].s64 + -1144;
	// 825F82C0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825F82C4: 388AA784  addi r4, r10, -0x587c
	ctx.r[4].s64 = ctx.r[10].s64 + -22652;
	// 825F82C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F82CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F82D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F82D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F82D8: 386AEF7C  addi r3, r10, -0x1084
	ctx.r[3].s64 = ctx.r[10].s64 + -4228;
	// 825F82DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F82E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F82E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F82E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F82EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F82F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F82F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F82F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F82FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8300: 4BE6EB21  bl 0x82466e20
	ctx.lr = 0x825F8304;
	sub_82466E20(ctx, base);
	// 825F8304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F830C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8318 size=100
    let mut pc: u32 = 0x825F8318;
    'dispatch: loop {
        match pc {
            0x825F8318 => {
    //   block [0x825F8318..0x825F837C)
	// 825F8318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F831C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8324: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F832C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F8330: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F8334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8338: 388AA79C  addi r4, r10, -0x5864
	ctx.r[4].s64 = ctx.r[10].s64 + -22628;
	// 825F833C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F834C: 386AEFAC  addi r3, r10, -0x1054
	ctx.r[3].s64 = ctx.r[10].s64 + -4180;
	// 825F8350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8354: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8358: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F835C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8360: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F8364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8368: 4BE6EAB9  bl 0x82466e20
	ctx.lr = 0x825F836C;
	sub_82466E20(ctx, base);
	// 825F836C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8380 size=112
    let mut pc: u32 = 0x825F8380;
    'dispatch: loop {
        match pc {
            0x825F8380 => {
    //   block [0x825F8380..0x825F83F0)
	// 825F8380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F838C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8390: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8394: 38AAEFAC  addi r5, r10, -0x1054
	ctx.r[5].s64 = ctx.r[10].s64 + -4180;
	// 825F8398: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F839C: 390BFC18  addi r8, r11, -0x3e8
	ctx.r[8].s64 = ctx.r[11].s64 + -1000;
	// 825F83A0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F83A4: 388AA7B8  addi r4, r10, -0x5848
	ctx.r[4].s64 = ctx.r[10].s64 + -22600;
	// 825F83A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F83AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F83B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F83B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F83B8: 386AEFDC  addi r3, r10, -0x1024
	ctx.r[3].s64 = ctx.r[10].s64 + -4132;
	// 825F83BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F83C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F83C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F83C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F83CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F83D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F83D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F83D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F83DC: 4BE6EA45  bl 0x82466e20
	ctx.lr = 0x825F83E0;
	sub_82466E20(ctx, base);
	// 825F83E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F83E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F83E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F83EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F83F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F83F0 size=108
    let mut pc: u32 = 0x825F83F0;
    'dispatch: loop {
        match pc {
            0x825F83F0 => {
    //   block [0x825F83F0..0x825F845C)
	// 825F83F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F83F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F83F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F83FC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8400: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F8404: 38EBFC78  addi r7, r11, -0x388
	ctx.r[7].s64 = ctx.r[11].s64 + -904;
	// 825F8408: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F840C: 388AA7DC  addi r4, r10, -0x5824
	ctx.r[4].s64 = ctx.r[10].s64 + -22564;
	// 825F8410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8414: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8418: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F841C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8420: 386AF00C  addi r3, r10, -0xff4
	ctx.r[3].s64 = ctx.r[10].s64 + -4084;
	// 825F8424: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F8428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F842C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F843C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8448: 4BE6E9D9  bl 0x82466e20
	ctx.lr = 0x825F844C;
	sub_82466E20(ctx, base);
	// 825F844C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8460 size=108
    let mut pc: u32 = 0x825F8460;
    'dispatch: loop {
        match pc {
            0x825F8460 => {
    //   block [0x825F8460..0x825F84CC)
	// 825F8460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F846C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8470: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F8474: 38EBFCA8  addi r7, r11, -0x358
	ctx.r[7].s64 = ctx.r[11].s64 + -856;
	// 825F8478: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F847C: 388AA7E4  addi r4, r10, -0x581c
	ctx.r[4].s64 = ctx.r[10].s64 + -22556;
	// 825F8480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8484: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8488: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F848C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8490: 386AF03C  addi r3, r10, -0xfc4
	ctx.r[3].s64 = ctx.r[10].s64 + -4036;
	// 825F8494: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F8498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F849C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F84A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F84A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F84A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F84AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F84B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F84B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F84B8: 4BE6E969  bl 0x82466e20
	ctx.lr = 0x825F84BC;
	sub_82466E20(ctx, base);
	// 825F84BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F84C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F84C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F84C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F84D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F84D0 size=108
    let mut pc: u32 = 0x825F84D0;
    'dispatch: loop {
        match pc {
            0x825F84D0 => {
    //   block [0x825F84D0..0x825F853C)
	// 825F84D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F84D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F84D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F84DC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F84E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F84E4: 38EBFD08  addi r7, r11, -0x2f8
	ctx.r[7].s64 = ctx.r[11].s64 + -760;
	// 825F84E8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F84EC: 388AA7F8  addi r4, r10, -0x5808
	ctx.r[4].s64 = ctx.r[10].s64 + -22536;
	// 825F84F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F84F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F84F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F84FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8500: 386AF06C  addi r3, r10, -0xf94
	ctx.r[3].s64 = ctx.r[10].s64 + -3988;
	// 825F8504: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F8508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F850C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F851C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8524: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8528: 4BE6E8F9  bl 0x82466e20
	ctx.lr = 0x825F852C;
	sub_82466E20(ctx, base);
	// 825F852C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8540 size=112
    let mut pc: u32 = 0x825F8540;
    'dispatch: loop {
        match pc {
            0x825F8540 => {
    //   block [0x825F8540..0x825F85B0)
	// 825F8540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F854C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F8550: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 825F8554: 38EAFD68  addi r7, r10, -0x298
	ctx.r[7].s64 = ctx.r[10].s64 + -664;
	// 825F8558: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F855C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F8560: 388AA804  addi r4, r10, -0x57fc
	ctx.r[4].s64 = ctx.r[10].s64 + -22524;
	// 825F8564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8568: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F856C: 396B7C80  addi r11, r11, 0x7c80
	ctx.r[11].s64 = ctx.r[11].s64 + 31872;
	// 825F8570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F8574: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8578: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F857C: 386AF09C  addi r3, r10, -0xf64
	ctx.r[3].s64 = ctx.r[10].s64 + -3940;
	// 825F8580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8584: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F8588: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F858C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F8590: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8594: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8598: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F859C: 4BE6E885  bl 0x82466e20
	ctx.lr = 0x825F85A0;
	sub_82466E20(ctx, base);
	// 825F85A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F85A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F85A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F85AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F85B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F85B0 size=96
    let mut pc: u32 = 0x825F85B0;
    'dispatch: loop {
        match pc {
            0x825F85B0 => {
    //   block [0x825F85B0..0x825F8610)
	// 825F85B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F85B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F85B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F85BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F85C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F85C4: 388AA82C  addi r4, r10, -0x57d4
	ctx.r[4].s64 = ctx.r[10].s64 + -22484;
	// 825F85C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F85CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F85D0: 386AF0CC  addi r3, r10, -0xf34
	ctx.r[3].s64 = ctx.r[10].s64 + -3892;
	// 825F85D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F85D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F85DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F85E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F85E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F85E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F85EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F85F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F85F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F85F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F85FC: 4BE6E825  bl 0x82466e20
	ctx.lr = 0x825F8600;
	sub_82466E20(ctx, base);
	// 825F8600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F860C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8610 size=112
    let mut pc: u32 = 0x825F8610;
    'dispatch: loop {
        match pc {
            0x825F8610 => {
    //   block [0x825F8610..0x825F8680)
	// 825F8610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F861C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8620: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8624: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F8628: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F862C: 390BFE88  addi r8, r11, -0x178
	ctx.r[8].s64 = ctx.r[11].s64 + -376;
	// 825F8630: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F8634: 388AA848  addi r4, r10, -0x57b8
	ctx.r[4].s64 = ctx.r[10].s64 + -22456;
	// 825F8638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F863C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8648: 386AF0FC  addi r3, r10, -0xf04
	ctx.r[3].s64 = ctx.r[10].s64 + -3844;
	// 825F864C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F8650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F865C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F866C: 4BE6E7B5  bl 0x82466e20
	ctx.lr = 0x825F8670;
	sub_82466E20(ctx, base);
	// 825F8670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F867C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F8680 size=24
    let mut pc: u32 = 0x825F8680;
    'dispatch: loop {
        match pc {
            0x825F8680 => {
    //   block [0x825F8680..0x825F8698)
	// 825F8680: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8684: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825F8688: 394A8C28  addi r10, r10, -0x73d8
	ctx.r[10].s64 = ctx.r[10].s64 + -29656;
	// 825F868C: 816BFAAC  lwz r11, -0x554(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1364 as u32) ) } as u64;
	// 825F8690: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 825F8694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8698 size=116
    let mut pc: u32 = 0x825F8698;
    'dispatch: loop {
        match pc {
            0x825F8698 => {
    //   block [0x825F8698..0x825F870C)
	// 825F8698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F869C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F86A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F86A4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825F86A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F86AC: 390B8C28  addi r8, r11, -0x73d8
	ctx.r[8].s64 = ctx.r[11].s64 + -29656;
	// 825F86B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F86B4: 392A7CFC  addi r9, r10, 0x7cfc
	ctx.r[9].s64 = ctx.r[10].s64 + 31996;
	// 825F86B8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F86BC: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 825F86C0: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F86C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F86C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F86CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F86D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F86D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F86D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F86DC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F86E0: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 825F86E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F86E8: 386BF12C  addi r3, r11, -0xed4
	ctx.r[3].s64 = ctx.r[11].s64 + -3796;
	// 825F86EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F86F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F86F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F86F8: 4BE6E729  bl 0x82466e20
	ctx.lr = 0x825F86FC;
	sub_82466E20(ctx, base);
	// 825F86FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F8710 size=24
    let mut pc: u32 = 0x825F8710;
    'dispatch: loop {
        match pc {
            0x825F8710 => {
    //   block [0x825F8710..0x825F8728)
	// 825F8710: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8714: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825F8718: 394A8D00  addi r10, r10, -0x7300
	ctx.r[10].s64 = ctx.r[10].s64 + -29440;
	// 825F871C: 816BFEF0  lwz r11, -0x110(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-272 as u32) ) } as u64;
	// 825F8720: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825F8724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8728 size=116
    let mut pc: u32 = 0x825F8728;
    'dispatch: loop {
        match pc {
            0x825F8728 => {
    //   block [0x825F8728..0x825F879C)
	// 825F8728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F872C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8734: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825F8738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F873C: 390B8D00  addi r8, r11, -0x7300
	ctx.r[8].s64 = ctx.r[11].s64 + -29440;
	// 825F8740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8744: 392A7DB0  addi r9, r10, 0x7db0
	ctx.r[9].s64 = ctx.r[10].s64 + 32176;
	// 825F8748: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F874C: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 825F8750: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825F8754: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F875C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F8760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F876C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F8770: 388AA870  addi r4, r10, -0x5790
	ctx.r[4].s64 = ctx.r[10].s64 + -22416;
	// 825F8774: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F8778: 386BF15C  addi r3, r11, -0xea4
	ctx.r[3].s64 = ctx.r[11].s64 + -3748;
	// 825F877C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F8780: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8788: 4BE6E699  bl 0x82466e20
	ctx.lr = 0x825F878C;
	sub_82466E20(ctx, base);
	// 825F878C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F87A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F87A0 size=112
    let mut pc: u32 = 0x825F87A0;
    'dispatch: loop {
        match pc {
            0x825F87A0 => {
    //   block [0x825F87A0..0x825F8810)
	// 825F87A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F87A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F87A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F87AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F87B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F87B4: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F87B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F87BC: 390BFEF8  addi r8, r11, -0x108
	ctx.r[8].s64 = ctx.r[11].s64 + -264;
	// 825F87C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F87C4: 388AA884  addi r4, r10, -0x577c
	ctx.r[4].s64 = ctx.r[10].s64 + -22396;
	// 825F87C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F87CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F87D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F87D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F87D8: 386AF18C  addi r3, r10, -0xe74
	ctx.r[3].s64 = ctx.r[10].s64 + -3700;
	// 825F87DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F87E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F87E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F87E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F87EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F87F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F87F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F87F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F87FC: 4BE6E625  bl 0x82466e20
	ctx.lr = 0x825F8800;
	sub_82466E20(ctx, base);
	// 825F8800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F880C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8810 size=112
    let mut pc: u32 = 0x825F8810;
    'dispatch: loop {
        match pc {
            0x825F8810 => {
    //   block [0x825F8810..0x825F8880)
	// 825F8810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F881C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8820: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8824: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F8828: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F882C: 390BFF40  addi r8, r11, -0xc0
	ctx.r[8].s64 = ctx.r[11].s64 + -192;
	// 825F8830: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F8834: 388AA89C  addi r4, r10, -0x5764
	ctx.r[4].s64 = ctx.r[10].s64 + -22372;
	// 825F8838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F883C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8848: 386AF1BC  addi r3, r10, -0xe44
	ctx.r[3].s64 = ctx.r[10].s64 + -3652;
	// 825F884C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F8850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F885C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F886C: 4BE6E5B5  bl 0x82466e20
	ctx.lr = 0x825F8870;
	sub_82466E20(ctx, base);
	// 825F8870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F887C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8880 size=108
    let mut pc: u32 = 0x825F8880;
    'dispatch: loop {
        match pc {
            0x825F8880 => {
    //   block [0x825F8880..0x825F88EC)
	// 825F8880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F888C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8894: 38EBFF88  addi r7, r11, -0x78
	ctx.r[7].s64 = ctx.r[11].s64 + -120;
	// 825F8898: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F889C: 388A2DD8  addi r4, r10, 0x2dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 11736;
	// 825F88A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F88A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F88A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F88AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F88B0: 386AF1EC  addi r3, r10, -0xe14
	ctx.r[3].s64 = ctx.r[10].s64 + -3604;
	// 825F88B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F88B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F88BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F88C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F88C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F88C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F88CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F88D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F88D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F88D8: 4BE6E549  bl 0x82466e20
	ctx.lr = 0x825F88DC;
	sub_82466E20(ctx, base);
	// 825F88DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F88E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F88E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F88E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F88F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F88F0 size=108
    let mut pc: u32 = 0x825F88F0;
    'dispatch: loop {
        match pc {
            0x825F88F0 => {
    //   block [0x825F88F0..0x825F895C)
	// 825F88F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F88F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F88F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F88FC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8904: 38EBFFE8  addi r7, r11, -0x18
	ctx.r[7].s64 = ctx.r[11].s64 + -24;
	// 825F8908: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825F890C: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 825F8910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8914: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8918: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F891C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8920: 386AF21C  addi r3, r10, -0xde4
	ctx.r[3].s64 = ctx.r[10].s64 + -3556;
	// 825F8924: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F8928: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F892C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F893C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8944: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8948: 4BE6E4D9  bl 0x82466e20
	ctx.lr = 0x825F894C;
	sub_82466E20(ctx, base);
	// 825F894C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8960 size=112
    let mut pc: u32 = 0x825F8960;
    'dispatch: loop {
        match pc {
            0x825F8960 => {
    //   block [0x825F8960..0x825F89D0)
	// 825F8960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F896C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F8970: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8974: 392B7DE4  addi r9, r11, 0x7de4
	ctx.r[9].s64 = ctx.r[11].s64 + 32228;
	// 825F8978: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 825F897C: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825F8980: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8984: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 825F8988: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F898C: 396B0090  addi r11, r11, 0x90
	ctx.r[11].s64 = ctx.r[11].s64 + 144;
	// 825F8990: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825F8994: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8998: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825F899C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F89A0: 386AF24C  addi r3, r10, -0xdb4
	ctx.r[3].s64 = ctx.r[10].s64 + -3508;
	// 825F89A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F89A8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825F89AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F89B0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825F89B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F89B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F89BC: 4BE6E465  bl 0x82466e20
	ctx.lr = 0x825F89C0;
	sub_82466E20(ctx, base);
	// 825F89C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F89C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F89C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F89CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F89D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F89D0 size=116
    let mut pc: u32 = 0x825F89D0;
    'dispatch: loop {
        match pc {
            0x825F89D0 => {
    //   block [0x825F89D0..0x825F8A44)
	// 825F89D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F89D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F89D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F89DC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F89E0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 825F89E4: 390A01E0  addi r8, r10, 0x1e0
	ctx.r[8].s64 = ctx.r[10].s64 + 480;
	// 825F89E8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F89EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F89F0: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F89F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F89F8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F89FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8A00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8A04: 388AA8B8  addi r4, r10, -0x5748
	ctx.r[4].s64 = ctx.r[10].s64 + -22344;
	// 825F8A08: 396B7E38  addi r11, r11, 0x7e38
	ctx.r[11].s64 = ctx.r[11].s64 + 32312;
	// 825F8A0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8A10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8A14: 386AF27C  addi r3, r10, -0xd84
	ctx.r[3].s64 = ctx.r[10].s64 + -3460;
	// 825F8A18: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F8A1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8A20: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F8A24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8A30: 4BE6E3F1  bl 0x82466e20
	ctx.lr = 0x825F8A34;
	sub_82466E20(ctx, base);
	// 825F8A34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8A48 size=112
    let mut pc: u32 = 0x825F8A48;
    'dispatch: loop {
        match pc {
            0x825F8A48 => {
    //   block [0x825F8A48..0x825F8AB8)
	// 825F8A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8A54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8A58: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8A5C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F8A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8A64: 390B0270  addi r8, r11, 0x270
	ctx.r[8].s64 = ctx.r[11].s64 + 624;
	// 825F8A68: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825F8A6C: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 825F8A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8A74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8A78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8A7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8A80: 386AF2AC  addi r3, r10, -0xd54
	ctx.r[3].s64 = ctx.r[10].s64 + -3412;
	// 825F8A84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F8A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8A94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8AA4: 4BE6E37D  bl 0x82466e20
	ctx.lr = 0x825F8AA8;
	sub_82466E20(ctx, base);
	// 825F8AA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8AB8 size=112
    let mut pc: u32 = 0x825F8AB8;
    'dispatch: loop {
        match pc {
            0x825F8AB8 => {
    //   block [0x825F8AB8..0x825F8B28)
	// 825F8AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8AC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8AC8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8ACC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F8AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8AD4: 390B0318  addi r8, r11, 0x318
	ctx.r[8].s64 = ctx.r[11].s64 + 792;
	// 825F8AD8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825F8ADC: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 825F8AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8AE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8AE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8AF0: 386AF2DC  addi r3, r10, -0xd24
	ctx.r[3].s64 = ctx.r[10].s64 + -3364;
	// 825F8AF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F8AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8B14: 4BE6E30D  bl 0x82466e20
	ctx.lr = 0x825F8B18;
	sub_82466E20(ctx, base);
	// 825F8B18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8B28 size=112
    let mut pc: u32 = 0x825F8B28;
    'dispatch: loop {
        match pc {
            0x825F8B28 => {
    //   block [0x825F8B28..0x825F8B98)
	// 825F8B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8B34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8B38: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8B3C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F8B40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8B44: 390B03A8  addi r8, r11, 0x3a8
	ctx.r[8].s64 = ctx.r[11].s64 + 936;
	// 825F8B48: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825F8B4C: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 825F8B50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8B54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8B58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8B5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8B60: 386AF30C  addi r3, r10, -0xcf4
	ctx.r[3].s64 = ctx.r[10].s64 + -3316;
	// 825F8B64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F8B68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8B6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8B70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8B74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8B78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8B80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8B84: 4BE6E29D  bl 0x82466e20
	ctx.lr = 0x825F8B88;
	sub_82466E20(ctx, base);
	// 825F8B88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8B8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8B90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8B94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8B98 size=108
    let mut pc: u32 = 0x825F8B98;
    'dispatch: loop {
        match pc {
            0x825F8B98 => {
    //   block [0x825F8B98..0x825F8C04)
	// 825F8B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8BA4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8BA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8BAC: 38EB0450  addi r7, r11, 0x450
	ctx.r[7].s64 = ctx.r[11].s64 + 1104;
	// 825F8BB0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825F8BB4: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 825F8BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8BBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8BC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F8BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8BC8: 386AF33C  addi r3, r10, -0xcc4
	ctx.r[3].s64 = ctx.r[10].s64 + -3268;
	// 825F8BCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F8BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8BDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8BEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8BF0: 4BE6E231  bl 0x82466e20
	ctx.lr = 0x825F8BF4;
	sub_82466E20(ctx, base);
	// 825F8BF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8BF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8BFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8C08 size=112
    let mut pc: u32 = 0x825F8C08;
    'dispatch: loop {
        match pc {
            0x825F8C08 => {
    //   block [0x825F8C08..0x825F8C78)
	// 825F8C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8C14: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8C18: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8C1C: 392A7EF8  addi r9, r10, 0x7ef8
	ctx.r[9].s64 = ctx.r[10].s64 + 32504;
	// 825F8C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8C24: 390B04FC  addi r8, r11, 0x4fc
	ctx.r[8].s64 = ctx.r[11].s64 + 1276;
	// 825F8C28: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825F8C2C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 825F8C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8C34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8C38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8C40: 386AF36C  addi r3, r10, -0xc94
	ctx.r[3].s64 = ctx.r[10].s64 + -3220;
	// 825F8C44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F8C48: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F8C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8C5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8C64: 4BE6E1BD  bl 0x82466e20
	ctx.lr = 0x825F8C68;
	sub_82466E20(ctx, base);
	// 825F8C68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8C78 size=100
    let mut pc: u32 = 0x825F8C78;
    'dispatch: loop {
        match pc {
            0x825F8C78 => {
    //   block [0x825F8C78..0x825F8CDC)
	// 825F8C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8C84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8C8C: 38AAFB7C  addi r5, r10, -0x484
	ctx.r[5].s64 = ctx.r[10].s64 + -1156;
	// 825F8C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8C94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8C98: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 825F8C9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8CA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8CAC: 386AF39C  addi r3, r10, -0xc64
	ctx.r[3].s64 = ctx.r[10].s64 + -3172;
	// 825F8CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8CB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8CB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F8CBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8CC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F8CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8CC8: 4BE6E159  bl 0x82466e20
	ctx.lr = 0x825F8CCC;
	sub_82466E20(ctx, base);
	// 825F8CCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F8CE0 size=24
    let mut pc: u32 = 0x825F8CE0;
    'dispatch: loop {
        match pc {
            0x825F8CE0 => {
    //   block [0x825F8CE0..0x825F8CF8)
	// 825F8CE0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8CE4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825F8CE8: 394A8E20  addi r10, r10, -0x71e0
	ctx.r[10].s64 = ctx.r[10].s64 + -29152;
	// 825F8CEC: 816B0530  lwz r11, 0x530(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1328 as u32) ) } as u64;
	// 825F8CF0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 825F8CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8CF8 size=108
    let mut pc: u32 = 0x825F8CF8;
    'dispatch: loop {
        match pc {
            0x825F8CF8 => {
    //   block [0x825F8CF8..0x825F8D64)
	// 825F8CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8D04: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825F8D08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F8D0C: 38EB8E20  addi r7, r11, -0x71e0
	ctx.r[7].s64 = ctx.r[11].s64 + -29152;
	// 825F8D10: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F8D14: 388AA8D0  addi r4, r10, -0x5730
	ctx.r[4].s64 = ctx.r[10].s64 + -22320;
	// 825F8D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8D1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8D20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F8D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8D28: 386AF3CC  addi r3, r10, -0xc34
	ctx.r[3].s64 = ctx.r[10].s64 + -3124;
	// 825F8D2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F8D30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8D38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8D40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8D44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8D48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8D4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8D50: 4BE6E0D1  bl 0x82466e20
	ctx.lr = 0x825F8D54;
	sub_82466E20(ctx, base);
	// 825F8D54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8D68 size=112
    let mut pc: u32 = 0x825F8D68;
    'dispatch: loop {
        match pc {
            0x825F8D68 => {
    //   block [0x825F8D68..0x825F8DD8)
	// 825F8D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8D74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8D78: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8D7C: 392A7F78  addi r9, r10, 0x7f78
	ctx.r[9].s64 = ctx.r[10].s64 + 32632;
	// 825F8D80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8D84: 390B0538  addi r8, r11, 0x538
	ctx.r[8].s64 = ctx.r[11].s64 + 1336;
	// 825F8D88: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825F8D8C: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 825F8D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8D94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8D98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8DA0: 386AF3FC  addi r3, r10, -0xc04
	ctx.r[3].s64 = ctx.r[10].s64 + -3076;
	// 825F8DA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F8DA8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825F8DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8DBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8DC4: 4BE6E05D  bl 0x82466e20
	ctx.lr = 0x825F8DC8;
	sub_82466E20(ctx, base);
	// 825F8DC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8DD8 size=112
    let mut pc: u32 = 0x825F8DD8;
    'dispatch: loop {
        match pc {
            0x825F8DD8 => {
    //   block [0x825F8DD8..0x825F8E48)
	// 825F8DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8DE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8DE8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8DEC: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825F8DF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8DF4: 390B0580  addi r8, r11, 0x580
	ctx.r[8].s64 = ctx.r[11].s64 + 1408;
	// 825F8DF8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F8DFC: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 825F8E00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8E04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8E08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8E10: 386AF42C  addi r3, r10, -0xbd4
	ctx.r[3].s64 = ctx.r[10].s64 + -3028;
	// 825F8E14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F8E18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8E1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8E20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8E24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8E28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8E2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8E30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8E34: 4BE6DFED  bl 0x82466e20
	ctx.lr = 0x825F8E38;
	sub_82466E20(ctx, base);
	// 825F8E38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8E48 size=116
    let mut pc: u32 = 0x825F8E48;
    'dispatch: loop {
        match pc {
            0x825F8E48 => {
    //   block [0x825F8E48..0x825F8EBC)
	// 825F8E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8E54: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F8E58: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825F8E5C: 390A05B0  addi r8, r10, 0x5b0
	ctx.r[8].s64 = ctx.r[10].s64 + 1456;
	// 825F8E60: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8E64: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F8E68: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825F8E6C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8E70: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F8E74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8E78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8E7C: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 825F8E80: 396B7FA0  addi r11, r11, 0x7fa0
	ctx.r[11].s64 = ctx.r[11].s64 + 32672;
	// 825F8E84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8E88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8E8C: 386AF45C  addi r3, r10, -0xba4
	ctx.r[3].s64 = ctx.r[10].s64 + -2980;
	// 825F8E90: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F8E94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8E98: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F8E9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8EA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8EA8: 4BE6DF79  bl 0x82466e20
	ctx.lr = 0x825F8EAC;
	sub_82466E20(ctx, base);
	// 825F8EAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8EC0 size=100
    let mut pc: u32 = 0x825F8EC0;
    'dispatch: loop {
        match pc {
            0x825F8EC0 => {
    //   block [0x825F8EC0..0x825F8F24)
	// 825F8EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8ECC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8ED4: 38AAF45C  addi r5, r10, -0xba4
	ctx.r[5].s64 = ctx.r[10].s64 + -2980;
	// 825F8ED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8EDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8EE0: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 825F8EE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8EF4: 386AF48C  addi r3, r10, -0xb74
	ctx.r[3].s64 = ctx.r[10].s64 + -2932;
	// 825F8EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8EFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8F00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F8F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8F08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F8F0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8F10: 4BE6DF11  bl 0x82466e20
	ctx.lr = 0x825F8F14;
	sub_82466E20(ctx, base);
	// 825F8F14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F8F28 size=24
    let mut pc: u32 = 0x825F8F28;
    'dispatch: loop {
        match pc {
            0x825F8F28 => {
    //   block [0x825F8F28..0x825F8F40)
	// 825F8F28: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8F2C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825F8F30: 394A8E68  addi r10, r10, -0x7198
	ctx.r[10].s64 = ctx.r[10].s64 + -29080;
	// 825F8F34: 816B0534  lwz r11, 0x534(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1332 as u32) ) } as u64;
	// 825F8F38: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825F8F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8F40 size=116
    let mut pc: u32 = 0x825F8F40;
    'dispatch: loop {
        match pc {
            0x825F8F40 => {
    //   block [0x825F8F40..0x825F8FB4)
	// 825F8F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8F48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8F4C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F8F50: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8F54: 392B7FDC  addi r9, r11, 0x7fdc
	ctx.r[9].s64 = ctx.r[11].s64 + 32732;
	// 825F8F58: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F8F5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8F60: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825F8F64: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 825F8F68: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825F8F6C: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 825F8F70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8F74: 396B8E68  addi r11, r11, -0x7198
	ctx.r[11].s64 = ctx.r[11].s64 + -29080;
	// 825F8F78: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825F8F7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8F80: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825F8F84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8F88: 386AF4BC  addi r3, r10, -0xb44
	ctx.r[3].s64 = ctx.r[10].s64 + -2884;
	// 825F8F8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F8F90: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825F8F94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8F98: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825F8F9C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F8FA0: 4BE6DE81  bl 0x82466e20
	ctx.lr = 0x825F8FA4;
	sub_82466E20(ctx, base);
	// 825F8FA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8FB8 size=116
    let mut pc: u32 = 0x825F8FB8;
    'dispatch: loop {
        match pc {
            0x825F8FB8 => {
    //   block [0x825F8FB8..0x825F902C)
	// 825F8FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8FC4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F8FC8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8FCC: 392B8038  addi r9, r11, -0x7fc8
	ctx.r[9].s64 = ctx.r[11].s64 + -32712;
	// 825F8FD0: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825F8FD4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8FD8: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 825F8FDC: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 825F8FE0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8FE4: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 825F8FE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8FEC: 396B0660  addi r11, r11, 0x660
	ctx.r[11].s64 = ctx.r[11].s64 + 1632;
	// 825F8FF0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825F8FF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8FF8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825F8FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9000: 386AF4EC  addi r3, r10, -0xb14
	ctx.r[3].s64 = ctx.r[10].s64 + -2836;
	// 825F9004: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F9008: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825F900C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9010: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825F9014: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F9018: 4BE6DE09  bl 0x82466e20
	ctx.lr = 0x825F901C;
	sub_82466E20(ctx, base);
	// 825F901C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9030 size=108
    let mut pc: u32 = 0x825F9030;
    'dispatch: loop {
        match pc {
            0x825F9030 => {
    //   block [0x825F9030..0x825F909C)
	// 825F9030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F903C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9044: 38EB07B0  addi r7, r11, 0x7b0
	ctx.r[7].s64 = ctx.r[11].s64 + 1968;
	// 825F9048: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F904C: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 825F9050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9054: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9058: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F905C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9060: 386AF51C  addi r3, r10, -0xae4
	ctx.r[3].s64 = ctx.r[10].s64 + -2788;
	// 825F9064: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F9068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F906C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F907C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F9088: 4BE6DD99  bl 0x82466e20
	ctx.lr = 0x825F908C;
	sub_82466E20(ctx, base);
	// 825F908C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F90A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F90A0 size=24
    let mut pc: u32 = 0x825F90A0;
    'dispatch: loop {
        match pc {
            0x825F90A0 => {
    //   block [0x825F90A0..0x825F90B8)
	// 825F90A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F90A4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825F90A8: 394A8F10  addi r10, r10, -0x70f0
	ctx.r[10].s64 = ctx.r[10].s64 + -28912;
	// 825F90AC: 816B065C  lwz r11, 0x65c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1628 as u32) ) } as u64;
	// 825F90B0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825F90B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F90B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F90B8 size=116
    let mut pc: u32 = 0x825F90B8;
    'dispatch: loop {
        match pc {
            0x825F90B8 => {
    //   block [0x825F90B8..0x825F912C)
	// 825F90B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F90BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F90C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F90C4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825F90C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F90CC: 390B8F10  addi r8, r11, -0x70f0
	ctx.r[8].s64 = ctx.r[11].s64 + -28912;
	// 825F90D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F90D4: 392A80D0  addi r9, r10, -0x7f30
	ctx.r[9].s64 = ctx.r[10].s64 + -32560;
	// 825F90D8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F90DC: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 825F90E0: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825F90E4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F90E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F90EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F90F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F90F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F90F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F90FC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F9100: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 825F9104: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F9108: 386BF54C  addi r3, r11, -0xab4
	ctx.r[3].s64 = ctx.r[11].s64 + -2740;
	// 825F910C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825F9110: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9118: 4BE6DD09  bl 0x82466e20
	ctx.lr = 0x825F911C;
	sub_82466E20(ctx, base);
	// 825F911C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9130 size=112
    let mut pc: u32 = 0x825F9130;
    'dispatch: loop {
        match pc {
            0x825F9130 => {
    //   block [0x825F9130..0x825F91A0)
	// 825F9130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F913C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9140: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9144: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825F9148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F914C: 390B0814  addi r8, r11, 0x814
	ctx.r[8].s64 = ctx.r[11].s64 + 2068;
	// 825F9150: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F9154: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 825F9158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F915C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9168: 386AF57C  addi r3, r10, -0xa84
	ctx.r[3].s64 = ctx.r[10].s64 + -2692;
	// 825F916C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F917C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F918C: 4BE6DC95  bl 0x82466e20
	ctx.lr = 0x825F9190;
	sub_82466E20(ctx, base);
	// 825F9190: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F919C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F91A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F91A0 size=24
    let mut pc: u32 = 0x825F91A0;
    'dispatch: loop {
        match pc {
            0x825F91A0 => {
    //   block [0x825F91A0..0x825F91B8)
	// 825F91A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F91A4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825F91A8: 394A90A8  addi r10, r10, -0x6f58
	ctx.r[10].s64 = ctx.r[10].s64 + -28504;
	// 825F91AC: 816B0844  lwz r11, 0x844(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2116 as u32) ) } as u64;
	// 825F91B0: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 825F91B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F91B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F91B8 size=116
    let mut pc: u32 = 0x825F91B8;
    'dispatch: loop {
        match pc {
            0x825F91B8 => {
    //   block [0x825F91B8..0x825F922C)
	// 825F91B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F91BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F91C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F91C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F91C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F91CC: 392B8108  addi r9, r11, -0x7ef8
	ctx.r[9].s64 = ctx.r[11].s64 + -32504;
	// 825F91D0: 38AAF4EC  addi r5, r10, -0xb14
	ctx.r[5].s64 = ctx.r[10].s64 + -2836;
	// 825F91D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F91D8: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 825F91DC: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 825F91E0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825F91E4: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 825F91E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F91EC: 396B90A8  addi r11, r11, -0x6f58
	ctx.r[11].s64 = ctx.r[11].s64 + -28504;
	// 825F91F0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825F91F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F91F8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825F91FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9200: 386AF5AC  addi r3, r10, -0xa54
	ctx.r[3].s64 = ctx.r[10].s64 + -2644;
	// 825F9204: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F9208: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825F920C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9210: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825F9214: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F9218: 4BE6DC09  bl 0x82466e20
	ctx.lr = 0x825F921C;
	sub_82466E20(ctx, base);
	// 825F921C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9230 size=112
    let mut pc: u32 = 0x825F9230;
    'dispatch: loop {
        match pc {
            0x825F9230 => {
    //   block [0x825F9230..0x825F92A0)
	// 825F9230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F923C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9240: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9244: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825F9248: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F924C: 390B0848  addi r8, r11, 0x848
	ctx.r[8].s64 = ctx.r[11].s64 + 2120;
	// 825F9250: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F9254: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 825F9258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F925C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9260: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9268: 386AF5DC  addi r3, r10, -0xa24
	ctx.r[3].s64 = ctx.r[10].s64 + -2596;
	// 825F926C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9270: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F927C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F928C: 4BE6DB95  bl 0x82466e20
	ctx.lr = 0x825F9290;
	sub_82466E20(ctx, base);
	// 825F9290: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F929C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F92A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F92A0 size=100
    let mut pc: u32 = 0x825F92A0;
    'dispatch: loop {
        match pc {
            0x825F92A0 => {
    //   block [0x825F92A0..0x825F9304)
	// 825F92A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F92A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F92A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F92AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F92B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F92B4: 38AAFB7C  addi r5, r10, -0x484
	ctx.r[5].s64 = ctx.r[10].s64 + -1156;
	// 825F92B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F92BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F92C0: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 825F92C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F92C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F92CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F92D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F92D4: 386AF60C  addi r3, r10, -0x9f4
	ctx.r[3].s64 = ctx.r[10].s64 + -2548;
	// 825F92D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F92DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F92E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F92E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F92E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F92EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F92F0: 4BE6DB31  bl 0x82466e20
	ctx.lr = 0x825F92F4;
	sub_82466E20(ctx, base);
	// 825F92F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F92F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F92FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9308 size=112
    let mut pc: u32 = 0x825F9308;
    'dispatch: loop {
        match pc {
            0x825F9308 => {
    //   block [0x825F9308..0x825F9378)
	// 825F9308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F930C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9314: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F9318: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 825F931C: 38EA0860  addi r7, r10, 0x860
	ctx.r[7].s64 = ctx.r[10].s64 + 2144;
	// 825F9320: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9324: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9328: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 825F932C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9330: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F9334: 396B8170  addi r11, r11, -0x7e90
	ctx.r[11].s64 = ctx.r[11].s64 + -32400;
	// 825F9338: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F933C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9340: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9344: 386AF63C  addi r3, r10, -0x9c4
	ctx.r[3].s64 = ctx.r[10].s64 + -2500;
	// 825F9348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F934C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F9350: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9354: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F9358: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F935C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9360: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F9364: 4BE6DABD  bl 0x82466e20
	ctx.lr = 0x825F9368;
	sub_82466E20(ctx, base);
	// 825F9368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F936C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9378 size=112
    let mut pc: u32 = 0x825F9378;
    'dispatch: loop {
        match pc {
            0x825F9378 => {
    //   block [0x825F9378..0x825F93E8)
	// 825F9378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F937C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9384: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9388: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F938C: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9394: 390B0998  addi r8, r11, 0x998
	ctx.r[8].s64 = ctx.r[11].s64 + 2456;
	// 825F9398: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F939C: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 825F93A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F93A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F93A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F93AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F93B0: 386AF66C  addi r3, r10, -0x994
	ctx.r[3].s64 = ctx.r[10].s64 + -2452;
	// 825F93B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F93B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F93BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F93C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F93C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F93C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F93CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F93D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F93D4: 4BE6DA4D  bl 0x82466e20
	ctx.lr = 0x825F93D8;
	sub_82466E20(ctx, base);
	// 825F93D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F93DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F93E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F93E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F93E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F93E8 size=112
    let mut pc: u32 = 0x825F93E8;
    'dispatch: loop {
        match pc {
            0x825F93E8 => {
    //   block [0x825F93E8..0x825F9458)
	// 825F93E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F93EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F93F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F93F4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F93F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F93FC: 38EA09C8  addi r7, r10, 0x9c8
	ctx.r[7].s64 = ctx.r[10].s64 + 2504;
	// 825F9400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9404: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9408: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 825F940C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9410: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F9414: 396B81C4  addi r11, r11, -0x7e3c
	ctx.r[11].s64 = ctx.r[11].s64 + -32316;
	// 825F9418: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F941C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9420: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9424: 386AF69C  addi r3, r10, -0x964
	ctx.r[3].s64 = ctx.r[10].s64 + -2404;
	// 825F9428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F942C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F9430: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9434: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F9438: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F943C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9440: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F9444: 4BE6D9DD  bl 0x82466e20
	ctx.lr = 0x825F9448;
	sub_82466E20(ctx, base);
	// 825F9448: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F944C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9458 size=112
    let mut pc: u32 = 0x825F9458;
    'dispatch: loop {
        match pc {
            0x825F9458 => {
    //   block [0x825F9458..0x825F94C8)
	// 825F9458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F945C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9464: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9468: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F946C: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9470: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9474: 390B09F8  addi r8, r11, 0x9f8
	ctx.r[8].s64 = ctx.r[11].s64 + 2552;
	// 825F9478: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F947C: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 825F9480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9484: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9488: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F948C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9490: 386AF6CC  addi r3, r10, -0x934
	ctx.r[3].s64 = ctx.r[10].s64 + -2356;
	// 825F9494: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F949C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F94A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F94A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F94A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F94AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F94B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F94B4: 4BE6D96D  bl 0x82466e20
	ctx.lr = 0x825F94B8;
	sub_82466E20(ctx, base);
	// 825F94B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F94BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F94C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F94C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F94C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F94C8 size=108
    let mut pc: u32 = 0x825F94C8;
    'dispatch: loop {
        match pc {
            0x825F94C8 => {
    //   block [0x825F94C8..0x825F9534)
	// 825F94C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F94CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F94D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F94D4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F94D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F94DC: 38EB0A10  addi r7, r11, 0xa10
	ctx.r[7].s64 = ctx.r[11].s64 + 2576;
	// 825F94E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F94E4: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 825F94E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F94EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F94F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F94F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F94F8: 386AF6FC  addi r3, r10, -0x904
	ctx.r[3].s64 = ctx.r[10].s64 + -2308;
	// 825F94FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F9500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F950C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F951C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F9520: 4BE6D901  bl 0x82466e20
	ctx.lr = 0x825F9524;
	sub_82466E20(ctx, base);
	// 825F9524: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F952C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9538 size=112
    let mut pc: u32 = 0x825F9538;
    'dispatch: loop {
        match pc {
            0x825F9538 => {
    //   block [0x825F9538..0x825F95A8)
	// 825F9538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F953C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9544: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9548: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F954C: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9554: 390B0A28  addi r8, r11, 0xa28
	ctx.r[8].s64 = ctx.r[11].s64 + 2600;
	// 825F9558: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F955C: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 825F9560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9564: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9568: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F956C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9570: 386AF72C  addi r3, r10, -0x8d4
	ctx.r[3].s64 = ctx.r[10].s64 + -2260;
	// 825F9574: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F957C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F958C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9594: 4BE6D88D  bl 0x82466e20
	ctx.lr = 0x825F9598;
	sub_82466E20(ctx, base);
	// 825F9598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F959C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F95A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F95A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F95A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F95A8 size=112
    let mut pc: u32 = 0x825F95A8;
    'dispatch: loop {
        match pc {
            0x825F95A8 => {
    //   block [0x825F95A8..0x825F9618)
	// 825F95A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F95AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F95B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F95B4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F95B8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825F95BC: 38EA0A40  addi r7, r10, 0xa40
	ctx.r[7].s64 = ctx.r[10].s64 + 2624;
	// 825F95C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F95C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F95C8: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 825F95CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F95D0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F95D4: 396B81D0  addi r11, r11, -0x7e30
	ctx.r[11].s64 = ctx.r[11].s64 + -32304;
	// 825F95D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F95DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F95E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F95E4: 386AF75C  addi r3, r10, -0x8a4
	ctx.r[3].s64 = ctx.r[10].s64 + -2212;
	// 825F95E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F95EC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F95F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F95F4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F95F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F95FC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9600: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F9604: 4BE6D81D  bl 0x82466e20
	ctx.lr = 0x825F9608;
	sub_82466E20(ctx, base);
	// 825F9608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F960C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9618 size=112
    let mut pc: u32 = 0x825F9618;
    'dispatch: loop {
        match pc {
            0x825F9618 => {
    //   block [0x825F9618..0x825F9688)
	// 825F9618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F961C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9624: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F9628: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 825F962C: 38EA0B18  addi r7, r10, 0xb18
	ctx.r[7].s64 = ctx.r[10].s64 + 2840;
	// 825F9630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9634: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9638: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 825F963C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9640: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F9644: 396B8210  addi r11, r11, -0x7df0
	ctx.r[11].s64 = ctx.r[11].s64 + -32240;
	// 825F9648: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F964C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9650: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9654: 386AF78C  addi r3, r10, -0x874
	ctx.r[3].s64 = ctx.r[10].s64 + -2164;
	// 825F9658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F965C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F9660: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9664: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F9668: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F966C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9670: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F9674: 4BE6D7AD  bl 0x82466e20
	ctx.lr = 0x825F9678;
	sub_82466E20(ctx, base);
	// 825F9678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F967C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9688 size=108
    let mut pc: u32 = 0x825F9688;
    'dispatch: loop {
        match pc {
            0x825F9688 => {
    //   block [0x825F9688..0x825F96F4)
	// 825F9688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F968C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9694: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9698: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F969C: 38EB0C68  addi r7, r11, 0xc68
	ctx.r[7].s64 = ctx.r[11].s64 + 3176;
	// 825F96A0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F96A4: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 825F96A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F96AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F96B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F96B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F96B8: 386AF7BC  addi r3, r10, -0x844
	ctx.r[3].s64 = ctx.r[10].s64 + -2116;
	// 825F96BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F96C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F96C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F96C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F96CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F96D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F96D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F96D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F96DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F96E0: 4BE6D741  bl 0x82466e20
	ctx.lr = 0x825F96E4;
	sub_82466E20(ctx, base);
	// 825F96E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F96E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F96EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F96F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F96F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F96F8 size=116
    let mut pc: u32 = 0x825F96F8;
    'dispatch: loop {
        match pc {
            0x825F96F8 => {
    //   block [0x825F96F8..0x825F976C)
	// 825F96F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F96FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9704: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F9708: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 825F970C: 390A0CC8  addi r8, r10, 0xcc8
	ctx.r[8].s64 = ctx.r[10].s64 + 3272;
	// 825F9710: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9714: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9718: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F971C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9720: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F9724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F972C: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 825F9730: 396B82B0  addi r11, r11, -0x7d50
	ctx.r[11].s64 = ctx.r[11].s64 + -32080;
	// 825F9734: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9738: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F973C: 386AF7EC  addi r3, r10, -0x814
	ctx.r[3].s64 = ctx.r[10].s64 + -2068;
	// 825F9740: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F9744: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9748: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F974C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9758: 4BE6D6C9  bl 0x82466e20
	ctx.lr = 0x825F975C;
	sub_82466E20(ctx, base);
	// 825F975C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9770 size=116
    let mut pc: u32 = 0x825F9770;
    'dispatch: loop {
        match pc {
            0x825F9770 => {
    //   block [0x825F9770..0x825F97E4)
	// 825F9770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F977C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F9780: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 825F9784: 390A0DE8  addi r8, r10, 0xde8
	ctx.r[8].s64 = ctx.r[10].s64 + 3560;
	// 825F9788: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F978C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9790: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9794: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9798: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F979C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F97A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F97A4: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 825F97A8: 396B82EC  addi r11, r11, -0x7d14
	ctx.r[11].s64 = ctx.r[11].s64 + -32020;
	// 825F97AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F97B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F97B4: 386AF81C  addi r3, r10, -0x7e4
	ctx.r[3].s64 = ctx.r[10].s64 + -2020;
	// 825F97B8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F97BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F97C0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F97C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F97C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F97CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F97D0: 4BE6D651  bl 0x82466e20
	ctx.lr = 0x825F97D4;
	sub_82466E20(ctx, base);
	// 825F97D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F97D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F97DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F97E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F97E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F97E8 size=108
    let mut pc: u32 = 0x825F97E8;
    'dispatch: loop {
        match pc {
            0x825F97E8 => {
    //   block [0x825F97E8..0x825F9854)
	// 825F97E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F97EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F97F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F97F4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F97F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F97FC: 38EB0E48  addi r7, r11, 0xe48
	ctx.r[7].s64 = ctx.r[11].s64 + 3656;
	// 825F9800: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825F9804: 388AA8F8  addi r4, r10, -0x5708
	ctx.r[4].s64 = ctx.r[10].s64 + -22280;
	// 825F9808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F980C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F9814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9818: 386AF84C  addi r3, r10, -0x7b4
	ctx.r[3].s64 = ctx.r[10].s64 + -1972;
	// 825F981C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F9820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F982C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F983C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F9840: 4BE6D5E1  bl 0x82466e20
	ctx.lr = 0x825F9844;
	sub_82466E20(ctx, base);
	// 825F9844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F984C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9858 size=112
    let mut pc: u32 = 0x825F9858;
    'dispatch: loop {
        match pc {
            0x825F9858 => {
    //   block [0x825F9858..0x825F98C8)
	// 825F9858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F985C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9864: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F9868: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825F986C: 38EA0EF0  addi r7, r10, 0xef0
	ctx.r[7].s64 = ctx.r[10].s64 + 3824;
	// 825F9870: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F9874: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9878: 388AA910  addi r4, r10, -0x56f0
	ctx.r[4].s64 = ctx.r[10].s64 + -22256;
	// 825F987C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9880: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F9884: 396B8300  addi r11, r11, -0x7d00
	ctx.r[11].s64 = ctx.r[11].s64 + -32000;
	// 825F9888: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F988C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9890: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9894: 386AF87C  addi r3, r10, -0x784
	ctx.r[3].s64 = ctx.r[10].s64 + -1924;
	// 825F9898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F989C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F98A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F98A4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F98A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F98AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F98B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F98B4: 4BE6D56D  bl 0x82466e20
	ctx.lr = 0x825F98B8;
	sub_82466E20(ctx, base);
	// 825F98B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F98BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F98C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F98C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F98C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F98C8 size=112
    let mut pc: u32 = 0x825F98C8;
    'dispatch: loop {
        match pc {
            0x825F98C8 => {
    //   block [0x825F98C8..0x825F9938)
	// 825F98C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F98CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F98D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F98D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F98D8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F98DC: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F98E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F98E4: 390B0F68  addi r8, r11, 0xf68
	ctx.r[8].s64 = ctx.r[11].s64 + 3944;
	// 825F98E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F98EC: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 825F98F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F98F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F98F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F98FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9900: 386AF8AC  addi r3, r10, -0x754
	ctx.r[3].s64 = ctx.r[10].s64 + -1876;
	// 825F9904: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F990C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F991C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9924: 4BE6D4FD  bl 0x82466e20
	ctx.lr = 0x825F9928;
	sub_82466E20(ctx, base);
	// 825F9928: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F992C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9938 size=112
    let mut pc: u32 = 0x825F9938;
    'dispatch: loop {
        match pc {
            0x825F9938 => {
    //   block [0x825F9938..0x825F99A8)
	// 825F9938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F993C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9944: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9948: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F994C: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9954: 390B0FB0  addi r8, r11, 0xfb0
	ctx.r[8].s64 = ctx.r[11].s64 + 4016;
	// 825F9958: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 825F995C: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 825F9960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9964: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F996C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9970: 386AF8DC  addi r3, r10, -0x724
	ctx.r[3].s64 = ctx.r[10].s64 + -1828;
	// 825F9974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F997C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F998C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9994: 4BE6D48D  bl 0x82466e20
	ctx.lr = 0x825F9998;
	sub_82466E20(ctx, base);
	// 825F9998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F999C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F99A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F99A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F99A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F99A8 size=100
    let mut pc: u32 = 0x825F99A8;
    'dispatch: loop {
        match pc {
            0x825F99A8 => {
    //   block [0x825F99A8..0x825F9A0C)
	// 825F99A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F99AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F99B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F99B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F99B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F99BC: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F99C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F99C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F99C8: 388AA928  addi r4, r10, -0x56d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22232;
	// 825F99CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F99D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F99D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F99D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F99DC: 386AF90C  addi r3, r10, -0x6f4
	ctx.r[3].s64 = ctx.r[10].s64 + -1780;
	// 825F99E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F99E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F99E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F99EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F99F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F99F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F99F8: 4BE6D429  bl 0x82466e20
	ctx.lr = 0x825F99FC;
	sub_82466E20(ctx, base);
	// 825F99FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9A10 size=112
    let mut pc: u32 = 0x825F9A10;
    'dispatch: loop {
        match pc {
            0x825F9A10 => {
    //   block [0x825F9A10..0x825F9A80)
	// 825F9A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9A1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9A20: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9A24: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9A28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9A2C: 390B10B8  addi r8, r11, 0x10b8
	ctx.r[8].s64 = ctx.r[11].s64 + 4280;
	// 825F9A30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F9A34: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 825F9A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9A3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9A40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9A48: 386AF93C  addi r3, r10, -0x6c4
	ctx.r[3].s64 = ctx.r[10].s64 + -1732;
	// 825F9A4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9A50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9A54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9A6C: 4BE6D3B5  bl 0x82466e20
	ctx.lr = 0x825F9A70;
	sub_82466E20(ctx, base);
	// 825F9A70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9A80 size=112
    let mut pc: u32 = 0x825F9A80;
    'dispatch: loop {
        match pc {
            0x825F9A80 => {
    //   block [0x825F9A80..0x825F9AF0)
	// 825F9A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9A8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9A90: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9A94: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F9A98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9A9C: 390B10D0  addi r8, r11, 0x10d0
	ctx.r[8].s64 = ctx.r[11].s64 + 4304;
	// 825F9AA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F9AA4: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 825F9AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9AAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9AB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9AB8: 386AF96C  addi r3, r10, -0x694
	ctx.r[3].s64 = ctx.r[10].s64 + -1684;
	// 825F9ABC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9AC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9ADC: 4BE6D345  bl 0x82466e20
	ctx.lr = 0x825F9AE0;
	sub_82466E20(ctx, base);
	// 825F9AE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9AF0 size=112
    let mut pc: u32 = 0x825F9AF0;
    'dispatch: loop {
        match pc {
            0x825F9AF0 => {
    //   block [0x825F9AF0..0x825F9B60)
	// 825F9AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9AFC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F9B00: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825F9B04: 38EA1100  addi r7, r10, 0x1100
	ctx.r[7].s64 = ctx.r[10].s64 + 4352;
	// 825F9B08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9B0C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9B10: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 825F9B14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9B18: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F9B1C: 396B8320  addi r11, r11, -0x7ce0
	ctx.r[11].s64 = ctx.r[11].s64 + -31968;
	// 825F9B20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F9B24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9B28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9B2C: 386AF99C  addi r3, r10, -0x664
	ctx.r[3].s64 = ctx.r[10].s64 + -1636;
	// 825F9B30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9B34: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F9B38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9B3C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F9B40: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9B44: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9B48: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F9B4C: 4BE6D2D5  bl 0x82466e20
	ctx.lr = 0x825F9B50;
	sub_82466E20(ctx, base);
	// 825F9B50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9B60 size=112
    let mut pc: u32 = 0x825F9B60;
    'dispatch: loop {
        match pc {
            0x825F9B60 => {
    //   block [0x825F9B60..0x825F9BD0)
	// 825F9B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9B6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9B70: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9B74: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9B78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9B7C: 390B1178  addi r8, r11, 0x1178
	ctx.r[8].s64 = ctx.r[11].s64 + 4472;
	// 825F9B80: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F9B84: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 825F9B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9B8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9B90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9B98: 386AF9CC  addi r3, r10, -0x634
	ctx.r[3].s64 = ctx.r[10].s64 + -1588;
	// 825F9B9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9BA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9BBC: 4BE6D265  bl 0x82466e20
	ctx.lr = 0x825F9BC0;
	sub_82466E20(ctx, base);
	// 825F9BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F9BD0 size=24
    let mut pc: u32 = 0x825F9BD0;
    'dispatch: loop {
        match pc {
            0x825F9BD0 => {
    //   block [0x825F9BD0..0x825F9BE8)
	// 825F9BD0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9BD4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825F9BD8: 394A91F8  addi r10, r10, -0x6e08
	ctx.r[10].s64 = ctx.r[10].s64 + -28168;
	// 825F9BDC: 816B11A8  lwz r11, 0x11a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4520 as u32) ) } as u64;
	// 825F9BE0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825F9BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9BE8 size=116
    let mut pc: u32 = 0x825F9BE8;
    'dispatch: loop {
        match pc {
            0x825F9BE8 => {
    //   block [0x825F9BE8..0x825F9C5C)
	// 825F9BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9BF4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825F9BF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F9BFC: 390B91F8  addi r8, r11, -0x6e08
	ctx.r[8].s64 = ctx.r[11].s64 + -28168;
	// 825F9C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9C04: 392A8360  addi r9, r10, -0x7ca0
	ctx.r[9].s64 = ctx.r[10].s64 + -31904;
	// 825F9C08: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9C0C: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 825F9C10: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9C14: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9C18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9C1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9C20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9C28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9C2C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F9C30: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 825F9C34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F9C38: 386BF9FC  addi r3, r11, -0x604
	ctx.r[3].s64 = ctx.r[11].s64 + -1540;
	// 825F9C3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F9C40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9C44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9C48: 4BE6D1D9  bl 0x82466e20
	ctx.lr = 0x825F9C4C;
	sub_82466E20(ctx, base);
	// 825F9C4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9C60 size=116
    let mut pc: u32 = 0x825F9C60;
    'dispatch: loop {
        match pc {
            0x825F9C60 => {
    //   block [0x825F9C60..0x825F9CD4)
	// 825F9C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9C68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9C6C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F9C70: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 825F9C74: 390A11B0  addi r8, r10, 0x11b0
	ctx.r[8].s64 = ctx.r[10].s64 + 4528;
	// 825F9C78: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9C7C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9C80: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9C84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9C88: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F9C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9C90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9C94: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 825F9C98: 396B8374  addi r11, r11, -0x7c8c
	ctx.r[11].s64 = ctx.r[11].s64 + -31884;
	// 825F9C9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9CA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9CA4: 386AFA2C  addi r3, r10, -0x5d4
	ctx.r[3].s64 = ctx.r[10].s64 + -1492;
	// 825F9CA8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F9CAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9CB0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F9CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9CB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9CC0: 4BE6D161  bl 0x82466e20
	ctx.lr = 0x825F9CC4;
	sub_82466E20(ctx, base);
	// 825F9CC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9CC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9CCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9CD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9CD8 size=112
    let mut pc: u32 = 0x825F9CD8;
    'dispatch: loop {
        match pc {
            0x825F9CD8 => {
    //   block [0x825F9CD8..0x825F9D48)
	// 825F9CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9CE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9CE8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9CEC: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9CF4: 390B1270  addi r8, r11, 0x1270
	ctx.r[8].s64 = ctx.r[11].s64 + 4720;
	// 825F9CF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F9CFC: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 825F9D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9D04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9D08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9D10: 386AFA5C  addi r3, r10, -0x5a4
	ctx.r[3].s64 = ctx.r[10].s64 + -1444;
	// 825F9D14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9D18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9D20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9D28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9D30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9D34: 4BE6D0ED  bl 0x82466e20
	ctx.lr = 0x825F9D38;
	sub_82466E20(ctx, base);
	// 825F9D38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9D48 size=112
    let mut pc: u32 = 0x825F9D48;
    'dispatch: loop {
        match pc {
            0x825F9D48 => {
    //   block [0x825F9D48..0x825F9DB8)
	// 825F9D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9D54: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F9D58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F9D5C: 38EA1288  addi r7, r10, 0x1288
	ctx.r[7].s64 = ctx.r[10].s64 + 4744;
	// 825F9D60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9D64: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9D68: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 825F9D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9D70: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F9D74: 396B8398  addi r11, r11, -0x7c68
	ctx.r[11].s64 = ctx.r[11].s64 + -31848;
	// 825F9D78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F9D7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9D80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9D84: 386AFA8C  addi r3, r10, -0x574
	ctx.r[3].s64 = ctx.r[10].s64 + -1396;
	// 825F9D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9D8C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F9D90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9D94: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F9D98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9D9C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9DA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F9DA4: 4BE6D07D  bl 0x82466e20
	ctx.lr = 0x825F9DA8;
	sub_82466E20(ctx, base);
	// 825F9DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9DB8 size=112
    let mut pc: u32 = 0x825F9DB8;
    'dispatch: loop {
        match pc {
            0x825F9DB8 => {
    //   block [0x825F9DB8..0x825F9E28)
	// 825F9DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9DC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9DC8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9DCC: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9DD4: 390B12B8  addi r8, r11, 0x12b8
	ctx.r[8].s64 = ctx.r[11].s64 + 4792;
	// 825F9DD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F9DDC: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 825F9DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9DE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9DE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9DF0: 386AFABC  addi r3, r10, -0x544
	ctx.r[3].s64 = ctx.r[10].s64 + -1348;
	// 825F9DF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9E14: 4BE6D00D  bl 0x82466e20
	ctx.lr = 0x825F9E18;
	sub_82466E20(ctx, base);
	// 825F9E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9E28 size=116
    let mut pc: u32 = 0x825F9E28;
    'dispatch: loop {
        match pc {
            0x825F9E28 => {
    //   block [0x825F9E28..0x825F9E9C)
	// 825F9E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9E34: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F9E38: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825F9E3C: 390A12D0  addi r8, r10, 0x12d0
	ctx.r[8].s64 = ctx.r[10].s64 + 4816;
	// 825F9E40: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9E44: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825F9E48: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9E4C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9E50: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F9E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9E58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9E5C: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 825F9E60: 396B83A4  addi r11, r11, -0x7c5c
	ctx.r[11].s64 = ctx.r[11].s64 + -31836;
	// 825F9E64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9E68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9E6C: 386AFAEC  addi r3, r10, -0x514
	ctx.r[3].s64 = ctx.r[10].s64 + -1300;
	// 825F9E70: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F9E74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9E78: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F9E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9E88: 4BE6CF99  bl 0x82466e20
	ctx.lr = 0x825F9E8C;
	sub_82466E20(ctx, base);
	// 825F9E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9EA0 size=112
    let mut pc: u32 = 0x825F9EA0;
    'dispatch: loop {
        match pc {
            0x825F9EA0 => {
    //   block [0x825F9EA0..0x825F9F10)
	// 825F9EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9EAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9EB0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9EB4: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9EBC: 390B1378  addi r8, r11, 0x1378
	ctx.r[8].s64 = ctx.r[11].s64 + 4984;
	// 825F9EC0: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 825F9EC4: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 825F9EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9ECC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9ED0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9ED8: 386AFB1C  addi r3, r10, -0x4e4
	ctx.r[3].s64 = ctx.r[10].s64 + -1252;
	// 825F9EDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9EFC: 4BE6CF25  bl 0x82466e20
	ctx.lr = 0x825F9F00;
	sub_82466E20(ctx, base);
	// 825F9F00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9F10 size=112
    let mut pc: u32 = 0x825F9F10;
    'dispatch: loop {
        match pc {
            0x825F9F10 => {
    //   block [0x825F9F10..0x825F9F80)
	// 825F9F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9F1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9F20: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9F24: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825F9F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9F2C: 390B14B0  addi r8, r11, 0x14b0
	ctx.r[8].s64 = ctx.r[11].s64 + 5296;
	// 825F9F30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F9F34: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 825F9F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9F3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9F40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9F48: 386AFB4C  addi r3, r10, -0x4b4
	ctx.r[3].s64 = ctx.r[10].s64 + -1204;
	// 825F9F4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F9F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F9F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F9F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9F6C: 4BE6CEB5  bl 0x82466e20
	ctx.lr = 0x825F9F70;
	sub_82466E20(ctx, base);
	// 825F9F70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9F80 size=116
    let mut pc: u32 = 0x825F9F80;
    'dispatch: loop {
        match pc {
            0x825F9F80 => {
    //   block [0x825F9F80..0x825F9FF4)
	// 825F9F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F9F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F9F8C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F9F90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F9F94: 390B14C8  addi r8, r11, 0x14c8
	ctx.r[8].s64 = ctx.r[11].s64 + 5320;
	// 825F9F98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F9F9C: 392A83DC  addi r9, r10, -0x7c24
	ctx.r[9].s64 = ctx.r[10].s64 + -31780;
	// 825F9FA0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F9FA4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825F9FA8: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F9FAC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F9FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F9FB4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F9FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F9FBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F9FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F9FC4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F9FC8: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 825F9FCC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F9FD0: 386BFB7C  addi r3, r11, -0x484
	ctx.r[3].s64 = ctx.r[11].s64 + -1156;
	// 825F9FD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F9FD8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F9FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F9FE0: 4BE6CE41  bl 0x82466e20
	ctx.lr = 0x825F9FE4;
	sub_82466E20(ctx, base);
	// 825F9FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F9FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F9FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F9FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F9FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F9FF8 size=100
    let mut pc: u32 = 0x825F9FF8;
    'dispatch: loop {
        match pc {
            0x825F9FF8 => {
    //   block [0x825F9FF8..0x825FA05C)
	// 825F9FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F9FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA004: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA00C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FA010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA018: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 825FA01C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA02C: 386AFBAC  addi r3, r10, -0x454
	ctx.r[3].s64 = ctx.r[10].s64 + -1108;
	// 825FA030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA034: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA038: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FA03C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA040: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FA044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA048: 4BE6CDD9  bl 0x82466e20
	ctx.lr = 0x825FA04C;
	sub_82466E20(ctx, base);
	// 825FA04C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA060 size=112
    let mut pc: u32 = 0x825FA060;
    'dispatch: loop {
        match pc {
            0x825FA060 => {
    //   block [0x825FA060..0x825FA0D0)
	// 825FA060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA06C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA070: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA074: 38AAFBAC  addi r5, r10, -0x454
	ctx.r[5].s64 = ctx.r[10].s64 + -1108;
	// 825FA078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA07C: 390B14F8  addi r8, r11, 0x14f8
	ctx.r[8].s64 = ctx.r[11].s64 + 5368;
	// 825FA080: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FA084: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 825FA088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA08C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA090: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA098: 386AFBDC  addi r3, r10, -0x424
	ctx.r[3].s64 = ctx.r[10].s64 + -1060;
	// 825FA09C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FA0A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA0A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA0A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA0B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA0B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA0B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA0BC: 4BE6CD65  bl 0x82466e20
	ctx.lr = 0x825FA0C0;
	sub_82466E20(ctx, base);
	// 825FA0C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA0D0 size=112
    let mut pc: u32 = 0x825FA0D0;
    'dispatch: loop {
        match pc {
            0x825FA0D0 => {
    //   block [0x825FA0D0..0x825FA140)
	// 825FA0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA0D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA0DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA0E0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA0E4: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FA0E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA0EC: 390B1510  addi r8, r11, 0x1510
	ctx.r[8].s64 = ctx.r[11].s64 + 5392;
	// 825FA0F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FA0F4: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 825FA0F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA0FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA108: 386AFC0C  addi r3, r10, -0x3f4
	ctx.r[3].s64 = ctx.r[10].s64 + -1012;
	// 825FA10C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FA110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA11C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA12C: 4BE6CCF5  bl 0x82466e20
	ctx.lr = 0x825FA130;
	sub_82466E20(ctx, base);
	// 825FA130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA13C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA140 size=112
    let mut pc: u32 = 0x825FA140;
    'dispatch: loop {
        match pc {
            0x825FA140 => {
    //   block [0x825FA140..0x825FA1B0)
	// 825FA140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA14C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA150: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA154: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FA158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA15C: 390B1558  addi r8, r11, 0x1558
	ctx.r[8].s64 = ctx.r[11].s64 + 5464;
	// 825FA160: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825FA164: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 825FA168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA16C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA178: 386AFC3C  addi r3, r10, -0x3c4
	ctx.r[3].s64 = ctx.r[10].s64 + -964;
	// 825FA17C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FA180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA18C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA19C: 4BE6CC85  bl 0x82466e20
	ctx.lr = 0x825FA1A0;
	sub_82466E20(ctx, base);
	// 825FA1A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA1A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA1A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA1B0 size=116
    let mut pc: u32 = 0x825FA1B0;
    'dispatch: loop {
        match pc {
            0x825FA1B0 => {
    //   block [0x825FA1B0..0x825FA224)
	// 825FA1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA1BC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FA1C0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 825FA1C4: 390A1600  addi r8, r10, 0x1600
	ctx.r[8].s64 = ctx.r[10].s64 + 5632;
	// 825FA1C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA1CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FA1D0: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FA1D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA1D8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FA1DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA1E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA1E4: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 825FA1E8: 396B83F0  addi r11, r11, -0x7c10
	ctx.r[11].s64 = ctx.r[11].s64 + -31760;
	// 825FA1EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA1F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA1F4: 386AFC6C  addi r3, r10, -0x394
	ctx.r[3].s64 = ctx.r[10].s64 + -916;
	// 825FA1F8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FA1FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA200: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FA204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA20C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA210: 4BE6CC11  bl 0x82466e20
	ctx.lr = 0x825FA214;
	sub_82466E20(ctx, base);
	// 825FA214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA21C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA228 size=112
    let mut pc: u32 = 0x825FA228;
    'dispatch: loop {
        match pc {
            0x825FA228 => {
    //   block [0x825FA228..0x825FA298)
	// 825FA228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA234: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA238: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA23C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FA240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA244: 390B16C0  addi r8, r11, 0x16c0
	ctx.r[8].s64 = ctx.r[11].s64 + 5824;
	// 825FA248: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FA24C: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 825FA250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA254: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA25C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA260: 386AFC9C  addi r3, r10, -0x364
	ctx.r[3].s64 = ctx.r[10].s64 + -868;
	// 825FA264: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FA268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA26C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA27C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA284: 4BE6CB9D  bl 0x82466e20
	ctx.lr = 0x825FA288;
	sub_82466E20(ctx, base);
	// 825FA288: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA298 size=100
    let mut pc: u32 = 0x825FA298;
    'dispatch: loop {
        match pc {
            0x825FA298 => {
    //   block [0x825FA298..0x825FA2FC)
	// 825FA298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA2A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA2A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA2AC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FA2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA2B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA2B8: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 825FA2BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA2C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA2C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA2C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA2CC: 386AFCCC  addi r3, r10, -0x334
	ctx.r[3].s64 = ctx.r[10].s64 + -820;
	// 825FA2D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA2D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA2D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FA2DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA2E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FA2E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA2E8: 4BE6CB39  bl 0x82466e20
	ctx.lr = 0x825FA2EC;
	sub_82466E20(ctx, base);
	// 825FA2EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA2F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA2F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA2F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA300 size=108
    let mut pc: u32 = 0x825FA300;
    'dispatch: loop {
        match pc {
            0x825FA300 => {
    //   block [0x825FA300..0x825FA36C)
	// 825FA300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA30C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA314: 38EB16F0  addi r7, r11, 0x16f0
	ctx.r[7].s64 = ctx.r[11].s64 + 5872;
	// 825FA318: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FA31C: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 825FA320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA324: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA328: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FA32C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA330: 386AFCFC  addi r3, r10, -0x304
	ctx.r[3].s64 = ctx.r[10].s64 + -772;
	// 825FA334: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FA338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA33C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FA358: 4BE6CAC9  bl 0x82466e20
	ctx.lr = 0x825FA35C;
	sub_82466E20(ctx, base);
	// 825FA35C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA370 size=112
    let mut pc: u32 = 0x825FA370;
    'dispatch: loop {
        match pc {
            0x825FA370 => {
    //   block [0x825FA370..0x825FA3E0)
	// 825FA370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA37C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA380: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA384: 38AAFCCC  addi r5, r10, -0x334
	ctx.r[5].s64 = ctx.r[10].s64 + -820;
	// 825FA388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA38C: 390B1720  addi r8, r11, 0x1720
	ctx.r[8].s64 = ctx.r[11].s64 + 5920;
	// 825FA390: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FA394: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 825FA398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA39C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA3A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA3A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA3A8: 386AFD2C  addi r3, r10, -0x2d4
	ctx.r[3].s64 = ctx.r[10].s64 + -724;
	// 825FA3AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FA3B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA3B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA3B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA3BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA3C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA3C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA3CC: 4BE6CA55  bl 0x82466e20
	ctx.lr = 0x825FA3D0;
	sub_82466E20(ctx, base);
	// 825FA3D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA3D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA3D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA3DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA3E0 size=108
    let mut pc: u32 = 0x825FA3E0;
    'dispatch: loop {
        match pc {
            0x825FA3E0 => {
    //   block [0x825FA3E0..0x825FA44C)
	// 825FA3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA3E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA3EC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA3F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FA3F4: 38EB1750  addi r7, r11, 0x1750
	ctx.r[7].s64 = ctx.r[11].s64 + 5968;
	// 825FA3F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FA3FC: 388AA93C  addi r4, r10, -0x56c4
	ctx.r[4].s64 = ctx.r[10].s64 + -22212;
	// 825FA400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA404: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA408: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FA40C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA410: 386AFD5C  addi r3, r10, -0x2a4
	ctx.r[3].s64 = ctx.r[10].s64 + -676;
	// 825FA414: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FA418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA41C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA434: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FA438: 4BE6C9E9  bl 0x82466e20
	ctx.lr = 0x825FA43C;
	sub_82466E20(ctx, base);
	// 825FA43C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA450 size=112
    let mut pc: u32 = 0x825FA450;
    'dispatch: loop {
        match pc {
            0x825FA450 => {
    //   block [0x825FA450..0x825FA4C0)
	// 825FA450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA45C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA460: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA464: 38AAFCCC  addi r5, r10, -0x334
	ctx.r[5].s64 = ctx.r[10].s64 + -820;
	// 825FA468: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA46C: 390B1780  addi r8, r11, 0x1780
	ctx.r[8].s64 = ctx.r[11].s64 + 6016;
	// 825FA470: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FA474: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 825FA478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA47C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA480: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA488: 386AFD8C  addi r3, r10, -0x274
	ctx.r[3].s64 = ctx.r[10].s64 + -628;
	// 825FA48C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FA490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA49C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA4A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA4A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA4A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA4AC: 4BE6C975  bl 0x82466e20
	ctx.lr = 0x825FA4B0;
	sub_82466E20(ctx, base);
	// 825FA4B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA4B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA4B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA4BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA4C0 size=108
    let mut pc: u32 = 0x825FA4C0;
    'dispatch: loop {
        match pc {
            0x825FA4C0 => {
    //   block [0x825FA4C0..0x825FA52C)
	// 825FA4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA4C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA4CC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA4D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FA4D4: 38EB17C8  addi r7, r11, 0x17c8
	ctx.r[7].s64 = ctx.r[11].s64 + 6088;
	// 825FA4D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FA4DC: 388AA960  addi r4, r10, -0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22176;
	// 825FA4E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA4E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA4E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FA4EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA4F0: 386AFDBC  addi r3, r10, -0x244
	ctx.r[3].s64 = ctx.r[10].s64 + -580;
	// 825FA4F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FA4F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA4FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA50C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA514: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FA518: 4BE6C909  bl 0x82466e20
	ctx.lr = 0x825FA51C;
	sub_82466E20(ctx, base);
	// 825FA51C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA530 size=112
    let mut pc: u32 = 0x825FA530;
    'dispatch: loop {
        match pc {
            0x825FA530 => {
    //   block [0x825FA530..0x825FA5A0)
	// 825FA530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA53C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA540: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA544: 38AAFCCC  addi r5, r10, -0x334
	ctx.r[5].s64 = ctx.r[10].s64 + -820;
	// 825FA548: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA54C: 390B17F8  addi r8, r11, 0x17f8
	ctx.r[8].s64 = ctx.r[11].s64 + 6136;
	// 825FA550: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FA554: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 825FA558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA55C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA560: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA568: 386AFDEC  addi r3, r10, -0x214
	ctx.r[3].s64 = ctx.r[10].s64 + -532;
	// 825FA56C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FA570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA58C: 4BE6C895  bl 0x82466e20
	ctx.lr = 0x825FA590;
	sub_82466E20(ctx, base);
	// 825FA590: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA59C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA5A0 size=108
    let mut pc: u32 = 0x825FA5A0;
    'dispatch: loop {
        match pc {
            0x825FA5A0 => {
    //   block [0x825FA5A0..0x825FA60C)
	// 825FA5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA5A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA5AC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA5B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FA5B4: 38EB1840  addi r7, r11, 0x1840
	ctx.r[7].s64 = ctx.r[11].s64 + 6208;
	// 825FA5B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FA5BC: 388AA984  addi r4, r10, -0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + -22140;
	// 825FA5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA5C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA5C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FA5CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA5D0: 386AFE1C  addi r3, r10, -0x1e4
	ctx.r[3].s64 = ctx.r[10].s64 + -484;
	// 825FA5D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FA5D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA5DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA5E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA5E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA5E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA5EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA5F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA5F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FA5F8: 4BE6C829  bl 0x82466e20
	ctx.lr = 0x825FA5FC;
	sub_82466E20(ctx, base);
	// 825FA5FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA610 size=112
    let mut pc: u32 = 0x825FA610;
    'dispatch: loop {
        match pc {
            0x825FA610 => {
    //   block [0x825FA610..0x825FA680)
	// 825FA610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA61C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA620: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA624: 38AAFCCC  addi r5, r10, -0x334
	ctx.r[5].s64 = ctx.r[10].s64 + -820;
	// 825FA628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA62C: 390B1870  addi r8, r11, 0x1870
	ctx.r[8].s64 = ctx.r[11].s64 + 6256;
	// 825FA630: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FA634: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 825FA638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA63C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA648: 386AFE4C  addi r3, r10, -0x1b4
	ctx.r[3].s64 = ctx.r[10].s64 + -436;
	// 825FA64C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FA650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA66C: 4BE6C7B5  bl 0x82466e20
	ctx.lr = 0x825FA670;
	sub_82466E20(ctx, base);
	// 825FA670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA67C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA680 size=108
    let mut pc: u32 = 0x825FA680;
    'dispatch: loop {
        match pc {
            0x825FA680 => {
    //   block [0x825FA680..0x825FA6EC)
	// 825FA680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA68C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA694: 38EB18C0  addi r7, r11, 0x18c0
	ctx.r[7].s64 = ctx.r[11].s64 + 6336;
	// 825FA698: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FA69C: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 825FA6A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA6A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA6A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FA6AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA6B0: 386AFE7C  addi r3, r10, -0x184
	ctx.r[3].s64 = ctx.r[10].s64 + -388;
	// 825FA6B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FA6B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA6BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA6C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA6C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA6C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA6CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA6D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA6D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FA6D8: 4BE6C749  bl 0x82466e20
	ctx.lr = 0x825FA6DC;
	sub_82466E20(ctx, base);
	// 825FA6DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA6E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA6E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FA6F0 size=24
    let mut pc: u32 = 0x825FA6F0;
    'dispatch: loop {
        match pc {
            0x825FA6F0 => {
    //   block [0x825FA6F0..0x825FA708)
	// 825FA6F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA6F4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FA6F8: 394A9300  addi r10, r10, -0x6d00
	ctx.r[10].s64 = ctx.r[10].s64 + -27904;
	// 825FA6FC: 816B18BC  lwz r11, 0x18bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6332 as u32) ) } as u64;
	// 825FA700: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 825FA704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA708 size=112
    let mut pc: u32 = 0x825FA708;
    'dispatch: loop {
        match pc {
            0x825FA708 => {
    //   block [0x825FA708..0x825FA778)
	// 825FA708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA714: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FA718: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FA71C: 392A84D8  addi r9, r10, -0x7b28
	ctx.r[9].s64 = ctx.r[10].s64 + -31528;
	// 825FA720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA724: 390B9300  addi r8, r11, -0x6d00
	ctx.r[8].s64 = ctx.r[11].s64 + -27904;
	// 825FA728: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 825FA72C: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 825FA730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA734: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA73C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA740: 386AFEAC  addi r3, r10, -0x154
	ctx.r[3].s64 = ctx.r[10].s64 + -340;
	// 825FA744: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FA748: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FA74C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA75C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FA760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA764: 4BE6C6BD  bl 0x82466e20
	ctx.lr = 0x825FA768;
	sub_82466E20(ctx, base);
	// 825FA768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA778 size=108
    let mut pc: u32 = 0x825FA778;
    'dispatch: loop {
        match pc {
            0x825FA778 => {
    //   block [0x825FA778..0x825FA7E4)
	// 825FA778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA784: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA78C: 38EB1928  addi r7, r11, 0x1928
	ctx.r[7].s64 = ctx.r[11].s64 + 6440;
	// 825FA790: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825FA794: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 825FA798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA79C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA7A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FA7A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA7A8: 386AFEDC  addi r3, r10, -0x124
	ctx.r[3].s64 = ctx.r[10].s64 + -292;
	// 825FA7AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FA7B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA7B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA7BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA7C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA7C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA7C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA7CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FA7D0: 4BE6C651  bl 0x82466e20
	ctx.lr = 0x825FA7D4;
	sub_82466E20(ctx, base);
	// 825FA7D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA7D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA7DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA7E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA7E8 size=108
    let mut pc: u32 = 0x825FA7E8;
    'dispatch: loop {
        match pc {
            0x825FA7E8 => {
    //   block [0x825FA7E8..0x825FA854)
	// 825FA7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA7F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA7F4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA7F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FA7FC: 38EB19A0  addi r7, r11, 0x19a0
	ctx.r[7].s64 = ctx.r[11].s64 + 6560;
	// 825FA800: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FA804: 388AA9B8  addi r4, r10, -0x5648
	ctx.r[4].s64 = ctx.r[10].s64 + -22088;
	// 825FA808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA80C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FA814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA818: 386AFF0C  addi r3, r10, -0xf4
	ctx.r[3].s64 = ctx.r[10].s64 + -244;
	// 825FA81C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FA820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA82C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA83C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FA840: 4BE6C5E1  bl 0x82466e20
	ctx.lr = 0x825FA844;
	sub_82466E20(ctx, base);
	// 825FA844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA84C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA858 size=108
    let mut pc: u32 = 0x825FA858;
    'dispatch: loop {
        match pc {
            0x825FA858 => {
    //   block [0x825FA858..0x825FA8C4)
	// 825FA858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA864: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA868: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA86C: 38EB1A00  addi r7, r11, 0x1a00
	ctx.r[7].s64 = ctx.r[11].s64 + 6656;
	// 825FA870: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825FA874: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 825FA878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA87C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA880: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FA884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FA888: 386AFF3C  addi r3, r10, -0xc4
	ctx.r[3].s64 = ctx.r[10].s64 + -196;
	// 825FA88C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FA890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FA894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA89C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA8A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA8A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA8A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA8AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FA8B0: 4BE6C571  bl 0x82466e20
	ctx.lr = 0x825FA8B4;
	sub_82466E20(ctx, base);
	// 825FA8B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA8B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA8BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FA8C8 size=24
    let mut pc: u32 = 0x825FA8C8;
    'dispatch: loop {
        match pc {
            0x825FA8C8 => {
    //   block [0x825FA8C8..0x825FA8E0)
	// 825FA8C8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA8CC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FA8D0: 394A9408  addi r10, r10, -0x6bf8
	ctx.r[10].s64 = ctx.r[10].s64 + -27640;
	// 825FA8D4: 816B18B8  lwz r11, 0x18b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6328 as u32) ) } as u64;
	// 825FA8D8: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 825FA8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA8E0 size=116
    let mut pc: u32 = 0x825FA8E0;
    'dispatch: loop {
        match pc {
            0x825FA8E0 => {
    //   block [0x825FA8E0..0x825FA954)
	// 825FA8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA8E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA8EC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FA8F0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA8F4: 392B8438  addi r9, r11, -0x7bc8
	ctx.r[9].s64 = ctx.r[11].s64 + -31688;
	// 825FA8F8: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825FA8FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA900: 38E900C8  addi r7, r9, 0xc8
	ctx.r[7].s64 = ctx.r[9].s64 + 200;
	// 825FA904: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 825FA908: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FA90C: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 825FA910: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA914: 396B9408  addi r11, r11, -0x6bf8
	ctx.r[11].s64 = ctx.r[11].s64 + -27640;
	// 825FA918: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825FA91C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA920: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825FA924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA928: 386AFF6C  addi r3, r10, -0x94
	ctx.r[3].s64 = ctx.r[10].s64 + -148;
	// 825FA92C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FA930: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825FA934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA938: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825FA93C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FA940: 4BE6C4E1  bl 0x82466e20
	ctx.lr = 0x825FA944;
	sub_82466E20(ctx, base);
	// 825FA944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA94C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FA958 size=36
    let mut pc: u32 = 0x825FA958;
    'dispatch: loop {
        match pc {
            0x825FA958 => {
    //   block [0x825FA958..0x825FA97C)
	// 825FA958: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA95C: 814B1924  lwz r10, 0x1924(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6436 as u32) ) } as u64;
	// 825FA960: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FA964: 396B96D8  addi r11, r11, -0x6928
	ctx.r[11].s64 = ctx.r[11].s64 + -26920;
	// 825FA968: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825FA96C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FA970: 814A1AA8  lwz r10, 0x1aa8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6824 as u32) ) } as u64;
	// 825FA974: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 825FA978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FA980 size=116
    let mut pc: u32 = 0x825FA980;
    'dispatch: loop {
        match pc {
            0x825FA980 => {
    //   block [0x825FA980..0x825FA9F4)
	// 825FA980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FA984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FA988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FA98C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FA990: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FA994: 390B96D8  addi r8, r11, -0x6928
	ctx.r[8].s64 = ctx.r[11].s64 + -26920;
	// 825FA998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FA99C: 392A85C0  addi r9, r10, -0x7a40
	ctx.r[9].s64 = ctx.r[10].s64 + -31296;
	// 825FA9A0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FA9A4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825FA9A8: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825FA9AC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FA9B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FA9B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FA9B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FA9BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FA9C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FA9C4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FA9C8: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 825FA9CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FA9D0: 386BFF9C  addi r3, r11, -0x64
	ctx.r[3].s64 = ctx.r[11].s64 + -100;
	// 825FA9D4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FA9D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FA9DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FA9E0: 4BE6C441  bl 0x82466e20
	ctx.lr = 0x825FA9E4;
	sub_82466E20(ctx, base);
	// 825FA9E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FA9E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FA9EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FA9F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FA9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FA9F8 size=24
    let mut pc: u32 = 0x825FA9F8;
    'dispatch: loop {
        match pc {
            0x825FA9F8 => {
    //   block [0x825FA9F8..0x825FAA10)
	// 825FA9F8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FA9FC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FAA00: 394A9708  addi r10, r10, -0x68f8
	ctx.r[10].s64 = ctx.r[10].s64 + -26872;
	// 825FAA04: 816B1AB0  lwz r11, 0x1ab0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6832 as u32) ) } as u64;
	// 825FAA08: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825FAA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAA10 size=116
    let mut pc: u32 = 0x825FAA10;
    'dispatch: loop {
        match pc {
            0x825FAA10 => {
    //   block [0x825FAA10..0x825FAA84)
	// 825FAA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAA1C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FAA20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FAA24: 390B9708  addi r8, r11, -0x68f8
	ctx.r[8].s64 = ctx.r[11].s64 + -26872;
	// 825FAA28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAA2C: 392A8618  addi r9, r10, -0x79e8
	ctx.r[9].s64 = ctx.r[10].s64 + -31208;
	// 825FAA30: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAA34: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 825FAA38: 38AAFF9C  addi r5, r10, -0x64
	ctx.r[5].s64 = ctx.r[10].s64 + -100;
	// 825FAA3C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FAA40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FAA44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAA48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAA4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAA54: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FAA58: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 825FAA5C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FAA60: 386BFFCC  addi r3, r11, -0x34
	ctx.r[3].s64 = ctx.r[11].s64 + -52;
	// 825FAA64: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FAA68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAA6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAA70: 4BE6C3B1  bl 0x82466e20
	ctx.lr = 0x825FAA74;
	sub_82466E20(ctx, base);
	// 825FAA74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAA78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAA7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAA80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAA88 size=112
    let mut pc: u32 = 0x825FAA88;
    'dispatch: loop {
        match pc {
            0x825FAA88 => {
    //   block [0x825FAA88..0x825FAAF8)
	// 825FAA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAA90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAA94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAA98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FAA9C: 38AAFF9C  addi r5, r10, -0x64
	ctx.r[5].s64 = ctx.r[10].s64 + -100;
	// 825FAAA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FAAA4: 390B1AB8  addi r8, r11, 0x1ab8
	ctx.r[8].s64 = ctx.r[11].s64 + 6840;
	// 825FAAA8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 825FAAAC: 388AAAE8  addi r4, r10, -0x5518
	ctx.r[4].s64 = ctx.r[10].s64 + -21784;
	// 825FAAB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAAB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAAB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FAABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAAC0: 386AFFFC  addi r3, r10, -4
	ctx.r[3].s64 = ctx.r[10].s64 + -4;
	// 825FAAC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FAAC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FAACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FAAD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FAAD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAAD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAAE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAAE4: 4BE6C33D  bl 0x82466e20
	ctx.lr = 0x825FAAE8;
	sub_82466E20(ctx, base);
	// 825FAAE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAAEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAAF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAAF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAAF8 size=112
    let mut pc: u32 = 0x825FAAF8;
    'dispatch: loop {
        match pc {
            0x825FAAF8 => {
    //   block [0x825FAAF8..0x825FAB68)
	// 825FAAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAB00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAB04: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FAB08: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825FAB0C: 38EA1BA8  addi r7, r10, 0x1ba8
	ctx.r[7].s64 = ctx.r[10].s64 + 7080;
	// 825FAB10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAB14: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FAB18: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 825FAB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FAB20: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FAB24: 396B8640  addi r11, r11, -0x79c0
	ctx.r[11].s64 = ctx.r[11].s64 + -31168;
	// 825FAB28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FAB2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAB30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAB34: 386A002C  addi r3, r10, 0x2c
	ctx.r[3].s64 = ctx.r[10].s64 + 44;
	// 825FAB38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FAB3C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FAB40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAB44: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FAB48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAB4C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAB50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FAB54: 4BE6C2CD  bl 0x82466e20
	ctx.lr = 0x825FAB58;
	sub_82466E20(ctx, base);
	// 825FAB58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAB5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAB60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAB64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAB68 size=112
    let mut pc: u32 = 0x825FAB68;
    'dispatch: loop {
        match pc {
            0x825FAB68 => {
    //   block [0x825FAB68..0x825FABD8)
	// 825FAB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAB70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAB74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAB78: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FAB7C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FAB80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAB84: 390B1C38  addi r8, r11, 0x1c38
	ctx.r[8].s64 = ctx.r[11].s64 + 7224;
	// 825FAB88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FAB8C: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 825FAB90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAB94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAB98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FAB9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FABA0: 386A005C  addi r3, r10, 0x5c
	ctx.r[3].s64 = ctx.r[10].s64 + 92;
	// 825FABA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FABA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FABAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FABB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FABB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FABB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FABBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FABC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FABC4: 4BE6C25D  bl 0x82466e20
	ctx.lr = 0x825FABC8;
	sub_82466E20(ctx, base);
	// 825FABC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FABCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FABD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FABD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FABD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FABD8 size=24
    let mut pc: u32 = 0x825FABD8;
    'dispatch: loop {
        match pc {
            0x825FABD8 => {
    //   block [0x825FABD8..0x825FABF0)
	// 825FABD8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FABDC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FABE0: 394A9828  addi r10, r10, -0x67d8
	ctx.r[10].s64 = ctx.r[10].s64 + -26584;
	// 825FABE4: 816B1AB4  lwz r11, 0x1ab4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6836 as u32) ) } as u64;
	// 825FABE8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825FABEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FABF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FABF0 size=112
    let mut pc: u32 = 0x825FABF0;
    'dispatch: loop {
        match pc {
            0x825FABF0 => {
    //   block [0x825FABF0..0x825FAC60)
	// 825FABF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FABF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FABF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FABFC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FAC00: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FAC04: 392A868C  addi r9, r10, -0x7974
	ctx.r[9].s64 = ctx.r[10].s64 + -31092;
	// 825FAC08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAC0C: 390B9828  addi r8, r11, -0x67d8
	ctx.r[8].s64 = ctx.r[11].s64 + -26584;
	// 825FAC10: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825FAC14: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 825FAC18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAC1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAC20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FAC24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAC28: 386A008C  addi r3, r10, 0x8c
	ctx.r[3].s64 = ctx.r[10].s64 + 140;
	// 825FAC2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FAC30: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FAC34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAC38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FAC3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAC40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAC44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FAC48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAC4C: 4BE6C1D5  bl 0x82466e20
	ctx.lr = 0x825FAC50;
	sub_82466E20(ctx, base);
	// 825FAC50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAC60 size=108
    let mut pc: u32 = 0x825FAC60;
    'dispatch: loop {
        match pc {
            0x825FAC60 => {
    //   block [0x825FAC60..0x825FACCC)
	// 825FAC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAC68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAC6C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FAC70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAC74: 38EB1C58  addi r7, r11, 0x1c58
	ctx.r[7].s64 = ctx.r[11].s64 + 7256;
	// 825FAC78: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FAC7C: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 825FAC80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAC84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAC88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FAC8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FAC90: 386A00BC  addi r3, r10, 0xbc
	ctx.r[3].s64 = ctx.r[10].s64 + 188;
	// 825FAC94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FAC98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FAC9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FACA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FACA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FACA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FACAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FACB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FACB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FACB8: 4BE6C169  bl 0x82466e20
	ctx.lr = 0x825FACBC;
	sub_82466E20(ctx, base);
	// 825FACBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FACC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FACC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FACC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FACD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FACD0 size=108
    let mut pc: u32 = 0x825FACD0;
    'dispatch: loop {
        match pc {
            0x825FACD0 => {
    //   block [0x825FACD0..0x825FAD3C)
	// 825FACD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FACD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FACD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FACDC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FACE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FACE4: 38EB1CB8  addi r7, r11, 0x1cb8
	ctx.r[7].s64 = ctx.r[11].s64 + 7352;
	// 825FACE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FACEC: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 825FACF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FACF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FACF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FACFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FAD00: 386A00EC  addi r3, r10, 0xec
	ctx.r[3].s64 = ctx.r[10].s64 + 236;
	// 825FAD04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FAD08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FAD0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAD10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FAD14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAD18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAD1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAD20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAD24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FAD28: 4BE6C0F9  bl 0x82466e20
	ctx.lr = 0x825FAD2C;
	sub_82466E20(ctx, base);
	// 825FAD2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAD30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAD34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAD38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAD40 size=116
    let mut pc: u32 = 0x825FAD40;
    'dispatch: loop {
        match pc {
            0x825FAD40 => {
    //   block [0x825FAD40..0x825FADB4)
	// 825FAD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAD44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAD48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAD4C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FAD50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FAD54: 390B1CE8  addi r8, r11, 0x1ce8
	ctx.r[8].s64 = ctx.r[11].s64 + 7400;
	// 825FAD58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAD5C: 392A86B8  addi r9, r10, -0x7948
	ctx.r[9].s64 = ctx.r[10].s64 + -31048;
	// 825FAD60: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAD64: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825FAD68: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FAD6C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FAD70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FAD74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAD78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAD7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAD80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAD84: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FAD88: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 825FAD8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FAD90: 386B011C  addi r3, r11, 0x11c
	ctx.r[3].s64 = ctx.r[11].s64 + 284;
	// 825FAD94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FAD98: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAD9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FADA0: 4BE6C081  bl 0x82466e20
	ctx.lr = 0x825FADA4;
	sub_82466E20(ctx, base);
	// 825FADA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FADA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FADAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FADB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FADB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FADB8 size=108
    let mut pc: u32 = 0x825FADB8;
    'dispatch: loop {
        match pc {
            0x825FADB8 => {
    //   block [0x825FADB8..0x825FAE24)
	// 825FADB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FADBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FADC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FADC4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FADC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FADCC: 38EB1D00  addi r7, r11, 0x1d00
	ctx.r[7].s64 = ctx.r[11].s64 + 7424;
	// 825FADD0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FADD4: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 825FADD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FADDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FADE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FADE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FADE8: 386A014C  addi r3, r10, 0x14c
	ctx.r[3].s64 = ctx.r[10].s64 + 332;
	// 825FADEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FADF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FADF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FADF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FADFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAE0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FAE10: 4BE6C011  bl 0x82466e20
	ctx.lr = 0x825FAE14;
	sub_82466E20(ctx, base);
	// 825FAE14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAE18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAE1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAE28 size=112
    let mut pc: u32 = 0x825FAE28;
    'dispatch: loop {
        match pc {
            0x825FAE28 => {
    //   block [0x825FAE28..0x825FAE98)
	// 825FAE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAE34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAE38: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FAE3C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FAE40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAE44: 390B1D18  addi r8, r11, 0x1d18
	ctx.r[8].s64 = ctx.r[11].s64 + 7448;
	// 825FAE48: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 825FAE4C: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 825FAE50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAE54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAE58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FAE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAE60: 386A017C  addi r3, r10, 0x17c
	ctx.r[3].s64 = ctx.r[10].s64 + 380;
	// 825FAE64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FAE68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FAE6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FAE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FAE74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAE84: 4BE6BF9D  bl 0x82466e20
	ctx.lr = 0x825FAE88;
	sub_82466E20(ctx, base);
	// 825FAE88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAE98 size=108
    let mut pc: u32 = 0x825FAE98;
    'dispatch: loop {
        match pc {
            0x825FAE98 => {
    //   block [0x825FAE98..0x825FAF04)
	// 825FAE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAEA4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FAEA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAEAC: 38EB1DF0  addi r7, r11, 0x1df0
	ctx.r[7].s64 = ctx.r[11].s64 + 7664;
	// 825FAEB0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825FAEB4: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 825FAEB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAEBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAEC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FAEC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FAEC8: 386A01AC  addi r3, r10, 0x1ac
	ctx.r[3].s64 = ctx.r[10].s64 + 428;
	// 825FAECC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FAED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FAED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FAEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAEE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAEE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAEE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAEEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FAEF0: 4BE6BF31  bl 0x82466e20
	ctx.lr = 0x825FAEF4;
	sub_82466E20(ctx, base);
	// 825FAEF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAEF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAEFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAF00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAF08 size=108
    let mut pc: u32 = 0x825FAF08;
    'dispatch: loop {
        match pc {
            0x825FAF08 => {
    //   block [0x825FAF08..0x825FAF74)
	// 825FAF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAF14: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FAF18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAF1C: 38EB1E68  addi r7, r11, 0x1e68
	ctx.r[7].s64 = ctx.r[11].s64 + 7784;
	// 825FAF20: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825FAF24: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 825FAF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAF2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAF30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FAF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FAF38: 386A01DC  addi r3, r10, 0x1dc
	ctx.r[3].s64 = ctx.r[10].s64 + 476;
	// 825FAF3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FAF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FAF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FAF4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAF54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAF5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FAF60: 4BE6BEC1  bl 0x82466e20
	ctx.lr = 0x825FAF64;
	sub_82466E20(ctx, base);
	// 825FAF64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAF70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FAF78 size=112
    let mut pc: u32 = 0x825FAF78;
    'dispatch: loop {
        match pc {
            0x825FAF78 => {
    //   block [0x825FAF78..0x825FAFE8)
	// 825FAF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FAF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FAF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FAF84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAF88: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FAF8C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FAF90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FAF94: 390B1EB0  addi r8, r11, 0x1eb0
	ctx.r[8].s64 = ctx.r[11].s64 + 7856;
	// 825FAF98: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 825FAF9C: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 825FAFA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FAFA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FAFA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FAFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FAFB0: 386A020C  addi r3, r10, 0x20c
	ctx.r[3].s64 = ctx.r[10].s64 + 524;
	// 825FAFB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FAFB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FAFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FAFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FAFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FAFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FAFCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FAFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FAFD4: 4BE6BE4D  bl 0x82466e20
	ctx.lr = 0x825FAFD8;
	sub_82466E20(ctx, base);
	// 825FAFD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FAFDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FAFE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FAFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FAFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FAFE8 size=76
    let mut pc: u32 = 0x825FAFE8;
    'dispatch: loop {
        match pc {
            0x825FAFE8 => {
    //   block [0x825FAFE8..0x825FB034)
	// 825FAFE8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FAFEC: 814B1C54  lwz r10, 0x1c54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7252 as u32) ) } as u64;
	// 825FAFF0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FAFF4: 396B9858  addi r11, r11, -0x67a8
	ctx.r[11].s64 = ctx.r[11].s64 + -26536;
	// 825FAFF8: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 825FAFFC: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 825FB000: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 825FB004: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 825FB008: 914B0158  stw r10, 0x158(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 825FB00C: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 825FB010: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FB014: 814A2090  lwz r10, 0x2090(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8336 as u32) ) } as u64;
	// 825FB018: 914B0188  stw r10, 0x188(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 825FB01C: 914B01A0  stw r10, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[10].u32 ) };
	// 825FB020: 914B01B8  stw r10, 0x1b8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 825FB024: 914B01D0  stw r10, 0x1d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 825FB028: 914B01E8  stw r10, 0x1e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), ctx.r[10].u32 ) };
	// 825FB02C: 914B0200  stw r10, 0x200(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(512 as u32), ctx.r[10].u32 ) };
	// 825FB030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB038 size=108
    let mut pc: u32 = 0x825FB038;
    'dispatch: loop {
        match pc {
            0x825FB038 => {
    //   block [0x825FB038..0x825FB0A4)
	// 825FB038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB044: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FB048: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB04C: 38EB9858  addi r7, r11, -0x67a8
	ctx.r[7].s64 = ctx.r[11].s64 + -26536;
	// 825FB050: 3900001A  li r8, 0x1a
	ctx.r[8].s64 = 26;
	// 825FB054: 388AABE8  addi r4, r10, -0x5418
	ctx.r[4].s64 = ctx.r[10].s64 + -21528;
	// 825FB058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB05C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB060: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FB064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB068: 386A023C  addi r3, r10, 0x23c
	ctx.r[3].s64 = ctx.r[10].s64 + 572;
	// 825FB06C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FB070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB07C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB08C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FB090: 4BE6BD91  bl 0x82466e20
	ctx.lr = 0x825FB094;
	sub_82466E20(ctx, base);
	// 825FB094: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB09C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB0A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FB0A8 size=76
    let mut pc: u32 = 0x825FB0A8;
    'dispatch: loop {
        match pc {
            0x825FB0A8 => {
    //   block [0x825FB0A8..0x825FB0F4)
	// 825FB0A8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB0AC: 814B1C54  lwz r10, 0x1c54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7252 as u32) ) } as u64;
	// 825FB0B0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FB0B4: 396B9AC8  addi r11, r11, -0x6538
	ctx.r[11].s64 = ctx.r[11].s64 + -25912;
	// 825FB0B8: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 825FB0BC: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 825FB0C0: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 825FB0C4: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 825FB0C8: 914B0158  stw r10, 0x158(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 825FB0CC: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 825FB0D0: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FB0D4: 814A2090  lwz r10, 0x2090(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8336 as u32) ) } as u64;
	// 825FB0D8: 914B0188  stw r10, 0x188(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 825FB0DC: 914B01A0  stw r10, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[10].u32 ) };
	// 825FB0E0: 914B01B8  stw r10, 0x1b8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 825FB0E4: 914B01D0  stw r10, 0x1d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 825FB0E8: 914B01E8  stw r10, 0x1e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), ctx.r[10].u32 ) };
	// 825FB0EC: 914B0200  stw r10, 0x200(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(512 as u32), ctx.r[10].u32 ) };
	// 825FB0F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB0F8 size=116
    let mut pc: u32 = 0x825FB0F8;
    'dispatch: loop {
        match pc {
            0x825FB0F8 => {
    //   block [0x825FB0F8..0x825FB16C)
	// 825FB0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB104: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FB108: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB10C: 390B9AC8  addi r8, r11, -0x6538
	ctx.r[8].s64 = ctx.r[11].s64 + -25912;
	// 825FB110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB114: 392A8704  addi r9, r10, -0x78fc
	ctx.r[9].s64 = ctx.r[10].s64 + -30972;
	// 825FB118: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB11C: 38E00028  li r7, 0x28
	ctx.r[7].s64 = 40;
	// 825FB120: 38AAF5DC  addi r5, r10, -0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + -2596;
	// 825FB124: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB12C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB13C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FB140: 388AACDC  addi r4, r10, -0x5324
	ctx.r[4].s64 = ctx.r[10].s64 + -21284;
	// 825FB144: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FB148: 386B026C  addi r3, r11, 0x26c
	ctx.r[3].s64 = ctx.r[11].s64 + 620;
	// 825FB14C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FB150: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB158: 4BE6BCC9  bl 0x82466e20
	ctx.lr = 0x825FB15C;
	sub_82466E20(ctx, base);
	// 825FB15C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB170 size=112
    let mut pc: u32 = 0x825FB170;
    'dispatch: loop {
        match pc {
            0x825FB170 => {
    //   block [0x825FB170..0x825FB1E0)
	// 825FB170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB17C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB180: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB184: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825FB188: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB18C: 390B2098  addi r8, r11, 0x2098
	ctx.r[8].s64 = ctx.r[11].s64 + 8344;
	// 825FB190: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FB194: 388AACF4  addi r4, r10, -0x530c
	ctx.r[4].s64 = ctx.r[10].s64 + -21260;
	// 825FB198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB19C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB1A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB1A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB1A8: 386A029C  addi r3, r10, 0x29c
	ctx.r[3].s64 = ctx.r[10].s64 + 668;
	// 825FB1AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FB1B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB1B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB1B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB1BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB1C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB1C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB1C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB1CC: 4BE6BC55  bl 0x82466e20
	ctx.lr = 0x825FB1D0;
	sub_82466E20(ctx, base);
	// 825FB1D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB1D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB1D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB1DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB1E0 size=108
    let mut pc: u32 = 0x825FB1E0;
    'dispatch: loop {
        match pc {
            0x825FB1E0 => {
    //   block [0x825FB1E0..0x825FB24C)
	// 825FB1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB1E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB1EC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB1F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB1F4: 38EB20E0  addi r7, r11, 0x20e0
	ctx.r[7].s64 = ctx.r[11].s64 + 8416;
	// 825FB1F8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825FB1FC: 388AAD10  addi r4, r10, -0x52f0
	ctx.r[4].s64 = ctx.r[10].s64 + -21232;
	// 825FB200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB204: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB208: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FB20C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB210: 386A02CC  addi r3, r10, 0x2cc
	ctx.r[3].s64 = ctx.r[10].s64 + 716;
	// 825FB214: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FB218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB21C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB22C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB234: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FB238: 4BE6BBE9  bl 0x82466e20
	ctx.lr = 0x825FB23C;
	sub_82466E20(ctx, base);
	// 825FB23C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB250 size=108
    let mut pc: u32 = 0x825FB250;
    'dispatch: loop {
        match pc {
            0x825FB250 => {
    //   block [0x825FB250..0x825FB2BC)
	// 825FB250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB25C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB260: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB264: 38EB2128  addi r7, r11, 0x2128
	ctx.r[7].s64 = ctx.r[11].s64 + 8488;
	// 825FB268: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825FB26C: 388AAD38  addi r4, r10, -0x52c8
	ctx.r[4].s64 = ctx.r[10].s64 + -21192;
	// 825FB270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB274: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB278: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FB27C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB280: 386A02FC  addi r3, r10, 0x2fc
	ctx.r[3].s64 = ctx.r[10].s64 + 764;
	// 825FB284: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FB288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB28C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB29C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB2A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB2A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FB2A8: 4BE6BB79  bl 0x82466e20
	ctx.lr = 0x825FB2AC;
	sub_82466E20(ctx, base);
	// 825FB2AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB2B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB2B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB2B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB2C0 size=116
    let mut pc: u32 = 0x825FB2C0;
    'dispatch: loop {
        match pc {
            0x825FB2C0 => {
    //   block [0x825FB2C0..0x825FB334)
	// 825FB2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB2C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB2CC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FB2D0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825FB2D4: 390A2170  addi r8, r10, 0x2170
	ctx.r[8].s64 = ctx.r[10].s64 + 8560;
	// 825FB2D8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB2DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FB2E0: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825FB2E4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB2E8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FB2EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB2F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB2F4: 388AAD60  addi r4, r10, -0x52a0
	ctx.r[4].s64 = ctx.r[10].s64 + -21152;
	// 825FB2F8: 396B872C  addi r11, r11, -0x78d4
	ctx.r[11].s64 = ctx.r[11].s64 + -30932;
	// 825FB2FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB300: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB304: 386A032C  addi r3, r10, 0x32c
	ctx.r[3].s64 = ctx.r[10].s64 + 812;
	// 825FB308: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FB30C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB310: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FB314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB320: 4BE6BB01  bl 0x82466e20
	ctx.lr = 0x825FB324;
	sub_82466E20(ctx, base);
	// 825FB324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB32C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB338 size=116
    let mut pc: u32 = 0x825FB338;
    'dispatch: loop {
        match pc {
            0x825FB338 => {
    //   block [0x825FB338..0x825FB3AC)
	// 825FB338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB344: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FB348: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825FB34C: 390A2218  addi r8, r10, 0x2218
	ctx.r[8].s64 = ctx.r[10].s64 + 8728;
	// 825FB350: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB354: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FB358: 38AA032C  addi r5, r10, 0x32c
	ctx.r[5].s64 = ctx.r[10].s64 + 812;
	// 825FB35C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB360: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FB364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB36C: 388AAD7C  addi r4, r10, -0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + -21124;
	// 825FB370: 396B8750  addi r11, r11, -0x78b0
	ctx.r[11].s64 = ctx.r[11].s64 + -30896;
	// 825FB374: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB378: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB37C: 386A035C  addi r3, r10, 0x35c
	ctx.r[3].s64 = ctx.r[10].s64 + 860;
	// 825FB380: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FB384: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB388: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FB38C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB398: 4BE6BA89  bl 0x82466e20
	ctx.lr = 0x825FB39C;
	sub_82466E20(ctx, base);
	// 825FB39C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB3A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB3A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB3A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FB3B0 size=36
    let mut pc: u32 = 0x825FB3B0;
    'dispatch: loop {
        match pc {
            0x825FB3B0 => {
    //   block [0x825FB3B0..0x825FB3D4)
	// 825FB3B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB3B4: 814B2094  lwz r10, 0x2094(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8340 as u32) ) } as u64;
	// 825FB3B8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FB3BC: 396B9E88  addi r11, r11, -0x6178
	ctx.r[11].s64 = ctx.r[11].s64 + -24952;
	// 825FB3C0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825FB3C4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FB3C8: 814A2260  lwz r10, 0x2260(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8800 as u32) ) } as u64;
	// 825FB3CC: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 825FB3D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB3D8 size=116
    let mut pc: u32 = 0x825FB3D8;
    'dispatch: loop {
        match pc {
            0x825FB3D8 => {
    //   block [0x825FB3D8..0x825FB44C)
	// 825FB3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB3E4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FB3E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB3EC: 390B9E88  addi r8, r11, -0x6178
	ctx.r[8].s64 = ctx.r[11].s64 + -24952;
	// 825FB3F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB3F4: 392A8790  addi r9, r10, -0x7870
	ctx.r[9].s64 = ctx.r[10].s64 + -30832;
	// 825FB3F8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB3FC: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 825FB400: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB404: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB40C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB41C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FB420: 388AAE18  addi r4, r10, -0x51e8
	ctx.r[4].s64 = ctx.r[10].s64 + -20968;
	// 825FB424: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FB428: 386B038C  addi r3, r11, 0x38c
	ctx.r[3].s64 = ctx.r[11].s64 + 908;
	// 825FB42C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FB430: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB438: 4BE6B9E9  bl 0x82466e20
	ctx.lr = 0x825FB43C;
	sub_82466E20(ctx, base);
	// 825FB43C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB450 size=112
    let mut pc: u32 = 0x825FB450;
    'dispatch: loop {
        match pc {
            0x825FB450 => {
    //   block [0x825FB450..0x825FB4C0)
	// 825FB450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB45C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB460: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB464: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB468: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB46C: 390B2268  addi r8, r11, 0x2268
	ctx.r[8].s64 = ctx.r[11].s64 + 8808;
	// 825FB470: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 825FB474: 388AAE30  addi r4, r10, -0x51d0
	ctx.r[4].s64 = ctx.r[10].s64 + -20944;
	// 825FB478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB47C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB480: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB488: 386A03BC  addi r3, r10, 0x3bc
	ctx.r[3].s64 = ctx.r[10].s64 + 956;
	// 825FB48C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FB490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB49C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB4A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB4A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB4A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB4AC: 4BE6B975  bl 0x82466e20
	ctx.lr = 0x825FB4B0;
	sub_82466E20(ctx, base);
	// 825FB4B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB4B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB4B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB4BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB4C0 size=108
    let mut pc: u32 = 0x825FB4C0;
    'dispatch: loop {
        match pc {
            0x825FB4C0 => {
    //   block [0x825FB4C0..0x825FB52C)
	// 825FB4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB4C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB4CC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB4D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB4D4: 38EB2328  addi r7, r11, 0x2328
	ctx.r[7].s64 = ctx.r[11].s64 + 9000;
	// 825FB4D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FB4DC: 388AAE50  addi r4, r10, -0x51b0
	ctx.r[4].s64 = ctx.r[10].s64 + -20912;
	// 825FB4E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB4E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB4E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FB4EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB4F0: 386A03EC  addi r3, r10, 0x3ec
	ctx.r[3].s64 = ctx.r[10].s64 + 1004;
	// 825FB4F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FB4F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB4FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB50C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB514: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FB518: 4BE6B909  bl 0x82466e20
	ctx.lr = 0x825FB51C;
	sub_82466E20(ctx, base);
	// 825FB51C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB530 size=112
    let mut pc: u32 = 0x825FB530;
    'dispatch: loop {
        match pc {
            0x825FB530 => {
    //   block [0x825FB530..0x825FB5A0)
	// 825FB530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB53C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB540: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB544: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB548: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB54C: 390B2358  addi r8, r11, 0x2358
	ctx.r[8].s64 = ctx.r[11].s64 + 9048;
	// 825FB550: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 825FB554: 388AAE6C  addi r4, r10, -0x5194
	ctx.r[4].s64 = ctx.r[10].s64 + -20884;
	// 825FB558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB55C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB560: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB568: 386A041C  addi r3, r10, 0x41c
	ctx.r[3].s64 = ctx.r[10].s64 + 1052;
	// 825FB56C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FB570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB58C: 4BE6B895  bl 0x82466e20
	ctx.lr = 0x825FB590;
	sub_82466E20(ctx, base);
	// 825FB590: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB59C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB5A0 size=112
    let mut pc: u32 = 0x825FB5A0;
    'dispatch: loop {
        match pc {
            0x825FB5A0 => {
    //   block [0x825FB5A0..0x825FB610)
	// 825FB5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB5A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB5AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB5B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB5B4: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB5B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB5BC: 390B2460  addi r8, r11, 0x2460
	ctx.r[8].s64 = ctx.r[11].s64 + 9312;
	// 825FB5C0: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 825FB5C4: 388AAE80  addi r4, r10, -0x5180
	ctx.r[4].s64 = ctx.r[10].s64 + -20864;
	// 825FB5C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB5CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB5D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB5D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB5D8: 386A044C  addi r3, r10, 0x44c
	ctx.r[3].s64 = ctx.r[10].s64 + 1100;
	// 825FB5DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FB5E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB5E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB5E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB5EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB5F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB5F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB5F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB5FC: 4BE6B825  bl 0x82466e20
	ctx.lr = 0x825FB600;
	sub_82466E20(ctx, base);
	// 825FB600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB60C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB610 size=116
    let mut pc: u32 = 0x825FB610;
    'dispatch: loop {
        match pc {
            0x825FB610 => {
    //   block [0x825FB610..0x825FB684)
	// 825FB610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB61C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB620: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB624: 390B2598  addi r8, r11, 0x2598
	ctx.r[8].s64 = ctx.r[11].s64 + 9624;
	// 825FB628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB62C: 392A87C8  addi r9, r10, -0x7838
	ctx.r[9].s64 = ctx.r[10].s64 + -30776;
	// 825FB630: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB634: 38E0001D  li r7, 0x1d
	ctx.r[7].s64 = 29;
	// 825FB638: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB63C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB644: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB64C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB654: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FB658: 388AAE98  addi r4, r10, -0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + -20840;
	// 825FB65C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FB660: 386B047C  addi r3, r11, 0x47c
	ctx.r[3].s64 = ctx.r[11].s64 + 1148;
	// 825FB664: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FB668: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB66C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB670: 4BE6B7B1  bl 0x82466e20
	ctx.lr = 0x825FB674;
	sub_82466E20(ctx, base);
	// 825FB674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB67C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB688 size=112
    let mut pc: u32 = 0x825FB688;
    'dispatch: loop {
        match pc {
            0x825FB688 => {
    //   block [0x825FB688..0x825FB6F8)
	// 825FB688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB694: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB698: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB69C: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB6A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB6A4: 390B2850  addi r8, r11, 0x2850
	ctx.r[8].s64 = ctx.r[11].s64 + 10320;
	// 825FB6A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FB6AC: 388AAEB0  addi r4, r10, -0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + -20816;
	// 825FB6B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB6B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB6B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB6C0: 386A04AC  addi r3, r10, 0x4ac
	ctx.r[3].s64 = ctx.r[10].s64 + 1196;
	// 825FB6C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FB6C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB6CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB6D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB6D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB6D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB6DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB6E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB6E4: 4BE6B73D  bl 0x82466e20
	ctx.lr = 0x825FB6E8;
	sub_82466E20(ctx, base);
	// 825FB6E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB6EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB6F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB6F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB6F8 size=116
    let mut pc: u32 = 0x825FB6F8;
    'dispatch: loop {
        match pc {
            0x825FB6F8 => {
    //   block [0x825FB6F8..0x825FB76C)
	// 825FB6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB704: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FB708: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825FB70C: 390A2898  addi r8, r10, 0x2898
	ctx.r[8].s64 = ctx.r[10].s64 + 10392;
	// 825FB710: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB714: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FB718: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB71C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB720: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FB724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB72C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 825FB730: 396B87DC  addi r11, r11, -0x7824
	ctx.r[11].s64 = ctx.r[11].s64 + -30756;
	// 825FB734: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB738: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB73C: 386A04DC  addi r3, r10, 0x4dc
	ctx.r[3].s64 = ctx.r[10].s64 + 1244;
	// 825FB740: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FB744: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB748: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FB74C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB758: 4BE6B6C9  bl 0x82466e20
	ctx.lr = 0x825FB75C;
	sub_82466E20(ctx, base);
	// 825FB75C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB770 size=112
    let mut pc: u32 = 0x825FB770;
    'dispatch: loop {
        match pc {
            0x825FB770 => {
    //   block [0x825FB770..0x825FB7E0)
	// 825FB770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB77C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB780: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB784: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB788: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB78C: 390B28E0  addi r8, r11, 0x28e0
	ctx.r[8].s64 = ctx.r[11].s64 + 10464;
	// 825FB790: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825FB794: 388AAEF4  addi r4, r10, -0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + -20748;
	// 825FB798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB79C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB7A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB7A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB7A8: 386A050C  addi r3, r10, 0x50c
	ctx.r[3].s64 = ctx.r[10].s64 + 1292;
	// 825FB7AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FB7B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB7B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB7B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB7BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB7C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB7C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB7C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB7CC: 4BE6B655  bl 0x82466e20
	ctx.lr = 0x825FB7D0;
	sub_82466E20(ctx, base);
	// 825FB7D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB7D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB7D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB7DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB7E0 size=112
    let mut pc: u32 = 0x825FB7E0;
    'dispatch: loop {
        match pc {
            0x825FB7E0 => {
    //   block [0x825FB7E0..0x825FB850)
	// 825FB7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB7EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB7F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB7F4: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB7F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB7FC: 390B2970  addi r8, r11, 0x2970
	ctx.r[8].s64 = ctx.r[11].s64 + 10608;
	// 825FB800: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825FB804: 388AAF08  addi r4, r10, -0x50f8
	ctx.r[4].s64 = ctx.r[10].s64 + -20728;
	// 825FB808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB80C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB810: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB818: 386A053C  addi r3, r10, 0x53c
	ctx.r[3].s64 = ctx.r[10].s64 + 1340;
	// 825FB81C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FB820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB82C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB83C: 4BE6B5E5  bl 0x82466e20
	ctx.lr = 0x825FB840;
	sub_82466E20(ctx, base);
	// 825FB840: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FB850 size=24
    let mut pc: u32 = 0x825FB850;
    'dispatch: loop {
        match pc {
            0x825FB850 => {
    //   block [0x825FB850..0x825FB868)
	// 825FB850: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB854: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FB858: 394A9F90  addi r10, r10, -0x6070
	ctx.r[10].s64 = ctx.r[10].s64 + -24688;
	// 825FB85C: 816B29E8  lwz r11, 0x29e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10728 as u32) ) } as u64;
	// 825FB860: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 825FB864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB868 size=116
    let mut pc: u32 = 0x825FB868;
    'dispatch: loop {
        match pc {
            0x825FB868 => {
    //   block [0x825FB868..0x825FB8DC)
	// 825FB868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB874: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FB878: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB87C: 392B8808  addi r9, r11, -0x77f8
	ctx.r[9].s64 = ctx.r[11].s64 + -30712;
	// 825FB880: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB884: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB888: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825FB88C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 825FB890: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FB894: 388AAF24  addi r4, r10, -0x50dc
	ctx.r[4].s64 = ctx.r[10].s64 + -20700;
	// 825FB898: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB89C: 396B9F90  addi r11, r11, -0x6070
	ctx.r[11].s64 = ctx.r[11].s64 + -24688;
	// 825FB8A0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825FB8A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB8A8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825FB8AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB8B0: 386A056C  addi r3, r10, 0x56c
	ctx.r[3].s64 = ctx.r[10].s64 + 1388;
	// 825FB8B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FB8B8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825FB8BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB8C0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825FB8C4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FB8C8: 4BE6B559  bl 0x82466e20
	ctx.lr = 0x825FB8CC;
	sub_82466E20(ctx, base);
	// 825FB8CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB8D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB8D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB8D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB8E0 size=112
    let mut pc: u32 = 0x825FB8E0;
    'dispatch: loop {
        match pc {
            0x825FB8E0 => {
    //   block [0x825FB8E0..0x825FB950)
	// 825FB8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB8E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB8EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB8F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB8F4: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB8F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB8FC: 390B29F0  addi r8, r11, 0x29f0
	ctx.r[8].s64 = ctx.r[11].s64 + 10736;
	// 825FB900: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825FB904: 388AAF40  addi r4, r10, -0x50c0
	ctx.r[4].s64 = ctx.r[10].s64 + -20672;
	// 825FB908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB90C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB910: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB918: 386A059C  addi r3, r10, 0x59c
	ctx.r[3].s64 = ctx.r[10].s64 + 1436;
	// 825FB91C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FB920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FB924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FB928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB92C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB93C: 4BE6B4E5  bl 0x82466e20
	ctx.lr = 0x825FB940;
	sub_82466E20(ctx, base);
	// 825FB940: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FB950 size=24
    let mut pc: u32 = 0x825FB950;
    'dispatch: loop {
        match pc {
            0x825FB950 => {
    //   block [0x825FB950..0x825FB968)
	// 825FB950: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB954: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FB958: 394AA038  addi r10, r10, -0x5fc8
	ctx.r[10].s64 = ctx.r[10].s64 + -24520;
	// 825FB95C: 816B29EC  lwz r11, 0x29ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10732 as u32) ) } as u64;
	// 825FB960: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825FB964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB968 size=116
    let mut pc: u32 = 0x825FB968;
    'dispatch: loop {
        match pc {
            0x825FB968 => {
    //   block [0x825FB968..0x825FB9DC)
	// 825FB968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB974: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FB978: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB97C: 390BA038  addi r8, r11, -0x5fc8
	ctx.r[8].s64 = ctx.r[11].s64 + -24520;
	// 825FB980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FB984: 392A8870  addi r9, r10, -0x7790
	ctx.r[9].s64 = ctx.r[10].s64 + -30608;
	// 825FB988: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB98C: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825FB990: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB994: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FB998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FB99C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB9A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FB9A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FB9A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FB9AC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FB9B0: 388AAF78  addi r4, r10, -0x5088
	ctx.r[4].s64 = ctx.r[10].s64 + -20616;
	// 825FB9B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FB9B8: 386B05CC  addi r3, r11, 0x5cc
	ctx.r[3].s64 = ctx.r[11].s64 + 1484;
	// 825FB9BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FB9C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FB9C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FB9C8: 4BE6B459  bl 0x82466e20
	ctx.lr = 0x825FB9CC;
	sub_82466E20(ctx, base);
	// 825FB9CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FB9D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FB9D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FB9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FB9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FB9E0 size=112
    let mut pc: u32 = 0x825FB9E0;
    'dispatch: loop {
        match pc {
            0x825FB9E0 => {
    //   block [0x825FB9E0..0x825FBA50)
	// 825FB9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FB9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FB9E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FB9EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FB9F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FB9F4: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FB9F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FB9FC: 390B2A68  addi r8, r11, 0x2a68
	ctx.r[8].s64 = ctx.r[11].s64 + 10856;
	// 825FBA00: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 825FBA04: 388AAF98  addi r4, r10, -0x5068
	ctx.r[4].s64 = ctx.r[10].s64 + -20584;
	// 825FBA08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBA0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBA10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FBA14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBA18: 386A05FC  addi r3, r10, 0x5fc
	ctx.r[3].s64 = ctx.r[10].s64 + 1532;
	// 825FBA1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FBA20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBA24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBA28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBA2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBA30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBA34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBA38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBA3C: 4BE6B3E5  bl 0x82466e20
	ctx.lr = 0x825FBA40;
	sub_82466E20(ctx, base);
	// 825FBA40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBA44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBA48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBA4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBA50 size=112
    let mut pc: u32 = 0x825FBA50;
    'dispatch: loop {
        match pc {
            0x825FBA50 => {
    //   block [0x825FBA50..0x825FBAC0)
	// 825FBA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBA58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBA5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBA60: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBA64: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FBA68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBA6C: 390B2B58  addi r8, r11, 0x2b58
	ctx.r[8].s64 = ctx.r[11].s64 + 11096;
	// 825FBA70: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825FBA74: 388AAFB4  addi r4, r10, -0x504c
	ctx.r[4].s64 = ctx.r[10].s64 + -20556;
	// 825FBA78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBA7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBA80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FBA84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBA88: 386A062C  addi r3, r10, 0x62c
	ctx.r[3].s64 = ctx.r[10].s64 + 1580;
	// 825FBA8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FBA90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBA94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBA98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBA9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBAA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBAA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBAA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBAAC: 4BE6B375  bl 0x82466e20
	ctx.lr = 0x825FBAB0;
	sub_82466E20(ctx, base);
	// 825FBAB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBAB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBAB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FBAC0 size=24
    let mut pc: u32 = 0x825FBAC0;
    'dispatch: loop {
        match pc {
            0x825FBAC0 => {
    //   block [0x825FBAC0..0x825FBAD8)
	// 825FBAC0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBAC4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FBAC8: 394AA0E0  addi r10, r10, -0x5f20
	ctx.r[10].s64 = ctx.r[10].s64 + -24352;
	// 825FBACC: 816B2BB8  lwz r11, 0x2bb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11192 as u32) ) } as u64;
	// 825FBAD0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 825FBAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBAD8 size=116
    let mut pc: u32 = 0x825FBAD8;
    'dispatch: loop {
        match pc {
            0x825FBAD8 => {
    //   block [0x825FBAD8..0x825FBB4C)
	// 825FBAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBAE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBAE4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FBAE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBAEC: 390BA0E0  addi r8, r11, -0x5f20
	ctx.r[8].s64 = ctx.r[11].s64 + -24352;
	// 825FBAF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBAF4: 392A88BC  addi r9, r10, -0x7744
	ctx.r[9].s64 = ctx.r[10].s64 + -30532;
	// 825FBAF8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBAFC: 38E00019  li r7, 0x19
	ctx.r[7].s64 = 25;
	// 825FBB00: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FBB04: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FBB08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBB0C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBB10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBB14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBB18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBB1C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FBB20: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 825FBB24: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FBB28: 386B065C  addi r3, r11, 0x65c
	ctx.r[3].s64 = ctx.r[11].s64 + 1628;
	// 825FBB2C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FBB30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBB34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBB38: 4BE6B2E9  bl 0x82466e20
	ctx.lr = 0x825FBB3C;
	sub_82466E20(ctx, base);
	// 825FBB3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBB40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBB44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBB48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBB50 size=116
    let mut pc: u32 = 0x825FBB50;
    'dispatch: loop {
        match pc {
            0x825FBB50 => {
    //   block [0x825FBB50..0x825FBBC4)
	// 825FBB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBB5C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FBB60: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBB64: 392B88F4  addi r9, r11, -0x770c
	ctx.r[9].s64 = ctx.r[11].s64 + -30476;
	// 825FBB68: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FBB6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBB70: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825FBB74: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 825FBB78: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBB7C: 388AB168  addi r4, r10, -0x4e98
	ctx.r[4].s64 = ctx.r[10].s64 + -20120;
	// 825FBB80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBB84: 396B2BC8  addi r11, r11, 0x2bc8
	ctx.r[11].s64 = ctx.r[11].s64 + 11208;
	// 825FBB88: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825FBB8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBB90: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825FBB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBB98: 386A068C  addi r3, r10, 0x68c
	ctx.r[3].s64 = ctx.r[10].s64 + 1676;
	// 825FBB9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FBBA0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825FBBA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBBA8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825FBBAC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FBBB0: 4BE6B271  bl 0x82466e20
	ctx.lr = 0x825FBBB4;
	sub_82466E20(ctx, base);
	// 825FBBB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBBB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBBBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBBC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBBC8 size=112
    let mut pc: u32 = 0x825FBBC8;
    'dispatch: loop {
        match pc {
            0x825FBBC8 => {
    //   block [0x825FBBC8..0x825FBC38)
	// 825FBBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBBD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBBD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBBD8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBBDC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FBBE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBBE4: 390B2D90  addi r8, r11, 0x2d90
	ctx.r[8].s64 = ctx.r[11].s64 + 11664;
	// 825FBBE8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825FBBEC: 388AB188  addi r4, r10, -0x4e78
	ctx.r[4].s64 = ctx.r[10].s64 + -20088;
	// 825FBBF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBBF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBBF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FBBFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBC00: 386A06BC  addi r3, r10, 0x6bc
	ctx.r[3].s64 = ctx.r[10].s64 + 1724;
	// 825FBC04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FBC08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBC0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBC10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBC14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBC18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBC1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBC20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBC24: 4BE6B1FD  bl 0x82466e20
	ctx.lr = 0x825FBC28;
	sub_82466E20(ctx, base);
	// 825FBC28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBC2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBC30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBC34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FBC38 size=48
    let mut pc: u32 = 0x825FBC38;
    'dispatch: loop {
        match pc {
            0x825FBC38 => {
    //   block [0x825FBC38..0x825FBC68)
	// 825FBC38: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBC3C: 814B2DF0  lwz r10, 0x2df0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11760 as u32) ) } as u64;
	// 825FBC40: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FBC44: 396BA338  addi r11, r11, -0x5cc8
	ctx.r[11].s64 = ctx.r[11].s64 + -23752;
	// 825FBC48: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825FBC4C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FBC50: 814A2DF4  lwz r10, 0x2df4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(11764 as u32) ) } as u64;
	// 825FBC54: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 825FBC58: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FBC5C: 814A2BC4  lwz r10, 0x2bc4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(11204 as u32) ) } as u64;
	// 825FBC60: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 825FBC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBC68 size=116
    let mut pc: u32 = 0x825FBC68;
    'dispatch: loop {
        match pc {
            0x825FBC68 => {
    //   block [0x825FBC68..0x825FBCDC)
	// 825FBC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBC74: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FBC78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBC7C: 390BA338  addi r8, r11, -0x5cc8
	ctx.r[8].s64 = ctx.r[11].s64 + -23752;
	// 825FBC80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBC84: 392A89B0  addi r9, r10, -0x7650
	ctx.r[9].s64 = ctx.r[10].s64 + -30288;
	// 825FBC88: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBC8C: 38E00016  li r7, 0x16
	ctx.r[7].s64 = 22;
	// 825FBC90: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FBC94: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FBC98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBC9C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBCA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBCA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBCA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBCAC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FBCB0: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 825FBCB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FBCB8: 386B06EC  addi r3, r11, 0x6ec
	ctx.r[3].s64 = ctx.r[11].s64 + 1772;
	// 825FBCBC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 825FBCC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBCC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBCC8: 4BE6B159  bl 0x82466e20
	ctx.lr = 0x825FBCCC;
	sub_82466E20(ctx, base);
	// 825FBCCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBCD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBCD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBCD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBCE0 size=112
    let mut pc: u32 = 0x825FBCE0;
    'dispatch: loop {
        match pc {
            0x825FBCE0 => {
    //   block [0x825FBCE0..0x825FBD50)
	// 825FBCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBCE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBCEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBCF0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBCF4: 38AAF60C  addi r5, r10, -0x9f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2548;
	// 825FBCF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBCFC: 390B2DF8  addi r8, r11, 0x2df8
	ctx.r[8].s64 = ctx.r[11].s64 + 11768;
	// 825FBD00: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FBD04: 388AB2C4  addi r4, r10, -0x4d3c
	ctx.r[4].s64 = ctx.r[10].s64 + -19772;
	// 825FBD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBD0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBD10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FBD14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBD18: 386A071C  addi r3, r10, 0x71c
	ctx.r[3].s64 = ctx.r[10].s64 + 1820;
	// 825FBD1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FBD20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBD24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBD28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBD2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBD30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBD34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBD38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBD3C: 4BE6B0E5  bl 0x82466e20
	ctx.lr = 0x825FBD40;
	sub_82466E20(ctx, base);
	// 825FBD40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBD44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBD48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBD4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBD50 size=112
    let mut pc: u32 = 0x825FBD50;
    'dispatch: loop {
        match pc {
            0x825FBD50 => {
    //   block [0x825FBD50..0x825FBDC0)
	// 825FBD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBD58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBD5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBD60: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBD64: 38AAFBAC  addi r5, r10, -0x454
	ctx.r[5].s64 = ctx.r[10].s64 + -1108;
	// 825FBD68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBD6C: 390B2E40  addi r8, r11, 0x2e40
	ctx.r[8].s64 = ctx.r[11].s64 + 11840;
	// 825FBD70: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825FBD74: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 825FBD78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBD7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBD80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FBD84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBD88: 386A074C  addi r3, r10, 0x74c
	ctx.r[3].s64 = ctx.r[10].s64 + 1868;
	// 825FBD8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FBD90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBD94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBD98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBDA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBDA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBDA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBDAC: 4BE6B075  bl 0x82466e20
	ctx.lr = 0x825FBDB0;
	sub_82466E20(ctx, base);
	// 825FBDB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBDB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBDB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBDBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBDC0 size=116
    let mut pc: u32 = 0x825FBDC0;
    'dispatch: loop {
        match pc {
            0x825FBDC0 => {
    //   block [0x825FBDC0..0x825FBE34)
	// 825FBDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBDC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBDCC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FBDD0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825FBDD4: 390A2EA0  addi r8, r10, 0x2ea0
	ctx.r[8].s64 = ctx.r[10].s64 + 11936;
	// 825FBDD8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBDDC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FBDE0: 38AAFFCC  addi r5, r10, -0x34
	ctx.r[5].s64 = ctx.r[10].s64 + -52;
	// 825FBDE4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBDE8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FBDEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBDF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FBDF4: 388AB2F4  addi r4, r10, -0x4d0c
	ctx.r[4].s64 = ctx.r[10].s64 + -19724;
	// 825FBDF8: 396B89EC  addi r11, r11, -0x7614
	ctx.r[11].s64 = ctx.r[11].s64 + -30228;
	// 825FBDFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBE00: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBE04: 386A077C  addi r3, r10, 0x77c
	ctx.r[3].s64 = ctx.r[10].s64 + 1916;
	// 825FBE08: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FBE0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBE10: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FBE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBE18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBE20: 4BE6B001  bl 0x82466e20
	ctx.lr = 0x825FBE24;
	sub_82466E20(ctx, base);
	// 825FBE24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBE28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBE2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBE38 size=108
    let mut pc: u32 = 0x825FBE38;
    'dispatch: loop {
        match pc {
            0x825FBE38 => {
    //   block [0x825FBE38..0x825FBEA4)
	// 825FBE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBE40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBE44: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBE48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBE4C: 38EB2F18  addi r7, r11, 0x2f18
	ctx.r[7].s64 = ctx.r[11].s64 + 12056;
	// 825FBE50: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825FBE54: 388AB318  addi r4, r10, -0x4ce8
	ctx.r[4].s64 = ctx.r[10].s64 + -19688;
	// 825FBE58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBE5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBE60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FBE64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBE68: 386A07AC  addi r3, r10, 0x7ac
	ctx.r[3].s64 = ctx.r[10].s64 + 1964;
	// 825FBE6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FBE70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBE74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBE78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBE7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBE80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBE84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBE88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBE8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FBE90: 4BE6AF91  bl 0x82466e20
	ctx.lr = 0x825FBE94;
	sub_82466E20(ctx, base);
	// 825FBE94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBE98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBE9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBEA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBEA8 size=108
    let mut pc: u32 = 0x825FBEA8;
    'dispatch: loop {
        match pc {
            0x825FBEA8 => {
    //   block [0x825FBEA8..0x825FBF14)
	// 825FBEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBEB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBEB4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBEB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBEBC: 38EB2F60  addi r7, r11, 0x2f60
	ctx.r[7].s64 = ctx.r[11].s64 + 12128;
	// 825FBEC0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825FBEC4: 388AB344  addi r4, r10, -0x4cbc
	ctx.r[4].s64 = ctx.r[10].s64 + -19644;
	// 825FBEC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBECC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FBED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBED8: 386A07DC  addi r3, r10, 0x7dc
	ctx.r[3].s64 = ctx.r[10].s64 + 2012;
	// 825FBEDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FBEE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBEE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBEE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBEEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBEF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBEF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBEF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBEFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FBF00: 4BE6AF21  bl 0x82466e20
	ctx.lr = 0x825FBF04;
	sub_82466E20(ctx, base);
	// 825FBF04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBF08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBF0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBF10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBF18 size=108
    let mut pc: u32 = 0x825FBF18;
    'dispatch: loop {
        match pc {
            0x825FBF18 => {
    //   block [0x825FBF18..0x825FBF84)
	// 825FBF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBF20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBF24: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBF28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBF2C: 38EB2FA8  addi r7, r11, 0x2fa8
	ctx.r[7].s64 = ctx.r[11].s64 + 12200;
	// 825FBF30: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825FBF34: 388AB36C  addi r4, r10, -0x4c94
	ctx.r[4].s64 = ctx.r[10].s64 + -19604;
	// 825FBF38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBF3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBF40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FBF44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBF48: 386A080C  addi r3, r10, 0x80c
	ctx.r[3].s64 = ctx.r[10].s64 + 2060;
	// 825FBF4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FBF50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBF54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBF5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBF64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBF6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FBF70: 4BE6AEB1  bl 0x82466e20
	ctx.lr = 0x825FBF74;
	sub_82466E20(ctx, base);
	// 825FBF74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBF78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBF7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBF80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBF88 size=108
    let mut pc: u32 = 0x825FBF88;
    'dispatch: loop {
        match pc {
            0x825FBF88 => {
    //   block [0x825FBF88..0x825FBFF4)
	// 825FBF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FBF90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FBF94: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FBF98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FBF9C: 38EB2FF0  addi r7, r11, 0x2ff0
	ctx.r[7].s64 = ctx.r[11].s64 + 12272;
	// 825FBFA0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825FBFA4: 388AB398  addi r4, r10, -0x4c68
	ctx.r[4].s64 = ctx.r[10].s64 + -19560;
	// 825FBFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FBFAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FBFB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FBFB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FBFB8: 386A083C  addi r3, r10, 0x83c
	ctx.r[3].s64 = ctx.r[10].s64 + 2108;
	// 825FBFBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FBFC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FBFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FBFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FBFCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FBFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FBFD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FBFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FBFDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FBFE0: 4BE6AE41  bl 0x82466e20
	ctx.lr = 0x825FBFE4;
	sub_82466E20(ctx, base);
	// 825FBFE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FBFE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FBFEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FBFF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FBFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FBFF8 size=108
    let mut pc: u32 = 0x825FBFF8;
    'dispatch: loop {
        match pc {
            0x825FBFF8 => {
    //   block [0x825FBFF8..0x825FC064)
	// 825FBFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FBFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC004: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC008: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC00C: 38EB3098  addi r7, r11, 0x3098
	ctx.r[7].s64 = ctx.r[11].s64 + 12440;
	// 825FC010: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FC014: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 825FC018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC01C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC020: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC028: 386A086C  addi r3, r10, 0x86c
	ctx.r[3].s64 = ctx.r[10].s64 + 2156;
	// 825FC02C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC03C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC04C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC050: 4BE6ADD1  bl 0x82466e20
	ctx.lr = 0x825FC054;
	sub_82466E20(ctx, base);
	// 825FC054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC05C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC068 size=112
    let mut pc: u32 = 0x825FC068;
    'dispatch: loop {
        match pc {
            0x825FC068 => {
    //   block [0x825FC068..0x825FC0D8)
	// 825FC068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC074: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FC078: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC07C: 392A8A2C  addi r9, r10, -0x75d4
	ctx.r[9].s64 = ctx.r[10].s64 + -30164;
	// 825FC080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FC084: 390B30D0  addi r8, r11, 0x30d0
	ctx.r[8].s64 = ctx.r[11].s64 + 12496;
	// 825FC088: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825FC08C: 388A8198  addi r4, r10, -0x7e68
	ctx.r[4].s64 = ctx.r[10].s64 + -32360;
	// 825FC090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC094: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC098: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FC09C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC0A0: 386A089C  addi r3, r10, 0x89c
	ctx.r[3].s64 = ctx.r[10].s64 + 2204;
	// 825FC0A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FC0A8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FC0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC0B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC0B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC0B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC0BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC0C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC0C4: 4BE6AD5D  bl 0x82466e20
	ctx.lr = 0x825FC0C8;
	sub_82466E20(ctx, base);
	// 825FC0C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC0D8 size=108
    let mut pc: u32 = 0x825FC0D8;
    'dispatch: loop {
        match pc {
            0x825FC0D8 => {
    //   block [0x825FC0D8..0x825FC144)
	// 825FC0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC0E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC0E4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC0E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825FC0EC: 38EB3118  addi r7, r11, 0x3118
	ctx.r[7].s64 = ctx.r[11].s64 + 12568;
	// 825FC0F0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825FC0F4: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 825FC0F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC0FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC108: 386A08CC  addi r3, r10, 0x8cc
	ctx.r[3].s64 = ctx.r[10].s64 + 2252;
	// 825FC10C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC11C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC12C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC130: 4BE6ACF1  bl 0x82466e20
	ctx.lr = 0x825FC134;
	sub_82466E20(ctx, base);
	// 825FC134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC13C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC148 size=108
    let mut pc: u32 = 0x825FC148;
    'dispatch: loop {
        match pc {
            0x825FC148 => {
    //   block [0x825FC148..0x825FC1B4)
	// 825FC148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC154: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC158: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC15C: 38EB3190  addi r7, r11, 0x3190
	ctx.r[7].s64 = ctx.r[11].s64 + 12688;
	// 825FC160: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FC164: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 825FC168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC16C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC170: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC178: 386A08FC  addi r3, r10, 0x8fc
	ctx.r[3].s64 = ctx.r[10].s64 + 2300;
	// 825FC17C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC18C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC19C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC1A0: 4BE6AC81  bl 0x82466e20
	ctx.lr = 0x825FC1A4;
	sub_82466E20(ctx, base);
	// 825FC1A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC1A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC1AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC1B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC1B8 size=108
    let mut pc: u32 = 0x825FC1B8;
    'dispatch: loop {
        match pc {
            0x825FC1B8 => {
    //   block [0x825FC1B8..0x825FC224)
	// 825FC1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC1C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC1C4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC1C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC1CC: 38EB31C0  addi r7, r11, 0x31c0
	ctx.r[7].s64 = ctx.r[11].s64 + 12736;
	// 825FC1D0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FC1D4: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 825FC1D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC1DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC1E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC1E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC1E8: 386A092C  addi r3, r10, 0x92c
	ctx.r[3].s64 = ctx.r[10].s64 + 2348;
	// 825FC1EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC1F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC1F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC1FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC20C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC210: 4BE6AC11  bl 0x82466e20
	ctx.lr = 0x825FC214;
	sub_82466E20(ctx, base);
	// 825FC214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC21C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FC228 size=24
    let mut pc: u32 = 0x825FC228;
    'dispatch: loop {
        match pc {
            0x825FC228 => {
    //   block [0x825FC228..0x825FC240)
	// 825FC228: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC22C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FC230: 394AA548  addi r10, r10, -0x5ab8
	ctx.r[10].s64 = ctx.r[10].s64 + -23224;
	// 825FC234: 816B31D8  lwz r11, 0x31d8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12760 as u32) ) } as u64;
	// 825FC238: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825FC23C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC240 size=112
    let mut pc: u32 = 0x825FC240;
    'dispatch: loop {
        match pc {
            0x825FC240 => {
    //   block [0x825FC240..0x825FC2B0)
	// 825FC240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC24C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FC250: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FC254: 392A8A6C  addi r9, r10, -0x7594
	ctx.r[9].s64 = ctx.r[10].s64 + -30100;
	// 825FC258: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC25C: 390BA548  addi r8, r11, -0x5ab8
	ctx.r[8].s64 = ctx.r[11].s64 + -23224;
	// 825FC260: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 825FC264: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 825FC268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC26C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FC274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC278: 386A095C  addi r3, r10, 0x95c
	ctx.r[3].s64 = ctx.r[10].s64 + 2396;
	// 825FC27C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FC280: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FC284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC294: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC29C: 4BE6AB85  bl 0x82466e20
	ctx.lr = 0x825FC2A0;
	sub_82466E20(ctx, base);
	// 825FC2A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC2A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC2A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC2AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC2B0 size=96
    let mut pc: u32 = 0x825FC2B0;
    'dispatch: loop {
        match pc {
            0x825FC2B0 => {
    //   block [0x825FC2B0..0x825FC310)
	// 825FC2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC2B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC2BC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC2C4: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 825FC2C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC2D0: 386A098C  addi r3, r10, 0x98c
	ctx.r[3].s64 = ctx.r[10].s64 + 2444;
	// 825FC2D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC2D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC2DC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FC2E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC2E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC2E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC2EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC2F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FC2F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC2F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FC2FC: 4BE6AB25  bl 0x82466e20
	ctx.lr = 0x825FC300;
	sub_82466E20(ctx, base);
	// 825FC300: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC30C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC310 size=112
    let mut pc: u32 = 0x825FC310;
    'dispatch: loop {
        match pc {
            0x825FC310 => {
    //   block [0x825FC310..0x825FC380)
	// 825FC310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC31C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC320: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC324: 38AA098C  addi r5, r10, 0x98c
	ctx.r[5].s64 = ctx.r[10].s64 + 2444;
	// 825FC328: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC32C: 390B31DC  addi r8, r11, 0x31dc
	ctx.r[8].s64 = ctx.r[11].s64 + 12764;
	// 825FC330: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FC334: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 825FC338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC33C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC340: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FC344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC348: 386A09BC  addi r3, r10, 0x9bc
	ctx.r[3].s64 = ctx.r[10].s64 + 2492;
	// 825FC34C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FC350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC35C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC36C: 4BE6AAB5  bl 0x82466e20
	ctx.lr = 0x825FC370;
	sub_82466E20(ctx, base);
	// 825FC370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC37C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FC380 size=24
    let mut pc: u32 = 0x825FC380;
    'dispatch: loop {
        match pc {
            0x825FC380 => {
    //   block [0x825FC380..0x825FC398)
	// 825FC380: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC384: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FC388: 394AA620  addi r10, r10, -0x59e0
	ctx.r[10].s64 = ctx.r[10].s64 + -23008;
	// 825FC38C: 816B3210  lwz r11, 0x3210(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12816 as u32) ) } as u64;
	// 825FC390: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 825FC394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC398 size=112
    let mut pc: u32 = 0x825FC398;
    'dispatch: loop {
        match pc {
            0x825FC398 => {
    //   block [0x825FC398..0x825FC408)
	// 825FC398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC3A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC3A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FC3A8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FC3AC: 392A8A98  addi r9, r10, -0x7568
	ctx.r[9].s64 = ctx.r[10].s64 + -30056;
	// 825FC3B0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC3B4: 390BA620  addi r8, r11, -0x59e0
	ctx.r[8].s64 = ctx.r[11].s64 + -23008;
	// 825FC3B8: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 825FC3BC: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 825FC3C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC3C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC3C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FC3CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC3D0: 386A09EC  addi r3, r10, 0x9ec
	ctx.r[3].s64 = ctx.r[10].s64 + 2540;
	// 825FC3D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FC3D8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FC3DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC3E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC3E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC3E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC3EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC3F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC3F4: 4BE6AA2D  bl 0x82466e20
	ctx.lr = 0x825FC3F8;
	sub_82466E20(ctx, base);
	// 825FC3F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC408 size=108
    let mut pc: u32 = 0x825FC408;
    'dispatch: loop {
        match pc {
            0x825FC408 => {
    //   block [0x825FC408..0x825FC474)
	// 825FC408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC414: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC418: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC41C: 38EB3218  addi r7, r11, 0x3218
	ctx.r[7].s64 = ctx.r[11].s64 + 12824;
	// 825FC420: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FC424: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 825FC428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC42C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC430: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC438: 386A0A1C  addi r3, r10, 0xa1c
	ctx.r[3].s64 = ctx.r[10].s64 + 2588;
	// 825FC43C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC44C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC45C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC460: 4BE6A9C1  bl 0x82466e20
	ctx.lr = 0x825FC464;
	sub_82466E20(ctx, base);
	// 825FC464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC46C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FC478 size=24
    let mut pc: u32 = 0x825FC478;
    'dispatch: loop {
        match pc {
            0x825FC478 => {
    //   block [0x825FC478..0x825FC490)
	// 825FC478: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC47C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FC480: 394AA710  addi r10, r10, -0x58f0
	ctx.r[10].s64 = ctx.r[10].s64 + -22768;
	// 825FC484: 816B3214  lwz r11, 0x3214(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12820 as u32) ) } as u64;
	// 825FC488: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825FC48C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC490 size=112
    let mut pc: u32 = 0x825FC490;
    'dispatch: loop {
        match pc {
            0x825FC490 => {
    //   block [0x825FC490..0x825FC500)
	// 825FC490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC49C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FC4A0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FC4A4: 392A8AC8  addi r9, r10, -0x7538
	ctx.r[9].s64 = ctx.r[10].s64 + -30008;
	// 825FC4A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC4AC: 390BA710  addi r8, r11, -0x58f0
	ctx.r[8].s64 = ctx.r[11].s64 + -22768;
	// 825FC4B0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 825FC4B4: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 825FC4B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC4BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC4C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FC4C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC4C8: 386A0A4C  addi r3, r10, 0xa4c
	ctx.r[3].s64 = ctx.r[10].s64 + 2636;
	// 825FC4CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FC4D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FC4D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC4D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC4DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC4E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC4E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC4E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC4EC: 4BE6A935  bl 0x82466e20
	ctx.lr = 0x825FC4F0;
	sub_82466E20(ctx, base);
	// 825FC4F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FC500 size=40
    let mut pc: u32 = 0x825FC500;
    'dispatch: loop {
        match pc {
            0x825FC500 => {
    //   block [0x825FC500..0x825FC528)
	// 825FC500: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC504: 814B3248  lwz r10, 0x3248(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12872 as u32) ) } as u64;
	// 825FC508: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FC50C: 396BA770  addi r11, r11, -0x5890
	ctx.r[11].s64 = ctx.r[11].s64 + -22672;
	// 825FC510: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825FC514: 914B0068  stw r10, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 825FC518: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825FC51C: 814A324C  lwz r10, 0x324c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12876 as u32) ) } as u64;
	// 825FC520: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 825FC524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC528 size=112
    let mut pc: u32 = 0x825FC528;
    'dispatch: loop {
        match pc {
            0x825FC528 => {
    //   block [0x825FC528..0x825FC598)
	// 825FC528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC534: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FC538: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FC53C: 392A8C40  addi r9, r10, -0x73c0
	ctx.r[9].s64 = ctx.r[10].s64 + -29632;
	// 825FC540: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC544: 390BA770  addi r8, r11, -0x5890
	ctx.r[8].s64 = ctx.r[11].s64 + -22672;
	// 825FC548: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 825FC54C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 825FC550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC554: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC558: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FC55C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC560: 386A0A7C  addi r3, r10, 0xa7c
	ctx.r[3].s64 = ctx.r[10].s64 + 2684;
	// 825FC564: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FC568: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 825FC56C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC57C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC584: 4BE6A89D  bl 0x82466e20
	ctx.lr = 0x825FC588;
	sub_82466E20(ctx, base);
	// 825FC588: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC58C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC598 size=108
    let mut pc: u32 = 0x825FC598;
    'dispatch: loop {
        match pc {
            0x825FC598 => {
    //   block [0x825FC598..0x825FC604)
	// 825FC598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC5A4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC5A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC5AC: 38EB3254  addi r7, r11, 0x3254
	ctx.r[7].s64 = ctx.r[11].s64 + 12884;
	// 825FC5B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FC5B4: 388A6E48  addi r4, r10, 0x6e48
	ctx.r[4].s64 = ctx.r[10].s64 + 28232;
	// 825FC5B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC5BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC5C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC5C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC5C8: 386A0AAC  addi r3, r10, 0xaac
	ctx.r[3].s64 = ctx.r[10].s64 + 2732;
	// 825FC5CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC5D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC5D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC5D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC5E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC5E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC5E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC5EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC5F0: 4BE6A831  bl 0x82466e20
	ctx.lr = 0x825FC5F4;
	sub_82466E20(ctx, base);
	// 825FC5F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC5F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC5FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC608 size=108
    let mut pc: u32 = 0x825FC608;
    'dispatch: loop {
        match pc {
            0x825FC608 => {
    //   block [0x825FC608..0x825FC674)
	// 825FC608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC614: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC618: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC61C: 38EB3284  addi r7, r11, 0x3284
	ctx.r[7].s64 = ctx.r[11].s64 + 12932;
	// 825FC620: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FC624: 388A6E64  addi r4, r10, 0x6e64
	ctx.r[4].s64 = ctx.r[10].s64 + 28260;
	// 825FC628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC62C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC638: 386A0ADC  addi r3, r10, 0xadc
	ctx.r[3].s64 = ctx.r[10].s64 + 2780;
	// 825FC63C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC64C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC65C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC660: 4BE6A7C1  bl 0x82466e20
	ctx.lr = 0x825FC664;
	sub_82466E20(ctx, base);
	// 825FC664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC66C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC678 size=108
    let mut pc: u32 = 0x825FC678;
    'dispatch: loop {
        match pc {
            0x825FC678 => {
    //   block [0x825FC678..0x825FC6E4)
	// 825FC678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC684: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC688: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC68C: 38EB329C  addi r7, r11, 0x329c
	ctx.r[7].s64 = ctx.r[11].s64 + 12956;
	// 825FC690: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FC694: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 825FC698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC69C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC6A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC6A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC6A8: 386A0B0C  addi r3, r10, 0xb0c
	ctx.r[3].s64 = ctx.r[10].s64 + 2828;
	// 825FC6AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC6B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC6B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC6B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC6C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC6C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC6C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC6CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC6D0: 4BE6A751  bl 0x82466e20
	ctx.lr = 0x825FC6D4;
	sub_82466E20(ctx, base);
	// 825FC6D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC6D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC6DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC6E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FC6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FC6E8 size=108
    let mut pc: u32 = 0x825FC6E8;
    'dispatch: loop {
        match pc {
            0x825FC6E8 => {
    //   block [0x825FC6E8..0x825FC754)
	// 825FC6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FC6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FC6F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FC6F4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FC6F8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825FC6FC: 38EB32D0  addi r7, r11, 0x32d0
	ctx.r[7].s64 = ctx.r[11].s64 + 13008;
	// 825FC700: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825FC704: 388A7BB8  addi r4, r10, 0x7bb8
	ctx.r[4].s64 = ctx.r[10].s64 + 31672;
	// 825FC708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FC70C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FC710: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FC714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FC718: 386A0B3C  addi r3, r10, 0xb3c
	ctx.r[3].s64 = ctx.r[10].s64 + 2876;
	// 825FC71C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FC720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FC724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FC728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FC72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FC730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FC734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FC738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FC73C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FC740: 4BE6A6E1  bl 0x82466e20
	ctx.lr = 0x825FC744;
	sub_82466E20(ctx, base);
	// 825FC744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FC748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FC74C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FC750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


