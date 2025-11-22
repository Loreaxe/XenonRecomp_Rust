pub fn sub_826C4300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C4300 size=28
    let mut pc: u32 = 0x826C4300;
    'dispatch: loop {
        match pc {
            0x826C4300 => {
    //   block [0x826C4300..0x826C431C)
	// 826C4300: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4304: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C4308: 394A9FC8  addi r10, r10, -0x6038
	ctx.r[10].s64 = ctx.r[10].s64 + -24632;
	// 826C430C: 816B4FEC  lwz r11, 0x4fec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20460 as u32) ) } as u64;
	// 826C4310: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826C4314: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826C4318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4320 size=112
    let mut pc: u32 = 0x826C4320;
    'dispatch: loop {
        match pc {
            0x826C4320 => {
    //   block [0x826C4320..0x826C4390)
	// 826C4320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C432C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C4330: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C4334: 392A20F8  addi r9, r10, 0x20f8
	ctx.r[9].s64 = ctx.r[10].s64 + 8440;
	// 826C4338: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C433C: 390B9FC8  addi r8, r11, -0x6038
	ctx.r[8].s64 = ctx.r[11].s64 + -24632;
	// 826C4340: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826C4344: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826C4348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C434C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4350: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C4354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4358: 386A4F64  addi r3, r10, 0x4f64
	ctx.r[3].s64 = ctx.r[10].s64 + 20324;
	// 826C435C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C4360: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826C4364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C436C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4374: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C437C: 4BDA2AA5  bl 0x82466e20
	ctx.lr = 0x826C4380;
	sub_82466E20(ctx, base);
	// 826C4380: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C438C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4390 size=108
    let mut pc: u32 = 0x826C4390;
    'dispatch: loop {
        match pc {
            0x826C4390 => {
    //   block [0x826C4390..0x826C43FC)
	// 826C4390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C439C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C43A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C43A4: 38EB5100  addi r7, r11, 0x5100
	ctx.r[7].s64 = ctx.r[11].s64 + 20736;
	// 826C43A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C43AC: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 826C43B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C43B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C43B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C43BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C43C0: 386A4F94  addi r3, r10, 0x4f94
	ctx.r[3].s64 = ctx.r[10].s64 + 20372;
	// 826C43C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C43C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C43CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C43D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C43D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C43D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C43DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C43E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C43E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C43E8: 4BDA2A39  bl 0x82466e20
	ctx.lr = 0x826C43EC;
	sub_82466E20(ctx, base);
	// 826C43EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C43F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C43F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C43F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4400 size=108
    let mut pc: u32 = 0x826C4400;
    'dispatch: loop {
        match pc {
            0x826C4400 => {
    //   block [0x826C4400..0x826C446C)
	// 826C4400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C440C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4410: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C4414: 38EB5130  addi r7, r11, 0x5130
	ctx.r[7].s64 = ctx.r[11].s64 + 20784;
	// 826C4418: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C441C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 826C4420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4424: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4428: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C442C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4430: 386A4FC4  addi r3, r10, 0x4fc4
	ctx.r[3].s64 = ctx.r[10].s64 + 20420;
	// 826C4434: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C443C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C444C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4458: 4BDA29C9  bl 0x82466e20
	ctx.lr = 0x826C445C;
	sub_82466E20(ctx, base);
	// 826C445C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C4470 size=24
    let mut pc: u32 = 0x826C4470;
    'dispatch: loop {
        match pc {
            0x826C4470 => {
    //   block [0x826C4470..0x826C4488)
	// 826C4470: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4474: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C4478: 394AA088  addi r10, r10, -0x5f78
	ctx.r[10].s64 = ctx.r[10].s64 + -24440;
	// 826C447C: 816B5148  lwz r11, 0x5148(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20808 as u32) ) } as u64;
	// 826C4480: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826C4484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4488 size=112
    let mut pc: u32 = 0x826C4488;
    'dispatch: loop {
        match pc {
            0x826C4488 => {
    //   block [0x826C4488..0x826C44F8)
	// 826C4488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C448C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4494: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C4498: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C449C: 392A214C  addi r9, r10, 0x214c
	ctx.r[9].s64 = ctx.r[10].s64 + 8524;
	// 826C44A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C44A4: 390BA088  addi r8, r11, -0x5f78
	ctx.r[8].s64 = ctx.r[11].s64 + -24440;
	// 826C44A8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826C44AC: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 826C44B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C44B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C44B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C44BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C44C0: 386A4FF4  addi r3, r10, 0x4ff4
	ctx.r[3].s64 = ctx.r[10].s64 + 20468;
	// 826C44C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C44C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C44CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C44D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C44D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C44D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C44DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C44E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C44E4: 4BDA293D  bl 0x82466e20
	ctx.lr = 0x826C44E8;
	sub_82466E20(ctx, base);
	// 826C44E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C44EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C44F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C44F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C44F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C44F8 size=112
    let mut pc: u32 = 0x826C44F8;
    'dispatch: loop {
        match pc {
            0x826C44F8 => {
    //   block [0x826C44F8..0x826C4568)
	// 826C44F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C44FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4504: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C4508: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C450C: 392A2188  addi r9, r10, 0x2188
	ctx.r[9].s64 = ctx.r[10].s64 + 8584;
	// 826C4510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4514: 390B5158  addi r8, r11, 0x5158
	ctx.r[8].s64 = ctx.r[11].s64 + 20824;
	// 826C4518: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826C451C: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 826C4520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4524: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C452C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4530: 386A5024  addi r3, r10, 0x5024
	ctx.r[3].s64 = ctx.r[10].s64 + 20516;
	// 826C4534: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C4538: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826C453C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C454C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4554: 4BDA28CD  bl 0x82466e20
	ctx.lr = 0x826C4558;
	sub_82466E20(ctx, base);
	// 826C4558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C455C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4568 size=108
    let mut pc: u32 = 0x826C4568;
    'dispatch: loop {
        match pc {
            0x826C4568 => {
    //   block [0x826C4568..0x826C45D4)
	// 826C4568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C456C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4574: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4578: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C457C: 38EB51A0  addi r7, r11, 0x51a0
	ctx.r[7].s64 = ctx.r[11].s64 + 20896;
	// 826C4580: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C4584: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826C4588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C458C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4590: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4598: 386A5054  addi r3, r10, 0x5054
	ctx.r[3].s64 = ctx.r[10].s64 + 20564;
	// 826C459C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C45A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C45A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C45A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C45AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C45B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C45B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C45B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C45BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C45C0: 4BDA2861  bl 0x82466e20
	ctx.lr = 0x826C45C4;
	sub_82466E20(ctx, base);
	// 826C45C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C45C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C45CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C45D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C45D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C45D8 size=108
    let mut pc: u32 = 0x826C45D8;
    'dispatch: loop {
        match pc {
            0x826C45D8 => {
    //   block [0x826C45D8..0x826C4644)
	// 826C45D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C45DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C45E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C45E4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C45E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C45EC: 38EB51D0  addi r7, r11, 0x51d0
	ctx.r[7].s64 = ctx.r[11].s64 + 20944;
	// 826C45F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C45F4: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 826C45F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C45FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4600: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4608: 386A5084  addi r3, r10, 0x5084
	ctx.r[3].s64 = ctx.r[10].s64 + 20612;
	// 826C460C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C461C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C462C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4630: 4BDA27F1  bl 0x82466e20
	ctx.lr = 0x826C4634;
	sub_82466E20(ctx, base);
	// 826C4634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C463C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4648 size=112
    let mut pc: u32 = 0x826C4648;
    'dispatch: loop {
        match pc {
            0x826C4648 => {
    //   block [0x826C4648..0x826C46B8)
	// 826C4648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C464C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4654: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C4658: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C465C: 392A21C0  addi r9, r10, 0x21c0
	ctx.r[9].s64 = ctx.r[10].s64 + 8640;
	// 826C4660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4664: 390B5200  addi r8, r11, 0x5200
	ctx.r[8].s64 = ctx.r[11].s64 + 20992;
	// 826C4668: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826C466C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 826C4670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4674: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C467C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4680: 386A50B4  addi r3, r10, 0x50b4
	ctx.r[3].s64 = ctx.r[10].s64 + 20660;
	// 826C4684: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C4688: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C468C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C469C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C46A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C46A4: 4BDA277D  bl 0x82466e20
	ctx.lr = 0x826C46A8;
	sub_82466E20(ctx, base);
	// 826C46A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C46AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C46B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C46B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C46B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C46B8 size=108
    let mut pc: u32 = 0x826C46B8;
    'dispatch: loop {
        match pc {
            0x826C46B8 => {
    //   block [0x826C46B8..0x826C4724)
	// 826C46B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C46BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C46C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C46C4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C46C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C46CC: 38EB5260  addi r7, r11, 0x5260
	ctx.r[7].s64 = ctx.r[11].s64 + 21088;
	// 826C46D0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826C46D4: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 826C46D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C46DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C46E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C46E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C46E8: 386A50E4  addi r3, r10, 0x50e4
	ctx.r[3].s64 = ctx.r[10].s64 + 20708;
	// 826C46EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C46F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C46F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C46F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C46FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C470C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4710: 4BDA2711  bl 0x82466e20
	ctx.lr = 0x826C4714;
	sub_82466E20(ctx, base);
	// 826C4714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C471C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4728 size=108
    let mut pc: u32 = 0x826C4728;
    'dispatch: loop {
        match pc {
            0x826C4728 => {
    //   block [0x826C4728..0x826C4794)
	// 826C4728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C472C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4734: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4738: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C473C: 38EB5350  addi r7, r11, 0x5350
	ctx.r[7].s64 = ctx.r[11].s64 + 21328;
	// 826C4740: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C4744: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 826C4748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C474C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4750: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4758: 386A5114  addi r3, r10, 0x5114
	ctx.r[3].s64 = ctx.r[10].s64 + 20756;
	// 826C475C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C476C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C477C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4780: 4BDA26A1  bl 0x82466e20
	ctx.lr = 0x826C4784;
	sub_82466E20(ctx, base);
	// 826C4784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C478C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4798 size=108
    let mut pc: u32 = 0x826C4798;
    'dispatch: loop {
        match pc {
            0x826C4798 => {
    //   block [0x826C4798..0x826C4804)
	// 826C4798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C479C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C47A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C47A4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C47A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C47AC: 38EB5368  addi r7, r11, 0x5368
	ctx.r[7].s64 = ctx.r[11].s64 + 21352;
	// 826C47B0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826C47B4: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 826C47B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C47BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C47C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C47C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C47C8: 386A5144  addi r3, r10, 0x5144
	ctx.r[3].s64 = ctx.r[10].s64 + 20804;
	// 826C47CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C47D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C47D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C47D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C47DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C47E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C47E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C47E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C47EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C47F0: 4BDA2631  bl 0x82466e20
	ctx.lr = 0x826C47F4;
	sub_82466E20(ctx, base);
	// 826C47F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C47F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C47FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C4808 size=24
    let mut pc: u32 = 0x826C4808;
    'dispatch: loop {
        match pc {
            0x826C4808 => {
    //   block [0x826C4808..0x826C4820)
	// 826C4808: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C480C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C4810: 394AA118  addi r10, r10, -0x5ee8
	ctx.r[10].s64 = ctx.r[10].s64 + -24296;
	// 826C4814: 816B53F8  lwz r11, 0x53f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21496 as u32) ) } as u64;
	// 826C4818: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826C481C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4820 size=108
    let mut pc: u32 = 0x826C4820;
    'dispatch: loop {
        match pc {
            0x826C4820 => {
    //   block [0x826C4820..0x826C488C)
	// 826C4820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C482C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C4830: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4834: 38EBA118  addi r7, r11, -0x5ee8
	ctx.r[7].s64 = ctx.r[11].s64 + -24296;
	// 826C4838: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C483C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 826C4840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4844: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4848: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C484C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4850: 386A5174  addi r3, r10, 0x5174
	ctx.r[3].s64 = ctx.r[10].s64 + 20852;
	// 826C4854: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C485C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C486C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4878: 4BDA25A9  bl 0x82466e20
	ctx.lr = 0x826C487C;
	sub_82466E20(ctx, base);
	// 826C487C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C4890 size=24
    let mut pc: u32 = 0x826C4890;
    'dispatch: loop {
        match pc {
            0x826C4890 => {
    //   block [0x826C4890..0x826C48A8)
	// 826C4890: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4894: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C4898: 394AA148  addi r10, r10, -0x5eb8
	ctx.r[10].s64 = ctx.r[10].s64 + -24248;
	// 826C489C: 816B53F8  lwz r11, 0x53f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21496 as u32) ) } as u64;
	// 826C48A0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826C48A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C48A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C48A8 size=108
    let mut pc: u32 = 0x826C48A8;
    'dispatch: loop {
        match pc {
            0x826C48A8 => {
    //   block [0x826C48A8..0x826C4914)
	// 826C48A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C48AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C48B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C48B4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C48B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C48BC: 38EBA148  addi r7, r11, -0x5eb8
	ctx.r[7].s64 = ctx.r[11].s64 + -24248;
	// 826C48C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C48C4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 826C48C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C48CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C48D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C48D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C48D8: 386A51A4  addi r3, r10, 0x51a4
	ctx.r[3].s64 = ctx.r[10].s64 + 20900;
	// 826C48DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C48E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C48E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C48E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C48EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C48F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C48F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C48F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C48FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4900: 4BDA2521  bl 0x82466e20
	ctx.lr = 0x826C4904;
	sub_82466E20(ctx, base);
	// 826C4904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C490C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4918 size=108
    let mut pc: u32 = 0x826C4918;
    'dispatch: loop {
        match pc {
            0x826C4918 => {
    //   block [0x826C4918..0x826C4984)
	// 826C4918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C491C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4924: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4928: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C492C: 38EB53E0  addi r7, r11, 0x53e0
	ctx.r[7].s64 = ctx.r[11].s64 + 21472;
	// 826C4930: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C4934: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 826C4938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C493C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4940: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4948: 386A51D4  addi r3, r10, 0x51d4
	ctx.r[3].s64 = ctx.r[10].s64 + 20948;
	// 826C494C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C495C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C496C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4970: 4BDA24B1  bl 0x82466e20
	ctx.lr = 0x826C4974;
	sub_82466E20(ctx, base);
	// 826C4974: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C497C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C4988 size=24
    let mut pc: u32 = 0x826C4988;
    'dispatch: loop {
        match pc {
            0x826C4988 => {
    //   block [0x826C4988..0x826C49A0)
	// 826C4988: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C498C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C4990: 394AA178  addi r10, r10, -0x5e88
	ctx.r[10].s64 = ctx.r[10].s64 + -24200;
	// 826C4994: 816B53F8  lwz r11, 0x53f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21496 as u32) ) } as u64;
	// 826C4998: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826C499C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C49A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C49A0 size=108
    let mut pc: u32 = 0x826C49A0;
    'dispatch: loop {
        match pc {
            0x826C49A0 => {
    //   block [0x826C49A0..0x826C4A0C)
	// 826C49A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C49A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C49A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C49AC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C49B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C49B4: 38EBA178  addi r7, r11, -0x5e88
	ctx.r[7].s64 = ctx.r[11].s64 + -24200;
	// 826C49B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C49BC: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 826C49C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C49C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C49C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C49CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C49D0: 386A5204  addi r3, r10, 0x5204
	ctx.r[3].s64 = ctx.r[10].s64 + 20996;
	// 826C49D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C49D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C49DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C49E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C49E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C49E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C49EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C49F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C49F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C49F8: 4BDA2429  bl 0x82466e20
	ctx.lr = 0x826C49FC;
	sub_82466E20(ctx, base);
	// 826C49FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4A10 size=112
    let mut pc: u32 = 0x826C4A10;
    'dispatch: loop {
        match pc {
            0x826C4A10 => {
    //   block [0x826C4A10..0x826C4A80)
	// 826C4A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4A1C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C4A20: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4A24: 392A2204  addi r9, r10, 0x2204
	ctx.r[9].s64 = ctx.r[10].s64 + 8708;
	// 826C4A28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4A2C: 390B53FC  addi r8, r11, 0x53fc
	ctx.r[8].s64 = ctx.r[11].s64 + 21500;
	// 826C4A30: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826C4A34: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 826C4A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4A3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4A40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C4A44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4A48: 386A5234  addi r3, r10, 0x5234
	ctx.r[3].s64 = ctx.r[10].s64 + 21044;
	// 826C4A4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C4A50: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C4A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4A64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4A6C: 4BDA23B5  bl 0x82466e20
	ctx.lr = 0x826C4A70;
	sub_82466E20(ctx, base);
	// 826C4A70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4A80 size=108
    let mut pc: u32 = 0x826C4A80;
    'dispatch: loop {
        match pc {
            0x826C4A80 => {
    //   block [0x826C4A80..0x826C4AEC)
	// 826C4A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4A8C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4A94: 38EB542C  addi r7, r11, 0x542c
	ctx.r[7].s64 = ctx.r[11].s64 + 21548;
	// 826C4A98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C4A9C: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 826C4AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4AA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4AA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4AB0: 386A5264  addi r3, r10, 0x5264
	ctx.r[3].s64 = ctx.r[10].s64 + 21092;
	// 826C4AB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4AD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4AD8: 4BDA2349  bl 0x82466e20
	ctx.lr = 0x826C4ADC;
	sub_82466E20(ctx, base);
	// 826C4ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4AF0 size=108
    let mut pc: u32 = 0x826C4AF0;
    'dispatch: loop {
        match pc {
            0x826C4AF0 => {
    //   block [0x826C4AF0..0x826C4B5C)
	// 826C4AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4AFC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4B04: 38EB545C  addi r7, r11, 0x545c
	ctx.r[7].s64 = ctx.r[11].s64 + 21596;
	// 826C4B08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C4B0C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826C4B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4B14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4B18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4B20: 386A5294  addi r3, r10, 0x5294
	ctx.r[3].s64 = ctx.r[10].s64 + 21140;
	// 826C4B24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4B2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4B44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4B48: 4BDA22D9  bl 0x82466e20
	ctx.lr = 0x826C4B4C;
	sub_82466E20(ctx, base);
	// 826C4B4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4B60 size=112
    let mut pc: u32 = 0x826C4B60;
    'dispatch: loop {
        match pc {
            0x826C4B60 => {
    //   block [0x826C4B60..0x826C4BD0)
	// 826C4B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4B6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4B70: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4B74: 38AA52F4  addi r5, r10, 0x52f4
	ctx.r[5].s64 = ctx.r[10].s64 + 21236;
	// 826C4B78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4B7C: 390B548C  addi r8, r11, 0x548c
	ctx.r[8].s64 = ctx.r[11].s64 + 21644;
	// 826C4B80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C4B84: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 826C4B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4B8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4B90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C4B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4B98: 386A52C4  addi r3, r10, 0x52c4
	ctx.r[3].s64 = ctx.r[10].s64 + 21188;
	// 826C4B9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C4BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4BA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4BBC: 4BDA2265  bl 0x82466e20
	ctx.lr = 0x826C4BC0;
	sub_82466E20(ctx, base);
	// 826C4BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4BD0 size=108
    let mut pc: u32 = 0x826C4BD0;
    'dispatch: loop {
        match pc {
            0x826C4BD0 => {
    //   block [0x826C4BD0..0x826C4C3C)
	// 826C4BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4BDC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4BE4: 38EB54A4  addi r7, r11, 0x54a4
	ctx.r[7].s64 = ctx.r[11].s64 + 21668;
	// 826C4BE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C4BEC: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826C4BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4BF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4BF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4C00: 386A52F4  addi r3, r10, 0x52f4
	ctx.r[3].s64 = ctx.r[10].s64 + 21236;
	// 826C4C04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4C24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4C28: 4BDA21F9  bl 0x82466e20
	ctx.lr = 0x826C4C2C;
	sub_82466E20(ctx, base);
	// 826C4C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4C40 size=108
    let mut pc: u32 = 0x826C4C40;
    'dispatch: loop {
        match pc {
            0x826C4C40 => {
    //   block [0x826C4C40..0x826C4CAC)
	// 826C4C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4C4C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4C54: 38EB54D4  addi r7, r11, 0x54d4
	ctx.r[7].s64 = ctx.r[11].s64 + 21716;
	// 826C4C58: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C4C5C: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 826C4C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4C64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4C68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4C70: 386A5324  addi r3, r10, 0x5324
	ctx.r[3].s64 = ctx.r[10].s64 + 21284;
	// 826C4C74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4C98: 4BDA2189  bl 0x82466e20
	ctx.lr = 0x826C4C9C;
	sub_82466E20(ctx, base);
	// 826C4C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4CB0 size=108
    let mut pc: u32 = 0x826C4CB0;
    'dispatch: loop {
        match pc {
            0x826C4CB0 => {
    //   block [0x826C4CB0..0x826C4D1C)
	// 826C4CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4CBC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4CC4: 38EB54EC  addi r7, r11, 0x54ec
	ctx.r[7].s64 = ctx.r[11].s64 + 21740;
	// 826C4CC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C4CCC: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 826C4CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4CD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4CE0: 386A5354  addi r3, r10, 0x5354
	ctx.r[3].s64 = ctx.r[10].s64 + 21332;
	// 826C4CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4D08: 4BDA2119  bl 0x82466e20
	ctx.lr = 0x826C4D0C;
	sub_82466E20(ctx, base);
	// 826C4D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4D20 size=108
    let mut pc: u32 = 0x826C4D20;
    'dispatch: loop {
        match pc {
            0x826C4D20 => {
    //   block [0x826C4D20..0x826C4D8C)
	// 826C4D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4D2C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4D34: 38EB5520  addi r7, r11, 0x5520
	ctx.r[7].s64 = ctx.r[11].s64 + 21792;
	// 826C4D38: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826C4D3C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826C4D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4D44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4D48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4D50: 386A5384  addi r3, r10, 0x5384
	ctx.r[3].s64 = ctx.r[10].s64 + 21380;
	// 826C4D54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4D74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4D78: 4BDA20A9  bl 0x82466e20
	ctx.lr = 0x826C4D7C;
	sub_82466E20(ctx, base);
	// 826C4D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4D90 size=108
    let mut pc: u32 = 0x826C4D90;
    'dispatch: loop {
        match pc {
            0x826C4D90 => {
    //   block [0x826C4D90..0x826C4DFC)
	// 826C4D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4D9C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4DA4: 38EB55C8  addi r7, r11, 0x55c8
	ctx.r[7].s64 = ctx.r[11].s64 + 21960;
	// 826C4DA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C4DAC: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 826C4DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4DB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4DB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4DC0: 386A53B4  addi r3, r10, 0x53b4
	ctx.r[3].s64 = ctx.r[10].s64 + 21428;
	// 826C4DC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4DE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4DE8: 4BDA2039  bl 0x82466e20
	ctx.lr = 0x826C4DEC;
	sub_82466E20(ctx, base);
	// 826C4DEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4E00 size=108
    let mut pc: u32 = 0x826C4E00;
    'dispatch: loop {
        match pc {
            0x826C4E00 => {
    //   block [0x826C4E00..0x826C4E6C)
	// 826C4E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4E0C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4E10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4E14: 38EB55F8  addi r7, r11, 0x55f8
	ctx.r[7].s64 = ctx.r[11].s64 + 22008;
	// 826C4E18: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C4E1C: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826C4E20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4E24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4E28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4E2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4E30: 386A53E4  addi r3, r10, 0x53e4
	ctx.r[3].s64 = ctx.r[10].s64 + 21476;
	// 826C4E34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4E38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4E40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4E48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4E50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4E54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4E58: 4BDA1FC9  bl 0x82466e20
	ctx.lr = 0x826C4E5C;
	sub_82466E20(ctx, base);
	// 826C4E5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4E70 size=108
    let mut pc: u32 = 0x826C4E70;
    'dispatch: loop {
        match pc {
            0x826C4E70 => {
    //   block [0x826C4E70..0x826C4EDC)
	// 826C4E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4E78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4E7C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4E80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4E84: 38EB5610  addi r7, r11, 0x5610
	ctx.r[7].s64 = ctx.r[11].s64 + 22032;
	// 826C4E88: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C4E8C: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 826C4E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4E94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4E98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4E9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4EA0: 386A5414  addi r3, r10, 0x5414
	ctx.r[3].s64 = ctx.r[10].s64 + 21524;
	// 826C4EA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4EAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4EBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4EC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4EC8: 4BDA1F59  bl 0x82466e20
	ctx.lr = 0x826C4ECC;
	sub_82466E20(ctx, base);
	// 826C4ECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4EE0 size=108
    let mut pc: u32 = 0x826C4EE0;
    'dispatch: loop {
        match pc {
            0x826C4EE0 => {
    //   block [0x826C4EE0..0x826C4F4C)
	// 826C4EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4EEC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4EF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4EF4: 38EB5640  addi r7, r11, 0x5640
	ctx.r[7].s64 = ctx.r[11].s64 + 22080;
	// 826C4EF8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826C4EFC: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826C4F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4F04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4F08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4F10: 386A5444  addi r3, r10, 0x5444
	ctx.r[3].s64 = ctx.r[10].s64 + 21572;
	// 826C4F14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4F24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4F34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4F38: 4BDA1EE9  bl 0x82466e20
	ctx.lr = 0x826C4F3C;
	sub_82466E20(ctx, base);
	// 826C4F3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C4F50 size=24
    let mut pc: u32 = 0x826C4F50;
    'dispatch: loop {
        match pc {
            0x826C4F50 => {
    //   block [0x826C4F50..0x826C4F68)
	// 826C4F50: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4F54: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C4F58: 394AA1A8  addi r10, r10, -0x5e58
	ctx.r[10].s64 = ctx.r[10].s64 + -24152;
	// 826C4F5C: 816B551C  lwz r11, 0x551c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21788 as u32) ) } as u64;
	// 826C4F60: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826C4F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4F68 size=112
    let mut pc: u32 = 0x826C4F68;
    'dispatch: loop {
        match pc {
            0x826C4F68 => {
    //   block [0x826C4F68..0x826C4FD8)
	// 826C4F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4F74: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C4F78: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C4F7C: 392A2230  addi r9, r10, 0x2230
	ctx.r[9].s64 = ctx.r[10].s64 + 8752;
	// 826C4F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4F84: 390BA1A8  addi r8, r11, -0x5e58
	ctx.r[8].s64 = ctx.r[11].s64 + -24152;
	// 826C4F88: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826C4F8C: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 826C4F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4F94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4F98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C4F9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4FA0: 386A5474  addi r3, r10, 0x5474
	ctx.r[3].s64 = ctx.r[10].s64 + 21620;
	// 826C4FA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C4FA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C4FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4FBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4FC4: 4BDA1E5D  bl 0x82466e20
	ctx.lr = 0x826C4FC8;
	sub_82466E20(ctx, base);
	// 826C4FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4FD8 size=108
    let mut pc: u32 = 0x826C4FD8;
    'dispatch: loop {
        match pc {
            0x826C4FD8 => {
    //   block [0x826C4FD8..0x826C5044)
	// 826C4FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4FE4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4FE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C4FEC: 38EB5704  addi r7, r11, 0x5704
	ctx.r[7].s64 = ctx.r[11].s64 + 22276;
	// 826C4FF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C4FF4: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 826C4FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4FFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5000: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5008: 386A54A4  addi r3, r10, 0x54a4
	ctx.r[3].s64 = ctx.r[10].s64 + 21668;
	// 826C500C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C501C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C502C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5030: 4BDA1DF1  bl 0x82466e20
	ctx.lr = 0x826C5034;
	sub_82466E20(ctx, base);
	// 826C5034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C503C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5048 size=112
    let mut pc: u32 = 0x826C5048;
    'dispatch: loop {
        match pc {
            0x826C5048 => {
    //   block [0x826C5048..0x826C50B8)
	// 826C5048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C504C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5054: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C5058: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C505C: 392A2274  addi r9, r10, 0x2274
	ctx.r[9].s64 = ctx.r[10].s64 + 8820;
	// 826C5060: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5064: 390B5738  addi r8, r11, 0x5738
	ctx.r[8].s64 = ctx.r[11].s64 + 22328;
	// 826C5068: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826C506C: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 826C5070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5074: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5078: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C507C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5080: 386A54D4  addi r3, r10, 0x54d4
	ctx.r[3].s64 = ctx.r[10].s64 + 21716;
	// 826C5084: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C5088: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C508C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C509C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C50A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C50A4: 4BDA1D7D  bl 0x82466e20
	ctx.lr = 0x826C50A8;
	sub_82466E20(ctx, base);
	// 826C50A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C50AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C50B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C50B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C50B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C50B8 size=24
    let mut pc: u32 = 0x826C50B8;
    'dispatch: loop {
        match pc {
            0x826C50B8 => {
    //   block [0x826C50B8..0x826C50D0)
	// 826C50B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C50BC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C50C0: 394AA220  addi r10, r10, -0x5de0
	ctx.r[10].s64 = ctx.r[10].s64 + -24032;
	// 826C50C4: 816B5734  lwz r11, 0x5734(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22324 as u32) ) } as u64;
	// 826C50C8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826C50CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C50D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C50D0 size=112
    let mut pc: u32 = 0x826C50D0;
    'dispatch: loop {
        match pc {
            0x826C50D0 => {
    //   block [0x826C50D0..0x826C5140)
	// 826C50D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C50D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C50D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C50DC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C50E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C50E4: 392A22B0  addi r9, r10, 0x22b0
	ctx.r[9].s64 = ctx.r[10].s64 + 8880;
	// 826C50E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C50EC: 390BA220  addi r8, r11, -0x5de0
	ctx.r[8].s64 = ctx.r[11].s64 + -24032;
	// 826C50F0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826C50F4: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 826C50F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C50FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C5104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5108: 386A5504  addi r3, r10, 0x5504
	ctx.r[3].s64 = ctx.r[10].s64 + 21764;
	// 826C510C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C5110: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C5114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C511C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C512C: 4BDA1CF5  bl 0x82466e20
	ctx.lr = 0x826C5130;
	sub_82466E20(ctx, base);
	// 826C5130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C513C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5140 size=108
    let mut pc: u32 = 0x826C5140;
    'dispatch: loop {
        match pc {
            0x826C5140 => {
    //   block [0x826C5140..0x826C51AC)
	// 826C5140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C514C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5154: 38EB57F8  addi r7, r11, 0x57f8
	ctx.r[7].s64 = ctx.r[11].s64 + 22520;
	// 826C5158: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C515C: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826C5160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5164: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5168: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C516C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5170: 386A5534  addi r3, r10, 0x5534
	ctx.r[3].s64 = ctx.r[10].s64 + 21812;
	// 826C5174: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C517C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C518C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5198: 4BDA1C89  bl 0x82466e20
	ctx.lr = 0x826C519C;
	sub_82466E20(ctx, base);
	// 826C519C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C51A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C51A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C51A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C51B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C51B0 size=108
    let mut pc: u32 = 0x826C51B0;
    'dispatch: loop {
        match pc {
            0x826C51B0 => {
    //   block [0x826C51B0..0x826C521C)
	// 826C51B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C51B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C51B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C51BC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C51C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C51C4: 38EB5810  addi r7, r11, 0x5810
	ctx.r[7].s64 = ctx.r[11].s64 + 22544;
	// 826C51C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C51CC: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 826C51D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C51D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C51D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C51DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C51E0: 386A5564  addi r3, r10, 0x5564
	ctx.r[3].s64 = ctx.r[10].s64 + 21860;
	// 826C51E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C51E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C51EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C51F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C51F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C51F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C51FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5208: 4BDA1C19  bl 0x82466e20
	ctx.lr = 0x826C520C;
	sub_82466E20(ctx, base);
	// 826C520C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C5220 size=24
    let mut pc: u32 = 0x826C5220;
    'dispatch: loop {
        match pc {
            0x826C5220 => {
    //   block [0x826C5220..0x826C5238)
	// 826C5220: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5224: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C5228: 394AA268  addi r10, r10, -0x5d98
	ctx.r[10].s64 = ctx.r[10].s64 + -23960;
	// 826C522C: 816B5840  lwz r11, 0x5840(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22592 as u32) ) } as u64;
	// 826C5230: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826C5234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5238 size=112
    let mut pc: u32 = 0x826C5238;
    'dispatch: loop {
        match pc {
            0x826C5238 => {
    //   block [0x826C5238..0x826C52A8)
	// 826C5238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C523C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5244: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C5248: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C524C: 392A22EC  addi r9, r10, 0x22ec
	ctx.r[9].s64 = ctx.r[10].s64 + 8940;
	// 826C5250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5254: 390BA268  addi r8, r11, -0x5d98
	ctx.r[8].s64 = ctx.r[11].s64 + -23960;
	// 826C5258: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826C525C: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826C5260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5264: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5268: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C526C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5270: 386A5594  addi r3, r10, 0x5594
	ctx.r[3].s64 = ctx.r[10].s64 + 21908;
	// 826C5274: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C5278: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C527C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C528C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5294: 4BDA1B8D  bl 0x82466e20
	ctx.lr = 0x826C5298;
	sub_82466E20(ctx, base);
	// 826C5298: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C529C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C52A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C52A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C52A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C52A8 size=108
    let mut pc: u32 = 0x826C52A8;
    'dispatch: loop {
        match pc {
            0x826C52A8 => {
    //   block [0x826C52A8..0x826C5314)
	// 826C52A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C52AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C52B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C52B4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C52B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C52BC: 38EB5844  addi r7, r11, 0x5844
	ctx.r[7].s64 = ctx.r[11].s64 + 22596;
	// 826C52C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C52C4: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826C52C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C52CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C52D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C52D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C52D8: 386A55C4  addi r3, r10, 0x55c4
	ctx.r[3].s64 = ctx.r[10].s64 + 21956;
	// 826C52DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C52E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C52E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C52E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C52EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C52F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C52F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C52F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C52FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5300: 4BDA1B21  bl 0x82466e20
	ctx.lr = 0x826C5304;
	sub_82466E20(ctx, base);
	// 826C5304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C530C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5318 size=108
    let mut pc: u32 = 0x826C5318;
    'dispatch: loop {
        match pc {
            0x826C5318 => {
    //   block [0x826C5318..0x826C5384)
	// 826C5318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C531C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5324: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C532C: 38EB5860  addi r7, r11, 0x5860
	ctx.r[7].s64 = ctx.r[11].s64 + 22624;
	// 826C5330: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C5334: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826C5338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C533C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5340: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5348: 386A55F4  addi r3, r10, 0x55f4
	ctx.r[3].s64 = ctx.r[10].s64 + 22004;
	// 826C534C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C535C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C536C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5370: 4BDA1AB1  bl 0x82466e20
	ctx.lr = 0x826C5374;
	sub_82466E20(ctx, base);
	// 826C5374: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C537C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5388 size=108
    let mut pc: u32 = 0x826C5388;
    'dispatch: loop {
        match pc {
            0x826C5388 => {
    //   block [0x826C5388..0x826C53F4)
	// 826C5388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C538C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5394: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C539C: 38EB58A8  addi r7, r11, 0x58a8
	ctx.r[7].s64 = ctx.r[11].s64 + 22696;
	// 826C53A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C53A4: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826C53A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C53AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C53B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C53B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C53B8: 386A5624  addi r3, r10, 0x5624
	ctx.r[3].s64 = ctx.r[10].s64 + 22052;
	// 826C53BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C53C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C53C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C53C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C53CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C53D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C53D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C53D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C53DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C53E0: 4BDA1A41  bl 0x82466e20
	ctx.lr = 0x826C53E4;
	sub_82466E20(ctx, base);
	// 826C53E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C53E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C53EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C53F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C53F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C53F8 size=108
    let mut pc: u32 = 0x826C53F8;
    'dispatch: loop {
        match pc {
            0x826C53F8 => {
    //   block [0x826C53F8..0x826C5464)
	// 826C53F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C53FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5404: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C540C: 38EB58D8  addi r7, r11, 0x58d8
	ctx.r[7].s64 = ctx.r[11].s64 + 22744;
	// 826C5410: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826C5414: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 826C5418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C541C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5428: 386A5654  addi r3, r10, 0x5654
	ctx.r[3].s64 = ctx.r[10].s64 + 22100;
	// 826C542C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C543C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C544C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5450: 4BDA19D1  bl 0x82466e20
	ctx.lr = 0x826C5454;
	sub_82466E20(ctx, base);
	// 826C5454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C545C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5468 size=108
    let mut pc: u32 = 0x826C5468;
    'dispatch: loop {
        match pc {
            0x826C5468 => {
    //   block [0x826C5468..0x826C54D4)
	// 826C5468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C546C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5474: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C547C: 38EB59F8  addi r7, r11, 0x59f8
	ctx.r[7].s64 = ctx.r[11].s64 + 23032;
	// 826C5480: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826C5484: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 826C5488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C548C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5490: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5498: 386A5684  addi r3, r10, 0x5684
	ctx.r[3].s64 = ctx.r[10].s64 + 22148;
	// 826C549C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C54A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C54A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C54A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C54AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C54B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C54B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C54B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C54BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C54C0: 4BDA1961  bl 0x82466e20
	ctx.lr = 0x826C54C4;
	sub_82466E20(ctx, base);
	// 826C54C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C54C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C54CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C54D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C54D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C54D8 size=108
    let mut pc: u32 = 0x826C54D8;
    'dispatch: loop {
        match pc {
            0x826C54D8 => {
    //   block [0x826C54D8..0x826C5544)
	// 826C54D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C54DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C54E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C54E4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C54E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C54EC: 38EB5A88  addi r7, r11, 0x5a88
	ctx.r[7].s64 = ctx.r[11].s64 + 23176;
	// 826C54F0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826C54F4: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 826C54F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C54FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5500: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5508: 386A56B4  addi r3, r10, 0x56b4
	ctx.r[3].s64 = ctx.r[10].s64 + 22196;
	// 826C550C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C551C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C552C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5530: 4BDA18F1  bl 0x82466e20
	ctx.lr = 0x826C5534;
	sub_82466E20(ctx, base);
	// 826C5534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C553C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5548 size=108
    let mut pc: u32 = 0x826C5548;
    'dispatch: loop {
        match pc {
            0x826C5548 => {
    //   block [0x826C5548..0x826C55B4)
	// 826C5548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C554C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5554: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C555C: 38EB5B48  addi r7, r11, 0x5b48
	ctx.r[7].s64 = ctx.r[11].s64 + 23368;
	// 826C5560: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826C5564: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 826C5568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C556C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5578: 386A56E4  addi r3, r10, 0x56e4
	ctx.r[3].s64 = ctx.r[10].s64 + 22244;
	// 826C557C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C558C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C559C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C55A0: 4BDA1881  bl 0x82466e20
	ctx.lr = 0x826C55A4;
	sub_82466E20(ctx, base);
	// 826C55A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C55A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C55AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C55B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C55B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C55B8 size=108
    let mut pc: u32 = 0x826C55B8;
    'dispatch: loop {
        match pc {
            0x826C55B8 => {
    //   block [0x826C55B8..0x826C5624)
	// 826C55B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C55BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C55C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C55C4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C55C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C55CC: 38EB5C20  addi r7, r11, 0x5c20
	ctx.r[7].s64 = ctx.r[11].s64 + 23584;
	// 826C55D0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826C55D4: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826C55D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C55DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C55E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C55E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C55E8: 386A5714  addi r3, r10, 0x5714
	ctx.r[3].s64 = ctx.r[10].s64 + 22292;
	// 826C55EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C55F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C55F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C55F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C55FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C560C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5610: 4BDA1811  bl 0x82466e20
	ctx.lr = 0x826C5614;
	sub_82466E20(ctx, base);
	// 826C5614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C561C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5628 size=108
    let mut pc: u32 = 0x826C5628;
    'dispatch: loop {
        match pc {
            0x826C5628 => {
    //   block [0x826C5628..0x826C5694)
	// 826C5628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C562C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5634: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C563C: 38EB5CE0  addi r7, r11, 0x5ce0
	ctx.r[7].s64 = ctx.r[11].s64 + 23776;
	// 826C5640: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826C5644: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 826C5648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C564C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5650: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5658: 386A5744  addi r3, r10, 0x5744
	ctx.r[3].s64 = ctx.r[10].s64 + 22340;
	// 826C565C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C566C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C567C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5680: 4BDA17A1  bl 0x82466e20
	ctx.lr = 0x826C5684;
	sub_82466E20(ctx, base);
	// 826C5684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C568C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5698 size=112
    let mut pc: u32 = 0x826C5698;
    'dispatch: loop {
        match pc {
            0x826C5698 => {
    //   block [0x826C5698..0x826C5708)
	// 826C5698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C569C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C56A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C56A4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C56A8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826C56AC: 38EA5D88  addi r7, r10, 0x5d88
	ctx.r[7].s64 = ctx.r[10].s64 + 23944;
	// 826C56B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C56B4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C56B8: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 826C56BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C56C0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C56C4: 396B2300  addi r11, r11, 0x2300
	ctx.r[11].s64 = ctx.r[11].s64 + 8960;
	// 826C56C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C56CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C56D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C56D4: 386A5774  addi r3, r10, 0x5774
	ctx.r[3].s64 = ctx.r[10].s64 + 22388;
	// 826C56D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C56DC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C56E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C56E4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C56E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C56EC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C56F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C56F4: 4BDA172D  bl 0x82466e20
	ctx.lr = 0x826C56F8;
	sub_82466E20(ctx, base);
	// 826C56F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C56FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5708 size=108
    let mut pc: u32 = 0x826C5708;
    'dispatch: loop {
        match pc {
            0x826C5708 => {
    //   block [0x826C5708..0x826C5774)
	// 826C5708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C570C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5714: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C571C: 38EB5EA8  addi r7, r11, 0x5ea8
	ctx.r[7].s64 = ctx.r[11].s64 + 24232;
	// 826C5720: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C5724: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 826C5728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C572C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5730: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5738: 386A57A4  addi r3, r10, 0x57a4
	ctx.r[3].s64 = ctx.r[10].s64 + 22436;
	// 826C573C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C574C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C575C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5760: 4BDA16C1  bl 0x82466e20
	ctx.lr = 0x826C5764;
	sub_82466E20(ctx, base);
	// 826C5764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C576C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5778 size=108
    let mut pc: u32 = 0x826C5778;
    'dispatch: loop {
        match pc {
            0x826C5778 => {
    //   block [0x826C5778..0x826C57E4)
	// 826C5778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C577C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5784: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C578C: 38EB5F08  addi r7, r11, 0x5f08
	ctx.r[7].s64 = ctx.r[11].s64 + 24328;
	// 826C5790: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826C5794: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 826C5798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C579C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C57A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C57A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C57A8: 386A57D4  addi r3, r10, 0x57d4
	ctx.r[3].s64 = ctx.r[10].s64 + 22484;
	// 826C57AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C57B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C57B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C57B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C57BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C57C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C57C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C57C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C57CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C57D0: 4BDA1651  bl 0x82466e20
	ctx.lr = 0x826C57D4;
	sub_82466E20(ctx, base);
	// 826C57D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C57D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C57DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C57E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C57E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C57E8 size=108
    let mut pc: u32 = 0x826C57E8;
    'dispatch: loop {
        match pc {
            0x826C57E8 => {
    //   block [0x826C57E8..0x826C5854)
	// 826C57E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C57EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C57F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C57F4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C57F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C57FC: 38EB6010  addi r7, r11, 0x6010
	ctx.r[7].s64 = ctx.r[11].s64 + 24592;
	// 826C5800: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826C5804: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826C5808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C580C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5818: 386A5804  addi r3, r10, 0x5804
	ctx.r[3].s64 = ctx.r[10].s64 + 22532;
	// 826C581C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C582C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C583C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5840: 4BDA15E1  bl 0x82466e20
	ctx.lr = 0x826C5844;
	sub_82466E20(ctx, base);
	// 826C5844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C584C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5858 size=108
    let mut pc: u32 = 0x826C5858;
    'dispatch: loop {
        match pc {
            0x826C5858 => {
    //   block [0x826C5858..0x826C58C4)
	// 826C5858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C585C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5864: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5868: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C586C: 38EB60E8  addi r7, r11, 0x60e8
	ctx.r[7].s64 = ctx.r[11].s64 + 24808;
	// 826C5870: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C5874: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 826C5878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C587C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5880: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5888: 386A5834  addi r3, r10, 0x5834
	ctx.r[3].s64 = ctx.r[10].s64 + 22580;
	// 826C588C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C589C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C58A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C58A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C58A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C58AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C58B0: 4BDA1571  bl 0x82466e20
	ctx.lr = 0x826C58B4;
	sub_82466E20(ctx, base);
	// 826C58B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C58B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C58BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C58C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C58C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C58C8 size=108
    let mut pc: u32 = 0x826C58C8;
    'dispatch: loop {
        match pc {
            0x826C58C8 => {
    //   block [0x826C58C8..0x826C5934)
	// 826C58C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C58CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C58D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C58D4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C58D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C58DC: 38EB6130  addi r7, r11, 0x6130
	ctx.r[7].s64 = ctx.r[11].s64 + 24880;
	// 826C58E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C58E4: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826C58E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C58EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C58F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C58F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C58F8: 386A5864  addi r3, r10, 0x5864
	ctx.r[3].s64 = ctx.r[10].s64 + 22628;
	// 826C58FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C590C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C591C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5920: 4BDA1501  bl 0x82466e20
	ctx.lr = 0x826C5924;
	sub_82466E20(ctx, base);
	// 826C5924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C592C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5938 size=108
    let mut pc: u32 = 0x826C5938;
    'dispatch: loop {
        match pc {
            0x826C5938 => {
    //   block [0x826C5938..0x826C59A4)
	// 826C5938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C593C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5944: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C594C: 38EB6148  addi r7, r11, 0x6148
	ctx.r[7].s64 = ctx.r[11].s64 + 24904;
	// 826C5950: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C5954: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 826C5958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C595C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5960: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5968: 386A5894  addi r3, r10, 0x5894
	ctx.r[3].s64 = ctx.r[10].s64 + 22676;
	// 826C596C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C597C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C598C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5990: 4BDA1491  bl 0x82466e20
	ctx.lr = 0x826C5994;
	sub_82466E20(ctx, base);
	// 826C5994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C599C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C59A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C59A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C59A8 size=108
    let mut pc: u32 = 0x826C59A8;
    'dispatch: loop {
        match pc {
            0x826C59A8 => {
    //   block [0x826C59A8..0x826C5A14)
	// 826C59A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C59AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C59B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C59B4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C59B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C59BC: 38EB6190  addi r7, r11, 0x6190
	ctx.r[7].s64 = ctx.r[11].s64 + 24976;
	// 826C59C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C59C4: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 826C59C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C59CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C59D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C59D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C59D8: 386A58C4  addi r3, r10, 0x58c4
	ctx.r[3].s64 = ctx.r[10].s64 + 22724;
	// 826C59DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C59E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C59E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C59E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C59EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C59F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C59F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C59F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C59FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5A00: 4BDA1421  bl 0x82466e20
	ctx.lr = 0x826C5A04;
	sub_82466E20(ctx, base);
	// 826C5A04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5A18 size=112
    let mut pc: u32 = 0x826C5A18;
    'dispatch: loop {
        match pc {
            0x826C5A18 => {
    //   block [0x826C5A18..0x826C5A88)
	// 826C5A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5A24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5A28: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5A2C: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C5A30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5A34: 390B61A8  addi r8, r11, 0x61a8
	ctx.r[8].s64 = ctx.r[11].s64 + 25000;
	// 826C5A38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C5A3C: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 826C5A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5A44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5A48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C5A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5A50: 386A58F4  addi r3, r10, 0x58f4
	ctx.r[3].s64 = ctx.r[10].s64 + 22772;
	// 826C5A54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C5A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5A74: 4BDA13AD  bl 0x82466e20
	ctx.lr = 0x826C5A78;
	sub_82466E20(ctx, base);
	// 826C5A78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5A88 size=108
    let mut pc: u32 = 0x826C5A88;
    'dispatch: loop {
        match pc {
            0x826C5A88 => {
    //   block [0x826C5A88..0x826C5AF4)
	// 826C5A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5A94: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5A98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5A9C: 38EB61F0  addi r7, r11, 0x61f0
	ctx.r[7].s64 = ctx.r[11].s64 + 25072;
	// 826C5AA0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C5AA4: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 826C5AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5AAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5AB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C5AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5AB8: 386A5924  addi r3, r10, 0x5924
	ctx.r[3].s64 = ctx.r[10].s64 + 22820;
	// 826C5ABC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C5AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5ADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5AE0: 4BDA1341  bl 0x82466e20
	ctx.lr = 0x826C5AE4;
	sub_82466E20(ctx, base);
	// 826C5AE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5AF8 size=112
    let mut pc: u32 = 0x826C5AF8;
    'dispatch: loop {
        match pc {
            0x826C5AF8 => {
    //   block [0x826C5AF8..0x826C5B68)
	// 826C5AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5B04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5B08: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5B0C: 38AA5924  addi r5, r10, 0x5924
	ctx.r[5].s64 = ctx.r[10].s64 + 22820;
	// 826C5B10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5B14: 390B6250  addi r8, r11, 0x6250
	ctx.r[8].s64 = ctx.r[11].s64 + 25168;
	// 826C5B18: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C5B1C: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 826C5B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5B24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5B28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C5B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5B30: 386A5954  addi r3, r10, 0x5954
	ctx.r[3].s64 = ctx.r[10].s64 + 22868;
	// 826C5B34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C5B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5B54: 4BDA12CD  bl 0x82466e20
	ctx.lr = 0x826C5B58;
	sub_82466E20(ctx, base);
	// 826C5B58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5B68 size=96
    let mut pc: u32 = 0x826C5B68;
    'dispatch: loop {
        match pc {
            0x826C5B68 => {
    //   block [0x826C5B68..0x826C5BC8)
	// 826C5B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5B74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5B7C: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 826C5B80: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5B88: 386A5984  addi r3, r10, 0x5984
	ctx.r[3].s64 = ctx.r[10].s64 + 22916;
	// 826C5B8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5B94: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C5B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5BA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5BA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C5BAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5BB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C5BB4: 4BDA126D  bl 0x82466e20
	ctx.lr = 0x826C5BB8;
	sub_82466E20(ctx, base);
	// 826C5BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5BC8 size=112
    let mut pc: u32 = 0x826C5BC8;
    'dispatch: loop {
        match pc {
            0x826C5BC8 => {
    //   block [0x826C5BC8..0x826C5C38)
	// 826C5BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5BD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5BD8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5BDC: 38AA7664  addi r5, r10, 0x7664
	ctx.r[5].s64 = ctx.r[10].s64 + 30308;
	// 826C5BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5BE4: 390B6298  addi r8, r11, 0x6298
	ctx.r[8].s64 = ctx.r[11].s64 + 25240;
	// 826C5BE8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C5BEC: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 826C5BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5BF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C5BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5C00: 386A59B4  addi r3, r10, 0x59b4
	ctx.r[3].s64 = ctx.r[10].s64 + 22964;
	// 826C5C04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C5C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5C24: 4BDA11FD  bl 0x82466e20
	ctx.lr = 0x826C5C28;
	sub_82466E20(ctx, base);
	// 826C5C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5C38 size=96
    let mut pc: u32 = 0x826C5C38;
    'dispatch: loop {
        match pc {
            0x826C5C38 => {
    //   block [0x826C5C38..0x826C5C98)
	// 826C5C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5C44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5C4C: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 826C5C50: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5C54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5C58: 386A59E4  addi r3, r10, 0x59e4
	ctx.r[3].s64 = ctx.r[10].s64 + 23012;
	// 826C5C5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5C60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5C64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C5C68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5C70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5C74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5C78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C5C7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5C80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C5C84: 4BDA119D  bl 0x82466e20
	ctx.lr = 0x826C5C88;
	sub_82466E20(ctx, base);
	// 826C5C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5C98 size=100
    let mut pc: u32 = 0x826C5C98;
    'dispatch: loop {
        match pc {
            0x826C5C98 => {
    //   block [0x826C5C98..0x826C5CFC)
	// 826C5C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5CA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5CAC: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C5CB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5CB8: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 826C5CBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5CC4: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C5CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5CCC: 386A5A14  addi r3, r10, 0x5a14
	ctx.r[3].s64 = ctx.r[10].s64 + 23060;
	// 826C5CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5CD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5CD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C5CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5CE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C5CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5CE8: 4BDA1139  bl 0x82466e20
	ctx.lr = 0x826C5CEC;
	sub_82466E20(ctx, base);
	// 826C5CEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5D00 size=96
    let mut pc: u32 = 0x826C5D00;
    'dispatch: loop {
        match pc {
            0x826C5D00 => {
    //   block [0x826C5D00..0x826C5D60)
	// 826C5D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5D0C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5D14: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 826C5D18: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5D20: 386A5A44  addi r3, r10, 0x5a44
	ctx.r[3].s64 = ctx.r[10].s64 + 23108;
	// 826C5D24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5D28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5D2C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C5D30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5D34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5D38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5D40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C5D44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5D48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C5D4C: 4BDA10D5  bl 0x82466e20
	ctx.lr = 0x826C5D50;
	sub_82466E20(ctx, base);
	// 826C5D50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5D60 size=112
    let mut pc: u32 = 0x826C5D60;
    'dispatch: loop {
        match pc {
            0x826C5D60 => {
    //   block [0x826C5D60..0x826C5DD0)
	// 826C5D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5D68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5D6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5D70: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5D74: 38AA5A14  addi r5, r10, 0x5a14
	ctx.r[5].s64 = ctx.r[10].s64 + 23060;
	// 826C5D78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5D7C: 390B62F8  addi r8, r11, 0x62f8
	ctx.r[8].s64 = ctx.r[11].s64 + 25336;
	// 826C5D80: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C5D84: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 826C5D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5D8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5D90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C5D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5D98: 386A5A74  addi r3, r10, 0x5a74
	ctx.r[3].s64 = ctx.r[10].s64 + 23156;
	// 826C5D9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C5DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5DA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5DB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5DBC: 4BDA1065  bl 0x82466e20
	ctx.lr = 0x826C5DC0;
	sub_82466E20(ctx, base);
	// 826C5DC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5DD0 size=112
    let mut pc: u32 = 0x826C5DD0;
    'dispatch: loop {
        match pc {
            0x826C5DD0 => {
    //   block [0x826C5DD0..0x826C5E40)
	// 826C5DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5DDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5DE0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5DE4: 38AA5A14  addi r5, r10, 0x5a14
	ctx.r[5].s64 = ctx.r[10].s64 + 23060;
	// 826C5DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5DEC: 390B6328  addi r8, r11, 0x6328
	ctx.r[8].s64 = ctx.r[11].s64 + 25384;
	// 826C5DF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C5DF4: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 826C5DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5DFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5E00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C5E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5E08: 386A5AA4  addi r3, r10, 0x5aa4
	ctx.r[3].s64 = ctx.r[10].s64 + 23204;
	// 826C5E0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C5E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5E14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5E1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5E2C: 4BDA0FF5  bl 0x82466e20
	ctx.lr = 0x826C5E30;
	sub_82466E20(ctx, base);
	// 826C5E30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5E40 size=100
    let mut pc: u32 = 0x826C5E40;
    'dispatch: loop {
        match pc {
            0x826C5E40 => {
    //   block [0x826C5E40..0x826C5EA4)
	// 826C5E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5E4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5E54: 38AA5A14  addi r5, r10, 0x5a14
	ctx.r[5].s64 = ctx.r[10].s64 + 23060;
	// 826C5E58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5E60: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 826C5E64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5E74: 386A5AD4  addi r3, r10, 0x5ad4
	ctx.r[3].s64 = ctx.r[10].s64 + 23252;
	// 826C5E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5E7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5E80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C5E84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5E88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C5E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5E90: 4BDA0F91  bl 0x82466e20
	ctx.lr = 0x826C5E94;
	sub_82466E20(ctx, base);
	// 826C5E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5EA8 size=96
    let mut pc: u32 = 0x826C5EA8;
    'dispatch: loop {
        match pc {
            0x826C5EA8 => {
    //   block [0x826C5EA8..0x826C5F08)
	// 826C5EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5EB4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5EBC: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 826C5EC0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5EC8: 386A5B04  addi r3, r10, 0x5b04
	ctx.r[3].s64 = ctx.r[10].s64 + 23300;
	// 826C5ECC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5ED4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C5ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5EE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C5EEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5EF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C5EF4: 4BDA0F2D  bl 0x82466e20
	ctx.lr = 0x826C5EF8;
	sub_82466E20(ctx, base);
	// 826C5EF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5F08 size=112
    let mut pc: u32 = 0x826C5F08;
    'dispatch: loop {
        match pc {
            0x826C5F08 => {
    //   block [0x826C5F08..0x826C5F78)
	// 826C5F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5F14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5F18: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5F1C: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C5F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5F24: 390B6340  addi r8, r11, 0x6340
	ctx.r[8].s64 = ctx.r[11].s64 + 25408;
	// 826C5F28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C5F2C: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 826C5F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5F34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5F38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C5F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5F40: 386A5B34  addi r3, r10, 0x5b34
	ctx.r[3].s64 = ctx.r[10].s64 + 23348;
	// 826C5F44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C5F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C5F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5F64: 4BDA0EBD  bl 0x82466e20
	ctx.lr = 0x826C5F68;
	sub_82466E20(ctx, base);
	// 826C5F68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5F78 size=96
    let mut pc: u32 = 0x826C5F78;
    'dispatch: loop {
        match pc {
            0x826C5F78 => {
    //   block [0x826C5F78..0x826C5FD8)
	// 826C5F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5F84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C5F8C: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 826C5F90: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5F94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C5F98: 386A5B64  addi r3, r10, 0x5b64
	ctx.r[3].s64 = ctx.r[10].s64 + 23396;
	// 826C5F9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C5FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C5FA4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C5FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C5FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C5FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C5FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C5FB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C5FBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C5FC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C5FC4: 4BDA0E5D  bl 0x82466e20
	ctx.lr = 0x826C5FC8;
	sub_82466E20(ctx, base);
	// 826C5FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C5FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C5FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C5FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C5FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C5FD8 size=112
    let mut pc: u32 = 0x826C5FD8;
    'dispatch: loop {
        match pc {
            0x826C5FD8 => {
    //   block [0x826C5FD8..0x826C6048)
	// 826C5FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C5FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C5FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C5FE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C5FE8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C5FEC: 38AA5B64  addi r5, r10, 0x5b64
	ctx.r[5].s64 = ctx.r[10].s64 + 23396;
	// 826C5FF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C5FF4: 390B6358  addi r8, r11, 0x6358
	ctx.r[8].s64 = ctx.r[11].s64 + 25432;
	// 826C5FF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C5FFC: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 826C6000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6004: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C600C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6010: 386A5B94  addi r3, r10, 0x5b94
	ctx.r[3].s64 = ctx.r[10].s64 + 23444;
	// 826C6014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C601C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C602C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6034: 4BDA0DED  bl 0x82466e20
	ctx.lr = 0x826C6038;
	sub_82466E20(ctx, base);
	// 826C6038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C603C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6048 size=108
    let mut pc: u32 = 0x826C6048;
    'dispatch: loop {
        match pc {
            0x826C6048 => {
    //   block [0x826C6048..0x826C60B4)
	// 826C6048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C604C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6054: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C605C: 38EB6370  addi r7, r11, 0x6370
	ctx.r[7].s64 = ctx.r[11].s64 + 25456;
	// 826C6060: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C6064: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 826C6068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C606C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6070: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C6074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6078: 386A5BC4  addi r3, r10, 0x5bc4
	ctx.r[3].s64 = ctx.r[10].s64 + 23492;
	// 826C607C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C6080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C608C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C609C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C60A0: 4BDA0D81  bl 0x82466e20
	ctx.lr = 0x826C60A4;
	sub_82466E20(ctx, base);
	// 826C60A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C60A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C60AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C60B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C60B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C60B8 size=112
    let mut pc: u32 = 0x826C60B8;
    'dispatch: loop {
        match pc {
            0x826C60B8 => {
    //   block [0x826C60B8..0x826C6128)
	// 826C60B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C60BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C60C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C60C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C60C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C60CC: 38AA5CE4  addi r5, r10, 0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + 23780;
	// 826C60D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C60D4: 390B63D0  addi r8, r11, 0x63d0
	ctx.r[8].s64 = ctx.r[11].s64 + 25552;
	// 826C60D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C60DC: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 826C60E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C60E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C60E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C60EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C60F0: 386A5BF4  addi r3, r10, 0x5bf4
	ctx.r[3].s64 = ctx.r[10].s64 + 23540;
	// 826C60F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C60F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C60FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C610C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6114: 4BDA0D0D  bl 0x82466e20
	ctx.lr = 0x826C6118;
	sub_82466E20(ctx, base);
	// 826C6118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C611C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6128 size=112
    let mut pc: u32 = 0x826C6128;
    'dispatch: loop {
        match pc {
            0x826C6128 => {
    //   block [0x826C6128..0x826C6198)
	// 826C6128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C612C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6134: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6138: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C613C: 38AA5B34  addi r5, r10, 0x5b34
	ctx.r[5].s64 = ctx.r[10].s64 + 23348;
	// 826C6140: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6144: 390B63E8  addi r8, r11, 0x63e8
	ctx.r[8].s64 = ctx.r[11].s64 + 25576;
	// 826C6148: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C614C: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 826C6150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6154: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6158: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C615C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6160: 386A5C24  addi r3, r10, 0x5c24
	ctx.r[3].s64 = ctx.r[10].s64 + 23588;
	// 826C6164: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C616C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C617C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6184: 4BDA0C9D  bl 0x82466e20
	ctx.lr = 0x826C6188;
	sub_82466E20(ctx, base);
	// 826C6188: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C618C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6198 size=112
    let mut pc: u32 = 0x826C6198;
    'dispatch: loop {
        match pc {
            0x826C6198 => {
    //   block [0x826C6198..0x826C6208)
	// 826C6198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C619C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C61A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C61A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C61A8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C61AC: 38AA5B34  addi r5, r10, 0x5b34
	ctx.r[5].s64 = ctx.r[10].s64 + 23348;
	// 826C61B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C61B4: 390B6418  addi r8, r11, 0x6418
	ctx.r[8].s64 = ctx.r[11].s64 + 25624;
	// 826C61B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C61BC: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 826C61C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C61C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C61C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C61CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C61D0: 386A5C54  addi r3, r10, 0x5c54
	ctx.r[3].s64 = ctx.r[10].s64 + 23636;
	// 826C61D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C61D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C61DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C61E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C61E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C61E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C61EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C61F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C61F4: 4BDA0C2D  bl 0x82466e20
	ctx.lr = 0x826C61F8;
	sub_82466E20(ctx, base);
	// 826C61F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C61FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6208 size=116
    let mut pc: u32 = 0x826C6208;
    'dispatch: loop {
        match pc {
            0x826C6208 => {
    //   block [0x826C6208..0x826C627C)
	// 826C6208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C620C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6214: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6218: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C621C: 390B6430  addi r8, r11, 0x6430
	ctx.r[8].s64 = ctx.r[11].s64 + 25648;
	// 826C6220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6224: 392A2378  addi r9, r10, 0x2378
	ctx.r[9].s64 = ctx.r[10].s64 + 9080;
	// 826C6228: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C622C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826C6230: 38AA5CE4  addi r5, r10, 0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + 23780;
	// 826C6234: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C623C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C624C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C6250: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 826C6254: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C6258: 386B5C84  addi r3, r11, 0x5c84
	ctx.r[3].s64 = ctx.r[11].s64 + 23684;
	// 826C625C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C6260: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6268: 4BDA0BB9  bl 0x82466e20
	ctx.lr = 0x826C626C;
	sub_82466E20(ctx, base);
	// 826C626C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6280 size=112
    let mut pc: u32 = 0x826C6280;
    'dispatch: loop {
        match pc {
            0x826C6280 => {
    //   block [0x826C6280..0x826C62F0)
	// 826C6280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C628C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6290: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6294: 38AA5B34  addi r5, r10, 0x5b34
	ctx.r[5].s64 = ctx.r[10].s64 + 23348;
	// 826C6298: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C629C: 390B6460  addi r8, r11, 0x6460
	ctx.r[8].s64 = ctx.r[11].s64 + 25696;
	// 826C62A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C62A4: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 826C62A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C62AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C62B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C62B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C62B8: 386A5CB4  addi r3, r10, 0x5cb4
	ctx.r[3].s64 = ctx.r[10].s64 + 23732;
	// 826C62BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C62C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C62C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C62C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C62CC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C62D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C62D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C62D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C62DC: 4BDA0B45  bl 0x82466e20
	ctx.lr = 0x826C62E0;
	sub_82466E20(ctx, base);
	// 826C62E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C62E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C62E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C62EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C62F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C62F0 size=112
    let mut pc: u32 = 0x826C62F0;
    'dispatch: loop {
        match pc {
            0x826C62F0 => {
    //   block [0x826C62F0..0x826C6360)
	// 826C62F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C62F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C62F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C62FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6300: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6304: 38AA6194  addi r5, r10, 0x6194
	ctx.r[5].s64 = ctx.r[10].s64 + 24980;
	// 826C6308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C630C: 390B6478  addi r8, r11, 0x6478
	ctx.r[8].s64 = ctx.r[11].s64 + 25720;
	// 826C6310: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C6314: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 826C6318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C631C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6320: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6324: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6328: 386A5CE4  addi r3, r10, 0x5ce4
	ctx.r[3].s64 = ctx.r[10].s64 + 23780;
	// 826C632C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C633C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C634C: 4BDA0AD5  bl 0x82466e20
	ctx.lr = 0x826C6350;
	sub_82466E20(ctx, base);
	// 826C6350: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C635C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6360 size=112
    let mut pc: u32 = 0x826C6360;
    'dispatch: loop {
        match pc {
            0x826C6360 => {
    //   block [0x826C6360..0x826C63D0)
	// 826C6360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C636C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6370: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6374: 38AA5EF4  addi r5, r10, 0x5ef4
	ctx.r[5].s64 = ctx.r[10].s64 + 24308;
	// 826C6378: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C637C: 390B6490  addi r8, r11, 0x6490
	ctx.r[8].s64 = ctx.r[11].s64 + 25744;
	// 826C6380: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C6384: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 826C6388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C638C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6390: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6398: 386A5D14  addi r3, r10, 0x5d14
	ctx.r[3].s64 = ctx.r[10].s64 + 23828;
	// 826C639C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C63A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C63A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C63A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C63AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C63B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C63B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C63B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C63BC: 4BDA0A65  bl 0x82466e20
	ctx.lr = 0x826C63C0;
	sub_82466E20(ctx, base);
	// 826C63C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C63C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C63C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C63CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C63D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C63D0 size=112
    let mut pc: u32 = 0x826C63D0;
    'dispatch: loop {
        match pc {
            0x826C63D0 => {
    //   block [0x826C63D0..0x826C6440)
	// 826C63D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C63D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C63D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C63DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C63E0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C63E4: 38AA5CB4  addi r5, r10, 0x5cb4
	ctx.r[5].s64 = ctx.r[10].s64 + 23732;
	// 826C63E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C63EC: 390B64A8  addi r8, r11, 0x64a8
	ctx.r[8].s64 = ctx.r[11].s64 + 25768;
	// 826C63F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C63F4: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 826C63F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C63FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6400: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6408: 386A5D44  addi r3, r10, 0x5d44
	ctx.r[3].s64 = ctx.r[10].s64 + 23876;
	// 826C640C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C641C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C642C: 4BDA09F5  bl 0x82466e20
	ctx.lr = 0x826C6430;
	sub_82466E20(ctx, base);
	// 826C6430: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C643C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6440 size=112
    let mut pc: u32 = 0x826C6440;
    'dispatch: loop {
        match pc {
            0x826C6440 => {
    //   block [0x826C6440..0x826C64B0)
	// 826C6440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C644C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6450: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6454: 38AA5CE4  addi r5, r10, 0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + 23780;
	// 826C6458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C645C: 390B64F0  addi r8, r11, 0x64f0
	ctx.r[8].s64 = ctx.r[11].s64 + 25840;
	// 826C6460: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C6464: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 826C6468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C646C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6470: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6478: 386A5D74  addi r3, r10, 0x5d74
	ctx.r[3].s64 = ctx.r[10].s64 + 23924;
	// 826C647C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C648C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C649C: 4BDA0985  bl 0x82466e20
	ctx.lr = 0x826C64A0;
	sub_82466E20(ctx, base);
	// 826C64A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C64A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C64A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C64AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C64B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C64B0 size=112
    let mut pc: u32 = 0x826C64B0;
    'dispatch: loop {
        match pc {
            0x826C64B0 => {
    //   block [0x826C64B0..0x826C6520)
	// 826C64B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C64B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C64B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C64BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C64C0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C64C4: 38AA5CE4  addi r5, r10, 0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + 23780;
	// 826C64C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C64CC: 390B6520  addi r8, r11, 0x6520
	ctx.r[8].s64 = ctx.r[11].s64 + 25888;
	// 826C64D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C64D4: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 826C64D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C64DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C64E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C64E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C64E8: 386A5DA4  addi r3, r10, 0x5da4
	ctx.r[3].s64 = ctx.r[10].s64 + 23972;
	// 826C64EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C64F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C64F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C64F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C64FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C650C: 4BDA0915  bl 0x82466e20
	ctx.lr = 0x826C6510;
	sub_82466E20(ctx, base);
	// 826C6510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C651C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6520 size=108
    let mut pc: u32 = 0x826C6520;
    'dispatch: loop {
        match pc {
            0x826C6520 => {
    //   block [0x826C6520..0x826C658C)
	// 826C6520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C652C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6530: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6534: 38EB6550  addi r7, r11, 0x6550
	ctx.r[7].s64 = ctx.r[11].s64 + 25936;
	// 826C6538: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C653C: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 826C6540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6544: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6548: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C654C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6550: 386A5DD4  addi r3, r10, 0x5dd4
	ctx.r[3].s64 = ctx.r[10].s64 + 24020;
	// 826C6554: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C6558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C655C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C656C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6574: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C6578: 4BDA08A9  bl 0x82466e20
	ctx.lr = 0x826C657C;
	sub_82466E20(ctx, base);
	// 826C657C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6590 size=112
    let mut pc: u32 = 0x826C6590;
    'dispatch: loop {
        match pc {
            0x826C6590 => {
    //   block [0x826C6590..0x826C6600)
	// 826C6590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C659C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C65A0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C65A4: 38AA5CE4  addi r5, r10, 0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + 23780;
	// 826C65A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C65AC: 390B6598  addi r8, r11, 0x6598
	ctx.r[8].s64 = ctx.r[11].s64 + 26008;
	// 826C65B0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826C65B4: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 826C65B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C65BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C65C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C65C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C65C8: 386A5E04  addi r3, r10, 0x5e04
	ctx.r[3].s64 = ctx.r[10].s64 + 24068;
	// 826C65CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C65D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C65D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C65D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C65DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C65E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C65E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C65E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C65EC: 4BDA0835  bl 0x82466e20
	ctx.lr = 0x826C65F0;
	sub_82466E20(ctx, base);
	// 826C65F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C65F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C65F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C65FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6600 size=116
    let mut pc: u32 = 0x826C6600;
    'dispatch: loop {
        match pc {
            0x826C6600 => {
    //   block [0x826C6600..0x826C6674)
	// 826C6600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C660C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C6610: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6614: 392B23B4  addi r9, r11, 0x23b4
	ctx.r[9].s64 = ctx.r[11].s64 + 9140;
	// 826C6618: 38AA5CE4  addi r5, r10, 0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + 23780;
	// 826C661C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6620: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826C6624: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 826C6628: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C662C: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 826C6630: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6634: 396B6618  addi r11, r11, 0x6618
	ctx.r[11].s64 = ctx.r[11].s64 + 26136;
	// 826C6638: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826C663C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6640: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826C6644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6648: 386A5E34  addi r3, r10, 0x5e34
	ctx.r[3].s64 = ctx.r[10].s64 + 24116;
	// 826C664C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C6650: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826C6654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6658: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826C665C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C6660: 4BDA07C1  bl 0x82466e20
	ctx.lr = 0x826C6664;
	sub_82466E20(ctx, base);
	// 826C6664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C666C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6678 size=100
    let mut pc: u32 = 0x826C6678;
    'dispatch: loop {
        match pc {
            0x826C6678 => {
    //   block [0x826C6678..0x826C66DC)
	// 826C6678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C667C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6684: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C668C: 38AA5F84  addi r5, r10, 0x5f84
	ctx.r[5].s64 = ctx.r[10].s64 + 24452;
	// 826C6690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6698: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 826C669C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C66A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C66A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C66A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C66AC: 386A5E64  addi r3, r10, 0x5e64
	ctx.r[3].s64 = ctx.r[10].s64 + 24164;
	// 826C66B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C66B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C66B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C66BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C66C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C66C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C66C8: 4BDA0759  bl 0x82466e20
	ctx.lr = 0x826C66CC;
	sub_82466E20(ctx, base);
	// 826C66CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C66D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C66D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C66D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C66E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C66E0 size=100
    let mut pc: u32 = 0x826C66E0;
    'dispatch: loop {
        match pc {
            0x826C66E0 => {
    //   block [0x826C66E0..0x826C6744)
	// 826C66E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C66E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C66E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C66EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C66F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C66F4: 38AA5B34  addi r5, r10, 0x5b34
	ctx.r[5].s64 = ctx.r[10].s64 + 23348;
	// 826C66F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C66FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6700: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 826C6704: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C670C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6714: 386A5E94  addi r3, r10, 0x5e94
	ctx.r[3].s64 = ctx.r[10].s64 + 24212;
	// 826C6718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C671C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6720: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C6724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6728: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C672C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6730: 4BDA06F1  bl 0x82466e20
	ctx.lr = 0x826C6734;
	sub_82466E20(ctx, base);
	// 826C6734: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C673C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6748 size=108
    let mut pc: u32 = 0x826C6748;
    'dispatch: loop {
        match pc {
            0x826C6748 => {
    //   block [0x826C6748..0x826C67B4)
	// 826C6748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C674C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6754: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C675C: 38EB66A8  addi r7, r11, 0x66a8
	ctx.r[7].s64 = ctx.r[11].s64 + 26280;
	// 826C6760: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C6764: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 826C6768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C676C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C6774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6778: 386A5EC4  addi r3, r10, 0x5ec4
	ctx.r[3].s64 = ctx.r[10].s64 + 24260;
	// 826C677C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C6780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C678C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C679C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C67A0: 4BDA0681  bl 0x82466e20
	ctx.lr = 0x826C67A4;
	sub_82466E20(ctx, base);
	// 826C67A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C67A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C67AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C67B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C67B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C67B8 size=112
    let mut pc: u32 = 0x826C67B8;
    'dispatch: loop {
        match pc {
            0x826C67B8 => {
    //   block [0x826C67B8..0x826C6828)
	// 826C67B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C67BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C67C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C67C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C67C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C67CC: 38AA5CB4  addi r5, r10, 0x5cb4
	ctx.r[5].s64 = ctx.r[10].s64 + 23732;
	// 826C67D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C67D4: 390B66D8  addi r8, r11, 0x66d8
	ctx.r[8].s64 = ctx.r[11].s64 + 26328;
	// 826C67D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C67DC: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 826C67E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C67E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C67E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C67EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C67F0: 386A5EF4  addi r3, r10, 0x5ef4
	ctx.r[3].s64 = ctx.r[10].s64 + 24308;
	// 826C67F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C67F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C67FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C680C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6814: 4BDA060D  bl 0x82466e20
	ctx.lr = 0x826C6818;
	sub_82466E20(ctx, base);
	// 826C6818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C681C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6828 size=108
    let mut pc: u32 = 0x826C6828;
    'dispatch: loop {
        match pc {
            0x826C6828 => {
    //   block [0x826C6828..0x826C6894)
	// 826C6828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C682C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6834: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C683C: 38EB66F0  addi r7, r11, 0x66f0
	ctx.r[7].s64 = ctx.r[11].s64 + 26352;
	// 826C6840: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C6844: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 826C6848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C684C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C6854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6858: 386A5F24  addi r3, r10, 0x5f24
	ctx.r[3].s64 = ctx.r[10].s64 + 24356;
	// 826C685C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C6860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C686C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C687C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C6880: 4BDA05A1  bl 0x82466e20
	ctx.lr = 0x826C6884;
	sub_82466E20(ctx, base);
	// 826C6884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C688C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C6898 size=28
    let mut pc: u32 = 0x826C6898;
    'dispatch: loop {
        match pc {
            0x826C6898 => {
    //   block [0x826C6898..0x826C68B4)
	// 826C6898: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C689C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C68A0: 394AA2E0  addi r10, r10, -0x5d20
	ctx.r[10].s64 = ctx.r[10].s64 + -23840;
	// 826C68A4: 816B6614  lwz r11, 0x6614(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26132 as u32) ) } as u64;
	// 826C68A8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826C68AC: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826C68B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C68B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C68B8 size=108
    let mut pc: u32 = 0x826C68B8;
    'dispatch: loop {
        match pc {
            0x826C68B8 => {
    //   block [0x826C68B8..0x826C6924)
	// 826C68B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C68BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C68C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C68C4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C68C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C68CC: 38EBA2E0  addi r7, r11, -0x5d20
	ctx.r[7].s64 = ctx.r[11].s64 + -23840;
	// 826C68D0: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826C68D4: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 826C68D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C68DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C68E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C68E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C68E8: 386A5F54  addi r3, r10, 0x5f54
	ctx.r[3].s64 = ctx.r[10].s64 + 24404;
	// 826C68EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C68F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C68F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C68F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C68FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C690C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C6910: 4BDA0511  bl 0x82466e20
	ctx.lr = 0x826C6914;
	sub_82466E20(ctx, base);
	// 826C6914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C691C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6928 size=116
    let mut pc: u32 = 0x826C6928;
    'dispatch: loop {
        match pc {
            0x826C6928 => {
    //   block [0x826C6928..0x826C699C)
	// 826C6928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C692C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6934: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6938: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C693C: 390B6710  addi r8, r11, 0x6710
	ctx.r[8].s64 = ctx.r[11].s64 + 26384;
	// 826C6940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6944: 392A2424  addi r9, r10, 0x2424
	ctx.r[9].s64 = ctx.r[10].s64 + 9252;
	// 826C6948: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C694C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826C6950: 38AA5CB4  addi r5, r10, 0x5cb4
	ctx.r[5].s64 = ctx.r[10].s64 + 23732;
	// 826C6954: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C695C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C696C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C6970: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 826C6974: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C6978: 386B5F84  addi r3, r11, 0x5f84
	ctx.r[3].s64 = ctx.r[11].s64 + 24452;
	// 826C697C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826C6980: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6988: 4BDA0499  bl 0x82466e20
	ctx.lr = 0x826C698C;
	sub_82466E20(ctx, base);
	// 826C698C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C69A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C69A0 size=112
    let mut pc: u32 = 0x826C69A0;
    'dispatch: loop {
        match pc {
            0x826C69A0 => {
    //   block [0x826C69A0..0x826C6A10)
	// 826C69A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C69A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C69A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C69AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C69B0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C69B4: 38AA5C54  addi r5, r10, 0x5c54
	ctx.r[5].s64 = ctx.r[10].s64 + 23636;
	// 826C69B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C69BC: 390B6788  addi r8, r11, 0x6788
	ctx.r[8].s64 = ctx.r[11].s64 + 26504;
	// 826C69C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C69C4: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 826C69C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C69CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C69D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C69D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C69D8: 386A5FB4  addi r3, r10, 0x5fb4
	ctx.r[3].s64 = ctx.r[10].s64 + 24500;
	// 826C69DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C69E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C69E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C69E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C69EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C69F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C69F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C69F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C69FC: 4BDA0425  bl 0x82466e20
	ctx.lr = 0x826C6A00;
	sub_82466E20(ctx, base);
	// 826C6A00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6A04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6A08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6A10 size=108
    let mut pc: u32 = 0x826C6A10;
    'dispatch: loop {
        match pc {
            0x826C6A10 => {
    //   block [0x826C6A10..0x826C6A7C)
	// 826C6A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6A1C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6A24: 38EB67A0  addi r7, r11, 0x67a0
	ctx.r[7].s64 = ctx.r[11].s64 + 26528;
	// 826C6A28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C6A2C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 826C6A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6A34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6A38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C6A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6A40: 386A5FE4  addi r3, r10, 0x5fe4
	ctx.r[3].s64 = ctx.r[10].s64 + 24548;
	// 826C6A44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C6A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6A64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C6A68: 4BDA03B9  bl 0x82466e20
	ctx.lr = 0x826C6A6C;
	sub_82466E20(ctx, base);
	// 826C6A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6A80 size=112
    let mut pc: u32 = 0x826C6A80;
    'dispatch: loop {
        match pc {
            0x826C6A80 => {
    //   block [0x826C6A80..0x826C6AF0)
	// 826C6A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6A8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6A90: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6A94: 38AA5B34  addi r5, r10, 0x5b34
	ctx.r[5].s64 = ctx.r[10].s64 + 23348;
	// 826C6A98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6A9C: 390B67D0  addi r8, r11, 0x67d0
	ctx.r[8].s64 = ctx.r[11].s64 + 26576;
	// 826C6AA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C6AA4: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 826C6AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6AAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6AB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6AB8: 386A6014  addi r3, r10, 0x6014
	ctx.r[3].s64 = ctx.r[10].s64 + 24596;
	// 826C6ABC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6AC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6ADC: 4BDA0345  bl 0x82466e20
	ctx.lr = 0x826C6AE0;
	sub_82466E20(ctx, base);
	// 826C6AE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6AF0 size=112
    let mut pc: u32 = 0x826C6AF0;
    'dispatch: loop {
        match pc {
            0x826C6AF0 => {
    //   block [0x826C6AF0..0x826C6B60)
	// 826C6AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6AFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6B00: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6B04: 38AA6194  addi r5, r10, 0x6194
	ctx.r[5].s64 = ctx.r[10].s64 + 24980;
	// 826C6B08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6B0C: 390B6800  addi r8, r11, 0x6800
	ctx.r[8].s64 = ctx.r[11].s64 + 26624;
	// 826C6B10: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C6B14: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 826C6B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6B1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6B20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6B28: 386A6044  addi r3, r10, 0x6044
	ctx.r[3].s64 = ctx.r[10].s64 + 24644;
	// 826C6B2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6B30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6B34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6B3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6B44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6B4C: 4BDA02D5  bl 0x82466e20
	ctx.lr = 0x826C6B50;
	sub_82466E20(ctx, base);
	// 826C6B50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6B60 size=100
    let mut pc: u32 = 0x826C6B60;
    'dispatch: loop {
        match pc {
            0x826C6B60 => {
    //   block [0x826C6B60..0x826C6BC4)
	// 826C6B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6B6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6B74: 38AA5B34  addi r5, r10, 0x5b34
	ctx.r[5].s64 = ctx.r[10].s64 + 23348;
	// 826C6B78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6B80: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 826C6B84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6B88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6B8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6B90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6B94: 386A6074  addi r3, r10, 0x6074
	ctx.r[3].s64 = ctx.r[10].s64 + 24692;
	// 826C6B98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6B9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6BA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C6BA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6BA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C6BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6BB0: 4BDA0271  bl 0x82466e20
	ctx.lr = 0x826C6BB4;
	sub_82466E20(ctx, base);
	// 826C6BB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6BC8 size=112
    let mut pc: u32 = 0x826C6BC8;
    'dispatch: loop {
        match pc {
            0x826C6BC8 => {
    //   block [0x826C6BC8..0x826C6C38)
	// 826C6BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6BD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6BD8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6BDC: 38AA5E94  addi r5, r10, 0x5e94
	ctx.r[5].s64 = ctx.r[10].s64 + 24212;
	// 826C6BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6BE4: 390B6830  addi r8, r11, 0x6830
	ctx.r[8].s64 = ctx.r[11].s64 + 26672;
	// 826C6BE8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C6BEC: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 826C6BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6BF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6C00: 386A60A4  addi r3, r10, 0x60a4
	ctx.r[3].s64 = ctx.r[10].s64 + 24740;
	// 826C6C04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6C24: 4BDA01FD  bl 0x82466e20
	ctx.lr = 0x826C6C28;
	sub_82466E20(ctx, base);
	// 826C6C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6C38 size=112
    let mut pc: u32 = 0x826C6C38;
    'dispatch: loop {
        match pc {
            0x826C6C38 => {
    //   block [0x826C6C38..0x826C6CA8)
	// 826C6C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6C44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6C48: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6C4C: 38AA5E94  addi r5, r10, 0x5e94
	ctx.r[5].s64 = ctx.r[10].s64 + 24212;
	// 826C6C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6C54: 390B6878  addi r8, r11, 0x6878
	ctx.r[8].s64 = ctx.r[11].s64 + 26744;
	// 826C6C58: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826C6C5C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 826C6C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6C64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6C70: 386A60D4  addi r3, r10, 0x60d4
	ctx.r[3].s64 = ctx.r[10].s64 + 24788;
	// 826C6C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6C94: 4BDA018D  bl 0x82466e20
	ctx.lr = 0x826C6C98;
	sub_82466E20(ctx, base);
	// 826C6C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6CA8 size=108
    let mut pc: u32 = 0x826C6CA8;
    'dispatch: loop {
        match pc {
            0x826C6CA8 => {
    //   block [0x826C6CA8..0x826C6D14)
	// 826C6CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6CB4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6CB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6CBC: 38EB6920  addi r7, r11, 0x6920
	ctx.r[7].s64 = ctx.r[11].s64 + 26912;
	// 826C6CC0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C6CC4: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 826C6CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6CCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6CD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C6CD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6CD8: 386A6104  addi r3, r10, 0x6104
	ctx.r[3].s64 = ctx.r[10].s64 + 24836;
	// 826C6CDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C6CE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6CFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C6D00: 4BDA0121  bl 0x82466e20
	ctx.lr = 0x826C6D04;
	sub_82466E20(ctx, base);
	// 826C6D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6D18 size=112
    let mut pc: u32 = 0x826C6D18;
    'dispatch: loop {
        match pc {
            0x826C6D18 => {
    //   block [0x826C6D18..0x826C6D88)
	// 826C6D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6D24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6D28: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6D2C: 38AA5CB4  addi r5, r10, 0x5cb4
	ctx.r[5].s64 = ctx.r[10].s64 + 23732;
	// 826C6D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6D34: 390B6968  addi r8, r11, 0x6968
	ctx.r[8].s64 = ctx.r[11].s64 + 26984;
	// 826C6D38: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C6D3C: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 826C6D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6D44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6D50: 386A6134  addi r3, r10, 0x6134
	ctx.r[3].s64 = ctx.r[10].s64 + 24884;
	// 826C6D54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6D64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6D74: 4BDA00AD  bl 0x82466e20
	ctx.lr = 0x826C6D78;
	sub_82466E20(ctx, base);
	// 826C6D78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6D88 size=100
    let mut pc: u32 = 0x826C6D88;
    'dispatch: loop {
        match pc {
            0x826C6D88 => {
    //   block [0x826C6D88..0x826C6DEC)
	// 826C6D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6D94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6D98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6D9C: 38AA5CE4  addi r5, r10, 0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + 23780;
	// 826C6DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6DA8: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 826C6DAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6DB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6DBC: 386A6164  addi r3, r10, 0x6164
	ctx.r[3].s64 = ctx.r[10].s64 + 24932;
	// 826C6DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6DC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6DC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C6DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6DD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C6DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6DD8: 4BDA0049  bl 0x82466e20
	ctx.lr = 0x826C6DDC;
	sub_82466E20(ctx, base);
	// 826C6DDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6DE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6DE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6DF0 size=100
    let mut pc: u32 = 0x826C6DF0;
    'dispatch: loop {
        match pc {
            0x826C6DF0 => {
    //   block [0x826C6DF0..0x826C6E54)
	// 826C6DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6DFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6E00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6E04: 38AA5B34  addi r5, r10, 0x5b34
	ctx.r[5].s64 = ctx.r[10].s64 + 23348;
	// 826C6E08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6E10: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 826C6E14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6E1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6E24: 386A6194  addi r3, r10, 0x6194
	ctx.r[3].s64 = ctx.r[10].s64 + 24980;
	// 826C6E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6E2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6E30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C6E34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6E38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C6E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6E40: 4BD9FFE1  bl 0x82466e20
	ctx.lr = 0x826C6E44;
	sub_82466E20(ctx, base);
	// 826C6E44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6E58 size=112
    let mut pc: u32 = 0x826C6E58;
    'dispatch: loop {
        match pc {
            0x826C6E58 => {
    //   block [0x826C6E58..0x826C6EC8)
	// 826C6E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6E60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6E64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6E68: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6E6C: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C6E70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6E74: 390B69C8  addi r8, r11, 0x69c8
	ctx.r[8].s64 = ctx.r[11].s64 + 27080;
	// 826C6E78: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826C6E7C: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 826C6E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6E84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6E88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6E8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6E90: 386A61C4  addi r3, r10, 0x61c4
	ctx.r[3].s64 = ctx.r[10].s64 + 25028;
	// 826C6E94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6E98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6E9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6EA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6EA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6EA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6EB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6EB4: 4BD9FF6D  bl 0x82466e20
	ctx.lr = 0x826C6EB8;
	sub_82466E20(ctx, base);
	// 826C6EB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6EC8 size=112
    let mut pc: u32 = 0x826C6EC8;
    'dispatch: loop {
        match pc {
            0x826C6EC8 => {
    //   block [0x826C6EC8..0x826C6F38)
	// 826C6EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6ED4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6ED8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6EDC: 38AA5F84  addi r5, r10, 0x5f84
	ctx.r[5].s64 = ctx.r[10].s64 + 24452;
	// 826C6EE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6EE4: 390B6A58  addi r8, r11, 0x6a58
	ctx.r[8].s64 = ctx.r[11].s64 + 27224;
	// 826C6EE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C6EEC: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 826C6EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6EF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6EF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6F00: 386A61F4  addi r3, r10, 0x61f4
	ctx.r[3].s64 = ctx.r[10].s64 + 25076;
	// 826C6F04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6F1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6F24: 4BD9FEFD  bl 0x82466e20
	ctx.lr = 0x826C6F28;
	sub_82466E20(ctx, base);
	// 826C6F28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6F38 size=112
    let mut pc: u32 = 0x826C6F38;
    'dispatch: loop {
        match pc {
            0x826C6F38 => {
    //   block [0x826C6F38..0x826C6FA8)
	// 826C6F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6F40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6F44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6F48: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6F4C: 38AA60D4  addi r5, r10, 0x60d4
	ctx.r[5].s64 = ctx.r[10].s64 + 24788;
	// 826C6F50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6F54: 390B6A70  addi r8, r11, 0x6a70
	ctx.r[8].s64 = ctx.r[11].s64 + 27248;
	// 826C6F58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C6F5C: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 826C6F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6F64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6F68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6F70: 386A6224  addi r3, r10, 0x6224
	ctx.r[3].s64 = ctx.r[10].s64 + 25124;
	// 826C6F74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6F7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6F80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6F88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6F8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6F94: 4BD9FE8D  bl 0x82466e20
	ctx.lr = 0x826C6F98;
	sub_82466E20(ctx, base);
	// 826C6F98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6FA8 size=112
    let mut pc: u32 = 0x826C6FA8;
    'dispatch: loop {
        match pc {
            0x826C6FA8 => {
    //   block [0x826C6FA8..0x826C7018)
	// 826C6FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6FB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6FB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6FB8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6FBC: 38AA5B34  addi r5, r10, 0x5b34
	ctx.r[5].s64 = ctx.r[10].s64 + 23348;
	// 826C6FC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6FC4: 390B6AA0  addi r8, r11, 0x6aa0
	ctx.r[8].s64 = ctx.r[11].s64 + 27296;
	// 826C6FC8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C6FCC: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 826C6FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6FD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6FD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6FDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6FE0: 386A6254  addi r3, r10, 0x6254
	ctx.r[3].s64 = ctx.r[10].s64 + 25172;
	// 826C6FE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6FE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6FEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6FF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6FF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6FFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7004: 4BD9FE1D  bl 0x82466e20
	ctx.lr = 0x826C7008;
	sub_82466E20(ctx, base);
	// 826C7008: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C700C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7018 size=112
    let mut pc: u32 = 0x826C7018;
    'dispatch: loop {
        match pc {
            0x826C7018 => {
    //   block [0x826C7018..0x826C7088)
	// 826C7018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C701C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7024: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7028: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C702C: 38AA5CE4  addi r5, r10, 0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + 23780;
	// 826C7030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7034: 390B6AE8  addi r8, r11, 0x6ae8
	ctx.r[8].s64 = ctx.r[11].s64 + 27368;
	// 826C7038: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C703C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 826C7040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7044: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7048: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C704C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7050: 386A6284  addi r3, r10, 0x6284
	ctx.r[3].s64 = ctx.r[10].s64 + 25220;
	// 826C7054: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C705C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7064: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C706C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7074: 4BD9FDAD  bl 0x82466e20
	ctx.lr = 0x826C7078;
	sub_82466E20(ctx, base);
	// 826C7078: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C707C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7088 size=112
    let mut pc: u32 = 0x826C7088;
    'dispatch: loop {
        match pc {
            0x826C7088 => {
    //   block [0x826C7088..0x826C70F8)
	// 826C7088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C708C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7094: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7098: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C709C: 38AA5C54  addi r5, r10, 0x5c54
	ctx.r[5].s64 = ctx.r[10].s64 + 23636;
	// 826C70A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C70A4: 390B6B30  addi r8, r11, 0x6b30
	ctx.r[8].s64 = ctx.r[11].s64 + 27440;
	// 826C70A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C70AC: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 826C70B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C70B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C70B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C70BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C70C0: 386A62B4  addi r3, r10, 0x62b4
	ctx.r[3].s64 = ctx.r[10].s64 + 25268;
	// 826C70C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C70C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C70CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C70D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C70D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C70D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C70DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C70E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C70E4: 4BD9FD3D  bl 0x82466e20
	ctx.lr = 0x826C70E8;
	sub_82466E20(ctx, base);
	// 826C70E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C70EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C70F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C70F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C70F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C70F8 size=112
    let mut pc: u32 = 0x826C70F8;
    'dispatch: loop {
        match pc {
            0x826C70F8 => {
    //   block [0x826C70F8..0x826C7168)
	// 826C70F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C70FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7104: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7108: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C710C: 38AA5CB4  addi r5, r10, 0x5cb4
	ctx.r[5].s64 = ctx.r[10].s64 + 23732;
	// 826C7110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7114: 390B6B48  addi r8, r11, 0x6b48
	ctx.r[8].s64 = ctx.r[11].s64 + 27464;
	// 826C7118: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C711C: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 826C7120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7124: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7128: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C712C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7130: 386A62E4  addi r3, r10, 0x62e4
	ctx.r[3].s64 = ctx.r[10].s64 + 25316;
	// 826C7134: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C713C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C714C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7154: 4BD9FCCD  bl 0x82466e20
	ctx.lr = 0x826C7158;
	sub_82466E20(ctx, base);
	// 826C7158: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C715C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C7168 size=24
    let mut pc: u32 = 0x826C7168;
    'dispatch: loop {
        match pc {
            0x826C7168 => {
    //   block [0x826C7168..0x826C7180)
	// 826C7168: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C716C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C7170: 394AA418  addi r10, r10, -0x5be8
	ctx.r[10].s64 = ctx.r[10].s64 + -23528;
	// 826C7174: 816B670C  lwz r11, 0x670c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26380 as u32) ) } as u64;
	// 826C7178: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826C717C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7180 size=112
    let mut pc: u32 = 0x826C7180;
    'dispatch: loop {
        match pc {
            0x826C7180 => {
    //   block [0x826C7180..0x826C71F0)
	// 826C7180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C718C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C7190: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C7194: 392A2548  addi r9, r10, 0x2548
	ctx.r[9].s64 = ctx.r[10].s64 + 9544;
	// 826C7198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C719C: 390BA418  addi r8, r11, -0x5be8
	ctx.r[8].s64 = ctx.r[11].s64 + -23528;
	// 826C71A0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826C71A4: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 826C71A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C71AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C71B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C71B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C71B8: 386A6314  addi r3, r10, 0x6314
	ctx.r[3].s64 = ctx.r[10].s64 + 25364;
	// 826C71BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C71C0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826C71C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C71C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C71CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C71D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C71D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C71D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C71DC: 4BD9FC45  bl 0x82466e20
	ctx.lr = 0x826C71E0;
	sub_82466E20(ctx, base);
	// 826C71E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C71E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C71E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C71EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C71F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C71F0 size=112
    let mut pc: u32 = 0x826C71F0;
    'dispatch: loop {
        match pc {
            0x826C71F0 => {
    //   block [0x826C71F0..0x826C7260)
	// 826C71F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C71F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C71F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C71FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7200: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7204: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C720C: 390B6B7C  addi r8, r11, 0x6b7c
	ctx.r[8].s64 = ctx.r[11].s64 + 27516;
	// 826C7210: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C7214: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 826C7218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C721C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7228: 386A6344  addi r3, r10, 0x6344
	ctx.r[3].s64 = ctx.r[10].s64 + 25412;
	// 826C722C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C723C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C724C: 4BD9FBD5  bl 0x82466e20
	ctx.lr = 0x826C7250;
	sub_82466E20(ctx, base);
	// 826C7250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C725C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7260 size=108
    let mut pc: u32 = 0x826C7260;
    'dispatch: loop {
        match pc {
            0x826C7260 => {
    //   block [0x826C7260..0x826C72CC)
	// 826C7260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C726C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7274: 38EB6BAC  addi r7, r11, 0x6bac
	ctx.r[7].s64 = ctx.r[11].s64 + 27564;
	// 826C7278: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C727C: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 826C7280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7284: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C728C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7290: 386A6374  addi r3, r10, 0x6374
	ctx.r[3].s64 = ctx.r[10].s64 + 25460;
	// 826C7294: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C7298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C729C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C72A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C72A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C72A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C72AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C72B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C72B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C72B8: 4BD9FB69  bl 0x82466e20
	ctx.lr = 0x826C72BC;
	sub_82466E20(ctx, base);
	// 826C72BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C72C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C72C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C72C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C72D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C72D0 size=100
    let mut pc: u32 = 0x826C72D0;
    'dispatch: loop {
        match pc {
            0x826C72D0 => {
    //   block [0x826C72D0..0x826C7334)
	// 826C72D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C72D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C72D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C72DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C72E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C72E4: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C72E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C72EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C72F0: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 826C72F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C72F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C72FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7304: 386A63A4  addi r3, r10, 0x63a4
	ctx.r[3].s64 = ctx.r[10].s64 + 25508;
	// 826C7308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C730C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7310: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C7314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7318: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C731C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7320: 4BD9FB01  bl 0x82466e20
	ctx.lr = 0x826C7324;
	sub_82466E20(ctx, base);
	// 826C7324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C732C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7338 size=112
    let mut pc: u32 = 0x826C7338;
    'dispatch: loop {
        match pc {
            0x826C7338 => {
    //   block [0x826C7338..0x826C73A8)
	// 826C7338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C733C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7344: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7348: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C734C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7354: 390B6BC4  addi r8, r11, 0x6bc4
	ctx.r[8].s64 = ctx.r[11].s64 + 27588;
	// 826C7358: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C735C: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 826C7360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7364: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C736C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7370: 386A63D4  addi r3, r10, 0x63d4
	ctx.r[3].s64 = ctx.r[10].s64 + 25556;
	// 826C7374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C737C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C738C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7394: 4BD9FA8D  bl 0x82466e20
	ctx.lr = 0x826C7398;
	sub_82466E20(ctx, base);
	// 826C7398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C739C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C73A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C73A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C73A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C73A8 size=112
    let mut pc: u32 = 0x826C73A8;
    'dispatch: loop {
        match pc {
            0x826C73A8 => {
    //   block [0x826C73A8..0x826C7418)
	// 826C73A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C73AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C73B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C73B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C73B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C73BC: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C73C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C73C4: 390B6BDC  addi r8, r11, 0x6bdc
	ctx.r[8].s64 = ctx.r[11].s64 + 27612;
	// 826C73C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C73CC: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 826C73D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C73D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C73D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C73DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C73E0: 386A6404  addi r3, r10, 0x6404
	ctx.r[3].s64 = ctx.r[10].s64 + 25604;
	// 826C73E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C73E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C73EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C73F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C73F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C73F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C73FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7404: 4BD9FA1D  bl 0x82466e20
	ctx.lr = 0x826C7408;
	sub_82466E20(ctx, base);
	// 826C7408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C740C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7418 size=112
    let mut pc: u32 = 0x826C7418;
    'dispatch: loop {
        match pc {
            0x826C7418 => {
    //   block [0x826C7418..0x826C7488)
	// 826C7418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C741C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7424: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7428: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C742C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7434: 390B6C0C  addi r8, r11, 0x6c0c
	ctx.r[8].s64 = ctx.r[11].s64 + 27660;
	// 826C7438: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C743C: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 826C7440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7444: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C744C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7450: 386A6434  addi r3, r10, 0x6434
	ctx.r[3].s64 = ctx.r[10].s64 + 25652;
	// 826C7454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C745C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C746C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7474: 4BD9F9AD  bl 0x82466e20
	ctx.lr = 0x826C7478;
	sub_82466E20(ctx, base);
	// 826C7478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C747C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7488 size=112
    let mut pc: u32 = 0x826C7488;
    'dispatch: loop {
        match pc {
            0x826C7488 => {
    //   block [0x826C7488..0x826C74F8)
	// 826C7488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C748C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7494: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7498: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C749C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C74A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C74A4: 390B6C3C  addi r8, r11, 0x6c3c
	ctx.r[8].s64 = ctx.r[11].s64 + 27708;
	// 826C74A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C74AC: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 826C74B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C74B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C74B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C74BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C74C0: 386A6464  addi r3, r10, 0x6464
	ctx.r[3].s64 = ctx.r[10].s64 + 25700;
	// 826C74C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C74C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C74CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C74D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C74D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C74D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C74DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C74E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C74E4: 4BD9F93D  bl 0x82466e20
	ctx.lr = 0x826C74E8;
	sub_82466E20(ctx, base);
	// 826C74E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C74EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C74F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C74F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C74F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C74F8 size=112
    let mut pc: u32 = 0x826C74F8;
    'dispatch: loop {
        match pc {
            0x826C74F8 => {
    //   block [0x826C74F8..0x826C7568)
	// 826C74F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C74FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7504: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7508: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C750C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7514: 390B6C6C  addi r8, r11, 0x6c6c
	ctx.r[8].s64 = ctx.r[11].s64 + 27756;
	// 826C7518: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C751C: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 826C7520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7524: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C752C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7530: 386A6494  addi r3, r10, 0x6494
	ctx.r[3].s64 = ctx.r[10].s64 + 25748;
	// 826C7534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C753C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C754C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7554: 4BD9F8CD  bl 0x82466e20
	ctx.lr = 0x826C7558;
	sub_82466E20(ctx, base);
	// 826C7558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C755C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7568 size=112
    let mut pc: u32 = 0x826C7568;
    'dispatch: loop {
        match pc {
            0x826C7568 => {
    //   block [0x826C7568..0x826C75D8)
	// 826C7568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C756C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7574: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7578: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C757C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7584: 390B6C84  addi r8, r11, 0x6c84
	ctx.r[8].s64 = ctx.r[11].s64 + 27780;
	// 826C7588: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C758C: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 826C7590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7594: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C759C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C75A0: 386A64C4  addi r3, r10, 0x64c4
	ctx.r[3].s64 = ctx.r[10].s64 + 25796;
	// 826C75A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C75A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C75AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C75B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C75B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C75B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C75BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C75C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C75C4: 4BD9F85D  bl 0x82466e20
	ctx.lr = 0x826C75C8;
	sub_82466E20(ctx, base);
	// 826C75C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C75CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C75D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C75D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C75D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C75D8 size=112
    let mut pc: u32 = 0x826C75D8;
    'dispatch: loop {
        match pc {
            0x826C75D8 => {
    //   block [0x826C75D8..0x826C7648)
	// 826C75D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C75DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C75E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C75E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C75E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C75EC: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C75F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C75F4: 390B6CA0  addi r8, r11, 0x6ca0
	ctx.r[8].s64 = ctx.r[11].s64 + 27808;
	// 826C75F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C75FC: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 826C7600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7604: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C760C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7610: 386A64F4  addi r3, r10, 0x64f4
	ctx.r[3].s64 = ctx.r[10].s64 + 25844;
	// 826C7614: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C761C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C762C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7634: 4BD9F7ED  bl 0x82466e20
	ctx.lr = 0x826C7638;
	sub_82466E20(ctx, base);
	// 826C7638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C763C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7648 size=112
    let mut pc: u32 = 0x826C7648;
    'dispatch: loop {
        match pc {
            0x826C7648 => {
    //   block [0x826C7648..0x826C76B8)
	// 826C7648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C764C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7654: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7658: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C765C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7664: 390B6CE8  addi r8, r11, 0x6ce8
	ctx.r[8].s64 = ctx.r[11].s64 + 27880;
	// 826C7668: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C766C: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 826C7670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7674: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C767C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7680: 386A6524  addi r3, r10, 0x6524
	ctx.r[3].s64 = ctx.r[10].s64 + 25892;
	// 826C7684: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C768C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C769C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C76A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C76A4: 4BD9F77D  bl 0x82466e20
	ctx.lr = 0x826C76A8;
	sub_82466E20(ctx, base);
	// 826C76A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C76AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C76B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C76B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C76B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C76B8 size=112
    let mut pc: u32 = 0x826C76B8;
    'dispatch: loop {
        match pc {
            0x826C76B8 => {
    //   block [0x826C76B8..0x826C7728)
	// 826C76B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C76BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C76C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C76C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C76C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C76CC: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C76D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C76D4: 390B6D30  addi r8, r11, 0x6d30
	ctx.r[8].s64 = ctx.r[11].s64 + 27952;
	// 826C76D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C76DC: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 826C76E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C76E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C76E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C76EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C76F0: 386A6554  addi r3, r10, 0x6554
	ctx.r[3].s64 = ctx.r[10].s64 + 25940;
	// 826C76F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C76F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C76FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C770C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7714: 4BD9F70D  bl 0x82466e20
	ctx.lr = 0x826C7718;
	sub_82466E20(ctx, base);
	// 826C7718: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C771C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7728 size=112
    let mut pc: u32 = 0x826C7728;
    'dispatch: loop {
        match pc {
            0x826C7728 => {
    //   block [0x826C7728..0x826C7798)
	// 826C7728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C772C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7734: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7738: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C773C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7744: 390B6D48  addi r8, r11, 0x6d48
	ctx.r[8].s64 = ctx.r[11].s64 + 27976;
	// 826C7748: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C774C: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 826C7750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7754: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C775C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7760: 386A6584  addi r3, r10, 0x6584
	ctx.r[3].s64 = ctx.r[10].s64 + 25988;
	// 826C7764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C776C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C777C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7784: 4BD9F69D  bl 0x82466e20
	ctx.lr = 0x826C7788;
	sub_82466E20(ctx, base);
	// 826C7788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C778C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7798 size=116
    let mut pc: u32 = 0x826C7798;
    'dispatch: loop {
        match pc {
            0x826C7798 => {
    //   block [0x826C7798..0x826C780C)
	// 826C7798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C779C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C77A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C77A4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C77A8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826C77AC: 390A6D78  addi r8, r10, 0x6d78
	ctx.r[8].s64 = ctx.r[10].s64 + 28024;
	// 826C77B0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C77B4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C77B8: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C77BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C77C0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C77C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C77C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C77CC: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 826C77D0: 396B2570  addi r11, r11, 0x2570
	ctx.r[11].s64 = ctx.r[11].s64 + 9584;
	// 826C77D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C77D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C77DC: 386A65B4  addi r3, r10, 0x65b4
	ctx.r[3].s64 = ctx.r[10].s64 + 26036;
	// 826C77E0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C77E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C77E8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C77EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C77F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C77F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C77F8: 4BD9F629  bl 0x82466e20
	ctx.lr = 0x826C77FC;
	sub_82466E20(ctx, base);
	// 826C77FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7810 size=116
    let mut pc: u32 = 0x826C7810;
    'dispatch: loop {
        match pc {
            0x826C7810 => {
    //   block [0x826C7810..0x826C7884)
	// 826C7810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C781C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C7820: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826C7824: 390A6DF0  addi r8, r10, 0x6df0
	ctx.r[8].s64 = ctx.r[10].s64 + 28144;
	// 826C7828: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C782C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C7830: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7834: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7838: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C783C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7844: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 826C7848: 396B2588  addi r11, r11, 0x2588
	ctx.r[11].s64 = ctx.r[11].s64 + 9608;
	// 826C784C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7850: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7854: 386A65E4  addi r3, r10, 0x65e4
	ctx.r[3].s64 = ctx.r[10].s64 + 26084;
	// 826C7858: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C785C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7860: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C7864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C786C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7870: 4BD9F5B1  bl 0x82466e20
	ctx.lr = 0x826C7874;
	sub_82466E20(ctx, base);
	// 826C7874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C787C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C7888 size=24
    let mut pc: u32 = 0x826C7888;
    'dispatch: loop {
        match pc {
            0x826C7888 => {
    //   block [0x826C7888..0x826C78A0)
	// 826C7888: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C788C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C7890: 394AA430  addi r10, r10, -0x5bd0
	ctx.r[10].s64 = ctx.r[10].s64 + -23504;
	// 826C7894: 816B6C9C  lwz r11, 0x6c9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27804 as u32) ) } as u64;
	// 826C7898: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826C789C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C78A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C78A0 size=116
    let mut pc: u32 = 0x826C78A0;
    'dispatch: loop {
        match pc {
            0x826C78A0 => {
    //   block [0x826C78A0..0x826C7914)
	// 826C78A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C78A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C78A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C78AC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C78B0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C78B4: 392B25B4  addi r9, r11, 0x25b4
	ctx.r[9].s64 = ctx.r[11].s64 + 9652;
	// 826C78B8: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C78BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C78C0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826C78C4: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826C78C8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C78CC: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 826C78D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C78D4: 396BA430  addi r11, r11, -0x5bd0
	ctx.r[11].s64 = ctx.r[11].s64 + -23504;
	// 826C78D8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826C78DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C78E0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826C78E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C78E8: 386A6614  addi r3, r10, 0x6614
	ctx.r[3].s64 = ctx.r[10].s64 + 26132;
	// 826C78EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C78F0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826C78F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C78F8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826C78FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C7900: 4BD9F521  bl 0x82466e20
	ctx.lr = 0x826C7904;
	sub_82466E20(ctx, base);
	// 826C7904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C790C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7918 size=112
    let mut pc: u32 = 0x826C7918;
    'dispatch: loop {
        match pc {
            0x826C7918 => {
    //   block [0x826C7918..0x826C7988)
	// 826C7918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C791C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7924: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7928: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C792C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7934: 390B6E80  addi r8, r11, 0x6e80
	ctx.r[8].s64 = ctx.r[11].s64 + 28288;
	// 826C7938: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C793C: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 826C7940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7944: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C794C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7950: 386A6644  addi r3, r10, 0x6644
	ctx.r[3].s64 = ctx.r[10].s64 + 26180;
	// 826C7954: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C795C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C796C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7974: 4BD9F4AD  bl 0x82466e20
	ctx.lr = 0x826C7978;
	sub_82466E20(ctx, base);
	// 826C7978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C797C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7988 size=112
    let mut pc: u32 = 0x826C7988;
    'dispatch: loop {
        match pc {
            0x826C7988 => {
    //   block [0x826C7988..0x826C79F8)
	// 826C7988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C798C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7994: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7998: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C799C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C79A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C79A4: 390B6EE0  addi r8, r11, 0x6ee0
	ctx.r[8].s64 = ctx.r[11].s64 + 28384;
	// 826C79A8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826C79AC: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 826C79B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C79B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C79B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C79BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C79C0: 386A6674  addi r3, r10, 0x6674
	ctx.r[3].s64 = ctx.r[10].s64 + 26228;
	// 826C79C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C79C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C79CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C79D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C79D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C79D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C79DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C79E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C79E4: 4BD9F43D  bl 0x82466e20
	ctx.lr = 0x826C79E8;
	sub_82466E20(ctx, base);
	// 826C79E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C79EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C79F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C79F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C79F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C79F8 size=112
    let mut pc: u32 = 0x826C79F8;
    'dispatch: loop {
        match pc {
            0x826C79F8 => {
    //   block [0x826C79F8..0x826C7A68)
	// 826C79F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C79FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7A04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7A08: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7A0C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7A14: 390B6F88  addi r8, r11, 0x6f88
	ctx.r[8].s64 = ctx.r[11].s64 + 28552;
	// 826C7A18: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826C7A1C: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 826C7A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7A24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7A28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7A30: 386A66A4  addi r3, r10, 0x66a4
	ctx.r[3].s64 = ctx.r[10].s64 + 26276;
	// 826C7A34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7A44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7A54: 4BD9F3CD  bl 0x82466e20
	ctx.lr = 0x826C7A58;
	sub_82466E20(ctx, base);
	// 826C7A58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7A68 size=112
    let mut pc: u32 = 0x826C7A68;
    'dispatch: loop {
        match pc {
            0x826C7A68 => {
    //   block [0x826C7A68..0x826C7AD8)
	// 826C7A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7A74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7A78: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7A7C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7A80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7A84: 390B7000  addi r8, r11, 0x7000
	ctx.r[8].s64 = ctx.r[11].s64 + 28672;
	// 826C7A88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C7A8C: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 826C7A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7A94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7A98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7AA0: 386A66D4  addi r3, r10, 0x66d4
	ctx.r[3].s64 = ctx.r[10].s64 + 26324;
	// 826C7AA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7AB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7AC4: 4BD9F35D  bl 0x82466e20
	ctx.lr = 0x826C7AC8;
	sub_82466E20(ctx, base);
	// 826C7AC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7AD8 size=112
    let mut pc: u32 = 0x826C7AD8;
    'dispatch: loop {
        match pc {
            0x826C7AD8 => {
    //   block [0x826C7AD8..0x826C7B48)
	// 826C7AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7AE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7AE8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7AEC: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7AF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7AF4: 390B7048  addi r8, r11, 0x7048
	ctx.r[8].s64 = ctx.r[11].s64 + 28744;
	// 826C7AF8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826C7AFC: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 826C7B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7B04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7B08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7B10: 386A6704  addi r3, r10, 0x6704
	ctx.r[3].s64 = ctx.r[10].s64 + 26372;
	// 826C7B14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7B34: 4BD9F2ED  bl 0x82466e20
	ctx.lr = 0x826C7B38;
	sub_82466E20(ctx, base);
	// 826C7B38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7B48 size=112
    let mut pc: u32 = 0x826C7B48;
    'dispatch: loop {
        match pc {
            0x826C7B48 => {
    //   block [0x826C7B48..0x826C7BB8)
	// 826C7B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7B54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7B58: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7B5C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7B64: 390B70D8  addi r8, r11, 0x70d8
	ctx.r[8].s64 = ctx.r[11].s64 + 28888;
	// 826C7B68: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C7B6C: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 826C7B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7B74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7B78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7B80: 386A6734  addi r3, r10, 0x6734
	ctx.r[3].s64 = ctx.r[10].s64 + 26420;
	// 826C7B84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7BA4: 4BD9F27D  bl 0x82466e20
	ctx.lr = 0x826C7BA8;
	sub_82466E20(ctx, base);
	// 826C7BA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7BB8 size=112
    let mut pc: u32 = 0x826C7BB8;
    'dispatch: loop {
        match pc {
            0x826C7BB8 => {
    //   block [0x826C7BB8..0x826C7C28)
	// 826C7BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7BC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7BC8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7BCC: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7BD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7BD4: 390B7138  addi r8, r11, 0x7138
	ctx.r[8].s64 = ctx.r[11].s64 + 28984;
	// 826C7BD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C7BDC: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 826C7BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7BE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7BE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7BF0: 386A6764  addi r3, r10, 0x6764
	ctx.r[3].s64 = ctx.r[10].s64 + 26468;
	// 826C7BF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7C14: 4BD9F20D  bl 0x82466e20
	ctx.lr = 0x826C7C18;
	sub_82466E20(ctx, base);
	// 826C7C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7C28 size=112
    let mut pc: u32 = 0x826C7C28;
    'dispatch: loop {
        match pc {
            0x826C7C28 => {
    //   block [0x826C7C28..0x826C7C98)
	// 826C7C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7C34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7C38: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7C3C: 38AA6764  addi r5, r10, 0x6764
	ctx.r[5].s64 = ctx.r[10].s64 + 26468;
	// 826C7C40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7C44: 390B7180  addi r8, r11, 0x7180
	ctx.r[8].s64 = ctx.r[11].s64 + 29056;
	// 826C7C48: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C7C4C: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 826C7C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7C54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7C58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7C60: 386A6794  addi r3, r10, 0x6794
	ctx.r[3].s64 = ctx.r[10].s64 + 26516;
	// 826C7C64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7C70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7C74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7C78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7C7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7C80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7C84: 4BD9F19D  bl 0x82466e20
	ctx.lr = 0x826C7C88;
	sub_82466E20(ctx, base);
	// 826C7C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7C98 size=112
    let mut pc: u32 = 0x826C7C98;
    'dispatch: loop {
        match pc {
            0x826C7C98 => {
    //   block [0x826C7C98..0x826C7D08)
	// 826C7C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7CA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7CA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7CAC: 38AA6764  addi r5, r10, 0x6764
	ctx.r[5].s64 = ctx.r[10].s64 + 26468;
	// 826C7CB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7CB4: 390B71B0  addi r8, r11, 0x71b0
	ctx.r[8].s64 = ctx.r[11].s64 + 29104;
	// 826C7CB8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C7CBC: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 826C7CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7CC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7CC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7CCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7CD0: 386A67C4  addi r3, r10, 0x67c4
	ctx.r[3].s64 = ctx.r[10].s64 + 26564;
	// 826C7CD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7CD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7CE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7CE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7CEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7CF4: 4BD9F12D  bl 0x82466e20
	ctx.lr = 0x826C7CF8;
	sub_82466E20(ctx, base);
	// 826C7CF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7D08 size=100
    let mut pc: u32 = 0x826C7D08;
    'dispatch: loop {
        match pc {
            0x826C7D08 => {
    //   block [0x826C7D08..0x826C7D6C)
	// 826C7D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7D14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7D1C: 38AA6764  addi r5, r10, 0x6764
	ctx.r[5].s64 = ctx.r[10].s64 + 26468;
	// 826C7D20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7D28: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 826C7D2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7D3C: 386A67F4  addi r3, r10, 0x67f4
	ctx.r[3].s64 = ctx.r[10].s64 + 26612;
	// 826C7D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7D44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7D48: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C7D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7D50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C7D54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7D58: 4BD9F0C9  bl 0x82466e20
	ctx.lr = 0x826C7D5C;
	sub_82466E20(ctx, base);
	// 826C7D5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7D60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7D64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7D70 size=112
    let mut pc: u32 = 0x826C7D70;
    'dispatch: loop {
        match pc {
            0x826C7D70 => {
    //   block [0x826C7D70..0x826C7DE0)
	// 826C7D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7D7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7D80: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7D84: 38AA6764  addi r5, r10, 0x6764
	ctx.r[5].s64 = ctx.r[10].s64 + 26468;
	// 826C7D88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7D8C: 390B71E0  addi r8, r11, 0x71e0
	ctx.r[8].s64 = ctx.r[11].s64 + 29152;
	// 826C7D90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C7D94: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 826C7D98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7D9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7DA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7DA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7DA8: 386A6824  addi r3, r10, 0x6824
	ctx.r[3].s64 = ctx.r[10].s64 + 26660;
	// 826C7DAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7DB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7DB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7DBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7DC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7DC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7DC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7DCC: 4BD9F055  bl 0x82466e20
	ctx.lr = 0x826C7DD0;
	sub_82466E20(ctx, base);
	// 826C7DD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7DE0 size=112
    let mut pc: u32 = 0x826C7DE0;
    'dispatch: loop {
        match pc {
            0x826C7DE0 => {
    //   block [0x826C7DE0..0x826C7E50)
	// 826C7DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7DEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7DF0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7DF4: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C7DF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7DFC: 390B71F8  addi r8, r11, 0x71f8
	ctx.r[8].s64 = ctx.r[11].s64 + 29176;
	// 826C7E00: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C7E04: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 826C7E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7E0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7E10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7E18: 386A6854  addi r3, r10, 0x6854
	ctx.r[3].s64 = ctx.r[10].s64 + 26708;
	// 826C7E1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7E20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7E2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7E34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7E3C: 4BD9EFE5  bl 0x82466e20
	ctx.lr = 0x826C7E40;
	sub_82466E20(ctx, base);
	// 826C7E40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7E50 size=112
    let mut pc: u32 = 0x826C7E50;
    'dispatch: loop {
        match pc {
            0x826C7E50 => {
    //   block [0x826C7E50..0x826C7EC0)
	// 826C7E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7E58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7E5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7E60: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7E64: 38AA6854  addi r5, r10, 0x6854
	ctx.r[5].s64 = ctx.r[10].s64 + 26708;
	// 826C7E68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7E6C: 390B7258  addi r8, r11, 0x7258
	ctx.r[8].s64 = ctx.r[11].s64 + 29272;
	// 826C7E70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C7E74: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 826C7E78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7E7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7E80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7E84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7E88: 386A6884  addi r3, r10, 0x6884
	ctx.r[3].s64 = ctx.r[10].s64 + 26756;
	// 826C7E8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7E90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7E94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7E98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7E9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7EA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7EA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7EAC: 4BD9EF75  bl 0x82466e20
	ctx.lr = 0x826C7EB0;
	sub_82466E20(ctx, base);
	// 826C7EB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7EC0 size=112
    let mut pc: u32 = 0x826C7EC0;
    'dispatch: loop {
        match pc {
            0x826C7EC0 => {
    //   block [0x826C7EC0..0x826C7F30)
	// 826C7EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7ECC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7ED0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7ED4: 38AA6854  addi r5, r10, 0x6854
	ctx.r[5].s64 = ctx.r[10].s64 + 26708;
	// 826C7ED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7EDC: 390B7270  addi r8, r11, 0x7270
	ctx.r[8].s64 = ctx.r[11].s64 + 29296;
	// 826C7EE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C7EE4: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 826C7EE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7EEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7EF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7EF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7EF8: 386A68B4  addi r3, r10, 0x68b4
	ctx.r[3].s64 = ctx.r[10].s64 + 26804;
	// 826C7EFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7F00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7F04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7F1C: 4BD9EF05  bl 0x82466e20
	ctx.lr = 0x826C7F20;
	sub_82466E20(ctx, base);
	// 826C7F20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7F30 size=112
    let mut pc: u32 = 0x826C7F30;
    'dispatch: loop {
        match pc {
            0x826C7F30 => {
    //   block [0x826C7F30..0x826C7FA0)
	// 826C7F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7F3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7F40: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7F44: 38AA6854  addi r5, r10, 0x6854
	ctx.r[5].s64 = ctx.r[10].s64 + 26708;
	// 826C7F48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7F4C: 390B72A0  addi r8, r11, 0x72a0
	ctx.r[8].s64 = ctx.r[11].s64 + 29344;
	// 826C7F50: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C7F54: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 826C7F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7F5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7F60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7F68: 386A68E4  addi r3, r10, 0x68e4
	ctx.r[3].s64 = ctx.r[10].s64 + 26852;
	// 826C7F6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7F70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7F7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7F8C: 4BD9EE95  bl 0x82466e20
	ctx.lr = 0x826C7F90;
	sub_82466E20(ctx, base);
	// 826C7F90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C7FA0 size=24
    let mut pc: u32 = 0x826C7FA0;
    'dispatch: loop {
        match pc {
            0x826C7FA0 => {
    //   block [0x826C7FA0..0x826C7FB8)
	// 826C7FA0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7FA4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C7FA8: 394AA4D8  addi r10, r10, -0x5b28
	ctx.r[10].s64 = ctx.r[10].s64 + -23336;
	// 826C7FAC: 816B72B8  lwz r11, 0x72b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29368 as u32) ) } as u64;
	// 826C7FB0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826C7FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7FB8 size=112
    let mut pc: u32 = 0x826C7FB8;
    'dispatch: loop {
        match pc {
            0x826C7FB8 => {
    //   block [0x826C7FB8..0x826C8028)
	// 826C7FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7FC4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C7FC8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C7FCC: 392A2610  addi r9, r10, 0x2610
	ctx.r[9].s64 = ctx.r[10].s64 + 9744;
	// 826C7FD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7FD4: 390BA4D8  addi r8, r11, -0x5b28
	ctx.r[8].s64 = ctx.r[11].s64 + -23336;
	// 826C7FD8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826C7FDC: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 826C7FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7FE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7FE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7FF0: 386A6914  addi r3, r10, 0x6914
	ctx.r[3].s64 = ctx.r[10].s64 + 26900;
	// 826C7FF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C7FF8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C7FFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C800C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C8010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8014: 4BD9EE0D  bl 0x82466e20
	ctx.lr = 0x826C8018;
	sub_82466E20(ctx, base);
	// 826C8018: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C801C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8028 size=108
    let mut pc: u32 = 0x826C8028;
    'dispatch: loop {
        match pc {
            0x826C8028 => {
    //   block [0x826C8028..0x826C8094)
	// 826C8028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C802C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8034: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8038: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C803C: 38EB72BC  addi r7, r11, 0x72bc
	ctx.r[7].s64 = ctx.r[11].s64 + 29372;
	// 826C8040: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C8044: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 826C8048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C804C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8050: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C8054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8058: 386A6944  addi r3, r10, 0x6944
	ctx.r[3].s64 = ctx.r[10].s64 + 26948;
	// 826C805C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C8060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8064: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C806C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C807C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C8080: 4BD9EDA1  bl 0x82466e20
	ctx.lr = 0x826C8084;
	sub_82466E20(ctx, base);
	// 826C8084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C808C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8098 size=108
    let mut pc: u32 = 0x826C8098;
    'dispatch: loop {
        match pc {
            0x826C8098 => {
    //   block [0x826C8098..0x826C8104)
	// 826C8098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C809C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C80A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C80A4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C80A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C80AC: 38EB72D8  addi r7, r11, 0x72d8
	ctx.r[7].s64 = ctx.r[11].s64 + 29400;
	// 826C80B0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C80B4: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 826C80B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C80BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C80C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C80C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C80C8: 386A6974  addi r3, r10, 0x6974
	ctx.r[3].s64 = ctx.r[10].s64 + 26996;
	// 826C80CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C80D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C80D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C80D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C80DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C80E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C80E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C80E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C80EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C80F0: 4BD9ED31  bl 0x82466e20
	ctx.lr = 0x826C80F4;
	sub_82466E20(ctx, base);
	// 826C80F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C80F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C80FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8108 size=116
    let mut pc: u32 = 0x826C8108;
    'dispatch: loop {
        match pc {
            0x826C8108 => {
    //   block [0x826C8108..0x826C817C)
	// 826C8108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C810C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8114: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8118: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C811C: 390B7320  addi r8, r11, 0x7320
	ctx.r[8].s64 = ctx.r[11].s64 + 29472;
	// 826C8120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8124: 392A26C8  addi r9, r10, 0x26c8
	ctx.r[9].s64 = ctx.r[10].s64 + 9928;
	// 826C8128: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C812C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826C8130: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C8134: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C813C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C814C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C8150: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 826C8154: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C8158: 386B69A4  addi r3, r11, 0x69a4
	ctx.r[3].s64 = ctx.r[11].s64 + 27044;
	// 826C815C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C8160: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8168: 4BD9ECB9  bl 0x82466e20
	ctx.lr = 0x826C816C;
	sub_82466E20(ctx, base);
	// 826C816C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8180 size=108
    let mut pc: u32 = 0x826C8180;
    'dispatch: loop {
        match pc {
            0x826C8180 => {
    //   block [0x826C8180..0x826C81EC)
	// 826C8180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C818C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8190: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826C8194: 38EB7338  addi r7, r11, 0x7338
	ctx.r[7].s64 = ctx.r[11].s64 + 29496;
	// 826C8198: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C819C: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 826C81A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C81A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C81A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C81AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C81B0: 386A69D4  addi r3, r10, 0x69d4
	ctx.r[3].s64 = ctx.r[10].s64 + 27092;
	// 826C81B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C81B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C81BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C81C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C81C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C81C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C81CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C81D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C81D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C81D8: 4BD9EC49  bl 0x82466e20
	ctx.lr = 0x826C81DC;
	sub_82466E20(ctx, base);
	// 826C81DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C81E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C81E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C81E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C81F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C81F0 size=24
    let mut pc: u32 = 0x826C81F0;
    'dispatch: loop {
        match pc {
            0x826C81F0 => {
    //   block [0x826C81F0..0x826C8208)
	// 826C81F0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C81F4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C81F8: 394AA520  addi r10, r10, -0x5ae0
	ctx.r[10].s64 = ctx.r[10].s64 + -23264;
	// 826C81FC: 816B7398  lwz r11, 0x7398(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29592 as u32) ) } as u64;
	// 826C8200: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826C8204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8208 size=116
    let mut pc: u32 = 0x826C8208;
    'dispatch: loop {
        match pc {
            0x826C8208 => {
    //   block [0x826C8208..0x826C827C)
	// 826C8208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C820C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8214: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C8218: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C821C: 390BA520  addi r8, r11, -0x5ae0
	ctx.r[8].s64 = ctx.r[11].s64 + -23264;
	// 826C8220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8224: 392A2724  addi r9, r10, 0x2724
	ctx.r[9].s64 = ctx.r[10].s64 + 10020;
	// 826C8228: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C822C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826C8230: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C8234: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C823C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C824C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C8250: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 826C8254: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C8258: 386B6A04  addi r3, r11, 0x6a04
	ctx.r[3].s64 = ctx.r[11].s64 + 27140;
	// 826C825C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826C8260: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8268: 4BD9EBB9  bl 0x82466e20
	ctx.lr = 0x826C826C;
	sub_82466E20(ctx, base);
	// 826C826C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8280 size=108
    let mut pc: u32 = 0x826C8280;
    'dispatch: loop {
        match pc {
            0x826C8280 => {
    //   block [0x826C8280..0x826C82EC)
	// 826C8280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C828C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8294: 38EB73A4  addi r7, r11, 0x73a4
	ctx.r[7].s64 = ctx.r[11].s64 + 29604;
	// 826C8298: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C829C: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 826C82A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C82A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C82A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C82AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C82B0: 386A6A34  addi r3, r10, 0x6a34
	ctx.r[3].s64 = ctx.r[10].s64 + 27188;
	// 826C82B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C82B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C82BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C82C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C82C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C82C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C82CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C82D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C82D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C82D8: 4BD9EB49  bl 0x82466e20
	ctx.lr = 0x826C82DC;
	sub_82466E20(ctx, base);
	// 826C82DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C82E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C82E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C82E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C82F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C82F0 size=112
    let mut pc: u32 = 0x826C82F0;
    'dispatch: loop {
        match pc {
            0x826C82F0 => {
    //   block [0x826C82F0..0x826C8360)
	// 826C82F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C82F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C82F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C82FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8300: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8304: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C8308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C830C: 390B73D4  addi r8, r11, 0x73d4
	ctx.r[8].s64 = ctx.r[11].s64 + 29652;
	// 826C8310: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C8314: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 826C8318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C831C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8320: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8324: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8328: 386A6A64  addi r3, r10, 0x6a64
	ctx.r[3].s64 = ctx.r[10].s64 + 27236;
	// 826C832C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C833C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C834C: 4BD9EAD5  bl 0x82466e20
	ctx.lr = 0x826C8350;
	sub_82466E20(ctx, base);
	// 826C8350: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C835C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8360 size=112
    let mut pc: u32 = 0x826C8360;
    'dispatch: loop {
        match pc {
            0x826C8360 => {
    //   block [0x826C8360..0x826C83D0)
	// 826C8360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C836C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C8370: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8374: 392A2768  addi r9, r10, 0x2768
	ctx.r[9].s64 = ctx.r[10].s64 + 10088;
	// 826C8378: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C837C: 390B73F0  addi r8, r11, 0x73f0
	ctx.r[8].s64 = ctx.r[11].s64 + 29680;
	// 826C8380: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826C8384: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 826C8388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C838C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8390: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8398: 386A6A94  addi r3, r10, 0x6a94
	ctx.r[3].s64 = ctx.r[10].s64 + 27284;
	// 826C839C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C83A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C83A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C83A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C83AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C83B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C83B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C83B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C83BC: 4BD9EA65  bl 0x82466e20
	ctx.lr = 0x826C83C0;
	sub_82466E20(ctx, base);
	// 826C83C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C83C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C83C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C83CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C83D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C83D0 size=112
    let mut pc: u32 = 0x826C83D0;
    'dispatch: loop {
        match pc {
            0x826C83D0 => {
    //   block [0x826C83D0..0x826C8440)
	// 826C83D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C83D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C83D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C83DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C83E0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C83E4: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C83E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C83EC: 390B7438  addi r8, r11, 0x7438
	ctx.r[8].s64 = ctx.r[11].s64 + 29752;
	// 826C83F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C83F4: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 826C83F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C83FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8400: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8408: 386A6AC4  addi r3, r10, 0x6ac4
	ctx.r[3].s64 = ctx.r[10].s64 + 27332;
	// 826C840C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C841C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C842C: 4BD9E9F5  bl 0x82466e20
	ctx.lr = 0x826C8430;
	sub_82466E20(ctx, base);
	// 826C8430: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C843C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8440 size=112
    let mut pc: u32 = 0x826C8440;
    'dispatch: loop {
        match pc {
            0x826C8440 => {
    //   block [0x826C8440..0x826C84B0)
	// 826C8440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C844C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C8450: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8454: 392A2794  addi r9, r10, 0x2794
	ctx.r[9].s64 = ctx.r[10].s64 + 10132;
	// 826C8458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C845C: 390B7458  addi r8, r11, 0x7458
	ctx.r[8].s64 = ctx.r[11].s64 + 29784;
	// 826C8460: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826C8464: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 826C8468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C846C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8470: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8478: 386A6AF4  addi r3, r10, 0x6af4
	ctx.r[3].s64 = ctx.r[10].s64 + 27380;
	// 826C847C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C8480: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C8484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C848C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C8498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C849C: 4BD9E985  bl 0x82466e20
	ctx.lr = 0x826C84A0;
	sub_82466E20(ctx, base);
	// 826C84A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C84A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C84A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C84AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C84B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C84B0 size=112
    let mut pc: u32 = 0x826C84B0;
    'dispatch: loop {
        match pc {
            0x826C84B0 => {
    //   block [0x826C84B0..0x826C8520)
	// 826C84B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C84B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C84B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C84BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C84C0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C84C4: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C84C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C84CC: 390B74E8  addi r8, r11, 0x74e8
	ctx.r[8].s64 = ctx.r[11].s64 + 29928;
	// 826C84D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C84D4: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 826C84D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C84DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C84E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C84E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C84E8: 386A6B24  addi r3, r10, 0x6b24
	ctx.r[3].s64 = ctx.r[10].s64 + 27428;
	// 826C84EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C84F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C84F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C84F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C84FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C850C: 4BD9E915  bl 0x82466e20
	ctx.lr = 0x826C8510;
	sub_82466E20(ctx, base);
	// 826C8510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C851C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8520 size=112
    let mut pc: u32 = 0x826C8520;
    'dispatch: loop {
        match pc {
            0x826C8520 => {
    //   block [0x826C8520..0x826C8590)
	// 826C8520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C852C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8530: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8534: 38AA6B84  addi r5, r10, 0x6b84
	ctx.r[5].s64 = ctx.r[10].s64 + 27524;
	// 826C8538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C853C: 390B7500  addi r8, r11, 0x7500
	ctx.r[8].s64 = ctx.r[11].s64 + 29952;
	// 826C8540: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826C8544: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 826C8548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C854C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8550: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8558: 386A6B54  addi r3, r10, 0x6b54
	ctx.r[3].s64 = ctx.r[10].s64 + 27476;
	// 826C855C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C856C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C857C: 4BD9E8A5  bl 0x82466e20
	ctx.lr = 0x826C8580;
	sub_82466E20(ctx, base);
	// 826C8580: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C858C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8590 size=100
    let mut pc: u32 = 0x826C8590;
    'dispatch: loop {
        match pc {
            0x826C8590 => {
    //   block [0x826C8590..0x826C85F4)
	// 826C8590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C859C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C85A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C85A4: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C85A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C85AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C85B0: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 826C85B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C85B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C85BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C85C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C85C4: 386A6B84  addi r3, r10, 0x6b84
	ctx.r[3].s64 = ctx.r[10].s64 + 27524;
	// 826C85C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C85CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C85D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C85D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C85D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C85DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C85E0: 4BD9E841  bl 0x82466e20
	ctx.lr = 0x826C85E4;
	sub_82466E20(ctx, base);
	// 826C85E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C85E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C85EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C85F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C85F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C85F8 size=24
    let mut pc: u32 = 0x826C85F8;
    'dispatch: loop {
        match pc {
            0x826C85F8 => {
    //   block [0x826C85F8..0x826C8610)
	// 826C85F8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C85FC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C8600: 394AA5F8  addi r10, r10, -0x5a08
	ctx.r[10].s64 = ctx.r[10].s64 + -23048;
	// 826C8604: 816B7454  lwz r11, 0x7454(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29780 as u32) ) } as u64;
	// 826C8608: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826C860C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8610 size=116
    let mut pc: u32 = 0x826C8610;
    'dispatch: loop {
        match pc {
            0x826C8610 => {
    //   block [0x826C8610..0x826C8684)
	// 826C8610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C861C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C8620: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C8624: 390BA5F8  addi r8, r11, -0x5a08
	ctx.r[8].s64 = ctx.r[11].s64 + -23048;
	// 826C8628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C862C: 392A27D0  addi r9, r10, 0x27d0
	ctx.r[9].s64 = ctx.r[10].s64 + 10192;
	// 826C8630: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8634: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826C8638: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C863C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8644: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C864C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8654: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C8658: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 826C865C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C8660: 386B6BB4  addi r3, r11, 0x6bb4
	ctx.r[3].s64 = ctx.r[11].s64 + 27572;
	// 826C8664: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C8668: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C866C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8670: 4BD9E7B1  bl 0x82466e20
	ctx.lr = 0x826C8674;
	sub_82466E20(ctx, base);
	// 826C8674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C867C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8688 size=108
    let mut pc: u32 = 0x826C8688;
    'dispatch: loop {
        match pc {
            0x826C8688 => {
    //   block [0x826C8688..0x826C86F4)
	// 826C8688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C868C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8694: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8698: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C869C: 38EB7578  addi r7, r11, 0x7578
	ctx.r[7].s64 = ctx.r[11].s64 + 30072;
	// 826C86A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C86A4: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 826C86A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C86AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C86B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C86B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C86B8: 386A6BE4  addi r3, r10, 0x6be4
	ctx.r[3].s64 = ctx.r[10].s64 + 27620;
	// 826C86BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C86C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C86C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C86C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C86CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C86D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C86D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C86D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C86DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C86E0: 4BD9E741  bl 0x82466e20
	ctx.lr = 0x826C86E4;
	sub_82466E20(ctx, base);
	// 826C86E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C86E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C86EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C86F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C86F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C86F8 size=112
    let mut pc: u32 = 0x826C86F8;
    'dispatch: loop {
        match pc {
            0x826C86F8 => {
    //   block [0x826C86F8..0x826C8768)
	// 826C86F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C86FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8704: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8708: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C870C: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C8710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8714: 390B75A8  addi r8, r11, 0x75a8
	ctx.r[8].s64 = ctx.r[11].s64 + 30120;
	// 826C8718: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C871C: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 826C8720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8724: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C872C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8730: 386A6C14  addi r3, r10, 0x6c14
	ctx.r[3].s64 = ctx.r[10].s64 + 27668;
	// 826C8734: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C873C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C874C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8754: 4BD9E6CD  bl 0x82466e20
	ctx.lr = 0x826C8758;
	sub_82466E20(ctx, base);
	// 826C8758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C875C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8768 size=112
    let mut pc: u32 = 0x826C8768;
    'dispatch: loop {
        match pc {
            0x826C8768 => {
    //   block [0x826C8768..0x826C87D8)
	// 826C8768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C876C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8774: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C8778: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C877C: 392A27F4  addi r9, r10, 0x27f4
	ctx.r[9].s64 = ctx.r[10].s64 + 10228;
	// 826C8780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8784: 390B75C8  addi r8, r11, 0x75c8
	ctx.r[8].s64 = ctx.r[11].s64 + 30152;
	// 826C8788: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826C878C: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 826C8790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8794: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8798: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C879C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C87A0: 386A6C44  addi r3, r10, 0x6c44
	ctx.r[3].s64 = ctx.r[10].s64 + 27716;
	// 826C87A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C87A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C87AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C87B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C87B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C87B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C87BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C87C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C87C4: 4BD9E65D  bl 0x82466e20
	ctx.lr = 0x826C87C8;
	sub_82466E20(ctx, base);
	// 826C87C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C87CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C87D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C87D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C87D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C87D8 size=112
    let mut pc: u32 = 0x826C87D8;
    'dispatch: loop {
        match pc {
            0x826C87D8 => {
    //   block [0x826C87D8..0x826C8848)
	// 826C87D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C87DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C87E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C87E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C87E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C87EC: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C87F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C87F4: 390B7670  addi r8, r11, 0x7670
	ctx.r[8].s64 = ctx.r[11].s64 + 30320;
	// 826C87F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C87FC: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 826C8800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8804: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C880C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8810: 386A6C74  addi r3, r10, 0x6c74
	ctx.r[3].s64 = ctx.r[10].s64 + 27764;
	// 826C8814: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C881C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C882C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8834: 4BD9E5ED  bl 0x82466e20
	ctx.lr = 0x826C8838;
	sub_82466E20(ctx, base);
	// 826C8838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C883C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8848 size=108
    let mut pc: u32 = 0x826C8848;
    'dispatch: loop {
        match pc {
            0x826C8848 => {
    //   block [0x826C8848..0x826C88B4)
	// 826C8848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C884C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8854: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C885C: 38EB7688  addi r7, r11, 0x7688
	ctx.r[7].s64 = ctx.r[11].s64 + 30344;
	// 826C8860: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C8864: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 826C8868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C886C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8870: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C8874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8878: 386A6CA4  addi r3, r10, 0x6ca4
	ctx.r[3].s64 = ctx.r[10].s64 + 27812;
	// 826C887C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C8880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C888C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C889C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C88A0: 4BD9E581  bl 0x82466e20
	ctx.lr = 0x826C88A4;
	sub_82466E20(ctx, base);
	// 826C88A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C88A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C88AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C88B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C88B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C88B8 size=112
    let mut pc: u32 = 0x826C88B8;
    'dispatch: loop {
        match pc {
            0x826C88B8 => {
    //   block [0x826C88B8..0x826C8928)
	// 826C88B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C88BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C88C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C88C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C88C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C88CC: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C88D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C88D4: 390B76B8  addi r8, r11, 0x76b8
	ctx.r[8].s64 = ctx.r[11].s64 + 30392;
	// 826C88D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C88DC: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 826C88E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C88E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C88E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C88EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C88F0: 386A6CD4  addi r3, r10, 0x6cd4
	ctx.r[3].s64 = ctx.r[10].s64 + 27860;
	// 826C88F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C88F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C88FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C890C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8914: 4BD9E50D  bl 0x82466e20
	ctx.lr = 0x826C8918;
	sub_82466E20(ctx, base);
	// 826C8918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C891C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8928 size=112
    let mut pc: u32 = 0x826C8928;
    'dispatch: loop {
        match pc {
            0x826C8928 => {
    //   block [0x826C8928..0x826C8998)
	// 826C8928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C892C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8934: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C8938: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C893C: 392A2828  addi r9, r10, 0x2828
	ctx.r[9].s64 = ctx.r[10].s64 + 10280;
	// 826C8940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8944: 390B76D0  addi r8, r11, 0x76d0
	ctx.r[8].s64 = ctx.r[11].s64 + 30416;
	// 826C8948: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826C894C: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 826C8950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8954: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C895C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8960: 386A6D04  addi r3, r10, 0x6d04
	ctx.r[3].s64 = ctx.r[10].s64 + 27908;
	// 826C8964: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C8968: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C896C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C897C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C8980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8984: 4BD9E49D  bl 0x82466e20
	ctx.lr = 0x826C8988;
	sub_82466E20(ctx, base);
	// 826C8988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C898C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8998 size=112
    let mut pc: u32 = 0x826C8998;
    'dispatch: loop {
        match pc {
            0x826C8998 => {
    //   block [0x826C8998..0x826C8A08)
	// 826C8998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C899C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C89A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C89A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C89A8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C89AC: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C89B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C89B4: 390B7778  addi r8, r11, 0x7778
	ctx.r[8].s64 = ctx.r[11].s64 + 30584;
	// 826C89B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C89BC: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 826C89C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C89C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C89C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C89CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C89D0: 386A6D34  addi r3, r10, 0x6d34
	ctx.r[3].s64 = ctx.r[10].s64 + 27956;
	// 826C89D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C89D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C89DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C89E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C89E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C89E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C89EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C89F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C89F4: 4BD9E42D  bl 0x82466e20
	ctx.lr = 0x826C89F8;
	sub_82466E20(ctx, base);
	// 826C89F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C89FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8A08 size=112
    let mut pc: u32 = 0x826C8A08;
    'dispatch: loop {
        match pc {
            0x826C8A08 => {
    //   block [0x826C8A08..0x826C8A78)
	// 826C8A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8A14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8A18: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8A1C: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C8A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8A24: 390B77C0  addi r8, r11, 0x77c0
	ctx.r[8].s64 = ctx.r[11].s64 + 30656;
	// 826C8A28: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826C8A2C: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 826C8A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8A34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8A38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8A40: 386A6D64  addi r3, r10, 0x6d64
	ctx.r[3].s64 = ctx.r[10].s64 + 28004;
	// 826C8A44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8A64: 4BD9E3BD  bl 0x82466e20
	ctx.lr = 0x826C8A68;
	sub_82466E20(ctx, base);
	// 826C8A68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8A78 size=100
    let mut pc: u32 = 0x826C8A78;
    'dispatch: loop {
        match pc {
            0x826C8A78 => {
    //   block [0x826C8A78..0x826C8ADC)
	// 826C8A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8A84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8A8C: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C8A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8A98: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 826C8A9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8AAC: 386A6D94  addi r3, r10, 0x6d94
	ctx.r[3].s64 = ctx.r[10].s64 + 28052;
	// 826C8AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8AB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8AB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C8ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8AC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C8AC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8AC8: 4BD9E359  bl 0x82466e20
	ctx.lr = 0x826C8ACC;
	sub_82466E20(ctx, base);
	// 826C8ACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8AE0 size=112
    let mut pc: u32 = 0x826C8AE0;
    'dispatch: loop {
        match pc {
            0x826C8AE0 => {
    //   block [0x826C8AE0..0x826C8B50)
	// 826C8AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8AEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8AF0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8AF4: 38AA6A04  addi r5, r10, 0x6a04
	ctx.r[5].s64 = ctx.r[10].s64 + 27140;
	// 826C8AF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8AFC: 390B7898  addi r8, r11, 0x7898
	ctx.r[8].s64 = ctx.r[11].s64 + 30872;
	// 826C8B00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C8B04: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 826C8B08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8B0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8B10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8B14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8B18: 386A6DC4  addi r3, r10, 0x6dc4
	ctx.r[3].s64 = ctx.r[10].s64 + 28100;
	// 826C8B1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8B24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8B28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8B2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8B30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8B38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8B3C: 4BD9E2E5  bl 0x82466e20
	ctx.lr = 0x826C8B40;
	sub_82466E20(ctx, base);
	// 826C8B40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8B50 size=112
    let mut pc: u32 = 0x826C8B50;
    'dispatch: loop {
        match pc {
            0x826C8B50 => {
    //   block [0x826C8B50..0x826C8BC0)
	// 826C8B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8B5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8B60: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8B64: 38AA6854  addi r5, r10, 0x6854
	ctx.r[5].s64 = ctx.r[10].s64 + 26708;
	// 826C8B68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8B6C: 390B78C8  addi r8, r11, 0x78c8
	ctx.r[8].s64 = ctx.r[11].s64 + 30920;
	// 826C8B70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C8B74: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 826C8B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8B7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8B80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8B84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8B88: 386A6DF4  addi r3, r10, 0x6df4
	ctx.r[3].s64 = ctx.r[10].s64 + 28148;
	// 826C8B8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8B94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8BA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8BAC: 4BD9E275  bl 0x82466e20
	ctx.lr = 0x826C8BB0;
	sub_82466E20(ctx, base);
	// 826C8BB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8BC0 size=108
    let mut pc: u32 = 0x826C8BC0;
    'dispatch: loop {
        match pc {
            0x826C8BC0 => {
    //   block [0x826C8BC0..0x826C8C2C)
	// 826C8BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8BCC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8BD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8BD4: 38EB78E0  addi r7, r11, 0x78e0
	ctx.r[7].s64 = ctx.r[11].s64 + 30944;
	// 826C8BD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C8BDC: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 826C8BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8BE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8BE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C8BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8BF0: 386A6E24  addi r3, r10, 0x6e24
	ctx.r[3].s64 = ctx.r[10].s64 + 28196;
	// 826C8BF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C8BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8BFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8C04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8C14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C8C18: 4BD9E209  bl 0x82466e20
	ctx.lr = 0x826C8C1C;
	sub_82466E20(ctx, base);
	// 826C8C1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8C20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8C24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8C30 size=112
    let mut pc: u32 = 0x826C8C30;
    'dispatch: loop {
        match pc {
            0x826C8C30 => {
    //   block [0x826C8C30..0x826C8CA0)
	// 826C8C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8C38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8C3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8C40: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8C44: 38AA6D94  addi r5, r10, 0x6d94
	ctx.r[5].s64 = ctx.r[10].s64 + 28052;
	// 826C8C48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8C4C: 390B7910  addi r8, r11, 0x7910
	ctx.r[8].s64 = ctx.r[11].s64 + 30992;
	// 826C8C50: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826C8C54: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 826C8C58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8C5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8C60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8C64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8C68: 386A6E54  addi r3, r10, 0x6e54
	ctx.r[3].s64 = ctx.r[10].s64 + 28244;
	// 826C8C6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8C70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8C78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8C80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8C84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8C88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8C8C: 4BD9E195  bl 0x82466e20
	ctx.lr = 0x826C8C90;
	sub_82466E20(ctx, base);
	// 826C8C90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8CA0 size=112
    let mut pc: u32 = 0x826C8CA0;
    'dispatch: loop {
        match pc {
            0x826C8CA0 => {
    //   block [0x826C8CA0..0x826C8D10)
	// 826C8CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8CA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8CAC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C8CB0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8CB4: 392A2854  addi r9, r10, 0x2854
	ctx.r[9].s64 = ctx.r[10].s64 + 10324;
	// 826C8CB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8CBC: 390B79A8  addi r8, r11, 0x79a8
	ctx.r[8].s64 = ctx.r[11].s64 + 31144;
	// 826C8CC0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826C8CC4: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 826C8CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8CCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8CD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8CD8: 386A6E84  addi r3, r10, 0x6e84
	ctx.r[3].s64 = ctx.r[10].s64 + 28292;
	// 826C8CDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C8CE0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C8CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8CF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C8CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8CFC: 4BD9E125  bl 0x82466e20
	ctx.lr = 0x826C8D00;
	sub_82466E20(ctx, base);
	// 826C8D00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8D10 size=112
    let mut pc: u32 = 0x826C8D10;
    'dispatch: loop {
        match pc {
            0x826C8D10 => {
    //   block [0x826C8D10..0x826C8D80)
	// 826C8D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8D1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8D20: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8D24: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C8D28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8D2C: 390B79F0  addi r8, r11, 0x79f0
	ctx.r[8].s64 = ctx.r[11].s64 + 31216;
	// 826C8D30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C8D34: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 826C8D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8D3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8D40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8D44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8D48: 386A6EB4  addi r3, r10, 0x6eb4
	ctx.r[3].s64 = ctx.r[10].s64 + 28340;
	// 826C8D4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8D54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8D6C: 4BD9E0B5  bl 0x82466e20
	ctx.lr = 0x826C8D70;
	sub_82466E20(ctx, base);
	// 826C8D70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8D80 size=108
    let mut pc: u32 = 0x826C8D80;
    'dispatch: loop {
        match pc {
            0x826C8D80 => {
    //   block [0x826C8D80..0x826C8DEC)
	// 826C8D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8D8C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8D90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8D94: 38EB7A08  addi r7, r11, 0x7a08
	ctx.r[7].s64 = ctx.r[11].s64 + 31240;
	// 826C8D98: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826C8D9C: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 826C8DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8DA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8DA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C8DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8DB0: 386A6EE4  addi r3, r10, 0x6ee4
	ctx.r[3].s64 = ctx.r[10].s64 + 28388;
	// 826C8DB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C8DB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8DBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8DC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8DC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8DC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8DCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8DD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8DD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C8DD8: 4BD9E049  bl 0x82466e20
	ctx.lr = 0x826C8DDC;
	sub_82466E20(ctx, base);
	// 826C8DDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8DE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8DE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8DF0 size=116
    let mut pc: u32 = 0x826C8DF0;
    'dispatch: loop {
        match pc {
            0x826C8DF0 => {
    //   block [0x826C8DF0..0x826C8E64)
	// 826C8DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8DFC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C8E00: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826C8E04: 390A7A98  addi r8, r10, 0x7a98
	ctx.r[8].s64 = ctx.r[10].s64 + 31384;
	// 826C8E08: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8E0C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C8E10: 38AA6D94  addi r5, r10, 0x6d94
	ctx.r[5].s64 = ctx.r[10].s64 + 28052;
	// 826C8E14: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8E18: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C8E1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8E20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8E24: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 826C8E28: 396B2868  addi r11, r11, 0x2868
	ctx.r[11].s64 = ctx.r[11].s64 + 10344;
	// 826C8E2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8E30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8E34: 386A6F14  addi r3, r10, 0x6f14
	ctx.r[3].s64 = ctx.r[10].s64 + 28436;
	// 826C8E38: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C8E3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8E40: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C8E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8E50: 4BD9DFD1  bl 0x82466e20
	ctx.lr = 0x826C8E54;
	sub_82466E20(ctx, base);
	// 826C8E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8E68 size=108
    let mut pc: u32 = 0x826C8E68;
    'dispatch: loop {
        match pc {
            0x826C8E68 => {
    //   block [0x826C8E68..0x826C8ED4)
	// 826C8E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8E74: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8E78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8E7C: 38EB7B70  addi r7, r11, 0x7b70
	ctx.r[7].s64 = ctx.r[11].s64 + 31600;
	// 826C8E80: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C8E84: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 826C8E88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8E8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8E90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C8E94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8E98: 386A6F44  addi r3, r10, 0x6f44
	ctx.r[3].s64 = ctx.r[10].s64 + 28484;
	// 826C8E9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C8EA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8EA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8EA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8EB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8EB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8EB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8EBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C8EC0: 4BD9DF61  bl 0x82466e20
	ctx.lr = 0x826C8EC4;
	sub_82466E20(ctx, base);
	// 826C8EC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8ED8 size=112
    let mut pc: u32 = 0x826C8ED8;
    'dispatch: loop {
        match pc {
            0x826C8ED8 => {
    //   block [0x826C8ED8..0x826C8F48)
	// 826C8ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8EE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8EE8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8EEC: 38AA6D94  addi r5, r10, 0x6d94
	ctx.r[5].s64 = ctx.r[10].s64 + 28052;
	// 826C8EF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8EF4: 390B7BB8  addi r8, r11, 0x7bb8
	ctx.r[8].s64 = ctx.r[11].s64 + 31672;
	// 826C8EF8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826C8EFC: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 826C8F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8F04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8F08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8F10: 386A6F74  addi r3, r10, 0x6f74
	ctx.r[3].s64 = ctx.r[10].s64 + 28532;
	// 826C8F14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8F34: 4BD9DEED  bl 0x82466e20
	ctx.lr = 0x826C8F38;
	sub_82466E20(ctx, base);
	// 826C8F38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8F48 size=112
    let mut pc: u32 = 0x826C8F48;
    'dispatch: loop {
        match pc {
            0x826C8F48 => {
    //   block [0x826C8F48..0x826C8FB8)
	// 826C8F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8F54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8F58: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8F5C: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C8F60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8F64: 390B7C30  addi r8, r11, 0x7c30
	ctx.r[8].s64 = ctx.r[11].s64 + 31792;
	// 826C8F68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C8F6C: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 826C8F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8F74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8F78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8F80: 386A6FA4  addi r3, r10, 0x6fa4
	ctx.r[3].s64 = ctx.r[10].s64 + 28580;
	// 826C8F84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8F88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8F90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8F94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8F98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8F9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8FA4: 4BD9DE7D  bl 0x82466e20
	ctx.lr = 0x826C8FA8;
	sub_82466E20(ctx, base);
	// 826C8FA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8FB8 size=108
    let mut pc: u32 = 0x826C8FB8;
    'dispatch: loop {
        match pc {
            0x826C8FB8 => {
    //   block [0x826C8FB8..0x826C9024)
	// 826C8FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8FC4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8FC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8FCC: 38EB7C60  addi r7, r11, 0x7c60
	ctx.r[7].s64 = ctx.r[11].s64 + 31840;
	// 826C8FD0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826C8FD4: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 826C8FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8FDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8FE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C8FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8FE8: 386A6FD4  addi r3, r10, 0x6fd4
	ctx.r[3].s64 = ctx.r[10].s64 + 28628;
	// 826C8FEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C8FF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8FF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C900C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C9010: 4BD9DE11  bl 0x82466e20
	ctx.lr = 0x826C9014;
	sub_82466E20(ctx, base);
	// 826C9014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C901C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9028 size=112
    let mut pc: u32 = 0x826C9028;
    'dispatch: loop {
        match pc {
            0x826C9028 => {
    //   block [0x826C9028..0x826C9098)
	// 826C9028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C902C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9034: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9038: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C903C: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C9040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9044: 390B7CD8  addi r8, r11, 0x7cd8
	ctx.r[8].s64 = ctx.r[11].s64 + 31960;
	// 826C9048: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C904C: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 826C9050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9054: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C905C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9060: 386A7004  addi r3, r10, 0x7004
	ctx.r[3].s64 = ctx.r[10].s64 + 28676;
	// 826C9064: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C906C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C907C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9084: 4BD9DD9D  bl 0x82466e20
	ctx.lr = 0x826C9088;
	sub_82466E20(ctx, base);
	// 826C9088: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C908C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C9098 size=24
    let mut pc: u32 = 0x826C9098;
    'dispatch: loop {
        match pc {
            0x826C9098 => {
    //   block [0x826C9098..0x826C90B0)
	// 826C9098: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C909C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C90A0: 394AA670  addi r10, r10, -0x5990
	ctx.r[10].s64 = ctx.r[10].s64 + -22928;
	// 826C90A4: 816B79A4  lwz r11, 0x79a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31140 as u32) ) } as u64;
	// 826C90A8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826C90AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C90B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C90B0 size=116
    let mut pc: u32 = 0x826C90B0;
    'dispatch: loop {
        match pc {
            0x826C90B0 => {
    //   block [0x826C90B0..0x826C9124)
	// 826C90B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C90B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C90B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C90BC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C90C0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C90C4: 390BA670  addi r8, r11, -0x5990
	ctx.r[8].s64 = ctx.r[11].s64 + -22928;
	// 826C90C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C90CC: 392A28C4  addi r9, r10, 0x28c4
	ctx.r[9].s64 = ctx.r[10].s64 + 10436;
	// 826C90D0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C90D4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826C90D8: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C90DC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C90E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C90E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C90E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C90EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C90F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C90F4: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C90F8: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 826C90FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C9100: 386B7034  addi r3, r11, 0x7034
	ctx.r[3].s64 = ctx.r[11].s64 + 28724;
	// 826C9104: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C9108: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C910C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9110: 4BD9DD11  bl 0x82466e20
	ctx.lr = 0x826C9114;
	sub_82466E20(ctx, base);
	// 826C9114: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C911C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9128 size=112
    let mut pc: u32 = 0x826C9128;
    'dispatch: loop {
        match pc {
            0x826C9128 => {
    //   block [0x826C9128..0x826C9198)
	// 826C9128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C912C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9134: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9138: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C913C: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 826C9140: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9144: 390B7D20  addi r8, r11, 0x7d20
	ctx.r[8].s64 = ctx.r[11].s64 + 32032;
	// 826C9148: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C914C: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 826C9150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9154: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9158: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C915C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9160: 386A7064  addi r3, r10, 0x7064
	ctx.r[3].s64 = ctx.r[10].s64 + 28772;
	// 826C9164: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C916C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C917C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9184: 4BD9DC9D  bl 0x82466e20
	ctx.lr = 0x826C9188;
	sub_82466E20(ctx, base);
	// 826C9188: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C918C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9198 size=112
    let mut pc: u32 = 0x826C9198;
    'dispatch: loop {
        match pc {
            0x826C9198 => {
    //   block [0x826C9198..0x826C9208)
	// 826C9198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C919C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C91A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C91A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C91A8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C91AC: 38AA7064  addi r5, r10, 0x7064
	ctx.r[5].s64 = ctx.r[10].s64 + 28772;
	// 826C91B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C91B4: 390B7D50  addi r8, r11, 0x7d50
	ctx.r[8].s64 = ctx.r[11].s64 + 32080;
	// 826C91B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C91BC: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 826C91C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C91C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C91C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C91CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C91D0: 386A7094  addi r3, r10, 0x7094
	ctx.r[3].s64 = ctx.r[10].s64 + 28820;
	// 826C91D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C91D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C91DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C91E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C91E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C91E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C91EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C91F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C91F4: 4BD9DC2D  bl 0x82466e20
	ctx.lr = 0x826C91F8;
	sub_82466E20(ctx, base);
	// 826C91F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C91FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9208 size=112
    let mut pc: u32 = 0x826C9208;
    'dispatch: loop {
        match pc {
            0x826C9208 => {
    //   block [0x826C9208..0x826C9278)
	// 826C9208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C920C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9214: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9218: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C921C: 38AA7064  addi r5, r10, 0x7064
	ctx.r[5].s64 = ctx.r[10].s64 + 28772;
	// 826C9220: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9224: 390B7DB0  addi r8, r11, 0x7db0
	ctx.r[8].s64 = ctx.r[11].s64 + 32176;
	// 826C9228: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C922C: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 826C9230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9234: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9238: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C923C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9240: 386A70C4  addi r3, r10, 0x70c4
	ctx.r[3].s64 = ctx.r[10].s64 + 28868;
	// 826C9244: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C924C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C925C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9264: 4BD9DBBD  bl 0x82466e20
	ctx.lr = 0x826C9268;
	sub_82466E20(ctx, base);
	// 826C9268: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C926C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9278 size=112
    let mut pc: u32 = 0x826C9278;
    'dispatch: loop {
        match pc {
            0x826C9278 => {
    //   block [0x826C9278..0x826C92E8)
	// 826C9278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C927C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9284: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9288: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C928C: 38AA7064  addi r5, r10, 0x7064
	ctx.r[5].s64 = ctx.r[10].s64 + 28772;
	// 826C9290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9294: 390B7DE0  addi r8, r11, 0x7de0
	ctx.r[8].s64 = ctx.r[11].s64 + 32224;
	// 826C9298: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C929C: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 826C92A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C92A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C92A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C92AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C92B0: 386A70F4  addi r3, r10, 0x70f4
	ctx.r[3].s64 = ctx.r[10].s64 + 28916;
	// 826C92B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C92B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C92BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C92C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C92C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C92C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C92CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C92D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C92D4: 4BD9DB4D  bl 0x82466e20
	ctx.lr = 0x826C92D8;
	sub_82466E20(ctx, base);
	// 826C92D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C92DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C92E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C92E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C92E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C92E8 size=108
    let mut pc: u32 = 0x826C92E8;
    'dispatch: loop {
        match pc {
            0x826C92E8 => {
    //   block [0x826C92E8..0x826C9354)
	// 826C92E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C92EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C92F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C92F4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C92F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C92FC: 38EB7E28  addi r7, r11, 0x7e28
	ctx.r[7].s64 = ctx.r[11].s64 + 32296;
	// 826C9300: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C9304: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 826C9308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C930C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9310: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C9314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9318: 386A7124  addi r3, r10, 0x7124
	ctx.r[3].s64 = ctx.r[10].s64 + 28964;
	// 826C931C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C9320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C932C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C933C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C9340: 4BD9DAE1  bl 0x82466e20
	ctx.lr = 0x826C9344;
	sub_82466E20(ctx, base);
	// 826C9344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C934C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9358 size=112
    let mut pc: u32 = 0x826C9358;
    'dispatch: loop {
        match pc {
            0x826C9358 => {
    //   block [0x826C9358..0x826C93C8)
	// 826C9358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C935C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9364: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9368: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C936C: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C9370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9374: 390B7E58  addi r8, r11, 0x7e58
	ctx.r[8].s64 = ctx.r[11].s64 + 32344;
	// 826C9378: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C937C: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 826C9380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9384: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C938C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9390: 386A7154  addi r3, r10, 0x7154
	ctx.r[3].s64 = ctx.r[10].s64 + 29012;
	// 826C9394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C939C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C93A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C93A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C93A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C93AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C93B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C93B4: 4BD9DA6D  bl 0x82466e20
	ctx.lr = 0x826C93B8;
	sub_82466E20(ctx, base);
	// 826C93B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C93BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C93C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C93C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C93C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C93C8 size=116
    let mut pc: u32 = 0x826C93C8;
    'dispatch: loop {
        match pc {
            0x826C93C8 => {
    //   block [0x826C93C8..0x826C943C)
	// 826C93C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C93CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C93D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C93D4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C93D8: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 826C93DC: 390A7E70  addi r8, r10, 0x7e70
	ctx.r[8].s64 = ctx.r[10].s64 + 32368;
	// 826C93E0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C93E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C93E8: 38AA75D4  addi r5, r10, 0x75d4
	ctx.r[5].s64 = ctx.r[10].s64 + 30164;
	// 826C93EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C93F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C93F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C93F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C93FC: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 826C9400: 396B28D8  addi r11, r11, 0x28d8
	ctx.r[11].s64 = ctx.r[11].s64 + 10456;
	// 826C9404: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9408: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C940C: 386A7184  addi r3, r10, 0x7184
	ctx.r[3].s64 = ctx.r[10].s64 + 29060;
	// 826C9410: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C9414: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9418: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C941C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9428: 4BD9D9F9  bl 0x82466e20
	ctx.lr = 0x826C942C;
	sub_82466E20(ctx, base);
	// 826C942C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9440 size=100
    let mut pc: u32 = 0x826C9440;
    'dispatch: loop {
        match pc {
            0x826C9440 => {
    //   block [0x826C9440..0x826C94A4)
	// 826C9440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C944C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9454: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C9458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C945C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9460: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 826C9464: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C946C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9474: 386A71B4  addi r3, r10, 0x71b4
	ctx.r[3].s64 = ctx.r[10].s64 + 29108;
	// 826C9478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C947C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9480: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C9484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9488: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C948C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9490: 4BD9D991  bl 0x82466e20
	ctx.lr = 0x826C9494;
	sub_82466E20(ctx, base);
	// 826C9494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C949C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C94A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C94A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C94A8 size=100
    let mut pc: u32 = 0x826C94A8;
    'dispatch: loop {
        match pc {
            0x826C94A8 => {
    //   block [0x826C94A8..0x826C950C)
	// 826C94A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C94AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C94B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C94B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C94B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C94BC: 38AA7244  addi r5, r10, 0x7244
	ctx.r[5].s64 = ctx.r[10].s64 + 29252;
	// 826C94C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C94C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C94C8: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 826C94CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C94D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C94D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C94D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C94DC: 386A71E4  addi r3, r10, 0x71e4
	ctx.r[3].s64 = ctx.r[10].s64 + 29156;
	// 826C94E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C94E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C94E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C94EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C94F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C94F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C94F8: 4BD9D929  bl 0x82466e20
	ctx.lr = 0x826C94FC;
	sub_82466E20(ctx, base);
	// 826C94FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9510 size=100
    let mut pc: u32 = 0x826C9510;
    'dispatch: loop {
        match pc {
            0x826C9510 => {
    //   block [0x826C9510..0x826C9574)
	// 826C9510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C951C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9524: 38AA7184  addi r5, r10, 0x7184
	ctx.r[5].s64 = ctx.r[10].s64 + 29060;
	// 826C9528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C952C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9530: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 826C9534: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C953C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9544: 386A7214  addi r3, r10, 0x7214
	ctx.r[3].s64 = ctx.r[10].s64 + 29204;
	// 826C9548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C954C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9550: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C9554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9558: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C955C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9560: 4BD9D8C1  bl 0x82466e20
	ctx.lr = 0x826C9564;
	sub_82466E20(ctx, base);
	// 826C9564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C956C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9578 size=104
    let mut pc: u32 = 0x826C9578;
    'dispatch: loop {
        match pc {
            0x826C9578 => {
    //   block [0x826C9578..0x826C95E0)
	// 826C9578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C957C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9584: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C9588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C958C: 392A293C  addi r9, r10, 0x293c
	ctx.r[9].s64 = ctx.r[10].s64 + 10556;
	// 826C9590: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9594: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9598: 38AA71B4  addi r5, r10, 0x71b4
	ctx.r[5].s64 = ctx.r[10].s64 + 29108;
	// 826C959C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C95A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C95A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C95A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C95AC: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 826C95B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C95B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C95B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C95BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C95C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C95C4: 386A7244  addi r3, r10, 0x7244
	ctx.r[3].s64 = ctx.r[10].s64 + 29252;
	// 826C95C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C95CC: 4BD9D855  bl 0x82466e20
	ctx.lr = 0x826C95D0;
	sub_82466E20(ctx, base);
	// 826C95D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C95D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C95D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C95DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C95E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C95E0 size=108
    let mut pc: u32 = 0x826C95E0;
    'dispatch: loop {
        match pc {
            0x826C95E0 => {
    //   block [0x826C95E0..0x826C964C)
	// 826C95E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C95E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C95E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C95EC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C95F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C95F4: 38EB7FF4  addi r7, r11, 0x7ff4
	ctx.r[7].s64 = ctx.r[11].s64 + 32756;
	// 826C95F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C95FC: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 826C9600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9604: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C960C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9610: 386A7274  addi r3, r10, 0x7274
	ctx.r[3].s64 = ctx.r[10].s64 + 29300;
	// 826C9614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C9618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C961C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C962C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C9638: 4BD9D7E9  bl 0x82466e20
	ctx.lr = 0x826C963C;
	sub_82466E20(ctx, base);
	// 826C963C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9650 size=112
    let mut pc: u32 = 0x826C9650;
    'dispatch: loop {
        match pc {
            0x826C9650 => {
    //   block [0x826C9650..0x826C96C0)
	// 826C9650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C965C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9660: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9664: 38AA7244  addi r5, r10, 0x7244
	ctx.r[5].s64 = ctx.r[10].s64 + 29252;
	// 826C9668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C966C: 390B8028  addi r8, r11, -0x7fd8
	ctx.r[8].s64 = ctx.r[11].s64 + -32728;
	// 826C9670: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826C9674: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 826C9678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C967C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9688: 386A72A4  addi r3, r10, 0x72a4
	ctx.r[3].s64 = ctx.r[10].s64 + 29348;
	// 826C968C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C969C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C96A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C96A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C96A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C96AC: 4BD9D775  bl 0x82466e20
	ctx.lr = 0x826C96B0;
	sub_82466E20(ctx, base);
	// 826C96B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C96B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C96B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C96BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C96C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C96C0 size=24
    let mut pc: u32 = 0x826C96C0;
    'dispatch: loop {
        match pc {
            0x826C96C0 => {
    //   block [0x826C96C0..0x826C96D8)
	// 826C96C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C96C4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C96C8: 394AA688  addi r10, r10, -0x5978
	ctx.r[10].s64 = ctx.r[10].s64 + -22904;
	// 826C96CC: 816B8024  lwz r11, -0x7fdc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32732 as u32) ) } as u64;
	// 826C96D0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826C96D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C96D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C96D8 size=116
    let mut pc: u32 = 0x826C96D8;
    'dispatch: loop {
        match pc {
            0x826C96D8 => {
    //   block [0x826C96D8..0x826C974C)
	// 826C96D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C96DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C96E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C96E4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C96E8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C96EC: 390BA688  addi r8, r11, -0x5978
	ctx.r[8].s64 = ctx.r[11].s64 + -22904;
	// 826C96F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C96F4: 392A29A0  addi r9, r10, 0x29a0
	ctx.r[9].s64 = ctx.r[10].s64 + 10656;
	// 826C96F8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C96FC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826C9700: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C9704: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C970C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C971C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C9720: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 826C9724: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C9728: 386B72D4  addi r3, r11, 0x72d4
	ctx.r[3].s64 = ctx.r[11].s64 + 29396;
	// 826C972C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C9730: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9738: 4BD9D6E9  bl 0x82466e20
	ctx.lr = 0x826C973C;
	sub_82466E20(ctx, base);
	// 826C973C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9750 size=100
    let mut pc: u32 = 0x826C9750;
    'dispatch: loop {
        match pc {
            0x826C9750 => {
    //   block [0x826C9750..0x826C97B4)
	// 826C9750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C975C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9764: 38AA72D4  addi r5, r10, 0x72d4
	ctx.r[5].s64 = ctx.r[10].s64 + 29396;
	// 826C9768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C976C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9770: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 826C9774: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C977C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9784: 386A7304  addi r3, r10, 0x7304
	ctx.r[3].s64 = ctx.r[10].s64 + 29444;
	// 826C9788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C978C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9790: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C9794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9798: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C979C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C97A0: 4BD9D681  bl 0x82466e20
	ctx.lr = 0x826C97A4;
	sub_82466E20(ctx, base);
	// 826C97A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C97A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C97AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C97B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C97B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C97B8 size=100
    let mut pc: u32 = 0x826C97B8;
    'dispatch: loop {
        match pc {
            0x826C97B8 => {
    //   block [0x826C97B8..0x826C981C)
	// 826C97B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C97BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C97C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C97C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C97C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C97CC: 38AA7364  addi r5, r10, 0x7364
	ctx.r[5].s64 = ctx.r[10].s64 + 29540;
	// 826C97D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C97D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C97D8: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 826C97DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C97E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C97E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C97E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C97EC: 386A7334  addi r3, r10, 0x7334
	ctx.r[3].s64 = ctx.r[10].s64 + 29492;
	// 826C97F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C97F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C97F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C97FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9800: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C9804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9808: 4BD9D619  bl 0x82466e20
	ctx.lr = 0x826C980C;
	sub_82466E20(ctx, base);
	// 826C980C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9820 size=112
    let mut pc: u32 = 0x826C9820;
    'dispatch: loop {
        match pc {
            0x826C9820 => {
    //   block [0x826C9820..0x826C9890)
	// 826C9820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C982C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9830: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9834: 38AA72D4  addi r5, r10, 0x72d4
	ctx.r[5].s64 = ctx.r[10].s64 + 29396;
	// 826C9838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C983C: 390B80D0  addi r8, r11, -0x7f30
	ctx.r[8].s64 = ctx.r[11].s64 + -32560;
	// 826C9840: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C9844: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 826C9848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C984C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9850: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9858: 386A7364  addi r3, r10, 0x7364
	ctx.r[3].s64 = ctx.r[10].s64 + 29540;
	// 826C985C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C986C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C987C: 4BD9D5A5  bl 0x82466e20
	ctx.lr = 0x826C9880;
	sub_82466E20(ctx, base);
	// 826C9880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C988C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9890 size=100
    let mut pc: u32 = 0x826C9890;
    'dispatch: loop {
        match pc {
            0x826C9890 => {
    //   block [0x826C9890..0x826C98F4)
	// 826C9890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C989C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C98A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C98A4: 38AA7364  addi r5, r10, 0x7364
	ctx.r[5].s64 = ctx.r[10].s64 + 29540;
	// 826C98A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C98AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C98B0: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 826C98B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C98B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C98BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C98C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C98C4: 386A7394  addi r3, r10, 0x7394
	ctx.r[3].s64 = ctx.r[10].s64 + 29588;
	// 826C98C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C98CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C98D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C98D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C98D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C98DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C98E0: 4BD9D541  bl 0x82466e20
	ctx.lr = 0x826C98E4;
	sub_82466E20(ctx, base);
	// 826C98E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C98E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C98EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C98F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C98F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C98F8 size=100
    let mut pc: u32 = 0x826C98F8;
    'dispatch: loop {
        match pc {
            0x826C98F8 => {
    //   block [0x826C98F8..0x826C995C)
	// 826C98F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C98FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9904: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C990C: 38AA72D4  addi r5, r10, 0x72d4
	ctx.r[5].s64 = ctx.r[10].s64 + 29396;
	// 826C9910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9918: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 826C991C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C992C: 386A73C4  addi r3, r10, 0x73c4
	ctx.r[3].s64 = ctx.r[10].s64 + 29636;
	// 826C9930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9934: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9938: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C993C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9940: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C9944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9948: 4BD9D4D9  bl 0x82466e20
	ctx.lr = 0x826C994C;
	sub_82466E20(ctx, base);
	// 826C994C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9960 size=100
    let mut pc: u32 = 0x826C9960;
    'dispatch: loop {
        match pc {
            0x826C9960 => {
    //   block [0x826C9960..0x826C99C4)
	// 826C9960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C996C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9974: 38AA7304  addi r5, r10, 0x7304
	ctx.r[5].s64 = ctx.r[10].s64 + 29444;
	// 826C9978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C997C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9980: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 826C9984: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C998C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9994: 386A73F4  addi r3, r10, 0x73f4
	ctx.r[3].s64 = ctx.r[10].s64 + 29684;
	// 826C9998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C999C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C99A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C99A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C99A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C99AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C99B0: 4BD9D471  bl 0x82466e20
	ctx.lr = 0x826C99B4;
	sub_82466E20(ctx, base);
	// 826C99B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C99B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C99BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C99C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C99C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C99C8 size=100
    let mut pc: u32 = 0x826C99C8;
    'dispatch: loop {
        match pc {
            0x826C99C8 => {
    //   block [0x826C99C8..0x826C9A2C)
	// 826C99C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C99CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C99D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C99D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C99D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C99DC: 38AA73C4  addi r5, r10, 0x73c4
	ctx.r[5].s64 = ctx.r[10].s64 + 29636;
	// 826C99E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C99E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C99E8: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 826C99EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C99F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C99F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C99F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C99FC: 386A7424  addi r3, r10, 0x7424
	ctx.r[3].s64 = ctx.r[10].s64 + 29732;
	// 826C9A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9A04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9A08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C9A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9A10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C9A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9A18: 4BD9D409  bl 0x82466e20
	ctx.lr = 0x826C9A1C;
	sub_82466E20(ctx, base);
	// 826C9A1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9A30 size=100
    let mut pc: u32 = 0x826C9A30;
    'dispatch: loop {
        match pc {
            0x826C9A30 => {
    //   block [0x826C9A30..0x826C9A94)
	// 826C9A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9A38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9A3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9A44: 38AA7304  addi r5, r10, 0x7304
	ctx.r[5].s64 = ctx.r[10].s64 + 29444;
	// 826C9A48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9A50: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 826C9A54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9A64: 386A7454  addi r3, r10, 0x7454
	ctx.r[3].s64 = ctx.r[10].s64 + 29780;
	// 826C9A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9A6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9A70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C9A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9A78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C9A7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9A80: 4BD9D3A1  bl 0x82466e20
	ctx.lr = 0x826C9A84;
	sub_82466E20(ctx, base);
	// 826C9A84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9A98 size=112
    let mut pc: u32 = 0x826C9A98;
    'dispatch: loop {
        match pc {
            0x826C9A98 => {
    //   block [0x826C9A98..0x826C9B08)
	// 826C9A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9AA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9AA8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9AAC: 38AA74E4  addi r5, r10, 0x74e4
	ctx.r[5].s64 = ctx.r[10].s64 + 29924;
	// 826C9AB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9AB4: 390B8100  addi r8, r11, -0x7f00
	ctx.r[8].s64 = ctx.r[11].s64 + -32512;
	// 826C9AB8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C9ABC: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 826C9AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9AC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9AC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9AD0: 386A7484  addi r3, r10, 0x7484
	ctx.r[3].s64 = ctx.r[10].s64 + 29828;
	// 826C9AD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9AD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9AF4: 4BD9D32D  bl 0x82466e20
	ctx.lr = 0x826C9AF8;
	sub_82466E20(ctx, base);
	// 826C9AF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9B08 size=112
    let mut pc: u32 = 0x826C9B08;
    'dispatch: loop {
        match pc {
            0x826C9B08 => {
    //   block [0x826C9B08..0x826C9B78)
	// 826C9B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9B10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9B14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9B18: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9B1C: 38AA7514  addi r5, r10, 0x7514
	ctx.r[5].s64 = ctx.r[10].s64 + 29972;
	// 826C9B20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9B24: 390B8130  addi r8, r11, -0x7ed0
	ctx.r[8].s64 = ctx.r[11].s64 + -32464;
	// 826C9B28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C9B2C: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 826C9B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9B34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9B38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9B40: 386A74B4  addi r3, r10, 0x74b4
	ctx.r[3].s64 = ctx.r[10].s64 + 29876;
	// 826C9B44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9B54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9B64: 4BD9D2BD  bl 0x82466e20
	ctx.lr = 0x826C9B68;
	sub_82466E20(ctx, base);
	// 826C9B68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9B78 size=112
    let mut pc: u32 = 0x826C9B78;
    'dispatch: loop {
        match pc {
            0x826C9B78 => {
    //   block [0x826C9B78..0x826C9BE8)
	// 826C9B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9B80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9B84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9B88: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9B8C: 38AA75D4  addi r5, r10, 0x75d4
	ctx.r[5].s64 = ctx.r[10].s64 + 30164;
	// 826C9B90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9B94: 390B8148  addi r8, r11, -0x7eb8
	ctx.r[8].s64 = ctx.r[11].s64 + -32440;
	// 826C9B98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C9B9C: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 826C9BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9BA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9BA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9BB0: 386A74E4  addi r3, r10, 0x74e4
	ctx.r[3].s64 = ctx.r[10].s64 + 29924;
	// 826C9BB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9BC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9BC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9BD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9BD4: 4BD9D24D  bl 0x82466e20
	ctx.lr = 0x826C9BD8;
	sub_82466E20(ctx, base);
	// 826C9BD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9BE8 size=112
    let mut pc: u32 = 0x826C9BE8;
    'dispatch: loop {
        match pc {
            0x826C9BE8 => {
    //   block [0x826C9BE8..0x826C9C58)
	// 826C9BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9BF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9BF8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9BFC: 38AA74E4  addi r5, r10, 0x74e4
	ctx.r[5].s64 = ctx.r[10].s64 + 29924;
	// 826C9C00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9C04: 390B8178  addi r8, r11, -0x7e88
	ctx.r[8].s64 = ctx.r[11].s64 + -32392;
	// 826C9C08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C9C0C: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 826C9C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9C14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9C18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9C20: 386A7514  addi r3, r10, 0x7514
	ctx.r[3].s64 = ctx.r[10].s64 + 29972;
	// 826C9C24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9C30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9C38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9C40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9C44: 4BD9D1DD  bl 0x82466e20
	ctx.lr = 0x826C9C48;
	sub_82466E20(ctx, base);
	// 826C9C48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9C58 size=112
    let mut pc: u32 = 0x826C9C58;
    'dispatch: loop {
        match pc {
            0x826C9C58 => {
    //   block [0x826C9C58..0x826C9CC8)
	// 826C9C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9C64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9C68: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9C6C: 38AA7514  addi r5, r10, 0x7514
	ctx.r[5].s64 = ctx.r[10].s64 + 29972;
	// 826C9C70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9C74: 390B8190  addi r8, r11, -0x7e70
	ctx.r[8].s64 = ctx.r[11].s64 + -32368;
	// 826C9C78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C9C7C: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 826C9C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9C84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9C88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9C90: 386A7544  addi r3, r10, 0x7544
	ctx.r[3].s64 = ctx.r[10].s64 + 30020;
	// 826C9C94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9C98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9C9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9CA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9CB4: 4BD9D16D  bl 0x82466e20
	ctx.lr = 0x826C9CB8;
	sub_82466E20(ctx, base);
	// 826C9CB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9CC8 size=116
    let mut pc: u32 = 0x826C9CC8;
    'dispatch: loop {
        match pc {
            0x826C9CC8 => {
    //   block [0x826C9CC8..0x826C9D3C)
	// 826C9CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9CD4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C9CD8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826C9CDC: 390A81A8  addi r8, r10, -0x7e58
	ctx.r[8].s64 = ctx.r[10].s64 + -32344;
	// 826C9CE0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9CE4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C9CE8: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C9CEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9CF0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C9CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9CF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9CFC: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 826C9D00: 396B29B4  addi r11, r11, 0x29b4
	ctx.r[11].s64 = ctx.r[11].s64 + 10676;
	// 826C9D04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9D08: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9D0C: 386A7574  addi r3, r10, 0x7574
	ctx.r[3].s64 = ctx.r[10].s64 + 30068;
	// 826C9D10: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C9D14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9D18: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C9D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9D28: 4BD9D0F9  bl 0x82466e20
	ctx.lr = 0x826C9D2C;
	sub_82466E20(ctx, base);
	// 826C9D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C9D40 size=48
    let mut pc: u32 = 0x826C9D40;
    'dispatch: loop {
        match pc {
            0x826C9D40 => {
    //   block [0x826C9D40..0x826C9D70)
	// 826C9D40: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9D44: 814B825C  lwz r10, -0x7da4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32164 as u32) ) } as u64;
	// 826C9D48: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9D4C: 396BA700  addi r11, r11, -0x5900
	ctx.r[11].s64 = ctx.r[11].s64 + -22784;
	// 826C9D50: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826C9D54: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C9D58: 814A8258  lwz r10, -0x7da8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-32168 as u32) ) } as u64;
	// 826C9D5C: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826C9D60: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C9D64: 814A8254  lwz r10, -0x7dac(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-32172 as u32) ) } as u64;
	// 826C9D68: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 826C9D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9D70 size=116
    let mut pc: u32 = 0x826C9D70;
    'dispatch: loop {
        match pc {
            0x826C9D70 => {
    //   block [0x826C9D70..0x826C9DE4)
	// 826C9D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9D7C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C9D80: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9D84: 392B2A88  addi r9, r11, 0x2a88
	ctx.r[9].s64 = ctx.r[11].s64 + 10888;
	// 826C9D88: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C9D8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9D90: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 826C9D94: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 826C9D98: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9D9C: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 826C9DA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9DA4: 396BA700  addi r11, r11, -0x5900
	ctx.r[11].s64 = ctx.r[11].s64 + -22784;
	// 826C9DA8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826C9DAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9DB0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826C9DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9DB8: 386A75A4  addi r3, r10, 0x75a4
	ctx.r[3].s64 = ctx.r[10].s64 + 30116;
	// 826C9DBC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826C9DC0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826C9DC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9DC8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826C9DCC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C9DD0: 4BD9D051  bl 0x82466e20
	ctx.lr = 0x826C9DD4;
	sub_82466E20(ctx, base);
	// 826C9DD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9DE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9DE8 size=116
    let mut pc: u32 = 0x826C9DE8;
    'dispatch: loop {
        match pc {
            0x826C9DE8 => {
    //   block [0x826C9DE8..0x826C9E5C)
	// 826C9DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9DF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9DF4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9DF8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C9DFC: 390B8268  addi r8, r11, -0x7d98
	ctx.r[8].s64 = ctx.r[11].s64 + -32152;
	// 826C9E00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9E04: 392A2BB0  addi r9, r10, 0x2bb0
	ctx.r[9].s64 = ctx.r[10].s64 + 11184;
	// 826C9E08: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9E0C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826C9E10: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C9E14: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9E1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9E24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9E2C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C9E30: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 826C9E34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C9E38: 386B75D4  addi r3, r11, 0x75d4
	ctx.r[3].s64 = ctx.r[11].s64 + 30164;
	// 826C9E3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C9E40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9E48: 4BD9CFD9  bl 0x82466e20
	ctx.lr = 0x826C9E4C;
	sub_82466E20(ctx, base);
	// 826C9E4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9E60 size=112
    let mut pc: u32 = 0x826C9E60;
    'dispatch: loop {
        match pc {
            0x826C9E60 => {
    //   block [0x826C9E60..0x826C9ED0)
	// 826C9E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9E6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9E70: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9E74: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C9E78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9E7C: 390B82F8  addi r8, r11, -0x7d08
	ctx.r[8].s64 = ctx.r[11].s64 + -32008;
	// 826C9E80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C9E84: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 826C9E88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9E8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9E90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9E98: 386A7604  addi r3, r10, 0x7604
	ctx.r[3].s64 = ctx.r[10].s64 + 30212;
	// 826C9E9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9EA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9EA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9EA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9EAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9EB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9EB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9EB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9EBC: 4BD9CF65  bl 0x82466e20
	ctx.lr = 0x826C9EC0;
	sub_82466E20(ctx, base);
	// 826C9EC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9ED0 size=112
    let mut pc: u32 = 0x826C9ED0;
    'dispatch: loop {
        match pc {
            0x826C9ED0 => {
    //   block [0x826C9ED0..0x826C9F40)
	// 826C9ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9ED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9EDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9EE0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9EE4: 38AA5954  addi r5, r10, 0x5954
	ctx.r[5].s64 = ctx.r[10].s64 + 22868;
	// 826C9EE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9EEC: 390B8310  addi r8, r11, -0x7cf0
	ctx.r[8].s64 = ctx.r[11].s64 + -31984;
	// 826C9EF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C9EF4: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 826C9EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9EFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9F00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9F08: 386A7634  addi r3, r10, 0x7634
	ctx.r[3].s64 = ctx.r[10].s64 + 30260;
	// 826C9F0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9F10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9F18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9F20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9F24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9F28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9F2C: 4BD9CEF5  bl 0x82466e20
	ctx.lr = 0x826C9F30;
	sub_82466E20(ctx, base);
	// 826C9F30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9F40 size=108
    let mut pc: u32 = 0x826C9F40;
    'dispatch: loop {
        match pc {
            0x826C9F40 => {
    //   block [0x826C9F40..0x826C9FAC)
	// 826C9F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9F48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9F4C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9F50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9F54: 38EB8328  addi r7, r11, -0x7cd8
	ctx.r[7].s64 = ctx.r[11].s64 + -31960;
	// 826C9F58: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C9F5C: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 826C9F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9F64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9F68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C9F6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9F70: 386A7664  addi r3, r10, 0x7664
	ctx.r[3].s64 = ctx.r[10].s64 + 30308;
	// 826C9F74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C9F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9F7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9F80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9F84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9F88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9F8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9F94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C9F98: 4BD9CE89  bl 0x82466e20
	ctx.lr = 0x826C9F9C;
	sub_82466E20(ctx, base);
	// 826C9F9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9FB0 size=112
    let mut pc: u32 = 0x826C9FB0;
    'dispatch: loop {
        match pc {
            0x826C9FB0 => {
    //   block [0x826C9FB0..0x826CA020)
	// 826C9FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9FBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9FC0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9FC4: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C9FC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9FCC: 390B8340  addi r8, r11, -0x7cc0
	ctx.r[8].s64 = ctx.r[11].s64 + -31936;
	// 826C9FD0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C9FD4: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 826C9FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9FDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9FE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9FE8: 386A7694  addi r3, r10, 0x7694
	ctx.r[3].s64 = ctx.r[10].s64 + 30356;
	// 826C9FEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9FF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9FF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9FF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9FFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA00C: 4BD9CE15  bl 0x82466e20
	ctx.lr = 0x826CA010;
	sub_82466E20(ctx, base);
	// 826CA010: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA020 size=108
    let mut pc: u32 = 0x826CA020;
    'dispatch: loop {
        match pc {
            0x826CA020 => {
    //   block [0x826CA020..0x826CA08C)
	// 826CA020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA02C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA034: 38EB8388  addi r7, r11, -0x7c78
	ctx.r[7].s64 = ctx.r[11].s64 + -31864;
	// 826CA038: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CA03C: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 826CA040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA044: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA048: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CA04C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA050: 386A76C4  addi r3, r10, 0x76c4
	ctx.r[3].s64 = ctx.r[10].s64 + 30404;
	// 826CA054: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CA058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA05C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA06C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CA078: 4BD9CDA9  bl 0x82466e20
	ctx.lr = 0x826CA07C;
	sub_82466E20(ctx, base);
	// 826CA07C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA090 size=112
    let mut pc: u32 = 0x826CA090;
    'dispatch: loop {
        match pc {
            0x826CA090 => {
    //   block [0x826CA090..0x826CA100)
	// 826CA090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA09C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA0A0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA0A4: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CA0A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA0AC: 390B83A0  addi r8, r11, -0x7c60
	ctx.r[8].s64 = ctx.r[11].s64 + -31840;
	// 826CA0B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CA0B4: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 826CA0B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA0BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA0C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA0C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA0C8: 386A76F4  addi r3, r10, 0x76f4
	ctx.r[3].s64 = ctx.r[10].s64 + 30452;
	// 826CA0CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA0D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA0D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA0D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA0DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA0E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA0E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA0E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA0EC: 4BD9CD35  bl 0x82466e20
	ctx.lr = 0x826CA0F0;
	sub_82466E20(ctx, base);
	// 826CA0F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA0F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA0F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA0FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA100 size=112
    let mut pc: u32 = 0x826CA100;
    'dispatch: loop {
        match pc {
            0x826CA100 => {
    //   block [0x826CA100..0x826CA170)
	// 826CA100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA10C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CA110: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA114: 392A2C08  addi r9, r10, 0x2c08
	ctx.r[9].s64 = ctx.r[10].s64 + 11272;
	// 826CA118: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA11C: 390B83D8  addi r8, r11, -0x7c28
	ctx.r[8].s64 = ctx.r[11].s64 + -31784;
	// 826CA120: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826CA124: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 826CA128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA12C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA130: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA138: 386A7724  addi r3, r10, 0x7724
	ctx.r[3].s64 = ctx.r[10].s64 + 30500;
	// 826CA13C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CA140: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CA144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA148: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA14C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA150: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA154: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CA158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA15C: 4BD9CCC5  bl 0x82466e20
	ctx.lr = 0x826CA160;
	sub_82466E20(ctx, base);
	// 826CA160: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA170 size=116
    let mut pc: u32 = 0x826CA170;
    'dispatch: loop {
        match pc {
            0x826CA170 => {
    //   block [0x826CA170..0x826CA1E4)
	// 826CA170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA17C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA180: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CA184: 390B8480  addi r8, r11, -0x7b80
	ctx.r[8].s64 = ctx.r[11].s64 + -31616;
	// 826CA188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA18C: 392A2BDC  addi r9, r10, 0x2bdc
	ctx.r[9].s64 = ctx.r[10].s64 + 11228;
	// 826CA190: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA194: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826CA198: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826CA19C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA1A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA1A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA1A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA1AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA1B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA1B4: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826CA1B8: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 826CA1BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CA1C0: 386B7754  addi r3, r11, 0x7754
	ctx.r[3].s64 = ctx.r[11].s64 + 30548;
	// 826CA1C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CA1C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA1D0: 4BD9CC51  bl 0x82466e20
	ctx.lr = 0x826CA1D4;
	sub_82466E20(ctx, base);
	// 826CA1D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA1D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA1DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA1E8 size=112
    let mut pc: u32 = 0x826CA1E8;
    'dispatch: loop {
        match pc {
            0x826CA1E8 => {
    //   block [0x826CA1E8..0x826CA258)
	// 826CA1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA1F4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CA1F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA1FC: 392A2C34  addi r9, r10, 0x2c34
	ctx.r[9].s64 = ctx.r[10].s64 + 11316;
	// 826CA200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA204: 390B8498  addi r8, r11, -0x7b68
	ctx.r[8].s64 = ctx.r[11].s64 + -31592;
	// 826CA208: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826CA20C: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 826CA210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA214: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA218: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA21C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA220: 386A7784  addi r3, r10, 0x7784
	ctx.r[3].s64 = ctx.r[10].s64 + 30596;
	// 826CA224: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CA228: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CA22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA23C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CA240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA244: 4BD9CBDD  bl 0x82466e20
	ctx.lr = 0x826CA248;
	sub_82466E20(ctx, base);
	// 826CA248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA258 size=112
    let mut pc: u32 = 0x826CA258;
    'dispatch: loop {
        match pc {
            0x826CA258 => {
    //   block [0x826CA258..0x826CA2C8)
	// 826CA258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA264: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA268: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA26C: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826CA270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA274: 390B84F8  addi r8, r11, -0x7b08
	ctx.r[8].s64 = ctx.r[11].s64 + -31496;
	// 826CA278: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CA27C: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 826CA280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA284: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA290: 386A77B4  addi r3, r10, 0x77b4
	ctx.r[3].s64 = ctx.r[10].s64 + 30644;
	// 826CA294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA29C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA2A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA2B4: 4BD9CB6D  bl 0x82466e20
	ctx.lr = 0x826CA2B8;
	sub_82466E20(ctx, base);
	// 826CA2B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA2C8 size=112
    let mut pc: u32 = 0x826CA2C8;
    'dispatch: loop {
        match pc {
            0x826CA2C8 => {
    //   block [0x826CA2C8..0x826CA338)
	// 826CA2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA2D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA2D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA2D8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA2DC: 38AA68B4  addi r5, r10, 0x68b4
	ctx.r[5].s64 = ctx.r[10].s64 + 26804;
	// 826CA2E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA2E4: 390B8510  addi r8, r11, -0x7af0
	ctx.r[8].s64 = ctx.r[11].s64 + -31472;
	// 826CA2E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CA2EC: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 826CA2F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA2F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA2F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA300: 386A77E4  addi r3, r10, 0x77e4
	ctx.r[3].s64 = ctx.r[10].s64 + 30692;
	// 826CA304: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA30C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA324: 4BD9CAFD  bl 0x82466e20
	ctx.lr = 0x826CA328;
	sub_82466E20(ctx, base);
	// 826CA328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA32C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA338 size=112
    let mut pc: u32 = 0x826CA338;
    'dispatch: loop {
        match pc {
            0x826CA338 => {
    //   block [0x826CA338..0x826CA3A8)
	// 826CA338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA344: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA348: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA34C: 38AA68B4  addi r5, r10, 0x68b4
	ctx.r[5].s64 = ctx.r[10].s64 + 26804;
	// 826CA350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA354: 390B8558  addi r8, r11, -0x7aa8
	ctx.r[8].s64 = ctx.r[11].s64 + -31400;
	// 826CA358: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826CA35C: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 826CA360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA364: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA36C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA370: 386A7814  addi r3, r10, 0x7814
	ctx.r[3].s64 = ctx.r[10].s64 + 30740;
	// 826CA374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA37C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA394: 4BD9CA8D  bl 0x82466e20
	ctx.lr = 0x826CA398;
	sub_82466E20(ctx, base);
	// 826CA398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA39C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA3A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA3A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA3A8 size=112
    let mut pc: u32 = 0x826CA3A8;
    'dispatch: loop {
        match pc {
            0x826CA3A8 => {
    //   block [0x826CA3A8..0x826CA418)
	// 826CA3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA3B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA3B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA3BC: 38AA68E4  addi r5, r10, 0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + 26852;
	// 826CA3C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA3C4: 390B85B8  addi r8, r11, -0x7a48
	ctx.r[8].s64 = ctx.r[11].s64 + -31304;
	// 826CA3C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826CA3CC: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 826CA3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA3D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA3D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA3DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA3E0: 386A7844  addi r3, r10, 0x7844
	ctx.r[3].s64 = ctx.r[10].s64 + 30788;
	// 826CA3E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA3EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA3F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA3F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA3F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA3FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA404: 4BD9CA1D  bl 0x82466e20
	ctx.lr = 0x826CA408;
	sub_82466E20(ctx, base);
	// 826CA408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA40C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA418 size=112
    let mut pc: u32 = 0x826CA418;
    'dispatch: loop {
        match pc {
            0x826CA418 => {
    //   block [0x826CA418..0x826CA488)
	// 826CA418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA424: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA428: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA42C: 38AA68E4  addi r5, r10, 0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + 26852;
	// 826CA430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA434: 390B8618  addi r8, r11, -0x79e8
	ctx.r[8].s64 = ctx.r[11].s64 + -31208;
	// 826CA438: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826CA43C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 826CA440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA444: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA44C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA450: 386A7874  addi r3, r10, 0x7874
	ctx.r[3].s64 = ctx.r[10].s64 + 30836;
	// 826CA454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA45C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA474: 4BD9C9AD  bl 0x82466e20
	ctx.lr = 0x826CA478;
	sub_82466E20(ctx, base);
	// 826CA478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA47C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA488 size=112
    let mut pc: u32 = 0x826CA488;
    'dispatch: loop {
        match pc {
            0x826CA488 => {
    //   block [0x826CA488..0x826CA4F8)
	// 826CA488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA494: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA498: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA49C: 38AA68B4  addi r5, r10, 0x68b4
	ctx.r[5].s64 = ctx.r[10].s64 + 26804;
	// 826CA4A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA4A4: 390B8678  addi r8, r11, -0x7988
	ctx.r[8].s64 = ctx.r[11].s64 + -31112;
	// 826CA4A8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826CA4AC: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 826CA4B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA4B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA4B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA4BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA4C0: 386A78A4  addi r3, r10, 0x78a4
	ctx.r[3].s64 = ctx.r[10].s64 + 30884;
	// 826CA4C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA4C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA4CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA4D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA4D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA4D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA4DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA4E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA4E4: 4BD9C93D  bl 0x82466e20
	ctx.lr = 0x826CA4E8;
	sub_82466E20(ctx, base);
	// 826CA4E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA4EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA4F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA4F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA4F8 size=112
    let mut pc: u32 = 0x826CA4F8;
    'dispatch: loop {
        match pc {
            0x826CA4F8 => {
    //   block [0x826CA4F8..0x826CA568)
	// 826CA4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA504: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CA508: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 826CA50C: 38EA8738  addi r7, r10, -0x78c8
	ctx.r[7].s64 = ctx.r[10].s64 + -30920;
	// 826CA510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA514: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826CA518: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 826CA51C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA520: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CA524: 396B2C48  addi r11, r11, 0x2c48
	ctx.r[11].s64 = ctx.r[11].s64 + 11336;
	// 826CA528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CA52C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA530: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA534: 386A78D4  addi r3, r10, 0x78d4
	ctx.r[3].s64 = ctx.r[10].s64 + 30932;
	// 826CA538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA53C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826CA540: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA544: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826CA548: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA54C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA550: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CA554: 4BD9C8CD  bl 0x82466e20
	ctx.lr = 0x826CA558;
	sub_82466E20(ctx, base);
	// 826CA558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA55C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA568 size=112
    let mut pc: u32 = 0x826CA568;
    'dispatch: loop {
        match pc {
            0x826CA568 => {
    //   block [0x826CA568..0x826CA5D8)
	// 826CA568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA574: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA578: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA57C: 38AA5A14  addi r5, r10, 0x5a14
	ctx.r[5].s64 = ctx.r[10].s64 + 23060;
	// 826CA580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA584: 390B8900  addi r8, r11, -0x7700
	ctx.r[8].s64 = ctx.r[11].s64 + -30464;
	// 826CA588: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CA58C: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 826CA590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA594: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA59C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA5A0: 386A7904  addi r3, r10, 0x7904
	ctx.r[3].s64 = ctx.r[10].s64 + 30980;
	// 826CA5A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA5A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA5B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA5B4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CA5B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA5BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA5C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA5C4: 4BD9C85D  bl 0x82466e20
	ctx.lr = 0x826CA5C8;
	sub_82466E20(ctx, base);
	// 826CA5C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA5D8 size=112
    let mut pc: u32 = 0x826CA5D8;
    'dispatch: loop {
        match pc {
            0x826CA5D8 => {
    //   block [0x826CA5D8..0x826CA648)
	// 826CA5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA5E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA5E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA5E8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA5EC: 38AA5A14  addi r5, r10, 0x5a14
	ctx.r[5].s64 = ctx.r[10].s64 + 23060;
	// 826CA5F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA5F4: 390B8918  addi r8, r11, -0x76e8
	ctx.r[8].s64 = ctx.r[11].s64 + -30440;
	// 826CA5F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CA5FC: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 826CA600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA604: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA60C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA610: 386A7934  addi r3, r10, 0x7934
	ctx.r[3].s64 = ctx.r[10].s64 + 31028;
	// 826CA614: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA61C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA624: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CA628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA62C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA634: 4BD9C7ED  bl 0x82466e20
	ctx.lr = 0x826CA638;
	sub_82466E20(ctx, base);
	// 826CA638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA63C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA648 size=112
    let mut pc: u32 = 0x826CA648;
    'dispatch: loop {
        match pc {
            0x826CA648 => {
    //   block [0x826CA648..0x826CA6B8)
	// 826CA648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA654: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA658: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA65C: 38AA5A14  addi r5, r10, 0x5a14
	ctx.r[5].s64 = ctx.r[10].s64 + 23060;
	// 826CA660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA664: 390B8930  addi r8, r11, -0x76d0
	ctx.r[8].s64 = ctx.r[11].s64 + -30416;
	// 826CA668: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CA66C: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 826CA670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA674: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA67C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA680: 386A7964  addi r3, r10, 0x7964
	ctx.r[3].s64 = ctx.r[10].s64 + 31076;
	// 826CA684: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA68C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA69C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA6A4: 4BD9C77D  bl 0x82466e20
	ctx.lr = 0x826CA6A8;
	sub_82466E20(ctx, base);
	// 826CA6A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA6AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA6B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA6B8 size=108
    let mut pc: u32 = 0x826CA6B8;
    'dispatch: loop {
        match pc {
            0x826CA6B8 => {
    //   block [0x826CA6B8..0x826CA724)
	// 826CA6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA6C4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA6C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA6CC: 38EB8960  addi r7, r11, -0x76a0
	ctx.r[7].s64 = ctx.r[11].s64 + -30368;
	// 826CA6D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CA6D4: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 826CA6D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA6DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA6E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CA6E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA6E8: 386A7994  addi r3, r10, 0x7994
	ctx.r[3].s64 = ctx.r[10].s64 + 31124;
	// 826CA6EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CA6F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA6F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA6F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA70C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CA710: 4BD9C711  bl 0x82466e20
	ctx.lr = 0x826CA714;
	sub_82466E20(ctx, base);
	// 826CA714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA71C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA728 size=112
    let mut pc: u32 = 0x826CA728;
    'dispatch: loop {
        match pc {
            0x826CA728 => {
    //   block [0x826CA728..0x826CA798)
	// 826CA728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA734: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA738: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA73C: 38AA5A14  addi r5, r10, 0x5a14
	ctx.r[5].s64 = ctx.r[10].s64 + 23060;
	// 826CA740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA744: 390B8990  addi r8, r11, -0x7670
	ctx.r[8].s64 = ctx.r[11].s64 + -30320;
	// 826CA748: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CA74C: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 826CA750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA754: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA75C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA760: 386A79C4  addi r3, r10, 0x79c4
	ctx.r[3].s64 = ctx.r[10].s64 + 31172;
	// 826CA764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA76C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA774: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CA778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA77C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA784: 4BD9C69D  bl 0x82466e20
	ctx.lr = 0x826CA788;
	sub_82466E20(ctx, base);
	// 826CA788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA78C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA798 size=108
    let mut pc: u32 = 0x826CA798;
    'dispatch: loop {
        match pc {
            0x826CA798 => {
    //   block [0x826CA798..0x826CA804)
	// 826CA798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA7A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA7A4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA7A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA7AC: 38EB89A8  addi r7, r11, -0x7658
	ctx.r[7].s64 = ctx.r[11].s64 + -30296;
	// 826CA7B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CA7B4: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 826CA7B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA7BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA7C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CA7C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA7C8: 386A79F4  addi r3, r10, 0x79f4
	ctx.r[3].s64 = ctx.r[10].s64 + 31220;
	// 826CA7CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CA7D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA7D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA7D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA7DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA7E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA7E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA7E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA7EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CA7F0: 4BD9C631  bl 0x82466e20
	ctx.lr = 0x826CA7F4;
	sub_82466E20(ctx, base);
	// 826CA7F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA7F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA7FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA808 size=108
    let mut pc: u32 = 0x826CA808;
    'dispatch: loop {
        match pc {
            0x826CA808 => {
    //   block [0x826CA808..0x826CA874)
	// 826CA808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA814: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA81C: 38EB89D8  addi r7, r11, -0x7628
	ctx.r[7].s64 = ctx.r[11].s64 + -30248;
	// 826CA820: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CA824: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 826CA828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA82C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA830: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CA834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA838: 386A7A24  addi r3, r10, 0x7a24
	ctx.r[3].s64 = ctx.r[10].s64 + 31268;
	// 826CA83C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CA840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA84C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA85C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CA860: 4BD9C5C1  bl 0x82466e20
	ctx.lr = 0x826CA864;
	sub_82466E20(ctx, base);
	// 826CA864: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA86C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA878 size=112
    let mut pc: u32 = 0x826CA878;
    'dispatch: loop {
        match pc {
            0x826CA878 => {
    //   block [0x826CA878..0x826CA8E8)
	// 826CA878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA884: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA888: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA88C: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CA890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA894: 390B8A20  addi r8, r11, -0x75e0
	ctx.r[8].s64 = ctx.r[11].s64 + -30176;
	// 826CA898: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CA89C: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 826CA8A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA8A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA8A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA8AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA8B0: 386A7A54  addi r3, r10, 0x7a54
	ctx.r[3].s64 = ctx.r[10].s64 + 31316;
	// 826CA8B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA8B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA8BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA8C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA8C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA8C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA8CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA8D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA8D4: 4BD9C54D  bl 0x82466e20
	ctx.lr = 0x826CA8D8;
	sub_82466E20(ctx, base);
	// 826CA8D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA8DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA8E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA8E8 size=112
    let mut pc: u32 = 0x826CA8E8;
    'dispatch: loop {
        match pc {
            0x826CA8E8 => {
    //   block [0x826CA8E8..0x826CA958)
	// 826CA8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA8F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA8F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA8F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA8FC: 38AA68E4  addi r5, r10, 0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + 26852;
	// 826CA900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA904: 390B8A68  addi r8, r11, -0x7598
	ctx.r[8].s64 = ctx.r[11].s64 + -30104;
	// 826CA908: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826CA90C: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 826CA910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA914: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA918: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA91C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA920: 386A7A84  addi r3, r10, 0x7a84
	ctx.r[3].s64 = ctx.r[10].s64 + 31364;
	// 826CA924: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA928: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA92C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA93C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA944: 4BD9C4DD  bl 0x82466e20
	ctx.lr = 0x826CA948;
	sub_82466E20(ctx, base);
	// 826CA948: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA94C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA958 size=108
    let mut pc: u32 = 0x826CA958;
    'dispatch: loop {
        match pc {
            0x826CA958 => {
    //   block [0x826CA958..0x826CA9C4)
	// 826CA958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA964: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA96C: 38EB8AF8  addi r7, r11, -0x7508
	ctx.r[7].s64 = ctx.r[11].s64 + -29960;
	// 826CA970: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CA974: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 826CA978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA97C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA980: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CA984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA988: 386A7AB4  addi r3, r10, 0x7ab4
	ctx.r[3].s64 = ctx.r[10].s64 + 31412;
	// 826CA98C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CA990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA99C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA9A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA9A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA9A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA9AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CA9B0: 4BD9C471  bl 0x82466e20
	ctx.lr = 0x826CA9B4;
	sub_82466E20(ctx, base);
	// 826CA9B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA9B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA9BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA9C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA9C8 size=108
    let mut pc: u32 = 0x826CA9C8;
    'dispatch: loop {
        match pc {
            0x826CA9C8 => {
    //   block [0x826CA9C8..0x826CAA34)
	// 826CA9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA9D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA9D4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA9D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA9DC: 38EB8B40  addi r7, r11, -0x74c0
	ctx.r[7].s64 = ctx.r[11].s64 + -29888;
	// 826CA9E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CA9E4: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 826CA9E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA9EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA9F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CA9F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA9F8: 386A7AE4  addi r3, r10, 0x7ae4
	ctx.r[3].s64 = ctx.r[10].s64 + 31460;
	// 826CA9FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CAA00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CAA04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CAA08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CAA0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CAA10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CAA14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CAA18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CAA1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CAA20: 4BD9C401  bl 0x82466e20
	ctx.lr = 0x826CAA24;
	sub_82466E20(ctx, base);
	// 826CAA24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CAA28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CAA2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CAA30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


