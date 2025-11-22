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


pub fn sub_826A66B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A66B0 size=112
    let mut pc: u32 = 0x826A66B0;
    'dispatch: loop {
        match pc {
            0x826A66B0 => {
    //   block [0x826A66B0..0x826A6720)
	// 826A66B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A66B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A66B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A66BC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A66C0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A66C4: 392AC800  addi r9, r10, -0x3800
	ctx.r[9].s64 = ctx.r[10].s64 + -14336;
	// 826A66C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A66CC: 390B3748  addi r8, r11, 0x3748
	ctx.r[8].s64 = ctx.r[11].s64 + 14152;
	// 826A66D0: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826A66D4: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 826A66D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A66DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A66E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A66E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A66E8: 386A8618  addi r3, r10, -0x79e8
	ctx.r[3].s64 = ctx.r[10].s64 + -31208;
	// 826A66EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A66F0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826A66F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A66F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A66FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6704: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A670C: 4BDC0715  bl 0x82466e20
	ctx.lr = 0x826A6710;
	sub_82466E20(ctx, base);
	// 826A6710: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A671C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6720 size=108
    let mut pc: u32 = 0x826A6720;
    'dispatch: loop {
        match pc {
            0x826A6720 => {
    //   block [0x826A6720..0x826A678C)
	// 826A6720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A672C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6730: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A6734: 38EBBF08  addi r7, r11, -0x40f8
	ctx.r[7].s64 = ctx.r[11].s64 + -16632;
	// 826A6738: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A673C: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826A6740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6744: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6748: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A674C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6750: 386A8648  addi r3, r10, -0x79b8
	ctx.r[3].s64 = ctx.r[10].s64 + -31160;
	// 826A6754: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A675C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A676C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6778: 4BDC06A9  bl 0x82466e20
	ctx.lr = 0x826A677C;
	sub_82466E20(ctx, base);
	// 826A677C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A6790 size=24
    let mut pc: u32 = 0x826A6790;
    'dispatch: loop {
        match pc {
            0x826A6790 => {
    //   block [0x826A6790..0x826A67A8)
	// 826A6790: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6794: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A6798: 394A3838  addi r10, r10, 0x3838
	ctx.r[10].s64 = ctx.r[10].s64 + 14392;
	// 826A679C: 816BBF04  lwz r11, -0x40fc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16636 as u32) ) } as u64;
	// 826A67A0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826A67A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A67A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A67A8 size=112
    let mut pc: u32 = 0x826A67A8;
    'dispatch: loop {
        match pc {
            0x826A67A8 => {
    //   block [0x826A67A8..0x826A6818)
	// 826A67A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A67AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A67B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A67B4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A67B8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A67BC: 392AC830  addi r9, r10, -0x37d0
	ctx.r[9].s64 = ctx.r[10].s64 + -14288;
	// 826A67C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A67C4: 390B3838  addi r8, r11, 0x3838
	ctx.r[8].s64 = ctx.r[11].s64 + 14392;
	// 826A67C8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826A67CC: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 826A67D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A67D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A67D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A67DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A67E0: 386A8678  addi r3, r10, -0x7988
	ctx.r[3].s64 = ctx.r[10].s64 + -31112;
	// 826A67E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A67E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A67EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A67F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A67F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A67F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A67FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6804: 4BDC061D  bl 0x82466e20
	ctx.lr = 0x826A6808;
	sub_82466E20(ctx, base);
	// 826A6808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A680C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A6818 size=40
    let mut pc: u32 = 0x826A6818;
    'dispatch: loop {
        match pc {
            0x826A6818 => {
    //   block [0x826A6818..0x826A6840)
	// 826A6818: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A681C: 814BBF38  lwz r10, -0x40c8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16584 as u32) ) } as u64;
	// 826A6820: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6824: 396B3898  addi r11, r11, 0x3898
	ctx.r[11].s64 = ctx.r[11].s64 + 14488;
	// 826A6828: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826A682C: 914B0068  stw r10, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 826A6830: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A6834: 814ABF3C  lwz r10, -0x40c4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16580 as u32) ) } as u64;
	// 826A6838: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 826A683C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6840 size=112
    let mut pc: u32 = 0x826A6840;
    'dispatch: loop {
        match pc {
            0x826A6840 => {
    //   block [0x826A6840..0x826A68B0)
	// 826A6840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A684C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A6850: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6854: 392AC9A8  addi r9, r10, -0x3658
	ctx.r[9].s64 = ctx.r[10].s64 + -13912;
	// 826A6858: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A685C: 390B3898  addi r8, r11, 0x3898
	ctx.r[8].s64 = ctx.r[11].s64 + 14488;
	// 826A6860: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826A6864: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826A6868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A686C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A6874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6878: 386A86A8  addi r3, r10, -0x7958
	ctx.r[3].s64 = ctx.r[10].s64 + -31064;
	// 826A687C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A6880: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826A6884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A688C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6894: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A689C: 4BDC0585  bl 0x82466e20
	ctx.lr = 0x826A68A0;
	sub_82466E20(ctx, base);
	// 826A68A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A68A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A68A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A68AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A68B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A68B0 size=108
    let mut pc: u32 = 0x826A68B0;
    'dispatch: loop {
        match pc {
            0x826A68B0 => {
    //   block [0x826A68B0..0x826A691C)
	// 826A68B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A68B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A68B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A68BC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A68C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A68C4: 38EBBF44  addi r7, r11, -0x40bc
	ctx.r[7].s64 = ctx.r[11].s64 + -16572;
	// 826A68C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A68CC: 388A6E48  addi r4, r10, 0x6e48
	ctx.r[4].s64 = ctx.r[10].s64 + 28232;
	// 826A68D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A68D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A68D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A68DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A68E0: 386A86D8  addi r3, r10, -0x7928
	ctx.r[3].s64 = ctx.r[10].s64 + -31016;
	// 826A68E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A68E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A68EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A68F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A68F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A68F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A68FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6904: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6908: 4BDC0519  bl 0x82466e20
	ctx.lr = 0x826A690C;
	sub_82466E20(ctx, base);
	// 826A690C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6920 size=108
    let mut pc: u32 = 0x826A6920;
    'dispatch: loop {
        match pc {
            0x826A6920 => {
    //   block [0x826A6920..0x826A698C)
	// 826A6920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A692C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6930: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A6934: 38EBBF74  addi r7, r11, -0x408c
	ctx.r[7].s64 = ctx.r[11].s64 + -16524;
	// 826A6938: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A693C: 388A6E64  addi r4, r10, 0x6e64
	ctx.r[4].s64 = ctx.r[10].s64 + 28260;
	// 826A6940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6944: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6948: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A694C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6950: 386A8708  addi r3, r10, -0x78f8
	ctx.r[3].s64 = ctx.r[10].s64 + -30968;
	// 826A6954: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A695C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A696C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6978: 4BDC04A9  bl 0x82466e20
	ctx.lr = 0x826A697C;
	sub_82466E20(ctx, base);
	// 826A697C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6990 size=108
    let mut pc: u32 = 0x826A6990;
    'dispatch: loop {
        match pc {
            0x826A6990 => {
    //   block [0x826A6990..0x826A69FC)
	// 826A6990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A699C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A69A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A69A4: 38EBBF8C  addi r7, r11, -0x4074
	ctx.r[7].s64 = ctx.r[11].s64 + -16500;
	// 826A69A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A69AC: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826A69B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A69B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A69B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A69BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A69C0: 386A8738  addi r3, r10, -0x78c8
	ctx.r[3].s64 = ctx.r[10].s64 + -30920;
	// 826A69C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A69C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A69CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A69D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A69D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A69D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A69DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A69E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A69E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A69E8: 4BDC0439  bl 0x82466e20
	ctx.lr = 0x826A69EC;
	sub_82466E20(ctx, base);
	// 826A69EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A69F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A69F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A69F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6A00 size=108
    let mut pc: u32 = 0x826A6A00;
    'dispatch: loop {
        match pc {
            0x826A6A00 => {
    //   block [0x826A6A00..0x826A6A6C)
	// 826A6A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6A0C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6A10: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A6A14: 38EBBFC0  addi r7, r11, -0x4040
	ctx.r[7].s64 = ctx.r[11].s64 + -16448;
	// 826A6A18: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826A6A1C: 388A7BB8  addi r4, r10, 0x7bb8
	ctx.r[4].s64 = ctx.r[10].s64 + 31672;
	// 826A6A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6A24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6A28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6A30: 386A8768  addi r3, r10, -0x7898
	ctx.r[3].s64 = ctx.r[10].s64 + -30872;
	// 826A6A34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6A3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6A54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6A58: 4BDC03C9  bl 0x82466e20
	ctx.lr = 0x826A6A5C;
	sub_82466E20(ctx, base);
	// 826A6A5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6A70 size=108
    let mut pc: u32 = 0x826A6A70;
    'dispatch: loop {
        match pc {
            0x826A6A70 => {
    //   block [0x826A6A70..0x826A6ADC)
	// 826A6A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6A7C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6A80: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A6A84: 38EBC050  addi r7, r11, -0x3fb0
	ctx.r[7].s64 = ctx.r[11].s64 + -16304;
	// 826A6A88: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A6A8C: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 826A6A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6A94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6A98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6AA0: 386A8798  addi r3, r10, -0x7868
	ctx.r[3].s64 = ctx.r[10].s64 + -30824;
	// 826A6AA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6AC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6AC8: 4BDC0359  bl 0x82466e20
	ctx.lr = 0x826A6ACC;
	sub_82466E20(ctx, base);
	// 826A6ACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6AE0 size=108
    let mut pc: u32 = 0x826A6AE0;
    'dispatch: loop {
        match pc {
            0x826A6AE0 => {
    //   block [0x826A6AE0..0x826A6B4C)
	// 826A6AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6AEC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6AF0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A6AF4: 38EBC068  addi r7, r11, -0x3f98
	ctx.r[7].s64 = ctx.r[11].s64 + -16280;
	// 826A6AF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A6AFC: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 826A6B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6B04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6B08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6B10: 386A87C8  addi r3, r10, -0x7838
	ctx.r[3].s64 = ctx.r[10].s64 + -30776;
	// 826A6B14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6B1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6B34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6B38: 4BDC02E9  bl 0x82466e20
	ctx.lr = 0x826A6B3C;
	sub_82466E20(ctx, base);
	// 826A6B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6B50 size=112
    let mut pc: u32 = 0x826A6B50;
    'dispatch: loop {
        match pc {
            0x826A6B50 => {
    //   block [0x826A6B50..0x826A6BC0)
	// 826A6B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6B5C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A6B60: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6B64: 392AC9FC  addi r9, r10, -0x3604
	ctx.r[9].s64 = ctx.r[10].s64 + -13828;
	// 826A6B68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A6B6C: 390BC098  addi r8, r11, -0x3f68
	ctx.r[8].s64 = ctx.r[11].s64 + -16232;
	// 826A6B70: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826A6B74: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 826A6B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6B7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6B80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A6B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6B88: 386A87F8  addi r3, r10, -0x7808
	ctx.r[3].s64 = ctx.r[10].s64 + -30728;
	// 826A6B8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A6B90: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A6B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6BA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6BAC: 4BDC0275  bl 0x82466e20
	ctx.lr = 0x826A6BB0;
	sub_82466E20(ctx, base);
	// 826A6BB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6BC0 size=108
    let mut pc: u32 = 0x826A6BC0;
    'dispatch: loop {
        match pc {
            0x826A6BC0 => {
    //   block [0x826A6BC0..0x826A6C2C)
	// 826A6BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6BCC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6BD0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A6BD4: 38EBC110  addi r7, r11, -0x3ef0
	ctx.r[7].s64 = ctx.r[11].s64 + -16112;
	// 826A6BD8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826A6BDC: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 826A6BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6BE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6BE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6BF0: 386A8828  addi r3, r10, -0x77d8
	ctx.r[3].s64 = ctx.r[10].s64 + -30680;
	// 826A6BF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6BFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6C04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6C14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6C18: 4BDC0209  bl 0x82466e20
	ctx.lr = 0x826A6C1C;
	sub_82466E20(ctx, base);
	// 826A6C1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6C20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6C24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A6C30 size=24
    let mut pc: u32 = 0x826A6C30;
    'dispatch: loop {
        match pc {
            0x826A6C30 => {
    //   block [0x826A6C30..0x826A6C48)
	// 826A6C30: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6C34: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A6C38: 394A3970  addi r10, r10, 0x3970
	ctx.r[10].s64 = ctx.r[10].s64 + 14704;
	// 826A6C3C: 816BC200  lwz r11, -0x3e00(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15872 as u32) ) } as u64;
	// 826A6C40: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826A6C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6C48 size=108
    let mut pc: u32 = 0x826A6C48;
    'dispatch: loop {
        match pc {
            0x826A6C48 => {
    //   block [0x826A6C48..0x826A6CB4)
	// 826A6C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6C54: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6C58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A6C5C: 38EB3970  addi r7, r11, 0x3970
	ctx.r[7].s64 = ctx.r[11].s64 + 14704;
	// 826A6C60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A6C64: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 826A6C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6C6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6C70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6C78: 386A8858  addi r3, r10, -0x77a8
	ctx.r[3].s64 = ctx.r[10].s64 + -30632;
	// 826A6C7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6C80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6C9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6CA0: 4BDC0181  bl 0x82466e20
	ctx.lr = 0x826A6CA4;
	sub_82466E20(ctx, base);
	// 826A6CA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A6CB8 size=24
    let mut pc: u32 = 0x826A6CB8;
    'dispatch: loop {
        match pc {
            0x826A6CB8 => {
    //   block [0x826A6CB8..0x826A6CD0)
	// 826A6CB8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6CBC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A6CC0: 394A39A0  addi r10, r10, 0x39a0
	ctx.r[10].s64 = ctx.r[10].s64 + 14752;
	// 826A6CC4: 816BC200  lwz r11, -0x3e00(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15872 as u32) ) } as u64;
	// 826A6CC8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826A6CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6CD0 size=108
    let mut pc: u32 = 0x826A6CD0;
    'dispatch: loop {
        match pc {
            0x826A6CD0 => {
    //   block [0x826A6CD0..0x826A6D3C)
	// 826A6CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6CDC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6CE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A6CE4: 38EB39A0  addi r7, r11, 0x39a0
	ctx.r[7].s64 = ctx.r[11].s64 + 14752;
	// 826A6CE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A6CEC: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 826A6CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6CF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6CF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6D00: 386A8888  addi r3, r10, -0x7778
	ctx.r[3].s64 = ctx.r[10].s64 + -30584;
	// 826A6D04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6D24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6D28: 4BDC00F9  bl 0x82466e20
	ctx.lr = 0x826A6D2C;
	sub_82466E20(ctx, base);
	// 826A6D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6D40 size=108
    let mut pc: u32 = 0x826A6D40;
    'dispatch: loop {
        match pc {
            0x826A6D40 => {
    //   block [0x826A6D40..0x826A6DAC)
	// 826A6D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6D4C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6D50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A6D54: 38EBC1E8  addi r7, r11, -0x3e18
	ctx.r[7].s64 = ctx.r[11].s64 + -15896;
	// 826A6D58: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A6D5C: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 826A6D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6D64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6D68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6D70: 386A88B8  addi r3, r10, -0x7748
	ctx.r[3].s64 = ctx.r[10].s64 + -30536;
	// 826A6D74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6D94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6D98: 4BDC0089  bl 0x82466e20
	ctx.lr = 0x826A6D9C;
	sub_82466E20(ctx, base);
	// 826A6D9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A6DB0 size=24
    let mut pc: u32 = 0x826A6DB0;
    'dispatch: loop {
        match pc {
            0x826A6DB0 => {
    //   block [0x826A6DB0..0x826A6DC8)
	// 826A6DB0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6DB4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A6DB8: 394A39D0  addi r10, r10, 0x39d0
	ctx.r[10].s64 = ctx.r[10].s64 + 14800;
	// 826A6DBC: 816BC200  lwz r11, -0x3e00(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15872 as u32) ) } as u64;
	// 826A6DC0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826A6DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6DC8 size=108
    let mut pc: u32 = 0x826A6DC8;
    'dispatch: loop {
        match pc {
            0x826A6DC8 => {
    //   block [0x826A6DC8..0x826A6E34)
	// 826A6DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6DD4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A6DDC: 38EB39D0  addi r7, r11, 0x39d0
	ctx.r[7].s64 = ctx.r[11].s64 + 14800;
	// 826A6DE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A6DE4: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 826A6DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6DEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6DF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6DF8: 386A88E8  addi r3, r10, -0x7718
	ctx.r[3].s64 = ctx.r[10].s64 + -30488;
	// 826A6DFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6E1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6E20: 4BDC0001  bl 0x82466e20
	ctx.lr = 0x826A6E24;
	sub_82466E20(ctx, base);
	// 826A6E24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6E38 size=112
    let mut pc: u32 = 0x826A6E38;
    'dispatch: loop {
        match pc {
            0x826A6E38 => {
    //   block [0x826A6E38..0x826A6EA8)
	// 826A6E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6E44: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A6E48: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6E4C: 392ACA40  addi r9, r10, -0x35c0
	ctx.r[9].s64 = ctx.r[10].s64 + -13760;
	// 826A6E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A6E54: 390BC204  addi r8, r11, -0x3dfc
	ctx.r[8].s64 = ctx.r[11].s64 + -15868;
	// 826A6E58: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826A6E5C: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 826A6E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6E64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6E68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A6E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6E70: 386A8918  addi r3, r10, -0x76e8
	ctx.r[3].s64 = ctx.r[10].s64 + -30440;
	// 826A6E74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A6E78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A6E7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6E80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6E84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6E88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6E8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6E90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6E94: 4BDBFF8D  bl 0x82466e20
	ctx.lr = 0x826A6E98;
	sub_82466E20(ctx, base);
	// 826A6E98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6EA8 size=108
    let mut pc: u32 = 0x826A6EA8;
    'dispatch: loop {
        match pc {
            0x826A6EA8 => {
    //   block [0x826A6EA8..0x826A6F14)
	// 826A6EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6EB4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A6EBC: 38EBC234  addi r7, r11, -0x3dcc
	ctx.r[7].s64 = ctx.r[11].s64 + -15820;
	// 826A6EC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A6EC4: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 826A6EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6ECC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6ED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6ED8: 386A8948  addi r3, r10, -0x76b8
	ctx.r[3].s64 = ctx.r[10].s64 + -30392;
	// 826A6EDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6EFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6F00: 4BDBFF21  bl 0x82466e20
	ctx.lr = 0x826A6F04;
	sub_82466E20(ctx, base);
	// 826A6F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6F18 size=108
    let mut pc: u32 = 0x826A6F18;
    'dispatch: loop {
        match pc {
            0x826A6F18 => {
    //   block [0x826A6F18..0x826A6F84)
	// 826A6F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6F24: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A6F2C: 38EBC264  addi r7, r11, -0x3d9c
	ctx.r[7].s64 = ctx.r[11].s64 + -15772;
	// 826A6F30: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A6F34: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 826A6F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6F3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6F40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6F48: 386A8978  addi r3, r10, -0x7688
	ctx.r[3].s64 = ctx.r[10].s64 + -30344;
	// 826A6F4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6F5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6F6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6F70: 4BDBFEB1  bl 0x82466e20
	ctx.lr = 0x826A6F74;
	sub_82466E20(ctx, base);
	// 826A6F74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6F88 size=108
    let mut pc: u32 = 0x826A6F88;
    'dispatch: loop {
        match pc {
            0x826A6F88 => {
    //   block [0x826A6F88..0x826A6FF4)
	// 826A6F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6F94: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A6F9C: 38EBC27C  addi r7, r11, -0x3d84
	ctx.r[7].s64 = ctx.r[11].s64 + -15748;
	// 826A6FA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A6FA4: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826A6FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6FAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6FB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6FB8: 386A89A8  addi r3, r10, -0x7658
	ctx.r[3].s64 = ctx.r[10].s64 + -30296;
	// 826A6FBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6FDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6FE0: 4BDBFE41  bl 0x82466e20
	ctx.lr = 0x826A6FE4;
	sub_82466E20(ctx, base);
	// 826A6FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6FF8 size=112
    let mut pc: u32 = 0x826A6FF8;
    'dispatch: loop {
        match pc {
            0x826A6FF8 => {
    //   block [0x826A6FF8..0x826A7068)
	// 826A6FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7004: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7008: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A700C: 38AA8A08  addi r5, r10, -0x75f8
	ctx.r[5].s64 = ctx.r[10].s64 + -30200;
	// 826A7010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7014: 390BC2AC  addi r8, r11, -0x3d54
	ctx.r[8].s64 = ctx.r[11].s64 + -15700;
	// 826A7018: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A701C: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 826A7020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7024: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A702C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7030: 386A89D8  addi r3, r10, -0x7628
	ctx.r[3].s64 = ctx.r[10].s64 + -30248;
	// 826A7034: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A7038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A703C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A704C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7054: 4BDBFDCD  bl 0x82466e20
	ctx.lr = 0x826A7058;
	sub_82466E20(ctx, base);
	// 826A7058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A705C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7068 size=108
    let mut pc: u32 = 0x826A7068;
    'dispatch: loop {
        match pc {
            0x826A7068 => {
    //   block [0x826A7068..0x826A70D4)
	// 826A7068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A706C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7074: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A707C: 38EBC2C4  addi r7, r11, -0x3d3c
	ctx.r[7].s64 = ctx.r[11].s64 + -15676;
	// 826A7080: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A7084: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826A7088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A708C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7098: 386A8A08  addi r3, r10, -0x75f8
	ctx.r[3].s64 = ctx.r[10].s64 + -30200;
	// 826A709C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A70A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A70A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A70A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A70AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A70B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A70B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A70B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A70BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A70C0: 4BDBFD61  bl 0x82466e20
	ctx.lr = 0x826A70C4;
	sub_82466E20(ctx, base);
	// 826A70C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A70C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A70CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A70D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A70D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A70D8 size=108
    let mut pc: u32 = 0x826A70D8;
    'dispatch: loop {
        match pc {
            0x826A70D8 => {
    //   block [0x826A70D8..0x826A7144)
	// 826A70D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A70DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A70E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A70E4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A70E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A70EC: 38EBC2F4  addi r7, r11, -0x3d0c
	ctx.r[7].s64 = ctx.r[11].s64 + -15628;
	// 826A70F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A70F4: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 826A70F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A70FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7108: 386A8A38  addi r3, r10, -0x75c8
	ctx.r[3].s64 = ctx.r[10].s64 + -30152;
	// 826A710C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A711C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A712C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7130: 4BDBFCF1  bl 0x82466e20
	ctx.lr = 0x826A7134;
	sub_82466E20(ctx, base);
	// 826A7134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A713C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7148 size=108
    let mut pc: u32 = 0x826A7148;
    'dispatch: loop {
        match pc {
            0x826A7148 => {
    //   block [0x826A7148..0x826A71B4)
	// 826A7148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A714C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7154: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A715C: 38EBC30C  addi r7, r11, -0x3cf4
	ctx.r[7].s64 = ctx.r[11].s64 + -15604;
	// 826A7160: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A7164: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 826A7168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A716C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7170: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7178: 386A8A68  addi r3, r10, -0x7598
	ctx.r[3].s64 = ctx.r[10].s64 + -30104;
	// 826A717C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A718C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A719C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A71A0: 4BDBFC81  bl 0x82466e20
	ctx.lr = 0x826A71A4;
	sub_82466E20(ctx, base);
	// 826A71A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A71A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A71AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A71B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A71B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A71B8 size=108
    let mut pc: u32 = 0x826A71B8;
    'dispatch: loop {
        match pc {
            0x826A71B8 => {
    //   block [0x826A71B8..0x826A7224)
	// 826A71B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A71BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A71C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A71C4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A71C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A71CC: 38EBC340  addi r7, r11, -0x3cc0
	ctx.r[7].s64 = ctx.r[11].s64 + -15552;
	// 826A71D0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826A71D4: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826A71D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A71DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A71E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A71E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A71E8: 386A8A98  addi r3, r10, -0x7568
	ctx.r[3].s64 = ctx.r[10].s64 + -30056;
	// 826A71EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A71F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A71F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A71F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A71FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A720C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7210: 4BDBFC11  bl 0x82466e20
	ctx.lr = 0x826A7214;
	sub_82466E20(ctx, base);
	// 826A7214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A721C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7228 size=108
    let mut pc: u32 = 0x826A7228;
    'dispatch: loop {
        match pc {
            0x826A7228 => {
    //   block [0x826A7228..0x826A7294)
	// 826A7228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A722C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7234: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7238: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A723C: 38EBC3E8  addi r7, r11, -0x3c18
	ctx.r[7].s64 = ctx.r[11].s64 + -15384;
	// 826A7240: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A7244: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 826A7248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A724C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7250: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7258: 386A8AC8  addi r3, r10, -0x7538
	ctx.r[3].s64 = ctx.r[10].s64 + -30008;
	// 826A725C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A726C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A727C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7280: 4BDBFBA1  bl 0x82466e20
	ctx.lr = 0x826A7284;
	sub_82466E20(ctx, base);
	// 826A7284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A728C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7298 size=108
    let mut pc: u32 = 0x826A7298;
    'dispatch: loop {
        match pc {
            0x826A7298 => {
    //   block [0x826A7298..0x826A7304)
	// 826A7298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A729C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A72A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A72A4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A72A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A72AC: 38EBC418  addi r7, r11, -0x3be8
	ctx.r[7].s64 = ctx.r[11].s64 + -15336;
	// 826A72B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A72B4: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826A72B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A72BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A72C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A72C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A72C8: 386A8AF8  addi r3, r10, -0x7508
	ctx.r[3].s64 = ctx.r[10].s64 + -29960;
	// 826A72CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A72D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A72D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A72D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A72DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A72E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A72E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A72E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A72EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A72F0: 4BDBFB31  bl 0x82466e20
	ctx.lr = 0x826A72F4;
	sub_82466E20(ctx, base);
	// 826A72F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A72F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A72FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7308 size=108
    let mut pc: u32 = 0x826A7308;
    'dispatch: loop {
        match pc {
            0x826A7308 => {
    //   block [0x826A7308..0x826A7374)
	// 826A7308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A730C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7314: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A731C: 38EBC430  addi r7, r11, -0x3bd0
	ctx.r[7].s64 = ctx.r[11].s64 + -15312;
	// 826A7320: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A7324: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 826A7328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A732C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7330: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7338: 386A8B28  addi r3, r10, -0x74d8
	ctx.r[3].s64 = ctx.r[10].s64 + -29912;
	// 826A733C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A734C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A735C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7360: 4BDBFAC1  bl 0x82466e20
	ctx.lr = 0x826A7364;
	sub_82466E20(ctx, base);
	// 826A7364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A736C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7378 size=112
    let mut pc: u32 = 0x826A7378;
    'dispatch: loop {
        match pc {
            0x826A7378 => {
    //   block [0x826A7378..0x826A73E8)
	// 826A7378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A737C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7384: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7388: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A738C: 38AA8978  addi r5, r10, -0x7688
	ctx.r[5].s64 = ctx.r[10].s64 + -30344;
	// 826A7390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7394: 390BC460  addi r8, r11, -0x3ba0
	ctx.r[8].s64 = ctx.r[11].s64 + -15264;
	// 826A7398: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826A739C: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826A73A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A73A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A73A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A73AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A73B0: 386A8B58  addi r3, r10, -0x74a8
	ctx.r[3].s64 = ctx.r[10].s64 + -29864;
	// 826A73B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A73B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A73BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A73C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A73C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A73C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A73CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A73D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A73D4: 4BDBFA4D  bl 0x82466e20
	ctx.lr = 0x826A73D8;
	sub_82466E20(ctx, base);
	// 826A73D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A73DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A73E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A73E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A73E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A73E8 size=24
    let mut pc: u32 = 0x826A73E8;
    'dispatch: loop {
        match pc {
            0x826A73E8 => {
    //   block [0x826A73E8..0x826A7400)
	// 826A73E8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A73EC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A73F0: 394A3A00  addi r10, r10, 0x3a00
	ctx.r[10].s64 = ctx.r[10].s64 + 14848;
	// 826A73F4: 816BC33C  lwz r11, -0x3cc4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15556 as u32) ) } as u64;
	// 826A73F8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826A73FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7400 size=112
    let mut pc: u32 = 0x826A7400;
    'dispatch: loop {
        match pc {
            0x826A7400 => {
    //   block [0x826A7400..0x826A7470)
	// 826A7400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A740C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A7410: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7414: 392ACA6C  addi r9, r10, -0x3594
	ctx.r[9].s64 = ctx.r[10].s64 + -13716;
	// 826A7418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A741C: 390B3A00  addi r8, r11, 0x3a00
	ctx.r[8].s64 = ctx.r[11].s64 + 14848;
	// 826A7420: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826A7424: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 826A7428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A742C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A7434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7438: 386A8B88  addi r3, r10, -0x7478
	ctx.r[3].s64 = ctx.r[10].s64 + -29816;
	// 826A743C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A7440: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A7444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A744C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A745C: 4BDBF9C5  bl 0x82466e20
	ctx.lr = 0x826A7460;
	sub_82466E20(ctx, base);
	// 826A7460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A746C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7470 size=108
    let mut pc: u32 = 0x826A7470;
    'dispatch: loop {
        match pc {
            0x826A7470 => {
    //   block [0x826A7470..0x826A74DC)
	// 826A7470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A747C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7484: 38EBC50C  addi r7, r11, -0x3af4
	ctx.r[7].s64 = ctx.r[11].s64 + -15092;
	// 826A7488: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A748C: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 826A7490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7494: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A749C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A74A0: 386A8BB8  addi r3, r10, -0x7448
	ctx.r[3].s64 = ctx.r[10].s64 + -29768;
	// 826A74A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A74A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A74AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A74B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A74B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A74B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A74BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A74C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A74C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A74C8: 4BDBF959  bl 0x82466e20
	ctx.lr = 0x826A74CC;
	sub_82466E20(ctx, base);
	// 826A74CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A74D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A74D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A74D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A74E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A74E0 size=116
    let mut pc: u32 = 0x826A74E0;
    'dispatch: loop {
        match pc {
            0x826A74E0 => {
    //   block [0x826A74E0..0x826A7554)
	// 826A74E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A74E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A74E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A74EC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A74F0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A74F4: 390BC540  addi r8, r11, -0x3ac0
	ctx.r[8].s64 = ctx.r[11].s64 + -15040;
	// 826A74F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A74FC: 392ACAB0  addi r9, r10, -0x3550
	ctx.r[9].s64 = ctx.r[10].s64 + -13648;
	// 826A7500: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7504: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826A7508: 38AA8978  addi r5, r10, -0x7688
	ctx.r[5].s64 = ctx.r[10].s64 + -30344;
	// 826A750C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A7510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7514: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A751C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7524: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826A7528: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 826A752C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A7530: 386B8BE8  addi r3, r11, -0x7418
	ctx.r[3].s64 = ctx.r[11].s64 + -29720;
	// 826A7534: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A7538: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A753C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7540: 4BDBF8E1  bl 0x82466e20
	ctx.lr = 0x826A7544;
	sub_82466E20(ctx, base);
	// 826A7544: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A754C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A7558 size=24
    let mut pc: u32 = 0x826A7558;
    'dispatch: loop {
        match pc {
            0x826A7558 => {
    //   block [0x826A7558..0x826A7570)
	// 826A7558: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A755C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A7560: 394A3A78  addi r10, r10, 0x3a78
	ctx.r[10].s64 = ctx.r[10].s64 + 14968;
	// 826A7564: 816BC53C  lwz r11, -0x3ac4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15044 as u32) ) } as u64;
	// 826A7568: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826A756C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7570 size=112
    let mut pc: u32 = 0x826A7570;
    'dispatch: loop {
        match pc {
            0x826A7570 => {
    //   block [0x826A7570..0x826A75E0)
	// 826A7570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A757C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A7580: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7584: 392ACAEC  addi r9, r10, -0x3514
	ctx.r[9].s64 = ctx.r[10].s64 + -13588;
	// 826A7588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A758C: 390B3A78  addi r8, r11, 0x3a78
	ctx.r[8].s64 = ctx.r[11].s64 + 14968;
	// 826A7590: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826A7594: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 826A7598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A759C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A75A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A75A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A75A8: 386A8C18  addi r3, r10, -0x73e8
	ctx.r[3].s64 = ctx.r[10].s64 + -29672;
	// 826A75AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A75B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A75B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A75B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A75BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A75C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A75C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A75C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A75CC: 4BDBF855  bl 0x82466e20
	ctx.lr = 0x826A75D0;
	sub_82466E20(ctx, base);
	// 826A75D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A75D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A75D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A75DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A75E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A75E0 size=108
    let mut pc: u32 = 0x826A75E0;
    'dispatch: loop {
        match pc {
            0x826A75E0 => {
    //   block [0x826A75E0..0x826A764C)
	// 826A75E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A75E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A75E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A75EC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A75F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A75F4: 38EBC600  addi r7, r11, -0x3a00
	ctx.r[7].s64 = ctx.r[11].s64 + -14848;
	// 826A75F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A75FC: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826A7600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7604: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A760C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7610: 386A8C48  addi r3, r10, -0x73b8
	ctx.r[3].s64 = ctx.r[10].s64 + -29624;
	// 826A7614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A761C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A762C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7638: 4BDBF7E9  bl 0x82466e20
	ctx.lr = 0x826A763C;
	sub_82466E20(ctx, base);
	// 826A763C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7650 size=108
    let mut pc: u32 = 0x826A7650;
    'dispatch: loop {
        match pc {
            0x826A7650 => {
    //   block [0x826A7650..0x826A76BC)
	// 826A7650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A765C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7664: 38EBC618  addi r7, r11, -0x39e8
	ctx.r[7].s64 = ctx.r[11].s64 + -14824;
	// 826A7668: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A766C: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 826A7670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7674: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A767C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7680: 386A8C78  addi r3, r10, -0x7388
	ctx.r[3].s64 = ctx.r[10].s64 + -29576;
	// 826A7684: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A768C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A769C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A76A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A76A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A76A8: 4BDBF779  bl 0x82466e20
	ctx.lr = 0x826A76AC;
	sub_82466E20(ctx, base);
	// 826A76AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A76B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A76B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A76B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A76C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A76C0 size=24
    let mut pc: u32 = 0x826A76C0;
    'dispatch: loop {
        match pc {
            0x826A76C0 => {
    //   block [0x826A76C0..0x826A76D8)
	// 826A76C0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A76C4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A76C8: 394A3AC0  addi r10, r10, 0x3ac0
	ctx.r[10].s64 = ctx.r[10].s64 + 15040;
	// 826A76CC: 816BC648  lwz r11, -0x39b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14776 as u32) ) } as u64;
	// 826A76D0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826A76D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A76D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A76D8 size=112
    let mut pc: u32 = 0x826A76D8;
    'dispatch: loop {
        match pc {
            0x826A76D8 => {
    //   block [0x826A76D8..0x826A7748)
	// 826A76D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A76DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A76E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A76E4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A76E8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A76EC: 392ACB28  addi r9, r10, -0x34d8
	ctx.r[9].s64 = ctx.r[10].s64 + -13528;
	// 826A76F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A76F4: 390B3AC0  addi r8, r11, 0x3ac0
	ctx.r[8].s64 = ctx.r[11].s64 + 15040;
	// 826A76F8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826A76FC: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826A7700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7704: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A770C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7710: 386A8CA8  addi r3, r10, -0x7358
	ctx.r[3].s64 = ctx.r[10].s64 + -29528;
	// 826A7714: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A7718: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A771C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A772C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7734: 4BDBF6ED  bl 0x82466e20
	ctx.lr = 0x826A7738;
	sub_82466E20(ctx, base);
	// 826A7738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A773C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7748 size=112
    let mut pc: u32 = 0x826A7748;
    'dispatch: loop {
        match pc {
            0x826A7748 => {
    //   block [0x826A7748..0x826A77B8)
	// 826A7748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A774C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7754: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7758: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A775C: 38AA8978  addi r5, r10, -0x7688
	ctx.r[5].s64 = ctx.r[10].s64 + -30344;
	// 826A7760: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7764: 390BC64C  addi r8, r11, -0x39b4
	ctx.r[8].s64 = ctx.r[11].s64 + -14772;
	// 826A7768: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A776C: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 826A7770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7774: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7778: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A777C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7780: 386A8CD8  addi r3, r10, -0x7328
	ctx.r[3].s64 = ctx.r[10].s64 + -29480;
	// 826A7784: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A7788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A778C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A779C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A77A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A77A4: 4BDBF67D  bl 0x82466e20
	ctx.lr = 0x826A77A8;
	sub_82466E20(ctx, base);
	// 826A77A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A77AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A77B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A77B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A77B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A77B8 size=108
    let mut pc: u32 = 0x826A77B8;
    'dispatch: loop {
        match pc {
            0x826A77B8 => {
    //   block [0x826A77B8..0x826A7824)
	// 826A77B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A77BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A77C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A77C4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A77C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A77CC: 38EBC67C  addi r7, r11, -0x3984
	ctx.r[7].s64 = ctx.r[11].s64 + -14724;
	// 826A77D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A77D4: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826A77D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A77DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A77E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A77E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A77E8: 386A8D08  addi r3, r10, -0x72f8
	ctx.r[3].s64 = ctx.r[10].s64 + -29432;
	// 826A77EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A77F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A77F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A77F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A77FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A780C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7810: 4BDBF611  bl 0x82466e20
	ctx.lr = 0x826A7814;
	sub_82466E20(ctx, base);
	// 826A7814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A781C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7828 size=108
    let mut pc: u32 = 0x826A7828;
    'dispatch: loop {
        match pc {
            0x826A7828 => {
    //   block [0x826A7828..0x826A7894)
	// 826A7828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A782C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7834: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A783C: 38EBC6B0  addi r7, r11, -0x3950
	ctx.r[7].s64 = ctx.r[11].s64 + -14672;
	// 826A7840: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A7844: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826A7848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A784C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7858: 386A8D38  addi r3, r10, -0x72c8
	ctx.r[3].s64 = ctx.r[10].s64 + -29384;
	// 826A785C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A786C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A787C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7880: 4BDBF5A1  bl 0x82466e20
	ctx.lr = 0x826A7884;
	sub_82466E20(ctx, base);
	// 826A7884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A788C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7898 size=108
    let mut pc: u32 = 0x826A7898;
    'dispatch: loop {
        match pc {
            0x826A7898 => {
    //   block [0x826A7898..0x826A7904)
	// 826A7898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A789C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A78A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A78A4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A78A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A78AC: 38EBC710  addi r7, r11, -0x38f0
	ctx.r[7].s64 = ctx.r[11].s64 + -14576;
	// 826A78B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A78B4: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826A78B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A78BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A78C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A78C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A78C8: 386A8D68  addi r3, r10, -0x7298
	ctx.r[3].s64 = ctx.r[10].s64 + -29336;
	// 826A78CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A78D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A78D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A78D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A78DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A78E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A78E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A78E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A78EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A78F0: 4BDBF531  bl 0x82466e20
	ctx.lr = 0x826A78F4;
	sub_82466E20(ctx, base);
	// 826A78F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A78F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A78FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7908 size=108
    let mut pc: u32 = 0x826A7908;
    'dispatch: loop {
        match pc {
            0x826A7908 => {
    //   block [0x826A7908..0x826A7974)
	// 826A7908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A790C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7914: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A791C: 38EBC740  addi r7, r11, -0x38c0
	ctx.r[7].s64 = ctx.r[11].s64 + -14528;
	// 826A7920: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826A7924: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 826A7928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A792C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7930: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7938: 386A8D98  addi r3, r10, -0x7268
	ctx.r[3].s64 = ctx.r[10].s64 + -29288;
	// 826A793C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A794C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A795C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7960: 4BDBF4C1  bl 0x82466e20
	ctx.lr = 0x826A7964;
	sub_82466E20(ctx, base);
	// 826A7964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A796C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7978 size=108
    let mut pc: u32 = 0x826A7978;
    'dispatch: loop {
        match pc {
            0x826A7978 => {
    //   block [0x826A7978..0x826A79E4)
	// 826A7978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A797C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7984: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A798C: 38EBC860  addi r7, r11, -0x37a0
	ctx.r[7].s64 = ctx.r[11].s64 + -14240;
	// 826A7990: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A7994: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 826A7998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A799C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A79A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A79A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A79A8: 386A8DC8  addi r3, r10, -0x7238
	ctx.r[3].s64 = ctx.r[10].s64 + -29240;
	// 826A79AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A79B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A79B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A79B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A79BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A79C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A79C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A79C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A79CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A79D0: 4BDBF451  bl 0x82466e20
	ctx.lr = 0x826A79D4;
	sub_82466E20(ctx, base);
	// 826A79D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A79D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A79DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A79E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A79E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A79E8 size=108
    let mut pc: u32 = 0x826A79E8;
    'dispatch: loop {
        match pc {
            0x826A79E8 => {
    //   block [0x826A79E8..0x826A7A54)
	// 826A79E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A79EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A79F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A79F4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A79F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A79FC: 38EBC878  addi r7, r11, -0x3788
	ctx.r[7].s64 = ctx.r[11].s64 + -14216;
	// 826A7A00: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A7A04: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 826A7A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7A0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7A10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7A18: 386A8DF8  addi r3, r10, -0x7208
	ctx.r[3].s64 = ctx.r[10].s64 + -29192;
	// 826A7A1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7A3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7A40: 4BDBF3E1  bl 0x82466e20
	ctx.lr = 0x826A7A44;
	sub_82466E20(ctx, base);
	// 826A7A44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7A58 size=108
    let mut pc: u32 = 0x826A7A58;
    'dispatch: loop {
        match pc {
            0x826A7A58 => {
    //   block [0x826A7A58..0x826A7AC4)
	// 826A7A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7A64: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7A68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7A6C: 38EBC890  addi r7, r11, -0x3770
	ctx.r[7].s64 = ctx.r[11].s64 + -14192;
	// 826A7A70: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A7A74: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 826A7A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7A7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7A80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7A88: 386A8E28  addi r3, r10, -0x71d8
	ctx.r[3].s64 = ctx.r[10].s64 + -29144;
	// 826A7A8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7A94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7AAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7AB0: 4BDBF371  bl 0x82466e20
	ctx.lr = 0x826A7AB4;
	sub_82466E20(ctx, base);
	// 826A7AB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7AB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7ABC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7AC8 size=108
    let mut pc: u32 = 0x826A7AC8;
    'dispatch: loop {
        match pc {
            0x826A7AC8 => {
    //   block [0x826A7AC8..0x826A7B34)
	// 826A7AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7AD4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7AD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7ADC: 38EBC8A8  addi r7, r11, -0x3758
	ctx.r[7].s64 = ctx.r[11].s64 + -14168;
	// 826A7AE0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A7AE4: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 826A7AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7AEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7AF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7AF8: 386A8E58  addi r3, r10, -0x71a8
	ctx.r[3].s64 = ctx.r[10].s64 + -29096;
	// 826A7AFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7B1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7B20: 4BDBF301  bl 0x82466e20
	ctx.lr = 0x826A7B24;
	sub_82466E20(ctx, base);
	// 826A7B24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7B38 size=108
    let mut pc: u32 = 0x826A7B38;
    'dispatch: loop {
        match pc {
            0x826A7B38 => {
    //   block [0x826A7B38..0x826A7BA4)
	// 826A7B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7B44: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7B4C: 38EBC8C0  addi r7, r11, -0x3740
	ctx.r[7].s64 = ctx.r[11].s64 + -14144;
	// 826A7B50: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A7B54: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 826A7B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7B5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7B60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7B64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7B68: 386A8E88  addi r3, r10, -0x7178
	ctx.r[3].s64 = ctx.r[10].s64 + -29048;
	// 826A7B6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7B70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7B74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7B8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7B90: 4BDBF291  bl 0x82466e20
	ctx.lr = 0x826A7B94;
	sub_82466E20(ctx, base);
	// 826A7B94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7BA8 size=108
    let mut pc: u32 = 0x826A7BA8;
    'dispatch: loop {
        match pc {
            0x826A7BA8 => {
    //   block [0x826A7BA8..0x826A7C14)
	// 826A7BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7BB4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7BBC: 38EBC8D8  addi r7, r11, -0x3728
	ctx.r[7].s64 = ctx.r[11].s64 + -14120;
	// 826A7BC0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A7BC4: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 826A7BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7BCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7BD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7BD8: 386A8EB8  addi r3, r10, -0x7148
	ctx.r[3].s64 = ctx.r[10].s64 + -29000;
	// 826A7BDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7BFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7C00: 4BDBF221  bl 0x82466e20
	ctx.lr = 0x826A7C04;
	sub_82466E20(ctx, base);
	// 826A7C04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7C18 size=108
    let mut pc: u32 = 0x826A7C18;
    'dispatch: loop {
        match pc {
            0x826A7C18 => {
    //   block [0x826A7C18..0x826A7C84)
	// 826A7C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7C24: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7C28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7C2C: 38EBC8F0  addi r7, r11, -0x3710
	ctx.r[7].s64 = ctx.r[11].s64 + -14096;
	// 826A7C30: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826A7C34: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 826A7C38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7C3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7C40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7C44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7C48: 386A8EE8  addi r3, r10, -0x7118
	ctx.r[3].s64 = ctx.r[10].s64 + -28952;
	// 826A7C4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7C50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7C54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7C58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7C60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7C64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7C68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7C6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7C70: 4BDBF1B1  bl 0x82466e20
	ctx.lr = 0x826A7C74;
	sub_82466E20(ctx, base);
	// 826A7C74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7C88 size=108
    let mut pc: u32 = 0x826A7C88;
    'dispatch: loop {
        match pc {
            0x826A7C88 => {
    //   block [0x826A7C88..0x826A7CF4)
	// 826A7C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7C94: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7C98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7C9C: 38EBC980  addi r7, r11, -0x3680
	ctx.r[7].s64 = ctx.r[11].s64 + -13952;
	// 826A7CA0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826A7CA4: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 826A7CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7CAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7CB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7CB8: 386A8F18  addi r3, r10, -0x70e8
	ctx.r[3].s64 = ctx.r[10].s64 + -28904;
	// 826A7CBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7CC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7CC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7CCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7CD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7CD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7CDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7CE0: 4BDBF141  bl 0x82466e20
	ctx.lr = 0x826A7CE4;
	sub_82466E20(ctx, base);
	// 826A7CE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7CE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7CEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7CF8 size=108
    let mut pc: u32 = 0x826A7CF8;
    'dispatch: loop {
        match pc {
            0x826A7CF8 => {
    //   block [0x826A7CF8..0x826A7D64)
	// 826A7CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7D04: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7D08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7D0C: 38EBCA40  addi r7, r11, -0x35c0
	ctx.r[7].s64 = ctx.r[11].s64 + -13760;
	// 826A7D10: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826A7D14: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 826A7D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7D1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7D20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7D28: 386A8F48  addi r3, r10, -0x70b8
	ctx.r[3].s64 = ctx.r[10].s64 + -28856;
	// 826A7D2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7D30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7D38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7D40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7D44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7D48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7D4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7D50: 4BDBF0D1  bl 0x82466e20
	ctx.lr = 0x826A7D54;
	sub_82466E20(ctx, base);
	// 826A7D54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7D68 size=108
    let mut pc: u32 = 0x826A7D68;
    'dispatch: loop {
        match pc {
            0x826A7D68 => {
    //   block [0x826A7D68..0x826A7DD4)
	// 826A7D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7D74: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7D78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7D7C: 38EBCB18  addi r7, r11, -0x34e8
	ctx.r[7].s64 = ctx.r[11].s64 + -13544;
	// 826A7D80: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826A7D84: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826A7D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7D8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7D90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7D98: 386A8F78  addi r3, r10, -0x7088
	ctx.r[3].s64 = ctx.r[10].s64 + -28808;
	// 826A7D9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7DA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7DAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7DB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7DBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7DC0: 4BDBF061  bl 0x82466e20
	ctx.lr = 0x826A7DC4;
	sub_82466E20(ctx, base);
	// 826A7DC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7DD8 size=108
    let mut pc: u32 = 0x826A7DD8;
    'dispatch: loop {
        match pc {
            0x826A7DD8 => {
    //   block [0x826A7DD8..0x826A7E44)
	// 826A7DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7DE4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7DEC: 38EBCBD8  addi r7, r11, -0x3428
	ctx.r[7].s64 = ctx.r[11].s64 + -13352;
	// 826A7DF0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826A7DF4: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 826A7DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7DFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7E00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7E08: 386A8FA8  addi r3, r10, -0x7058
	ctx.r[3].s64 = ctx.r[10].s64 + -28760;
	// 826A7E0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7E1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7E2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7E30: 4BDBEFF1  bl 0x82466e20
	ctx.lr = 0x826A7E34;
	sub_82466E20(ctx, base);
	// 826A7E34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7E48 size=112
    let mut pc: u32 = 0x826A7E48;
    'dispatch: loop {
        match pc {
            0x826A7E48 => {
    //   block [0x826A7E48..0x826A7EB8)
	// 826A7E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7E54: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A7E58: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826A7E5C: 38EACC80  addi r7, r10, -0x3380
	ctx.r[7].s64 = ctx.r[10].s64 + -13184;
	// 826A7E60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7E64: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A7E68: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 826A7E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7E70: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7E74: 396BCB40  addi r11, r11, -0x34c0
	ctx.r[11].s64 = ctx.r[11].s64 + -13504;
	// 826A7E78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7E7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7E80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7E84: 386A8FD8  addi r3, r10, -0x7028
	ctx.r[3].s64 = ctx.r[10].s64 + -28712;
	// 826A7E88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7E8C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A7E90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7E94: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A7E98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7E9C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7EA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7EA4: 4BDBEF7D  bl 0x82466e20
	ctx.lr = 0x826A7EA8;
	sub_82466E20(ctx, base);
	// 826A7EA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7EB8 size=112
    let mut pc: u32 = 0x826A7EB8;
    'dispatch: loop {
        match pc {
            0x826A7EB8 => {
    //   block [0x826A7EB8..0x826A7F28)
	// 826A7EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7EC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7EC8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7ECC: 38AA8978  addi r5, r10, -0x7688
	ctx.r[5].s64 = ctx.r[10].s64 + -30344;
	// 826A7ED0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7ED4: 390BCDB8  addi r8, r11, -0x3248
	ctx.r[8].s64 = ctx.r[11].s64 + -12872;
	// 826A7ED8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A7EDC: 388AA3A4  addi r4, r10, -0x5c5c
	ctx.r[4].s64 = ctx.r[10].s64 + -23644;
	// 826A7EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7EE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7EE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A7EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7EF0: 386A9008  addi r3, r10, -0x6ff8
	ctx.r[3].s64 = ctx.r[10].s64 + -28664;
	// 826A7EF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A7EF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7EFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7F00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7F04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7F08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7F0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7F10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7F14: 4BDBEF0D  bl 0x82466e20
	ctx.lr = 0x826A7F18;
	sub_82466E20(ctx, base);
	// 826A7F18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7F28 size=108
    let mut pc: u32 = 0x826A7F28;
    'dispatch: loop {
        match pc {
            0x826A7F28 => {
    //   block [0x826A7F28..0x826A7F94)
	// 826A7F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7F34: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7F38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7F3C: 38EBCDE8  addi r7, r11, -0x3218
	ctx.r[7].s64 = ctx.r[11].s64 + -12824;
	// 826A7F40: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A7F44: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 826A7F48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7F4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7F50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7F58: 386A9038  addi r3, r10, -0x6fc8
	ctx.r[3].s64 = ctx.r[10].s64 + -28616;
	// 826A7F5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7F60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7F68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7F70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7F74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7F78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7F7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7F80: 4BDBEEA1  bl 0x82466e20
	ctx.lr = 0x826A7F84;
	sub_82466E20(ctx, base);
	// 826A7F84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7F98 size=108
    let mut pc: u32 = 0x826A7F98;
    'dispatch: loop {
        match pc {
            0x826A7F98 => {
    //   block [0x826A7F98..0x826A8004)
	// 826A7F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7FA4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7FA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7FAC: 38EBCE48  addi r7, r11, -0x31b8
	ctx.r[7].s64 = ctx.r[11].s64 + -12728;
	// 826A7FB0: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826A7FB4: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 826A7FB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7FBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7FC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7FC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7FC8: 386A9068  addi r3, r10, -0x6f98
	ctx.r[3].s64 = ctx.r[10].s64 + -28568;
	// 826A7FCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7FD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7FD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7FDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7FE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7FE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7FE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7FEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7FF0: 4BDBEE31  bl 0x82466e20
	ctx.lr = 0x826A7FF4;
	sub_82466E20(ctx, base);
	// 826A7FF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8008 size=108
    let mut pc: u32 = 0x826A8008;
    'dispatch: loop {
        match pc {
            0x826A8008 => {
    //   block [0x826A8008..0x826A8074)
	// 826A8008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A800C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8014: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A801C: 38EBCF50  addi r7, r11, -0x30b0
	ctx.r[7].s64 = ctx.r[11].s64 + -12464;
	// 826A8020: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826A8024: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826A8028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A802C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8030: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A8034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8038: 386A9098  addi r3, r10, -0x6f68
	ctx.r[3].s64 = ctx.r[10].s64 + -28520;
	// 826A803C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A8040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A804C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A805C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A8060: 4BDBEDC1  bl 0x82466e20
	ctx.lr = 0x826A8064;
	sub_82466E20(ctx, base);
	// 826A8064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A806C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8078 size=108
    let mut pc: u32 = 0x826A8078;
    'dispatch: loop {
        match pc {
            0x826A8078 => {
    //   block [0x826A8078..0x826A80E4)
	// 826A8078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A807C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8084: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A808C: 38EBD028  addi r7, r11, -0x2fd8
	ctx.r[7].s64 = ctx.r[11].s64 + -12248;
	// 826A8090: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A8094: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 826A8098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A809C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A80A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A80A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A80A8: 386A90C8  addi r3, r10, -0x6f38
	ctx.r[3].s64 = ctx.r[10].s64 + -28472;
	// 826A80AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A80B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A80B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A80B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A80BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A80C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A80C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A80C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A80CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A80D0: 4BDBED51  bl 0x82466e20
	ctx.lr = 0x826A80D4;
	sub_82466E20(ctx, base);
	// 826A80D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A80D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A80DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A80E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A80E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A80E8 size=108
    let mut pc: u32 = 0x826A80E8;
    'dispatch: loop {
        match pc {
            0x826A80E8 => {
    //   block [0x826A80E8..0x826A8154)
	// 826A80E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A80EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A80F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A80F4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A80F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A80FC: 38EBD070  addi r7, r11, -0x2f90
	ctx.r[7].s64 = ctx.r[11].s64 + -12176;
	// 826A8100: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A8104: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826A8108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A810C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8110: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A8114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8118: 386A90F8  addi r3, r10, -0x6f08
	ctx.r[3].s64 = ctx.r[10].s64 + -28424;
	// 826A811C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A8120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A812C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A813C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A8140: 4BDBECE1  bl 0x82466e20
	ctx.lr = 0x826A8144;
	sub_82466E20(ctx, base);
	// 826A8144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A814C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8158 size=108
    let mut pc: u32 = 0x826A8158;
    'dispatch: loop {
        match pc {
            0x826A8158 => {
    //   block [0x826A8158..0x826A81C4)
	// 826A8158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A815C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8164: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8168: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A816C: 38EBD088  addi r7, r11, -0x2f78
	ctx.r[7].s64 = ctx.r[11].s64 + -12152;
	// 826A8170: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A8174: 388AB3B4  addi r4, r10, -0x4c4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19532;
	// 826A8178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A817C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8180: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A8184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8188: 386A9128  addi r3, r10, -0x6ed8
	ctx.r[3].s64 = ctx.r[10].s64 + -28376;
	// 826A818C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A8190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A819C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A81A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A81A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A81A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A81AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A81B0: 4BDBEC71  bl 0x82466e20
	ctx.lr = 0x826A81B4;
	sub_82466E20(ctx, base);
	// 826A81B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A81B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A81BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A81C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A81C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A81C8 size=108
    let mut pc: u32 = 0x826A81C8;
    'dispatch: loop {
        match pc {
            0x826A81C8 => {
    //   block [0x826A81C8..0x826A8234)
	// 826A81C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A81CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A81D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A81D4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A81D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A81DC: 38EBD0E8  addi r7, r11, -0x2f18
	ctx.r[7].s64 = ctx.r[11].s64 + -12056;
	// 826A81E0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826A81E4: 388AB3C0  addi r4, r10, -0x4c40
	ctx.r[4].s64 = ctx.r[10].s64 + -19520;
	// 826A81E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A81EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A81F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A81F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A81F8: 386A9158  addi r3, r10, -0x6ea8
	ctx.r[3].s64 = ctx.r[10].s64 + -28328;
	// 826A81FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A8200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A820C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A821C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A8220: 4BDBEC01  bl 0x82466e20
	ctx.lr = 0x826A8224;
	sub_82466E20(ctx, base);
	// 826A8224: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A822C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8238 size=116
    let mut pc: u32 = 0x826A8238;
    'dispatch: loop {
        match pc {
            0x826A8238 => {
    //   block [0x826A8238..0x826A82AC)
	// 826A8238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A823C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8244: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8248: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A824C: 390BD1A8  addi r8, r11, -0x2e58
	ctx.r[8].s64 = ctx.r[11].s64 + -11864;
	// 826A8250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8254: 392ACBBC  addi r9, r10, -0x3444
	ctx.r[9].s64 = ctx.r[10].s64 + -13380;
	// 826A8258: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A825C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826A8260: 38AA9128  addi r5, r10, -0x6ed8
	ctx.r[5].s64 = ctx.r[10].s64 + -28376;
	// 826A8264: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A826C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A827C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826A8280: 388AB3E0  addi r4, r10, -0x4c20
	ctx.r[4].s64 = ctx.r[10].s64 + -19488;
	// 826A8284: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A8288: 386B9188  addi r3, r11, -0x6e78
	ctx.r[3].s64 = ctx.r[11].s64 + -28280;
	// 826A828C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A8290: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8298: 4BDBEB89  bl 0x82466e20
	ctx.lr = 0x826A829C;
	sub_82466E20(ctx, base);
	// 826A829C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A82A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A82A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A82A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A82B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A82B0 size=112
    let mut pc: u32 = 0x826A82B0;
    'dispatch: loop {
        match pc {
            0x826A82B0 => {
    //   block [0x826A82B0..0x826A8320)
	// 826A82B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A82B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A82B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A82BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A82C0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A82C4: 38AAB468  addi r5, r10, -0x4b98
	ctx.r[5].s64 = ctx.r[10].s64 + -19352;
	// 826A82C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A82CC: 390BD238  addi r8, r11, -0x2dc8
	ctx.r[8].s64 = ctx.r[11].s64 + -11720;
	// 826A82D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826A82D4: 388AB3F0  addi r4, r10, -0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + -19472;
	// 826A82D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A82DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A82E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A82E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A82E8: 386A91B8  addi r3, r10, -0x6e48
	ctx.r[3].s64 = ctx.r[10].s64 + -28232;
	// 826A82EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A82F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A82F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A82F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A82FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A830C: 4BDBEB15  bl 0x82466e20
	ctx.lr = 0x826A8310;
	sub_82466E20(ctx, base);
	// 826A8310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A831C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8320 size=96
    let mut pc: u32 = 0x826A8320;
    'dispatch: loop {
        match pc {
            0x826A8320 => {
    //   block [0x826A8320..0x826A8380)
	// 826A8320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A832C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8334: 388AB40C  addi r4, r10, -0x4bf4
	ctx.r[4].s64 = ctx.r[10].s64 + -19444;
	// 826A8338: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A833C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8340: 386A91E8  addi r3, r10, -0x6e18
	ctx.r[3].s64 = ctx.r[10].s64 + -28184;
	// 826A8344: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A834C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A8350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A835C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8360: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A8364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A8368: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A836C: 4BDBEAB5  bl 0x82466e20
	ctx.lr = 0x826A8370;
	sub_82466E20(ctx, base);
	// 826A8370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A837C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A8380 size=24
    let mut pc: u32 = 0x826A8380;
    'dispatch: loop {
        match pc {
            0x826A8380 => {
    //   block [0x826A8380..0x826A8398)
	// 826A8380: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8384: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A8388: 394A3B38  addi r10, r10, 0x3b38
	ctx.r[10].s64 = ctx.r[10].s64 + 15160;
	// 826A838C: 816BD298  lwz r11, -0x2d68(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11624 as u32) ) } as u64;
	// 826A8390: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826A8394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8398 size=116
    let mut pc: u32 = 0x826A8398;
    'dispatch: loop {
        match pc {
            0x826A8398 => {
    //   block [0x826A8398..0x826A840C)
	// 826A8398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A839C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A83A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A83A4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A83A8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A83AC: 390B3B38  addi r8, r11, 0x3b38
	ctx.r[8].s64 = ctx.r[11].s64 + 15160;
	// 826A83B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A83B4: 392ACC08  addi r9, r10, -0x33f8
	ctx.r[9].s64 = ctx.r[10].s64 + -13304;
	// 826A83B8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A83BC: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826A83C0: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A83C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A83C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A83CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A83D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A83D4: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A83D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A83DC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826A83E0: 388AB42C  addi r4, r10, -0x4bd4
	ctx.r[4].s64 = ctx.r[10].s64 + -19412;
	// 826A83E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A83E8: 386B9218  addi r3, r11, -0x6de8
	ctx.r[3].s64 = ctx.r[11].s64 + -28136;
	// 826A83EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A83F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A83F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A83F8: 4BDBEA29  bl 0x82466e20
	ctx.lr = 0x826A83FC;
	sub_82466E20(ctx, base);
	// 826A83FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8410 size=104
    let mut pc: u32 = 0x826A8410;
    'dispatch: loop {
        match pc {
            0x826A8410 => {
    //   block [0x826A8410..0x826A8478)
	// 826A8410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A841C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A8420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8424: 392ACC34  addi r9, r10, -0x33cc
	ctx.r[9].s64 = ctx.r[10].s64 + -13260;
	// 826A8428: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A842C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8430: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A8434: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A843C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8444: 388AB440  addi r4, r10, -0x4bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -19392;
	// 826A8448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A844C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8450: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A8454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8458: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A845C: 386A9248  addi r3, r10, -0x6db8
	ctx.r[3].s64 = ctx.r[10].s64 + -28088;
	// 826A8460: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A8464: 4BDBE9BD  bl 0x82466e20
	ctx.lr = 0x826A8468;
	sub_82466E20(ctx, base);
	// 826A8468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A846C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8478 size=96
    let mut pc: u32 = 0x826A8478;
    'dispatch: loop {
        match pc {
            0x826A8478 => {
    //   block [0x826A8478..0x826A84D8)
	// 826A8478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A847C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8484: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A848C: 388AB454  addi r4, r10, -0x4bac
	ctx.r[4].s64 = ctx.r[10].s64 + -19372;
	// 826A8490: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8498: 386A9278  addi r3, r10, -0x6d88
	ctx.r[3].s64 = ctx.r[10].s64 + -28040;
	// 826A849C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A84A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A84A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A84A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A84AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A84B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A84B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A84B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A84BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A84C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A84C4: 4BDBE95D  bl 0x82466e20
	ctx.lr = 0x826A84C8;
	sub_82466E20(ctx, base);
	// 826A84C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A84CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A84D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A84D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A84D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A84D8 size=96
    let mut pc: u32 = 0x826A84D8;
    'dispatch: loop {
        match pc {
            0x826A84D8 => {
    //   block [0x826A84D8..0x826A8538)
	// 826A84D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A84DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A84E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A84E4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A84E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A84EC: 388AB46C  addi r4, r10, -0x4b94
	ctx.r[4].s64 = ctx.r[10].s64 + -19348;
	// 826A84F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A84F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A84F8: 386A92A8  addi r3, r10, -0x6d58
	ctx.r[3].s64 = ctx.r[10].s64 + -27992;
	// 826A84FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8504: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A8508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A850C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8518: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A851C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A8520: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A8524: 4BDBE8FD  bl 0x82466e20
	ctx.lr = 0x826A8528;
	sub_82466E20(ctx, base);
	// 826A8528: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A852C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8538 size=100
    let mut pc: u32 = 0x826A8538;
    'dispatch: loop {
        match pc {
            0x826A8538 => {
    //   block [0x826A8538..0x826A859C)
	// 826A8538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A853C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8544: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A854C: 38AA9248  addi r5, r10, -0x6db8
	ctx.r[5].s64 = ctx.r[10].s64 + -28088;
	// 826A8550: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8558: 388AB488  addi r4, r10, -0x4b78
	ctx.r[4].s64 = ctx.r[10].s64 + -19320;
	// 826A855C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A856C: 386A92D8  addi r3, r10, -0x6d28
	ctx.r[3].s64 = ctx.r[10].s64 + -27944;
	// 826A8570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8574: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8578: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A857C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8580: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A8584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8588: 4BDBE899  bl 0x82466e20
	ctx.lr = 0x826A858C;
	sub_82466E20(ctx, base);
	// 826A858C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A85A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A85A0 size=112
    let mut pc: u32 = 0x826A85A0;
    'dispatch: loop {
        match pc {
            0x826A85A0 => {
    //   block [0x826A85A0..0x826A8610)
	// 826A85A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A85A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A85A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A85AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A85B0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A85B4: 38AA9218  addi r5, r10, -0x6de8
	ctx.r[5].s64 = ctx.r[10].s64 + -28136;
	// 826A85B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A85BC: 390BD2A0  addi r8, r11, -0x2d60
	ctx.r[8].s64 = ctx.r[11].s64 + -11616;
	// 826A85C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A85C4: 388AB4A4  addi r4, r10, -0x4b5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19292;
	// 826A85C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A85CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A85D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A85D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A85D8: 386A9308  addi r3, r10, -0x6cf8
	ctx.r[3].s64 = ctx.r[10].s64 + -27896;
	// 826A85DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A85E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A85E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A85E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A85EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A85F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A85F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A85F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A85FC: 4BDBE825  bl 0x82466e20
	ctx.lr = 0x826A8600;
	sub_82466E20(ctx, base);
	// 826A8600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A860C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8610 size=112
    let mut pc: u32 = 0x826A8610;
    'dispatch: loop {
        match pc {
            0x826A8610 => {
    //   block [0x826A8610..0x826A8680)
	// 826A8610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A861C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8620: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8624: 38AA9218  addi r5, r10, -0x6de8
	ctx.r[5].s64 = ctx.r[10].s64 + -28136;
	// 826A8628: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A862C: 390BD2E8  addi r8, r11, -0x2d18
	ctx.r[8].s64 = ctx.r[11].s64 + -11544;
	// 826A8630: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A8634: 388AB4B4  addi r4, r10, -0x4b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19276;
	// 826A8638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A863C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8648: 386A9338  addi r3, r10, -0x6cc8
	ctx.r[3].s64 = ctx.r[10].s64 + -27848;
	// 826A864C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A865C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A866C: 4BDBE7B5  bl 0x82466e20
	ctx.lr = 0x826A8670;
	sub_82466E20(ctx, base);
	// 826A8670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A867C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8680 size=100
    let mut pc: u32 = 0x826A8680;
    'dispatch: loop {
        match pc {
            0x826A8680 => {
    //   block [0x826A8680..0x826A86E4)
	// 826A8680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A868C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8694: 38AA9218  addi r5, r10, -0x6de8
	ctx.r[5].s64 = ctx.r[10].s64 + -28136;
	// 826A8698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A869C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A86A0: 388AB4CC  addi r4, r10, -0x4b34
	ctx.r[4].s64 = ctx.r[10].s64 + -19252;
	// 826A86A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A86A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A86AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A86B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A86B4: 386A9368  addi r3, r10, -0x6c98
	ctx.r[3].s64 = ctx.r[10].s64 + -27800;
	// 826A86B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A86BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A86C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A86C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A86C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A86CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A86D0: 4BDBE751  bl 0x82466e20
	ctx.lr = 0x826A86D4;
	sub_82466E20(ctx, base);
	// 826A86D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A86D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A86DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A86E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A86E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A86E8 size=112
    let mut pc: u32 = 0x826A86E8;
    'dispatch: loop {
        match pc {
            0x826A86E8 => {
    //   block [0x826A86E8..0x826A8758)
	// 826A86E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A86EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A86F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A86F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A86F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A86FC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A8700: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8704: 390BD300  addi r8, r11, -0x2d00
	ctx.r[8].s64 = ctx.r[11].s64 + -11520;
	// 826A8708: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A870C: 388AB4E4  addi r4, r10, -0x4b1c
	ctx.r[4].s64 = ctx.r[10].s64 + -19228;
	// 826A8710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8714: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A871C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8720: 386A9398  addi r3, r10, -0x6c68
	ctx.r[3].s64 = ctx.r[10].s64 + -27752;
	// 826A8724: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A872C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A873C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8744: 4BDBE6DD  bl 0x82466e20
	ctx.lr = 0x826A8748;
	sub_82466E20(ctx, base);
	// 826A8748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A874C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8758 size=96
    let mut pc: u32 = 0x826A8758;
    'dispatch: loop {
        match pc {
            0x826A8758 => {
    //   block [0x826A8758..0x826A87B8)
	// 826A8758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A875C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8764: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A876C: 388AB4F0  addi r4, r10, -0x4b10
	ctx.r[4].s64 = ctx.r[10].s64 + -19216;
	// 826A8770: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8778: 386A93C8  addi r3, r10, -0x6c38
	ctx.r[3].s64 = ctx.r[10].s64 + -27704;
	// 826A877C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8784: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A8788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A878C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8798: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A879C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A87A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A87A4: 4BDBE67D  bl 0x82466e20
	ctx.lr = 0x826A87A8;
	sub_82466E20(ctx, base);
	// 826A87A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A87AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A87B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A87B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A87B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A87B8 size=112
    let mut pc: u32 = 0x826A87B8;
    'dispatch: loop {
        match pc {
            0x826A87B8 => {
    //   block [0x826A87B8..0x826A8828)
	// 826A87B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A87BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A87C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A87C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A87C8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A87CC: 38AA93C8  addi r5, r10, -0x6c38
	ctx.r[5].s64 = ctx.r[10].s64 + -27704;
	// 826A87D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A87D4: 390BD330  addi r8, r11, -0x2cd0
	ctx.r[8].s64 = ctx.r[11].s64 + -11472;
	// 826A87D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A87DC: 388AB504  addi r4, r10, -0x4afc
	ctx.r[4].s64 = ctx.r[10].s64 + -19196;
	// 826A87E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A87E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A87E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A87EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A87F0: 386A93F8  addi r3, r10, -0x6c08
	ctx.r[3].s64 = ctx.r[10].s64 + -27656;
	// 826A87F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A87F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A87FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A880C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8814: 4BDBE60D  bl 0x82466e20
	ctx.lr = 0x826A8818;
	sub_82466E20(ctx, base);
	// 826A8818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A881C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8828 size=112
    let mut pc: u32 = 0x826A8828;
    'dispatch: loop {
        match pc {
            0x826A8828 => {
    //   block [0x826A8828..0x826A8898)
	// 826A8828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A882C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8834: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8838: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A883C: 38AA9398  addi r5, r10, -0x6c68
	ctx.r[5].s64 = ctx.r[10].s64 + -27752;
	// 826A8840: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8844: 390BD348  addi r8, r11, -0x2cb8
	ctx.r[8].s64 = ctx.r[11].s64 + -11448;
	// 826A8848: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A884C: 388AB51C  addi r4, r10, -0x4ae4
	ctx.r[4].s64 = ctx.r[10].s64 + -19172;
	// 826A8850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8854: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8858: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A885C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8860: 386A9428  addi r3, r10, -0x6bd8
	ctx.r[3].s64 = ctx.r[10].s64 + -27608;
	// 826A8864: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A886C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8874: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A8878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A887C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8884: 4BDBE59D  bl 0x82466e20
	ctx.lr = 0x826A8888;
	sub_82466E20(ctx, base);
	// 826A8888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A888C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A8898 size=36
    let mut pc: u32 = 0x826A8898;
    'dispatch: loop {
        match pc {
            0x826A8898 => {
    //   block [0x826A8898..0x826A88BC)
	// 826A8898: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A889C: 814BD368  lwz r10, -0x2c98(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11416 as u32) ) } as u64;
	// 826A88A0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A88A4: 396B3B80  addi r11, r11, 0x3b80
	ctx.r[11].s64 = ctx.r[11].s64 + 15232;
	// 826A88A8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826A88AC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A88B0: 814AD360  lwz r10, -0x2ca0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-11424 as u32) ) } as u64;
	// 826A88B4: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826A88B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A88C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A88C0 size=108
    let mut pc: u32 = 0x826A88C0;
    'dispatch: loop {
        match pc {
            0x826A88C0 => {
    //   block [0x826A88C0..0x826A892C)
	// 826A88C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A88C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A88C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A88CC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A88D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A88D4: 38EB3B80  addi r7, r11, 0x3b80
	ctx.r[7].s64 = ctx.r[11].s64 + 15232;
	// 826A88D8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826A88DC: 388AB548  addi r4, r10, -0x4ab8
	ctx.r[4].s64 = ctx.r[10].s64 + -19128;
	// 826A88E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A88E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A88E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A88EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A88F0: 386A9458  addi r3, r10, -0x6ba8
	ctx.r[3].s64 = ctx.r[10].s64 + -27560;
	// 826A88F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A88F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A88FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A890C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8914: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A8918: 4BDBE509  bl 0x82466e20
	ctx.lr = 0x826A891C;
	sub_82466E20(ctx, base);
	// 826A891C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A8930 size=24
    let mut pc: u32 = 0x826A8930;
    'dispatch: loop {
        match pc {
            0x826A8930 => {
    //   block [0x826A8930..0x826A8948)
	// 826A8930: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8934: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A8938: 394A3C28  addi r10, r10, 0x3c28
	ctx.r[10].s64 = ctx.r[10].s64 + 15400;
	// 826A893C: 816BD360  lwz r11, -0x2ca0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11424 as u32) ) } as u64;
	// 826A8940: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826A8944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8948 size=116
    let mut pc: u32 = 0x826A8948;
    'dispatch: loop {
        match pc {
            0x826A8948 => {
    //   block [0x826A8948..0x826A89BC)
	// 826A8948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A894C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8954: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A8958: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826A895C: 390A3C28  addi r8, r10, 0x3c28
	ctx.r[8].s64 = ctx.r[10].s64 + 15400;
	// 826A8960: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8964: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A8968: 38AA9458  addi r5, r10, -0x6ba8
	ctx.r[5].s64 = ctx.r[10].s64 + -27560;
	// 826A896C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8970: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A8974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8978: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A897C: 388AB57C  addi r4, r10, -0x4a84
	ctx.r[4].s64 = ctx.r[10].s64 + -19076;
	// 826A8980: 396BCCD4  addi r11, r11, -0x332c
	ctx.r[11].s64 = ctx.r[11].s64 + -13100;
	// 826A8984: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8988: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A898C: 386A9488  addi r3, r10, -0x6b78
	ctx.r[3].s64 = ctx.r[10].s64 + -27512;
	// 826A8990: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A8994: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8998: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A899C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A89A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A89A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A89A8: 4BDBE479  bl 0x82466e20
	ctx.lr = 0x826A89AC;
	sub_82466E20(ctx, base);
	// 826A89AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A89B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A89B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A89B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A89C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A89C0 size=112
    let mut pc: u32 = 0x826A89C0;
    'dispatch: loop {
        match pc {
            0x826A89C0 => {
    //   block [0x826A89C0..0x826A8A30)
	// 826A89C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A89C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A89C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A89CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A89D0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A89D4: 38AA9458  addi r5, r10, -0x6ba8
	ctx.r[5].s64 = ctx.r[10].s64 + -27560;
	// 826A89D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A89DC: 390BD370  addi r8, r11, -0x2c90
	ctx.r[8].s64 = ctx.r[11].s64 + -11408;
	// 826A89E0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826A89E4: 388AB5A4  addi r4, r10, -0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19036;
	// 826A89E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A89EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A89F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A89F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A89F8: 386A94B8  addi r3, r10, -0x6b48
	ctx.r[3].s64 = ctx.r[10].s64 + -27464;
	// 826A89FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8A00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8A0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8A1C: 4BDBE405  bl 0x82466e20
	ctx.lr = 0x826A8A20;
	sub_82466E20(ctx, base);
	// 826A8A20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A8A30 size=24
    let mut pc: u32 = 0x826A8A30;
    'dispatch: loop {
        match pc {
            0x826A8A30 => {
    //   block [0x826A8A30..0x826A8A48)
	// 826A8A30: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8A34: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A8A38: 394A3D18  addi r10, r10, 0x3d18
	ctx.r[10].s64 = ctx.r[10].s64 + 15640;
	// 826A8A3C: 816BDE18  lwz r11, -0x21e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8680 as u32) ) } as u64;
	// 826A8A40: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826A8A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8A48 size=116
    let mut pc: u32 = 0x826A8A48;
    'dispatch: loop {
        match pc {
            0x826A8A48 => {
    //   block [0x826A8A48..0x826A8ABC)
	// 826A8A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8A54: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A8A58: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8A5C: 392BCC98  addi r9, r11, -0x3368
	ctx.r[9].s64 = ctx.r[11].s64 + -13160;
	// 826A8A60: 38AA9428  addi r5, r10, -0x6bd8
	ctx.r[5].s64 = ctx.r[10].s64 + -27608;
	// 826A8A64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8A68: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 826A8A6C: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 826A8A70: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8A74: 388AB5E4  addi r4, r10, -0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + -18972;
	// 826A8A78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8A7C: 396B3D18  addi r11, r11, 0x3d18
	ctx.r[11].s64 = ctx.r[11].s64 + 15640;
	// 826A8A80: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A8A84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8A88: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A8A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8A90: 386A94E8  addi r3, r10, -0x6b18
	ctx.r[3].s64 = ctx.r[10].s64 + -27416;
	// 826A8A94: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826A8A98: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A8A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8AA0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A8AA4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A8AA8: 4BDBE379  bl 0x82466e20
	ctx.lr = 0x826A8AAC;
	sub_82466E20(ctx, base);
	// 826A8AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8AC0 size=108
    let mut pc: u32 = 0x826A8AC0;
    'dispatch: loop {
        match pc {
            0x826A8AC0 => {
    //   block [0x826A8AC0..0x826A8B2C)
	// 826A8AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8ACC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8AD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8AD4: 38EBD3E8  addi r7, r11, -0x2c18
	ctx.r[7].s64 = ctx.r[11].s64 + -11288;
	// 826A8AD8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A8ADC: 388AB610  addi r4, r10, -0x49f0
	ctx.r[4].s64 = ctx.r[10].s64 + -18928;
	// 826A8AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8AE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8AE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A8AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8AF0: 386A9518  addi r3, r10, -0x6ae8
	ctx.r[3].s64 = ctx.r[10].s64 + -27368;
	// 826A8AF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A8AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8B04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8B14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A8B18: 4BDBE309  bl 0x82466e20
	ctx.lr = 0x826A8B1C;
	sub_82466E20(ctx, base);
	// 826A8B1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8B30 size=112
    let mut pc: u32 = 0x826A8B30;
    'dispatch: loop {
        match pc {
            0x826A8B30 => {
    //   block [0x826A8B30..0x826A8BA0)
	// 826A8B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8B3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8B40: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8B44: 38AA9428  addi r5, r10, -0x6bd8
	ctx.r[5].s64 = ctx.r[10].s64 + -27608;
	// 826A8B48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8B4C: 390BD448  addi r8, r11, -0x2bb8
	ctx.r[8].s64 = ctx.r[11].s64 + -11192;
	// 826A8B50: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826A8B54: 388AB628  addi r4, r10, -0x49d8
	ctx.r[4].s64 = ctx.r[10].s64 + -18904;
	// 826A8B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8B5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8B60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8B68: 386A9548  addi r3, r10, -0x6ab8
	ctx.r[3].s64 = ctx.r[10].s64 + -27320;
	// 826A8B6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8B70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8B74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8B7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8B8C: 4BDBE295  bl 0x82466e20
	ctx.lr = 0x826A8B90;
	sub_82466E20(ctx, base);
	// 826A8B90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8BA0 size=108
    let mut pc: u32 = 0x826A8BA0;
    'dispatch: loop {
        match pc {
            0x826A8BA0 => {
    //   block [0x826A8BA0..0x826A8C0C)
	// 826A8BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8BAC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8BB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8BB4: 38EBD4A8  addi r7, r11, -0x2b58
	ctx.r[7].s64 = ctx.r[11].s64 + -11096;
	// 826A8BB8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A8BBC: 388AB638  addi r4, r10, -0x49c8
	ctx.r[4].s64 = ctx.r[10].s64 + -18888;
	// 826A8BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8BC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8BC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A8BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8BD0: 386A9578  addi r3, r10, -0x6a88
	ctx.r[3].s64 = ctx.r[10].s64 + -27272;
	// 826A8BD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A8BD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8BE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8BE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8BE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8BF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8BF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A8BF8: 4BDBE229  bl 0x82466e20
	ctx.lr = 0x826A8BFC;
	sub_82466E20(ctx, base);
	// 826A8BFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8C10 size=108
    let mut pc: u32 = 0x826A8C10;
    'dispatch: loop {
        match pc {
            0x826A8C10 => {
    //   block [0x826A8C10..0x826A8C7C)
	// 826A8C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8C1C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8C20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8C24: 38EBD4C0  addi r7, r11, -0x2b40
	ctx.r[7].s64 = ctx.r[11].s64 + -11072;
	// 826A8C28: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A8C2C: 388AB66C  addi r4, r10, -0x4994
	ctx.r[4].s64 = ctx.r[10].s64 + -18836;
	// 826A8C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8C34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8C38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A8C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8C40: 386A95A8  addi r3, r10, -0x6a58
	ctx.r[3].s64 = ctx.r[10].s64 + -27224;
	// 826A8C44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A8C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8C64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A8C68: 4BDBE1B9  bl 0x82466e20
	ctx.lr = 0x826A8C6C;
	sub_82466E20(ctx, base);
	// 826A8C6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A8C80 size=24
    let mut pc: u32 = 0x826A8C80;
    'dispatch: loop {
        match pc {
            0x826A8C80 => {
    //   block [0x826A8C80..0x826A8C98)
	// 826A8C80: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8C84: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A8C88: 394A3E20  addi r10, r10, 0x3e20
	ctx.r[10].s64 = ctx.r[10].s64 + 15904;
	// 826A8C8C: 816BDE18  lwz r11, -0x21e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8680 as u32) ) } as u64;
	// 826A8C90: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826A8C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8C98 size=116
    let mut pc: u32 = 0x826A8C98;
    'dispatch: loop {
        match pc {
            0x826A8C98 => {
    //   block [0x826A8C98..0x826A8D0C)
	// 826A8C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8CA4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A8CA8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826A8CAC: 390A3E20  addi r8, r10, 0x3e20
	ctx.r[8].s64 = ctx.r[10].s64 + 15904;
	// 826A8CB0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8CB4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A8CB8: 38AA9428  addi r5, r10, -0x6bd8
	ctx.r[5].s64 = ctx.r[10].s64 + -27608;
	// 826A8CBC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8CC0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A8CC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8CC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8CCC: 388AB688  addi r4, r10, -0x4978
	ctx.r[4].s64 = ctx.r[10].s64 + -18808;
	// 826A8CD0: 396BCD30  addi r11, r11, -0x32d0
	ctx.r[11].s64 = ctx.r[11].s64 + -13008;
	// 826A8CD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8CD8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8CDC: 386A95D8  addi r3, r10, -0x6a28
	ctx.r[3].s64 = ctx.r[10].s64 + -27176;
	// 826A8CE0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A8CE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8CE8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A8CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8CF8: 4BDBE129  bl 0x82466e20
	ctx.lr = 0x826A8CFC;
	sub_82466E20(ctx, base);
	// 826A8CFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8D00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8D04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8D10 size=112
    let mut pc: u32 = 0x826A8D10;
    'dispatch: loop {
        match pc {
            0x826A8D10 => {
    //   block [0x826A8D10..0x826A8D80)
	// 826A8D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8D1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8D20: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8D24: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A8D28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8D2C: 390BD520  addi r8, r11, -0x2ae0
	ctx.r[8].s64 = ctx.r[11].s64 + -10976;
	// 826A8D30: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826A8D34: 388AB69C  addi r4, r10, -0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + -18788;
	// 826A8D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8D3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8D40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8D44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8D48: 386A9608  addi r3, r10, -0x69f8
	ctx.r[3].s64 = ctx.r[10].s64 + -27128;
	// 826A8D4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8D54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8D6C: 4BDBE0B5  bl 0x82466e20
	ctx.lr = 0x826A8D70;
	sub_82466E20(ctx, base);
	// 826A8D70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8D80 size=112
    let mut pc: u32 = 0x826A8D80;
    'dispatch: loop {
        match pc {
            0x826A8D80 => {
    //   block [0x826A8D80..0x826A8DF0)
	// 826A8D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8D8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8D90: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8D94: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A8D98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8D9C: 390BD5B0  addi r8, r11, -0x2a50
	ctx.r[8].s64 = ctx.r[11].s64 + -10832;
	// 826A8DA0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826A8DA4: 388AB6CC  addi r4, r10, -0x4934
	ctx.r[4].s64 = ctx.r[10].s64 + -18740;
	// 826A8DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8DAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8DB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8DB8: 386A9638  addi r3, r10, -0x69c8
	ctx.r[3].s64 = ctx.r[10].s64 + -27080;
	// 826A8DBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8DC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8DDC: 4BDBE045  bl 0x82466e20
	ctx.lr = 0x826A8DE0;
	sub_82466E20(ctx, base);
	// 826A8DE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8DF0 size=112
    let mut pc: u32 = 0x826A8DF0;
    'dispatch: loop {
        match pc {
            0x826A8DF0 => {
    //   block [0x826A8DF0..0x826A8E60)
	// 826A8DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8DFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8E00: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8E04: 38AA94E8  addi r5, r10, -0x6b18
	ctx.r[5].s64 = ctx.r[10].s64 + -27416;
	// 826A8E08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8E0C: 390BD610  addi r8, r11, -0x29f0
	ctx.r[8].s64 = ctx.r[11].s64 + -10736;
	// 826A8E10: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A8E14: 388AB6FC  addi r4, r10, -0x4904
	ctx.r[4].s64 = ctx.r[10].s64 + -18692;
	// 826A8E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8E1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8E20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8E28: 386A9668  addi r3, r10, -0x6998
	ctx.r[3].s64 = ctx.r[10].s64 + -27032;
	// 826A8E2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8E34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8E4C: 4BDBDFD5  bl 0x82466e20
	ctx.lr = 0x826A8E50;
	sub_82466E20(ctx, base);
	// 826A8E50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8E60 size=100
    let mut pc: u32 = 0x826A8E60;
    'dispatch: loop {
        match pc {
            0x826A8E60 => {
    //   block [0x826A8E60..0x826A8EC4)
	// 826A8E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8E6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8E74: 38AA9398  addi r5, r10, -0x6c68
	ctx.r[5].s64 = ctx.r[10].s64 + -27752;
	// 826A8E78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8E80: 388AB750  addi r4, r10, -0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + -18608;
	// 826A8E84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8E88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8E8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8E90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8E94: 386A9698  addi r3, r10, -0x6968
	ctx.r[3].s64 = ctx.r[10].s64 + -26984;
	// 826A8E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8E9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8EA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A8EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8EA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A8EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8EB0: 4BDBDF71  bl 0x82466e20
	ctx.lr = 0x826A8EB4;
	sub_82466E20(ctx, base);
	// 826A8EB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8EC8 size=112
    let mut pc: u32 = 0x826A8EC8;
    'dispatch: loop {
        match pc {
            0x826A8EC8 => {
    //   block [0x826A8EC8..0x826A8F38)
	// 826A8EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8ED4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8ED8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8EDC: 38AA9698  addi r5, r10, -0x6968
	ctx.r[5].s64 = ctx.r[10].s64 + -26984;
	// 826A8EE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8EE4: 390BD640  addi r8, r11, -0x29c0
	ctx.r[8].s64 = ctx.r[11].s64 + -10688;
	// 826A8EE8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826A8EEC: 388AB760  addi r4, r10, -0x48a0
	ctx.r[4].s64 = ctx.r[10].s64 + -18592;
	// 826A8EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8EF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8EF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8F00: 386A96C8  addi r3, r10, -0x6938
	ctx.r[3].s64 = ctx.r[10].s64 + -26936;
	// 826A8F04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8F1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8F24: 4BDBDEFD  bl 0x82466e20
	ctx.lr = 0x826A8F28;
	sub_82466E20(ctx, base);
	// 826A8F28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8F38 size=112
    let mut pc: u32 = 0x826A8F38;
    'dispatch: loop {
        match pc {
            0x826A8F38 => {
    //   block [0x826A8F38..0x826A8FA8)
	// 826A8F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8F40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8F44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8F48: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8F4C: 38AA96C8  addi r5, r10, -0x6938
	ctx.r[5].s64 = ctx.r[10].s64 + -26936;
	// 826A8F50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8F54: 390BD6A0  addi r8, r11, -0x2960
	ctx.r[8].s64 = ctx.r[11].s64 + -10592;
	// 826A8F58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A8F5C: 388AB778  addi r4, r10, -0x4888
	ctx.r[4].s64 = ctx.r[10].s64 + -18568;
	// 826A8F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8F64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8F68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8F70: 386A96F8  addi r3, r10, -0x6908
	ctx.r[3].s64 = ctx.r[10].s64 + -26888;
	// 826A8F74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8F7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8F80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8F88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8F8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8F94: 4BDBDE8D  bl 0x82466e20
	ctx.lr = 0x826A8F98;
	sub_82466E20(ctx, base);
	// 826A8F98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8FA8 size=112
    let mut pc: u32 = 0x826A8FA8;
    'dispatch: loop {
        match pc {
            0x826A8FA8 => {
    //   block [0x826A8FA8..0x826A9018)
	// 826A8FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8FB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8FB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8FB8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8FBC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A8FC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8FC4: 390BD6D0  addi r8, r11, -0x2930
	ctx.r[8].s64 = ctx.r[11].s64 + -10544;
	// 826A8FC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A8FCC: 388AB7A4  addi r4, r10, -0x485c
	ctx.r[4].s64 = ctx.r[10].s64 + -18524;
	// 826A8FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8FD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8FD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8FDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8FE0: 386A9728  addi r3, r10, -0x68d8
	ctx.r[3].s64 = ctx.r[10].s64 + -26840;
	// 826A8FE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8FE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8FEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8FF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8FF4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A8FF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8FFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9004: 4BDBDE1D  bl 0x82466e20
	ctx.lr = 0x826A9008;
	sub_82466E20(ctx, base);
	// 826A9008: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A900C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9018 size=116
    let mut pc: u32 = 0x826A9018;
    'dispatch: loop {
        match pc {
            0x826A9018 => {
    //   block [0x826A9018..0x826A908C)
	// 826A9018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A901C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9024: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9028: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A902C: 390BD700  addi r8, r11, -0x2900
	ctx.r[8].s64 = ctx.r[11].s64 + -10496;
	// 826A9030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9034: 392ACD60  addi r9, r10, -0x32a0
	ctx.r[9].s64 = ctx.r[10].s64 + -12960;
	// 826A9038: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A903C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826A9040: 38AA9AE8  addi r5, r10, -0x6518
	ctx.r[5].s64 = ctx.r[10].s64 + -25880;
	// 826A9044: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A904C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A905C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826A9060: 388AB7C4  addi r4, r10, -0x483c
	ctx.r[4].s64 = ctx.r[10].s64 + -18492;
	// 826A9064: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A9068: 386B9758  addi r3, r11, -0x68a8
	ctx.r[3].s64 = ctx.r[11].s64 + -26792;
	// 826A906C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A9070: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9078: 4BDBDDA9  bl 0x82466e20
	ctx.lr = 0x826A907C;
	sub_82466E20(ctx, base);
	// 826A907C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9090 size=112
    let mut pc: u32 = 0x826A9090;
    'dispatch: loop {
        match pc {
            0x826A9090 => {
    //   block [0x826A9090..0x826A9100)
	// 826A9090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A909C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A90A0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A90A4: 38AA9758  addi r5, r10, -0x68a8
	ctx.r[5].s64 = ctx.r[10].s64 + -26792;
	// 826A90A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A90AC: 390BD718  addi r8, r11, -0x28e8
	ctx.r[8].s64 = ctx.r[11].s64 + -10472;
	// 826A90B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A90B4: 388AB7D4  addi r4, r10, -0x482c
	ctx.r[4].s64 = ctx.r[10].s64 + -18476;
	// 826A90B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A90BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A90C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A90C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A90C8: 386A9788  addi r3, r10, -0x6878
	ctx.r[3].s64 = ctx.r[10].s64 + -26744;
	// 826A90CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A90D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A90D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A90D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A90DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A90E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A90E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A90E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A90EC: 4BDBDD35  bl 0x82466e20
	ctx.lr = 0x826A90F0;
	sub_82466E20(ctx, base);
	// 826A90F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A90F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A90F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A90FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9100 size=116
    let mut pc: u32 = 0x826A9100;
    'dispatch: loop {
        match pc {
            0x826A9100 => {
    //   block [0x826A9100..0x826A9174)
	// 826A9100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A910C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9110: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A9114: 390BD734  addi r8, r11, -0x28cc
	ctx.r[8].s64 = ctx.r[11].s64 + -10444;
	// 826A9118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A911C: 392ACD8C  addi r9, r10, -0x3274
	ctx.r[9].s64 = ctx.r[10].s64 + -12916;
	// 826A9120: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9124: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826A9128: 38AA9758  addi r5, r10, -0x68a8
	ctx.r[5].s64 = ctx.r[10].s64 + -26792;
	// 826A912C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9134: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A913C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9144: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826A9148: 388AB7E0  addi r4, r10, -0x4820
	ctx.r[4].s64 = ctx.r[10].s64 + -18464;
	// 826A914C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A9150: 386B97B8  addi r3, r11, -0x6848
	ctx.r[3].s64 = ctx.r[11].s64 + -26696;
	// 826A9154: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A9158: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A915C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9160: 4BDBDCC1  bl 0x82466e20
	ctx.lr = 0x826A9164;
	sub_82466E20(ctx, base);
	// 826A9164: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A916C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9178 size=112
    let mut pc: u32 = 0x826A9178;
    'dispatch: loop {
        match pc {
            0x826A9178 => {
    //   block [0x826A9178..0x826A91E8)
	// 826A9178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A917C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9184: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9188: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A918C: 38AA9758  addi r5, r10, -0x68a8
	ctx.r[5].s64 = ctx.r[10].s64 + -26792;
	// 826A9190: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9194: 390BD768  addi r8, r11, -0x2898
	ctx.r[8].s64 = ctx.r[11].s64 + -10392;
	// 826A9198: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A919C: 388AB7F0  addi r4, r10, -0x4810
	ctx.r[4].s64 = ctx.r[10].s64 + -18448;
	// 826A91A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A91A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A91A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A91AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A91B0: 386A97E8  addi r3, r10, -0x6818
	ctx.r[3].s64 = ctx.r[10].s64 + -26648;
	// 826A91B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A91B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A91BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A91C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A91C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A91C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A91CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A91D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A91D4: 4BDBDC4D  bl 0x82466e20
	ctx.lr = 0x826A91D8;
	sub_82466E20(ctx, base);
	// 826A91D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A91DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A91E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A91E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A91E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A91E8 size=112
    let mut pc: u32 = 0x826A91E8;
    'dispatch: loop {
        match pc {
            0x826A91E8 => {
    //   block [0x826A91E8..0x826A9258)
	// 826A91E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A91EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A91F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A91F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A91F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A91FC: 38AA9758  addi r5, r10, -0x68a8
	ctx.r[5].s64 = ctx.r[10].s64 + -26792;
	// 826A9200: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9204: 390BD7B0  addi r8, r11, -0x2850
	ctx.r[8].s64 = ctx.r[11].s64 + -10320;
	// 826A9208: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A920C: 388AB808  addi r4, r10, -0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + -18424;
	// 826A9210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9214: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9218: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A921C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9220: 386A9818  addi r3, r10, -0x67e8
	ctx.r[3].s64 = ctx.r[10].s64 + -26600;
	// 826A9224: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A922C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A923C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9244: 4BDBDBDD  bl 0x82466e20
	ctx.lr = 0x826A9248;
	sub_82466E20(ctx, base);
	// 826A9248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A924C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9258 size=112
    let mut pc: u32 = 0x826A9258;
    'dispatch: loop {
        match pc {
            0x826A9258 => {
    //   block [0x826A9258..0x826A92C8)
	// 826A9258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A925C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9264: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9268: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A926C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A9270: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A9274: 390BD7F8  addi r8, r11, -0x2808
	ctx.r[8].s64 = ctx.r[11].s64 + -10248;
	// 826A9278: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A927C: 388AB39C  addi r4, r10, -0x4c64
	ctx.r[4].s64 = ctx.r[10].s64 + -19556;
	// 826A9280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9284: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A928C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9290: 386A9848  addi r3, r10, -0x67b8
	ctx.r[3].s64 = ctx.r[10].s64 + -26552;
	// 826A9294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A929C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A92A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A92A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A92A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A92AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A92B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A92B4: 4BDBDB6D  bl 0x82466e20
	ctx.lr = 0x826A92B8;
	sub_82466E20(ctx, base);
	// 826A92B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A92BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A92C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A92C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A92C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A92C8 size=108
    let mut pc: u32 = 0x826A92C8;
    'dispatch: loop {
        match pc {
            0x826A92C8 => {
    //   block [0x826A92C8..0x826A9334)
	// 826A92C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A92CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A92D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A92D4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A92D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A92DC: 38EBD828  addi r7, r11, -0x27d8
	ctx.r[7].s64 = ctx.r[11].s64 + -10200;
	// 826A92E0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A92E4: 388AB820  addi r4, r10, -0x47e0
	ctx.r[4].s64 = ctx.r[10].s64 + -18400;
	// 826A92E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A92EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A92F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A92F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A92F8: 386A9878  addi r3, r10, -0x6788
	ctx.r[3].s64 = ctx.r[10].s64 + -26504;
	// 826A92FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A9300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A930C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A931C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A9320: 4BDBDB01  bl 0x82466e20
	ctx.lr = 0x826A9324;
	sub_82466E20(ctx, base);
	// 826A9324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A932C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9338 size=112
    let mut pc: u32 = 0x826A9338;
    'dispatch: loop {
        match pc {
            0x826A9338 => {
    //   block [0x826A9338..0x826A93A8)
	// 826A9338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A933C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9344: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9348: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A934C: 38AA9758  addi r5, r10, -0x68a8
	ctx.r[5].s64 = ctx.r[10].s64 + -26792;
	// 826A9350: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9354: 390BD870  addi r8, r11, -0x2790
	ctx.r[8].s64 = ctx.r[11].s64 + -10128;
	// 826A9358: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826A935C: 388AB844  addi r4, r10, -0x47bc
	ctx.r[4].s64 = ctx.r[10].s64 + -18364;
	// 826A9360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9364: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A936C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9370: 386A98A8  addi r3, r10, -0x6758
	ctx.r[3].s64 = ctx.r[10].s64 + -26456;
	// 826A9374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A937C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A938C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9394: 4BDBDA8D  bl 0x82466e20
	ctx.lr = 0x826A9398;
	sub_82466E20(ctx, base);
	// 826A9398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A939C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A93A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A93A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A93A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A93A8 size=116
    let mut pc: u32 = 0x826A93A8;
    'dispatch: loop {
        match pc {
            0x826A93A8 => {
    //   block [0x826A93A8..0x826A941C)
	// 826A93A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A93AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A93B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A93B4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A93B8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A93BC: 392BCDC8  addi r9, r11, -0x3238
	ctx.r[9].s64 = ctx.r[11].s64 + -12856;
	// 826A93C0: 38AA9758  addi r5, r10, -0x68a8
	ctx.r[5].s64 = ctx.r[10].s64 + -26792;
	// 826A93C4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A93C8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826A93CC: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 826A93D0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A93D4: 388AB85C  addi r4, r10, -0x47a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18340;
	// 826A93D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A93DC: 396BD900  addi r11, r11, -0x2700
	ctx.r[11].s64 = ctx.r[11].s64 + -9984;
	// 826A93E0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A93E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A93E8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A93EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A93F0: 386A98D8  addi r3, r10, -0x6728
	ctx.r[3].s64 = ctx.r[10].s64 + -26408;
	// 826A93F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A93F8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A93FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9400: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A9404: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A9408: 4BDBDA19  bl 0x82466e20
	ctx.lr = 0x826A940C;
	sub_82466E20(ctx, base);
	// 826A940C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9420 size=112
    let mut pc: u32 = 0x826A9420;
    'dispatch: loop {
        match pc {
            0x826A9420 => {
    //   block [0x826A9420..0x826A9490)
	// 826A9420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A942C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9430: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9434: 38AA9758  addi r5, r10, -0x68a8
	ctx.r[5].s64 = ctx.r[10].s64 + -26792;
	// 826A9438: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A943C: 390BD990  addi r8, r11, -0x2670
	ctx.r[8].s64 = ctx.r[11].s64 + -9840;
	// 826A9440: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A9444: 388AB8E0  addi r4, r10, -0x4720
	ctx.r[4].s64 = ctx.r[10].s64 + -18208;
	// 826A9448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A944C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9458: 386A9908  addi r3, r10, -0x66f8
	ctx.r[3].s64 = ctx.r[10].s64 + -26360;
	// 826A945C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A946C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A947C: 4BDBD9A5  bl 0x82466e20
	ctx.lr = 0x826A9480;
	sub_82466E20(ctx, base);
	// 826A9480: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A948C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A9490 size=24
    let mut pc: u32 = 0x826A9490;
    'dispatch: loop {
        match pc {
            0x826A9490 => {
    //   block [0x826A9490..0x826A94A8)
	// 826A9490: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9494: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A9498: 394A3E98  addi r10, r10, 0x3e98
	ctx.r[10].s64 = ctx.r[10].s64 + 16024;
	// 826A949C: 816BDE18  lwz r11, -0x21e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8680 as u32) ) } as u64;
	// 826A94A0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826A94A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A94A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A94A8 size=116
    let mut pc: u32 = 0x826A94A8;
    'dispatch: loop {
        match pc {
            0x826A94A8 => {
    //   block [0x826A94A8..0x826A951C)
	// 826A94A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A94AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A94B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A94B4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A94B8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826A94BC: 390A3E98  addi r8, r10, 0x3e98
	ctx.r[8].s64 = ctx.r[10].s64 + 16024;
	// 826A94C0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A94C4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A94C8: 38AA9758  addi r5, r10, -0x68a8
	ctx.r[5].s64 = ctx.r[10].s64 + -26792;
	// 826A94CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A94D0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A94D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A94D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A94DC: 388AB8F0  addi r4, r10, -0x4710
	ctx.r[4].s64 = ctx.r[10].s64 + -18192;
	// 826A94E0: 396BCDF8  addi r11, r11, -0x3208
	ctx.r[11].s64 = ctx.r[11].s64 + -12808;
	// 826A94E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A94E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A94EC: 386A9938  addi r3, r10, -0x66c8
	ctx.r[3].s64 = ctx.r[10].s64 + -26312;
	// 826A94F0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A94F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A94F8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A94FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9508: 4BDBD919  bl 0x82466e20
	ctx.lr = 0x826A950C;
	sub_82466E20(ctx, base);
	// 826A950C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9520 size=112
    let mut pc: u32 = 0x826A9520;
    'dispatch: loop {
        match pc {
            0x826A9520 => {
    //   block [0x826A9520..0x826A9590)
	// 826A9520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A952C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9530: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9534: 38AA9428  addi r5, r10, -0x6bd8
	ctx.r[5].s64 = ctx.r[10].s64 + -27608;
	// 826A9538: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A953C: 390BD9A8  addi r8, r11, -0x2658
	ctx.r[8].s64 = ctx.r[11].s64 + -9816;
	// 826A9540: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A9544: 388AB530  addi r4, r10, -0x4ad0
	ctx.r[4].s64 = ctx.r[10].s64 + -19152;
	// 826A9548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A954C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9550: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9558: 386A9968  addi r3, r10, -0x6698
	ctx.r[3].s64 = ctx.r[10].s64 + -26264;
	// 826A955C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A956C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A957C: 4BDBD8A5  bl 0x82466e20
	ctx.lr = 0x826A9580;
	sub_82466E20(ctx, base);
	// 826A9580: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A958C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9590 size=100
    let mut pc: u32 = 0x826A9590;
    'dispatch: loop {
        match pc {
            0x826A9590 => {
    //   block [0x826A9590..0x826A95F4)
	// 826A9590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A959C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A95A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A95A4: 38AA99F8  addi r5, r10, -0x6608
	ctx.r[5].s64 = ctx.r[10].s64 + -26120;
	// 826A95A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A95AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A95B0: 388AB5FC  addi r4, r10, -0x4a04
	ctx.r[4].s64 = ctx.r[10].s64 + -18948;
	// 826A95B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A95B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A95BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A95C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A95C4: 386A9998  addi r3, r10, -0x6668
	ctx.r[3].s64 = ctx.r[10].s64 + -26216;
	// 826A95C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A95CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A95D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A95D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A95D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A95DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A95E0: 4BDBD841  bl 0x82466e20
	ctx.lr = 0x826A95E4;
	sub_82466E20(ctx, base);
	// 826A95E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A95E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A95EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A95F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A95F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A95F8 size=28
    let mut pc: u32 = 0x826A95F8;
    'dispatch: loop {
        match pc {
            0x826A95F8 => {
    //   block [0x826A95F8..0x826A9614)
	// 826A95F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A95FC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A9600: 394A3F40  addi r10, r10, 0x3f40
	ctx.r[10].s64 = ctx.r[10].s64 + 16192;
	// 826A9604: 816BD9F0  lwz r11, -0x2610(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9744 as u32) ) } as u64;
	// 826A9608: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826A960C: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826A9610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9618 size=112
    let mut pc: u32 = 0x826A9618;
    'dispatch: loop {
        match pc {
            0x826A9618 => {
    //   block [0x826A9618..0x826A9688)
	// 826A9618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A961C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9624: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A9628: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 826A962C: 38EA3F40  addi r7, r10, 0x3f40
	ctx.r[7].s64 = ctx.r[10].s64 + 16192;
	// 826A9630: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9634: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A9638: 388AB648  addi r4, r10, -0x49b8
	ctx.r[4].s64 = ctx.r[10].s64 + -18872;
	// 826A963C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9640: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A9644: 396BCE80  addi r11, r11, -0x3180
	ctx.r[11].s64 = ctx.r[11].s64 + -12672;
	// 826A9648: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A964C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9650: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9654: 386A99C8  addi r3, r10, -0x6638
	ctx.r[3].s64 = ctx.r[10].s64 + -26168;
	// 826A9658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A965C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A9660: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9664: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A9668: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A966C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9670: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A9674: 4BDBD7AD  bl 0x82466e20
	ctx.lr = 0x826A9678;
	sub_82466E20(ctx, base);
	// 826A9678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A967C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A9688 size=24
    let mut pc: u32 = 0x826A9688;
    'dispatch: loop {
        match pc {
            0x826A9688 => {
    //   block [0x826A9688..0x826A96A0)
	// 826A9688: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A968C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A9690: 394A40A8  addi r10, r10, 0x40a8
	ctx.r[10].s64 = ctx.r[10].s64 + 16552;
	// 826A9694: 816BDE18  lwz r11, -0x21e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8680 as u32) ) } as u64;
	// 826A9698: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826A969C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A96A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A96A0 size=116
    let mut pc: u32 = 0x826A96A0;
    'dispatch: loop {
        match pc {
            0x826A96A0 => {
    //   block [0x826A96A0..0x826A9714)
	// 826A96A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A96A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A96A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A96AC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A96B0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A96B4: 392BCE58  addi r9, r11, -0x31a8
	ctx.r[9].s64 = ctx.r[11].s64 + -12712;
	// 826A96B8: 38AA9428  addi r5, r10, -0x6bd8
	ctx.r[5].s64 = ctx.r[10].s64 + -27608;
	// 826A96BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A96C0: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 826A96C4: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826A96C8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A96CC: 388AB65C  addi r4, r10, -0x49a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18852;
	// 826A96D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A96D4: 396B40A8  addi r11, r11, 0x40a8
	ctx.r[11].s64 = ctx.r[11].s64 + 16552;
	// 826A96D8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A96DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A96E0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A96E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A96E8: 386A99F8  addi r3, r10, -0x6608
	ctx.r[3].s64 = ctx.r[10].s64 + -26120;
	// 826A96EC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826A96F0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A96F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A96F8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A96FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A9700: 4BDBD721  bl 0x82466e20
	ctx.lr = 0x826A9704;
	sub_82466E20(ctx, base);
	// 826A9704: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A970C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9718 size=112
    let mut pc: u32 = 0x826A9718;
    'dispatch: loop {
        match pc {
            0x826A9718 => {
    //   block [0x826A9718..0x826A9788)
	// 826A9718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A971C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9724: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9728: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A972C: 38AA9AE8  addi r5, r10, -0x6518
	ctx.r[5].s64 = ctx.r[10].s64 + -25880;
	// 826A9730: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9734: 390BD9F8  addi r8, r11, -0x2608
	ctx.r[8].s64 = ctx.r[11].s64 + -9736;
	// 826A9738: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A973C: 388ABA0C  addi r4, r10, -0x45f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17908;
	// 826A9740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9744: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9748: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A974C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9750: 386A9A28  addi r3, r10, -0x65d8
	ctx.r[3].s64 = ctx.r[10].s64 + -26072;
	// 826A9754: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A975C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A976C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9774: 4BDBD6AD  bl 0x82466e20
	ctx.lr = 0x826A9778;
	sub_82466E20(ctx, base);
	// 826A9778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A977C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9788 size=112
    let mut pc: u32 = 0x826A9788;
    'dispatch: loop {
        match pc {
            0x826A9788 => {
    //   block [0x826A9788..0x826A97F8)
	// 826A9788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A978C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9794: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9798: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A979C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A97A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A97A4: 390BDA28  addi r8, r11, -0x25d8
	ctx.r[8].s64 = ctx.r[11].s64 + -9688;
	// 826A97A8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826A97AC: 388AB718  addi r4, r10, -0x48e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18664;
	// 826A97B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A97B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A97B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A97BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A97C0: 386A9A58  addi r3, r10, -0x65a8
	ctx.r[3].s64 = ctx.r[10].s64 + -26024;
	// 826A97C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A97C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A97CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A97D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A97D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A97D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A97DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A97E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A97E4: 4BDBD63D  bl 0x82466e20
	ctx.lr = 0x826A97E8;
	sub_82466E20(ctx, base);
	// 826A97E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A97EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A97F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A97F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A97F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A97F8 size=112
    let mut pc: u32 = 0x826A97F8;
    'dispatch: loop {
        match pc {
            0x826A97F8 => {
    //   block [0x826A97F8..0x826A9868)
	// 826A97F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A97FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9804: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9808: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A980C: 38AA99F8  addi r5, r10, -0x6608
	ctx.r[5].s64 = ctx.r[10].s64 + -26120;
	// 826A9810: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9814: 390BDAB8  addi r8, r11, -0x2548
	ctx.r[8].s64 = ctx.r[11].s64 + -9544;
	// 826A9818: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A981C: 388AB73C  addi r4, r10, -0x48c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18628;
	// 826A9820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9824: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9828: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A982C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9830: 386A9A88  addi r3, r10, -0x6578
	ctx.r[3].s64 = ctx.r[10].s64 + -25976;
	// 826A9834: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9838: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A983C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A984C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9854: 4BDBD5CD  bl 0x82466e20
	ctx.lr = 0x826A9858;
	sub_82466E20(ctx, base);
	// 826A9858: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A985C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9868 size=100
    let mut pc: u32 = 0x826A9868;
    'dispatch: loop {
        match pc {
            0x826A9868 => {
    //   block [0x826A9868..0x826A98CC)
	// 826A9868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A986C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9874: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A987C: 38AA9398  addi r5, r10, -0x6c68
	ctx.r[5].s64 = ctx.r[10].s64 + -27752;
	// 826A9880: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9888: 388AB904  addi r4, r10, -0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + -18172;
	// 826A988C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A989C: 386A9AB8  addi r3, r10, -0x6548
	ctx.r[3].s64 = ctx.r[10].s64 + -25928;
	// 826A98A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A98A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A98A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A98AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A98B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A98B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A98B8: 4BDBD569  bl 0x82466e20
	ctx.lr = 0x826A98BC;
	sub_82466E20(ctx, base);
	// 826A98BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A98C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A98C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A98C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A98D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A98D0 size=100
    let mut pc: u32 = 0x826A98D0;
    'dispatch: loop {
        match pc {
            0x826A98D0 => {
    //   block [0x826A98D0..0x826A9934)
	// 826A98D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A98D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A98D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A98DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A98E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A98E4: 38AA9398  addi r5, r10, -0x6c68
	ctx.r[5].s64 = ctx.r[10].s64 + -27752;
	// 826A98E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A98EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A98F0: 388AB918  addi r4, r10, -0x46e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18152;
	// 826A98F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A98F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A98FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9904: 386A9AE8  addi r3, r10, -0x6518
	ctx.r[3].s64 = ctx.r[10].s64 + -25880;
	// 826A9908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A990C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9910: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A9914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9918: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A991C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9920: 4BDBD501  bl 0x82466e20
	ctx.lr = 0x826A9924;
	sub_82466E20(ctx, base);
	// 826A9924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A992C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9938 size=112
    let mut pc: u32 = 0x826A9938;
    'dispatch: loop {
        match pc {
            0x826A9938 => {
    //   block [0x826A9938..0x826A99A8)
	// 826A9938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A993C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9944: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9948: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A994C: 38AA9AB8  addi r5, r10, -0x6548
	ctx.r[5].s64 = ctx.r[10].s64 + -25928;
	// 826A9950: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9954: 390BDAD0  addi r8, r11, -0x2530
	ctx.r[8].s64 = ctx.r[11].s64 + -9520;
	// 826A9958: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A995C: 388AB92C  addi r4, r10, -0x46d4
	ctx.r[4].s64 = ctx.r[10].s64 + -18132;
	// 826A9960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9964: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A996C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9970: 386A9B18  addi r3, r10, -0x64e8
	ctx.r[3].s64 = ctx.r[10].s64 + -25832;
	// 826A9974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A997C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A998C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9994: 4BDBD48D  bl 0x82466e20
	ctx.lr = 0x826A9998;
	sub_82466E20(ctx, base);
	// 826A9998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A999C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A99A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A99A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A99A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A99A8 size=112
    let mut pc: u32 = 0x826A99A8;
    'dispatch: loop {
        match pc {
            0x826A99A8 => {
    //   block [0x826A99A8..0x826A9A18)
	// 826A99A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A99AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A99B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A99B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A99B8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A99BC: 38AA9AB8  addi r5, r10, -0x6548
	ctx.r[5].s64 = ctx.r[10].s64 + -25928;
	// 826A99C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A99C4: 390BDB18  addi r8, r11, -0x24e8
	ctx.r[8].s64 = ctx.r[11].s64 + -9448;
	// 826A99C8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826A99CC: 388AB93C  addi r4, r10, -0x46c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18116;
	// 826A99D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A99D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A99D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A99DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A99E0: 386A9B48  addi r3, r10, -0x64b8
	ctx.r[3].s64 = ctx.r[10].s64 + -25784;
	// 826A99E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A99E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A99EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A99F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A99F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A99F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A99FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9A04: 4BDBD41D  bl 0x82466e20
	ctx.lr = 0x826A9A08;
	sub_82466E20(ctx, base);
	// 826A9A08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9A18 size=112
    let mut pc: u32 = 0x826A9A18;
    'dispatch: loop {
        match pc {
            0x826A9A18 => {
    //   block [0x826A9A18..0x826A9A88)
	// 826A9A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9A24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9A28: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9A2C: 38AA9B48  addi r5, r10, -0x64b8
	ctx.r[5].s64 = ctx.r[10].s64 + -25784;
	// 826A9A30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9A34: 390BDBD8  addi r8, r11, -0x2428
	ctx.r[8].s64 = ctx.r[11].s64 + -9256;
	// 826A9A38: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A9A3C: 388AB958  addi r4, r10, -0x46a8
	ctx.r[4].s64 = ctx.r[10].s64 + -18088;
	// 826A9A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9A44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9A48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9A50: 386A9B78  addi r3, r10, -0x6488
	ctx.r[3].s64 = ctx.r[10].s64 + -25736;
	// 826A9A54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9A74: 4BDBD3AD  bl 0x82466e20
	ctx.lr = 0x826A9A78;
	sub_82466E20(ctx, base);
	// 826A9A78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9A88 size=112
    let mut pc: u32 = 0x826A9A88;
    'dispatch: loop {
        match pc {
            0x826A9A88 => {
    //   block [0x826A9A88..0x826A9AF8)
	// 826A9A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9A94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9A98: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9A9C: 38AA9698  addi r5, r10, -0x6968
	ctx.r[5].s64 = ctx.r[10].s64 + -26984;
	// 826A9AA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9AA4: 390BDC08  addi r8, r11, -0x23f8
	ctx.r[8].s64 = ctx.r[11].s64 + -9208;
	// 826A9AA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A9AAC: 388AB97C  addi r4, r10, -0x4684
	ctx.r[4].s64 = ctx.r[10].s64 + -18052;
	// 826A9AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9AB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9AB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9AC0: 386A9BA8  addi r3, r10, -0x6458
	ctx.r[3].s64 = ctx.r[10].s64 + -25688;
	// 826A9AC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9ACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9AD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9AE4: 4BDBD33D  bl 0x82466e20
	ctx.lr = 0x826A9AE8;
	sub_82466E20(ctx, base);
	// 826A9AE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9AF8 size=112
    let mut pc: u32 = 0x826A9AF8;
    'dispatch: loop {
        match pc {
            0x826A9AF8 => {
    //   block [0x826A9AF8..0x826A9B68)
	// 826A9AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9B04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9B08: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9B0C: 38AA9428  addi r5, r10, -0x6bd8
	ctx.r[5].s64 = ctx.r[10].s64 + -27608;
	// 826A9B10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9B14: 390BDC38  addi r8, r11, -0x23c8
	ctx.r[8].s64 = ctx.r[11].s64 + -9160;
	// 826A9B18: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A9B1C: 388AB9A0  addi r4, r10, -0x4660
	ctx.r[4].s64 = ctx.r[10].s64 + -18016;
	// 826A9B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9B24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9B28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9B30: 386A9BD8  addi r3, r10, -0x6428
	ctx.r[3].s64 = ctx.r[10].s64 + -25640;
	// 826A9B34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9B54: 4BDBD2CD  bl 0x82466e20
	ctx.lr = 0x826A9B58;
	sub_82466E20(ctx, base);
	// 826A9B58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9B68 size=112
    let mut pc: u32 = 0x826A9B68;
    'dispatch: loop {
        match pc {
            0x826A9B68 => {
    //   block [0x826A9B68..0x826A9BD8)
	// 826A9B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9B74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9B78: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9B7C: 38AA9398  addi r5, r10, -0x6c68
	ctx.r[5].s64 = ctx.r[10].s64 + -27752;
	// 826A9B80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9B84: 390BDC68  addi r8, r11, -0x2398
	ctx.r[8].s64 = ctx.r[11].s64 + -9112;
	// 826A9B88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A9B8C: 388AB9C4  addi r4, r10, -0x463c
	ctx.r[4].s64 = ctx.r[10].s64 + -17980;
	// 826A9B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9B94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9B98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9BA0: 386A9C08  addi r3, r10, -0x63f8
	ctx.r[3].s64 = ctx.r[10].s64 + -25592;
	// 826A9BA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9BA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9BAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9BB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9BB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9BB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9BC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9BC4: 4BDBD25D  bl 0x82466e20
	ctx.lr = 0x826A9BC8;
	sub_82466E20(ctx, base);
	// 826A9BC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9BD8 size=112
    let mut pc: u32 = 0x826A9BD8;
    'dispatch: loop {
        match pc {
            0x826A9BD8 => {
    //   block [0x826A9BD8..0x826A9C48)
	// 826A9BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9BE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9BE8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9BEC: 38AA9758  addi r5, r10, -0x68a8
	ctx.r[5].s64 = ctx.r[10].s64 + -26792;
	// 826A9BF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9BF4: 390BDC98  addi r8, r11, -0x2368
	ctx.r[8].s64 = ctx.r[11].s64 + -9064;
	// 826A9BF8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826A9BFC: 388AB9D0  addi r4, r10, -0x4630
	ctx.r[4].s64 = ctx.r[10].s64 + -17968;
	// 826A9C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9C04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9C08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9C10: 386A9C38  addi r3, r10, -0x63c8
	ctx.r[3].s64 = ctx.r[10].s64 + -25544;
	// 826A9C14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9C24: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A9C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9C34: 4BDBD1ED  bl 0x82466e20
	ctx.lr = 0x826A9C38;
	sub_82466E20(ctx, base);
	// 826A9C38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9C48 size=108
    let mut pc: u32 = 0x826A9C48;
    'dispatch: loop {
        match pc {
            0x826A9C48 => {
    //   block [0x826A9C48..0x826A9CB4)
	// 826A9C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9C54: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9C58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9C5C: 38EBDD10  addi r7, r11, -0x22f0
	ctx.r[7].s64 = ctx.r[11].s64 + -8944;
	// 826A9C60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A9C64: 388AB9E4  addi r4, r10, -0x461c
	ctx.r[4].s64 = ctx.r[10].s64 + -17948;
	// 826A9C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9C6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9C70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A9C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9C78: 386A9C68  addi r3, r10, -0x6398
	ctx.r[3].s64 = ctx.r[10].s64 + -25496;
	// 826A9C7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A9C80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9C9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A9CA0: 4BDBD181  bl 0x82466e20
	ctx.lr = 0x826A9CA4;
	sub_82466E20(ctx, base);
	// 826A9CA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9CB8 size=112
    let mut pc: u32 = 0x826A9CB8;
    'dispatch: loop {
        match pc {
            0x826A9CB8 => {
    //   block [0x826A9CB8..0x826A9D28)
	// 826A9CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9CC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9CC8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9CCC: 38AA9398  addi r5, r10, -0x6c68
	ctx.r[5].s64 = ctx.r[10].s64 + -27752;
	// 826A9CD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9CD4: 390BDD40  addi r8, r11, -0x22c0
	ctx.r[8].s64 = ctx.r[11].s64 + -8896;
	// 826A9CD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A9CDC: 388AB9F8  addi r4, r10, -0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + -17928;
	// 826A9CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9CE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9CE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9CF0: 386A9C98  addi r3, r10, -0x6368
	ctx.r[3].s64 = ctx.r[10].s64 + -25448;
	// 826A9CF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9D14: 4BDBD10D  bl 0x82466e20
	ctx.lr = 0x826A9D18;
	sub_82466E20(ctx, base);
	// 826A9D18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9D28 size=100
    let mut pc: u32 = 0x826A9D28;
    'dispatch: loop {
        match pc {
            0x826A9D28 => {
    //   block [0x826A9D28..0x826A9D8C)
	// 826A9D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9D30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9D34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9D3C: 38AA9398  addi r5, r10, -0x6c68
	ctx.r[5].s64 = ctx.r[10].s64 + -27752;
	// 826A9D40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9D48: 388ABA20  addi r4, r10, -0x45e0
	ctx.r[4].s64 = ctx.r[10].s64 + -17888;
	// 826A9D4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9D50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9D58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9D5C: 386A9CC8  addi r3, r10, -0x6338
	ctx.r[3].s64 = ctx.r[10].s64 + -25400;
	// 826A9D60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9D64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9D68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A9D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9D70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A9D74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9D78: 4BDBD0A9  bl 0x82466e20
	ctx.lr = 0x826A9D7C;
	sub_82466E20(ctx, base);
	// 826A9D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9D90 size=112
    let mut pc: u32 = 0x826A9D90;
    'dispatch: loop {
        match pc {
            0x826A9D90 => {
    //   block [0x826A9D90..0x826A9E00)
	// 826A9D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9D9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9DA0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9DA4: 38AA9398  addi r5, r10, -0x6c68
	ctx.r[5].s64 = ctx.r[10].s64 + -27752;
	// 826A9DA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9DAC: 390BDD70  addi r8, r11, -0x2290
	ctx.r[8].s64 = ctx.r[11].s64 + -8848;
	// 826A9DB0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A9DB4: 388ABA38  addi r4, r10, -0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + -17864;
	// 826A9DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9DBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9DC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9DC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9DC8: 386A9CF8  addi r3, r10, -0x6308
	ctx.r[3].s64 = ctx.r[10].s64 + -25352;
	// 826A9DCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9DD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9DD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9DE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9DE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9DEC: 4BDBD035  bl 0x82466e20
	ctx.lr = 0x826A9DF0;
	sub_82466E20(ctx, base);
	// 826A9DF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9DF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9DF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9E00 size=96
    let mut pc: u32 = 0x826A9E00;
    'dispatch: loop {
        match pc {
            0x826A9E00 => {
    //   block [0x826A9E00..0x826A9E60)
	// 826A9E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9E0C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9E10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9E14: 388ABA4C  addi r4, r10, -0x45b4
	ctx.r[4].s64 = ctx.r[10].s64 + -17844;
	// 826A9E18: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9E1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9E20: 386A9D28  addi r3, r10, -0x62d8
	ctx.r[3].s64 = ctx.r[10].s64 + -25304;
	// 826A9E24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9E2C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A9E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9E34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9E40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A9E44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A9E48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A9E4C: 4BDBCFD5  bl 0x82466e20
	ctx.lr = 0x826A9E50;
	sub_82466E20(ctx, base);
	// 826A9E50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9E60 size=108
    let mut pc: u32 = 0x826A9E60;
    'dispatch: loop {
        match pc {
            0x826A9E60 => {
    //   block [0x826A9E60..0x826A9ECC)
	// 826A9E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9E6C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9E70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9E74: 38EBDDB8  addi r7, r11, -0x2248
	ctx.r[7].s64 = ctx.r[11].s64 + -8776;
	// 826A9E78: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A9E7C: 388ABA68  addi r4, r10, -0x4598
	ctx.r[4].s64 = ctx.r[10].s64 + -17816;
	// 826A9E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9E84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9E88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A9E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9E90: 386A9D58  addi r3, r10, -0x62a8
	ctx.r[3].s64 = ctx.r[10].s64 + -25256;
	// 826A9E94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A9E98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9E9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9EA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9EA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9EB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9EB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A9EB8: 4BDBCF69  bl 0x82466e20
	ctx.lr = 0x826A9EBC;
	sub_82466E20(ctx, base);
	// 826A9EBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9ED0 size=100
    let mut pc: u32 = 0x826A9ED0;
    'dispatch: loop {
        match pc {
            0x826A9ED0 => {
    //   block [0x826A9ED0..0x826A9F34)
	// 826A9ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9ED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9EDC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A9EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9EE4: 392ACF30  addi r9, r10, -0x30d0
	ctx.r[9].s64 = ctx.r[10].s64 + -12496;
	// 826A9EE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9EF0: 388ABA80  addi r4, r10, -0x4580
	ctx.r[4].s64 = ctx.r[10].s64 + -17792;
	// 826A9EF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9EF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9F00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9F04: 386A9D88  addi r3, r10, -0x6278
	ctx.r[3].s64 = ctx.r[10].s64 + -25208;
	// 826A9F08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9F0C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826A9F10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A9F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9F18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A9F1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A9F20: 4BDBCF01  bl 0x82466e20
	ctx.lr = 0x826A9F24;
	sub_82466E20(ctx, base);
	// 826A9F24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A9F38 size=24
    let mut pc: u32 = 0x826A9F38;
    'dispatch: loop {
        match pc {
            0x826A9F38 => {
    //   block [0x826A9F38..0x826A9F50)
	// 826A9F38: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9F3C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A9F40: 394A4150  addi r10, r10, 0x4150
	ctx.r[10].s64 = ctx.r[10].s64 + 16720;
	// 826A9F44: 816BDE24  lwz r11, -0x21dc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8668 as u32) ) } as u64;
	// 826A9F48: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826A9F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9F50 size=112
    let mut pc: u32 = 0x826A9F50;
    'dispatch: loop {
        match pc {
            0x826A9F50 => {
    //   block [0x826A9F50..0x826A9FC0)
	// 826A9F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9F58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9F5C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A9F60: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9F64: 392AD078  addi r9, r10, -0x2f88
	ctx.r[9].s64 = ctx.r[10].s64 + -12168;
	// 826A9F68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9F6C: 390B4150  addi r8, r11, 0x4150
	ctx.r[8].s64 = ctx.r[11].s64 + 16720;
	// 826A9F70: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826A9F74: 388ABA94  addi r4, r10, -0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + -17772;
	// 826A9F78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9F7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9F80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9F88: 386A9DB8  addi r3, r10, -0x6248
	ctx.r[3].s64 = ctx.r[10].s64 + -25160;
	// 826A9F8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A9F90: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826A9F94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9F98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9FA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9FA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A9FA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9FAC: 4BDBCE75  bl 0x82466e20
	ctx.lr = 0x826A9FB0;
	sub_82466E20(ctx, base);
	// 826A9FB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9FC0 size=112
    let mut pc: u32 = 0x826A9FC0;
    'dispatch: loop {
        match pc {
            0x826A9FC0 => {
    //   block [0x826A9FC0..0x826AA030)
	// 826A9FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9FCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9FD0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9FD4: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826A9FD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9FDC: 390BDE2C  addi r8, r11, -0x21d4
	ctx.r[8].s64 = ctx.r[11].s64 + -8660;
	// 826A9FE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A9FE4: 388ABAA8  addi r4, r10, -0x4558
	ctx.r[4].s64 = ctx.r[10].s64 + -17752;
	// 826A9FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9FEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9FF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9FF8: 386A9DE8  addi r3, r10, -0x6218
	ctx.r[3].s64 = ctx.r[10].s64 + -25112;
	// 826A9FFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA00C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA01C: 4BDBCE05  bl 0x82466e20
	ctx.lr = 0x826AA020;
	sub_82466E20(ctx, base);
	// 826AA020: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA02C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA030 size=108
    let mut pc: u32 = 0x826AA030;
    'dispatch: loop {
        match pc {
            0x826AA030 => {
    //   block [0x826AA030..0x826AA09C)
	// 826AA030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA03C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA040: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA044: 38EBDE5C  addi r7, r11, -0x21a4
	ctx.r[7].s64 = ctx.r[11].s64 + -8612;
	// 826AA048: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826AA04C: 388ABAC0  addi r4, r10, -0x4540
	ctx.r[4].s64 = ctx.r[10].s64 + -17728;
	// 826AA050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA054: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA058: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AA05C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA060: 386A9E18  addi r3, r10, -0x61e8
	ctx.r[3].s64 = ctx.r[10].s64 + -25064;
	// 826AA064: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AA068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA06C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA07C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AA088: 4BDBCD99  bl 0x82466e20
	ctx.lr = 0x826AA08C;
	sub_82466E20(ctx, base);
	// 826AA08C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA0A0 size=112
    let mut pc: u32 = 0x826AA0A0;
    'dispatch: loop {
        match pc {
            0x826AA0A0 => {
    //   block [0x826AA0A0..0x826AA110)
	// 826AA0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA0A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA0AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA0B0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA0B4: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA0B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA0BC: 390BDE78  addi r8, r11, -0x2188
	ctx.r[8].s64 = ctx.r[11].s64 + -8584;
	// 826AA0C0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826AA0C4: 388ABAD0  addi r4, r10, -0x4530
	ctx.r[4].s64 = ctx.r[10].s64 + -17712;
	// 826AA0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA0CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA0D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA0D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA0D8: 386A9E48  addi r3, r10, -0x61b8
	ctx.r[3].s64 = ctx.r[10].s64 + -25016;
	// 826AA0DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA0E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA0E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA0E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA0EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA0F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA0F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA0FC: 4BDBCD25  bl 0x82466e20
	ctx.lr = 0x826AA100;
	sub_82466E20(ctx, base);
	// 826AA100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA110 size=100
    let mut pc: u32 = 0x826AA110;
    'dispatch: loop {
        match pc {
            0x826AA110 => {
    //   block [0x826AA110..0x826AA174)
	// 826AA110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA11C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA124: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA128: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA12C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA130: 388ABAF0  addi r4, r10, -0x4510
	ctx.r[4].s64 = ctx.r[10].s64 + -17680;
	// 826AA134: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA13C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA144: 386A9E78  addi r3, r10, -0x6188
	ctx.r[3].s64 = ctx.r[10].s64 + -24968;
	// 826AA148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA14C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA150: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AA154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA158: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AA15C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA160: 4BDBCCC1  bl 0x82466e20
	ctx.lr = 0x826AA164;
	sub_82466E20(ctx, base);
	// 826AA164: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA16C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA178 size=112
    let mut pc: u32 = 0x826AA178;
    'dispatch: loop {
        match pc {
            0x826AA178 => {
    //   block [0x826AA178..0x826AA1E8)
	// 826AA178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA184: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA188: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA18C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA190: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA194: 390BDF38  addi r8, r11, -0x20c8
	ctx.r[8].s64 = ctx.r[11].s64 + -8392;
	// 826AA198: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AA19C: 388ABB0C  addi r4, r10, -0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17652;
	// 826AA1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA1A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA1A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA1AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA1B0: 386A9EA8  addi r3, r10, -0x6158
	ctx.r[3].s64 = ctx.r[10].s64 + -24920;
	// 826AA1B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA1BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA1C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA1D4: 4BDBCC4D  bl 0x82466e20
	ctx.lr = 0x826AA1D8;
	sub_82466E20(ctx, base);
	// 826AA1D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA1DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA1E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA1E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA1E8 size=112
    let mut pc: u32 = 0x826AA1E8;
    'dispatch: loop {
        match pc {
            0x826AA1E8 => {
    //   block [0x826AA1E8..0x826AA258)
	// 826AA1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA1F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA1F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA1FC: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA200: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA204: 390BDF50  addi r8, r11, -0x20b0
	ctx.r[8].s64 = ctx.r[11].s64 + -8368;
	// 826AA208: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AA20C: 388ABB2C  addi r4, r10, -0x44d4
	ctx.r[4].s64 = ctx.r[10].s64 + -17620;
	// 826AA210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA214: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA218: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA220: 386A9ED8  addi r3, r10, -0x6128
	ctx.r[3].s64 = ctx.r[10].s64 + -24872;
	// 826AA224: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA22C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA23C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA244: 4BDBCBDD  bl 0x82466e20
	ctx.lr = 0x826AA248;
	sub_82466E20(ctx, base);
	// 826AA248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA258 size=112
    let mut pc: u32 = 0x826AA258;
    'dispatch: loop {
        match pc {
            0x826AA258 => {
    //   block [0x826AA258..0x826AA2C8)
	// 826AA258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA264: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA268: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA26C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA270: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA274: 390BDF80  addi r8, r11, -0x2080
	ctx.r[8].s64 = ctx.r[11].s64 + -8320;
	// 826AA278: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AA27C: 388ABB50  addi r4, r10, -0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + -17584;
	// 826AA280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA284: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA290: 386A9F08  addi r3, r10, -0x60f8
	ctx.r[3].s64 = ctx.r[10].s64 + -24824;
	// 826AA294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA29C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA2A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA2B4: 4BDBCB6D  bl 0x82466e20
	ctx.lr = 0x826AA2B8;
	sub_82466E20(ctx, base);
	// 826AA2B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


