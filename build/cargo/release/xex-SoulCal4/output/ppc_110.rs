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


pub fn sub_82679908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679908 size=108
    let mut pc: u32 = 0x82679908;
    'dispatch: loop {
        match pc {
            0x82679908 => {
    //   block [0x82679908..0x82679974)
	// 82679908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267990C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679914: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267991C: 38EB7318  addi r7, r11, 0x7318
	ctx.r[7].s64 = ctx.r[11].s64 + 29464;
	// 82679920: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82679924: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 82679928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267992C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679930: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679938: 386A5958  addi r3, r10, 0x5958
	ctx.r[3].s64 = ctx.r[10].s64 + 22872;
	// 8267993C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267994C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267995C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679960: 4BDED4C1  bl 0x82466e20
	ctx.lr = 0x82679964;
	sub_82466E20(ctx, base);
	// 82679964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267996C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679978 size=112
    let mut pc: u32 = 0x82679978;
    'dispatch: loop {
        match pc {
            0x82679978 => {
    //   block [0x82679978..0x826799E8)
	// 82679978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267997C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679984: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679988: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267998C: 38AA5928  addi r5, r10, 0x5928
	ctx.r[5].s64 = ctx.r[10].s64 + 22824;
	// 82679990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679994: 390B73A8  addi r8, r11, 0x73a8
	ctx.r[8].s64 = ctx.r[11].s64 + 29608;
	// 82679998: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 8267999C: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 826799A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826799A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826799A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826799AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826799B0: 386A5988  addi r3, r10, 0x5988
	ctx.r[3].s64 = ctx.r[10].s64 + 22920;
	// 826799B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826799B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826799BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826799C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826799C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826799C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826799CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826799D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826799D4: 4BDED44D  bl 0x82466e20
	ctx.lr = 0x826799D8;
	sub_82466E20(ctx, base);
	// 826799D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826799DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826799E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826799E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826799E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826799E8 size=112
    let mut pc: u32 = 0x826799E8;
    'dispatch: loop {
        match pc {
            0x826799E8 => {
    //   block [0x826799E8..0x82679A58)
	// 826799E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826799EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826799F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826799F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826799F8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826799FC: 38AA5928  addi r5, r10, 0x5928
	ctx.r[5].s64 = ctx.r[10].s64 + 22824;
	// 82679A00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679A04: 390B74C8  addi r8, r11, 0x74c8
	ctx.r[8].s64 = ctx.r[11].s64 + 29896;
	// 82679A08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82679A0C: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 82679A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679A14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679A18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82679A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679A20: 386A59B8  addi r3, r10, 0x59b8
	ctx.r[3].s64 = ctx.r[10].s64 + 22968;
	// 82679A24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82679A28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679A30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679A38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679A40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679A44: 4BDED3DD  bl 0x82466e20
	ctx.lr = 0x82679A48;
	sub_82466E20(ctx, base);
	// 82679A48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679A58 size=108
    let mut pc: u32 = 0x82679A58;
    'dispatch: loop {
        match pc {
            0x82679A58 => {
    //   block [0x82679A58..0x82679AC4)
	// 82679A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679A64: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679A68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679A6C: 38EB74E0  addi r7, r11, 0x74e0
	ctx.r[7].s64 = ctx.r[11].s64 + 29920;
	// 82679A70: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82679A74: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 82679A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679A7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679A80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679A88: 386A59E8  addi r3, r10, 0x59e8
	ctx.r[3].s64 = ctx.r[10].s64 + 23016;
	// 82679A8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679A94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679AAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679AB0: 4BDED371  bl 0x82466e20
	ctx.lr = 0x82679AB4;
	sub_82466E20(ctx, base);
	// 82679AB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679AB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679ABC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679AC8 size=112
    let mut pc: u32 = 0x82679AC8;
    'dispatch: loop {
        match pc {
            0x82679AC8 => {
    //   block [0x82679AC8..0x82679B38)
	// 82679AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679AD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679AD8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679ADC: 38AA5928  addi r5, r10, 0x5928
	ctx.r[5].s64 = ctx.r[10].s64 + 22824;
	// 82679AE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679AE4: 390B7570  addi r8, r11, 0x7570
	ctx.r[8].s64 = ctx.r[11].s64 + 30064;
	// 82679AE8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82679AEC: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 82679AF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679AF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679AF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82679AFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679B00: 386A5A18  addi r3, r10, 0x5a18
	ctx.r[3].s64 = ctx.r[10].s64 + 23064;
	// 82679B04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82679B08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679B10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679B18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679B1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679B20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679B24: 4BDED2FD  bl 0x82466e20
	ctx.lr = 0x82679B28;
	sub_82466E20(ctx, base);
	// 82679B28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679B38 size=108
    let mut pc: u32 = 0x82679B38;
    'dispatch: loop {
        match pc {
            0x82679B38 => {
    //   block [0x82679B38..0x82679BA4)
	// 82679B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679B44: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679B4C: 38EB7660  addi r7, r11, 0x7660
	ctx.r[7].s64 = ctx.r[11].s64 + 30304;
	// 82679B50: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82679B54: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 82679B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679B5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679B60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679B64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679B68: 386A5A48  addi r3, r10, 0x5a48
	ctx.r[3].s64 = ctx.r[10].s64 + 23112;
	// 82679B6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679B70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679B74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679B8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679B90: 4BDED291  bl 0x82466e20
	ctx.lr = 0x82679B94;
	sub_82466E20(ctx, base);
	// 82679B94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679BA8 size=108
    let mut pc: u32 = 0x82679BA8;
    'dispatch: loop {
        match pc {
            0x82679BA8 => {
    //   block [0x82679BA8..0x82679C14)
	// 82679BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679BB4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679BBC: 38EB7678  addi r7, r11, 0x7678
	ctx.r[7].s64 = ctx.r[11].s64 + 30328;
	// 82679BC0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82679BC4: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 82679BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679BCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679BD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679BD8: 386A5A78  addi r3, r10, 0x5a78
	ctx.r[3].s64 = ctx.r[10].s64 + 23160;
	// 82679BDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679BFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679C00: 4BDED221  bl 0x82466e20
	ctx.lr = 0x82679C04;
	sub_82466E20(ctx, base);
	// 82679C04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679C18 size=116
    let mut pc: u32 = 0x82679C18;
    'dispatch: loop {
        match pc {
            0x82679C18 => {
    //   block [0x82679C18..0x82679C8C)
	// 82679C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679C24: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679C28: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82679C2C: 390B76DC  addi r8, r11, 0x76dc
	ctx.r[8].s64 = ctx.r[11].s64 + 30428;
	// 82679C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679C34: 392A3C44  addi r9, r10, 0x3c44
	ctx.r[9].s64 = ctx.r[10].s64 + 15428;
	// 82679C38: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679C3C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82679C40: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82679C44: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82679C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679C4C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679C54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679C5C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82679C60: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 82679C64: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82679C68: 386B5AA8  addi r3, r11, 0x5aa8
	ctx.r[3].s64 = ctx.r[11].s64 + 23208;
	// 82679C6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82679C70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679C74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679C78: 4BDED1A9  bl 0x82466e20
	ctx.lr = 0x82679C7C;
	sub_82466E20(ctx, base);
	// 82679C7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679C80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679C84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679C90 size=108
    let mut pc: u32 = 0x82679C90;
    'dispatch: loop {
        match pc {
            0x82679C90 => {
    //   block [0x82679C90..0x82679CFC)
	// 82679C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679C98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679C9C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679CA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679CA4: 38EB76F8  addi r7, r11, 0x76f8
	ctx.r[7].s64 = ctx.r[11].s64 + 30456;
	// 82679CA8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82679CAC: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 82679CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679CB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679CB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679CC0: 386A5AD8  addi r3, r10, 0x5ad8
	ctx.r[3].s64 = ctx.r[10].s64 + 23256;
	// 82679CC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679CC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679CCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679CD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679CD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679CD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679CDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679CE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679CE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679CE8: 4BDED139  bl 0x82466e20
	ctx.lr = 0x82679CEC;
	sub_82466E20(ctx, base);
	// 82679CEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679D00 size=108
    let mut pc: u32 = 0x82679D00;
    'dispatch: loop {
        match pc {
            0x82679D00 => {
    //   block [0x82679D00..0x82679D6C)
	// 82679D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679D0C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679D10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679D14: 38EB7740  addi r7, r11, 0x7740
	ctx.r[7].s64 = ctx.r[11].s64 + 30528;
	// 82679D18: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82679D1C: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 82679D20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679D24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679D28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679D30: 386A5B08  addi r3, r10, 0x5b08
	ctx.r[3].s64 = ctx.r[10].s64 + 23304;
	// 82679D34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679D38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679D3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679D40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679D44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679D48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679D4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679D50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679D54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679D58: 4BDED0C9  bl 0x82466e20
	ctx.lr = 0x82679D5C;
	sub_82466E20(ctx, base);
	// 82679D5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679D60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679D64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679D70 size=108
    let mut pc: u32 = 0x82679D70;
    'dispatch: loop {
        match pc {
            0x82679D70 => {
    //   block [0x82679D70..0x82679DDC)
	// 82679D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679D7C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679D80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679D84: 38EB77D0  addi r7, r11, 0x77d0
	ctx.r[7].s64 = ctx.r[11].s64 + 30672;
	// 82679D88: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82679D8C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 82679D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679D94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679D98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679D9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679DA0: 386A5B38  addi r3, r10, 0x5b38
	ctx.r[3].s64 = ctx.r[10].s64 + 23352;
	// 82679DA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679DA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679DBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679DC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679DC8: 4BDED059  bl 0x82466e20
	ctx.lr = 0x82679DCC;
	sub_82466E20(ctx, base);
	// 82679DCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679DE0 size=100
    let mut pc: u32 = 0x82679DE0;
    'dispatch: loop {
        match pc {
            0x82679DE0 => {
    //   block [0x82679DE0..0x82679E44)
	// 82679DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679DEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679DF4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82679DF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679E00: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 82679E04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679E14: 386A5B68  addi r3, r10, 0x5b68
	ctx.r[3].s64 = ctx.r[10].s64 + 23400;
	// 82679E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679E1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679E20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82679E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679E28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82679E2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679E30: 4BDECFF1  bl 0x82466e20
	ctx.lr = 0x82679E34;
	sub_82466E20(ctx, base);
	// 82679E34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679E48 size=112
    let mut pc: u32 = 0x82679E48;
    'dispatch: loop {
        match pc {
            0x82679E48 => {
    //   block [0x82679E48..0x82679EB8)
	// 82679E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679E54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679E58: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679E5C: 38AA5B68  addi r5, r10, 0x5b68
	ctx.r[5].s64 = ctx.r[10].s64 + 23400;
	// 82679E60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679E64: 390B7860  addi r8, r11, 0x7860
	ctx.r[8].s64 = ctx.r[11].s64 + 30816;
	// 82679E68: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82679E6C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 82679E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679E74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679E78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82679E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679E80: 386A5B98  addi r3, r10, 0x5b98
	ctx.r[3].s64 = ctx.r[10].s64 + 23448;
	// 82679E84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82679E88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679E90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679E94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679E98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679E9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679EA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679EA4: 4BDECF7D  bl 0x82466e20
	ctx.lr = 0x82679EA8;
	sub_82466E20(ctx, base);
	// 82679EA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679EB8 size=108
    let mut pc: u32 = 0x82679EB8;
    'dispatch: loop {
        match pc {
            0x82679EB8 => {
    //   block [0x82679EB8..0x82679F24)
	// 82679EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679EC4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679EC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679ECC: 38EB78C0  addi r7, r11, 0x78c0
	ctx.r[7].s64 = ctx.r[11].s64 + 30912;
	// 82679ED0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82679ED4: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 82679ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679EDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679EE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679EE8: 386A5BC8  addi r3, r10, 0x5bc8
	ctx.r[3].s64 = ctx.r[10].s64 + 23496;
	// 82679EEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679EF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679EF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679F00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679F08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679F0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679F10: 4BDECF11  bl 0x82466e20
	ctx.lr = 0x82679F14;
	sub_82466E20(ctx, base);
	// 82679F14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679F28 size=108
    let mut pc: u32 = 0x82679F28;
    'dispatch: loop {
        match pc {
            0x82679F28 => {
    //   block [0x82679F28..0x82679F94)
	// 82679F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679F34: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679F38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679F3C: 38EB78F0  addi r7, r11, 0x78f0
	ctx.r[7].s64 = ctx.r[11].s64 + 30960;
	// 82679F40: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82679F44: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 82679F48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679F4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679F50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679F58: 386A5BF8  addi r3, r10, 0x5bf8
	ctx.r[3].s64 = ctx.r[10].s64 + 23544;
	// 82679F5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679F60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679F68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679F70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679F74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679F78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679F7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679F80: 4BDECEA1  bl 0x82466e20
	ctx.lr = 0x82679F84;
	sub_82466E20(ctx, base);
	// 82679F84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679F98 size=108
    let mut pc: u32 = 0x82679F98;
    'dispatch: loop {
        match pc {
            0x82679F98 => {
    //   block [0x82679F98..0x8267A004)
	// 82679F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679FA4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679FA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679FAC: 38EB7950  addi r7, r11, 0x7950
	ctx.r[7].s64 = ctx.r[11].s64 + 31056;
	// 82679FB0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82679FB4: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 82679FB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679FBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679FC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679FC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679FC8: 386A5C28  addi r3, r10, 0x5c28
	ctx.r[3].s64 = ctx.r[10].s64 + 23592;
	// 82679FCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679FD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679FD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679FDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679FE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679FE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679FE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679FEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679FF0: 4BDECE31  bl 0x82466e20
	ctx.lr = 0x82679FF4;
	sub_82466E20(ctx, base);
	// 82679FF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267A008 size=24
    let mut pc: u32 = 0x8267A008;
    'dispatch: loop {
        match pc {
            0x8267A008 => {
    //   block [0x8267A008..0x8267A020)
	// 8267A008: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A00C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267A010: 394AE498  addi r10, r10, -0x1b68
	ctx.r[10].s64 = ctx.r[10].s64 + -7016;
	// 8267A014: 816B76F4  lwz r11, 0x76f4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30452 as u32) ) } as u64;
	// 8267A018: 916A0128  stw r11, 0x128(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(296 as u32), ctx.r[11].u32 ) };
	// 8267A01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A020 size=116
    let mut pc: u32 = 0x8267A020;
    'dispatch: loop {
        match pc {
            0x8267A020 => {
    //   block [0x8267A020..0x8267A094)
	// 8267A020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A02C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267A030: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267A034: 390BE498  addi r8, r11, -0x1b68
	ctx.r[8].s64 = ctx.r[11].s64 + -7016;
	// 8267A038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A03C: 392A3C78  addi r9, r10, 0x3c78
	ctx.r[9].s64 = ctx.r[10].s64 + 15480;
	// 8267A040: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A044: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 8267A048: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267A04C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A054: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A05C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A064: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267A068: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 8267A06C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267A070: 386B5C58  addi r3, r11, 0x5c58
	ctx.r[3].s64 = ctx.r[11].s64 + 23640;
	// 8267A074: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267A078: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A07C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A080: 4BDECDA1  bl 0x82466e20
	ctx.lr = 0x8267A084;
	sub_82466E20(ctx, base);
	// 8267A084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A08C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A098 size=112
    let mut pc: u32 = 0x8267A098;
    'dispatch: loop {
        match pc {
            0x8267A098 => {
    //   block [0x8267A098..0x8267A108)
	// 8267A098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A0A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A0A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A0A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A0AC: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267A0B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A0B4: 390B79B0  addi r8, r11, 0x79b0
	ctx.r[8].s64 = ctx.r[11].s64 + 31152;
	// 8267A0B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267A0BC: 388A2DA4  addi r4, r10, 0x2da4
	ctx.r[4].s64 = ctx.r[10].s64 + 11684;
	// 8267A0C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A0C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A0C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A0CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A0D0: 386A5C88  addi r3, r10, 0x5c88
	ctx.r[3].s64 = ctx.r[10].s64 + 23688;
	// 8267A0D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267A0D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A0DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A0E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A0E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A0E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A0EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A0F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A0F4: 4BDECD2D  bl 0x82466e20
	ctx.lr = 0x8267A0F8;
	sub_82466E20(ctx, base);
	// 8267A0F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A0FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A108 size=112
    let mut pc: u32 = 0x8267A108;
    'dispatch: loop {
        match pc {
            0x8267A108 => {
    //   block [0x8267A108..0x8267A178)
	// 8267A108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A114: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A118: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A11C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267A120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A124: 390B79F8  addi r8, r11, 0x79f8
	ctx.r[8].s64 = ctx.r[11].s64 + 31224;
	// 8267A128: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267A12C: 388A2DBC  addi r4, r10, 0x2dbc
	ctx.r[4].s64 = ctx.r[10].s64 + 11708;
	// 8267A130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A134: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A138: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A13C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A140: 386A5CB8  addi r3, r10, 0x5cb8
	ctx.r[3].s64 = ctx.r[10].s64 + 23736;
	// 8267A144: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267A148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A14C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A15C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A164: 4BDECCBD  bl 0x82466e20
	ctx.lr = 0x8267A168;
	sub_82466E20(ctx, base);
	// 8267A168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A16C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A178 size=112
    let mut pc: u32 = 0x8267A178;
    'dispatch: loop {
        match pc {
            0x8267A178 => {
    //   block [0x8267A178..0x8267A1E8)
	// 8267A178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A184: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A188: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A18C: 38AA5DD8  addi r5, r10, 0x5dd8
	ctx.r[5].s64 = ctx.r[10].s64 + 24024;
	// 8267A190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A194: 390B7A40  addi r8, r11, 0x7a40
	ctx.r[8].s64 = ctx.r[11].s64 + 31296;
	// 8267A198: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8267A19C: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 8267A1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A1A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A1A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A1AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A1B0: 386A5CE8  addi r3, r10, 0x5ce8
	ctx.r[3].s64 = ctx.r[10].s64 + 23784;
	// 8267A1B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267A1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A1BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A1C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A1D4: 4BDECC4D  bl 0x82466e20
	ctx.lr = 0x8267A1D8;
	sub_82466E20(ctx, base);
	// 8267A1D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A1DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A1E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A1E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A1E8 size=112
    let mut pc: u32 = 0x8267A1E8;
    'dispatch: loop {
        match pc {
            0x8267A1E8 => {
    //   block [0x8267A1E8..0x8267A258)
	// 8267A1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A1F4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267A1F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A1FC: 392B3CAC  addi r9, r11, 0x3cac
	ctx.r[9].s64 = ctx.r[11].s64 + 15532;
	// 8267A200: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 8267A204: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8267A208: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A20C: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 8267A210: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A214: 396B7B20  addi r11, r11, 0x7b20
	ctx.r[11].s64 = ctx.r[11].s64 + 31520;
	// 8267A218: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8267A21C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A220: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8267A224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A228: 386A5D18  addi r3, r10, 0x5d18
	ctx.r[3].s64 = ctx.r[10].s64 + 23832;
	// 8267A22C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267A230: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8267A234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A238: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8267A23C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267A240: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267A244: 4BDECBDD  bl 0x82466e20
	ctx.lr = 0x8267A248;
	sub_82466E20(ctx, base);
	// 8267A248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A258 size=112
    let mut pc: u32 = 0x8267A258;
    'dispatch: loop {
        match pc {
            0x8267A258 => {
    //   block [0x8267A258..0x8267A2C8)
	// 8267A258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A264: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A268: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A26C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267A270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A274: 390B7C70  addi r8, r11, 0x7c70
	ctx.r[8].s64 = ctx.r[11].s64 + 31856;
	// 8267A278: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8267A27C: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 8267A280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A284: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A290: 386A5D48  addi r3, r10, 0x5d48
	ctx.r[3].s64 = ctx.r[10].s64 + 23880;
	// 8267A294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267A298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A29C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A2A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A2B4: 4BDECB6D  bl 0x82466e20
	ctx.lr = 0x8267A2B8;
	sub_82466E20(ctx, base);
	// 8267A2B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A2C8 size=112
    let mut pc: u32 = 0x8267A2C8;
    'dispatch: loop {
        match pc {
            0x8267A2C8 => {
    //   block [0x8267A2C8..0x8267A338)
	// 8267A2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A2D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A2D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A2D8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A2DC: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267A2E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A2E4: 390B7D18  addi r8, r11, 0x7d18
	ctx.r[8].s64 = ctx.r[11].s64 + 32024;
	// 8267A2E8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8267A2EC: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 8267A2F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A2F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A2F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A300: 386A5D78  addi r3, r10, 0x5d78
	ctx.r[3].s64 = ctx.r[10].s64 + 23928;
	// 8267A304: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267A308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A30C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A324: 4BDECAFD  bl 0x82466e20
	ctx.lr = 0x8267A328;
	sub_82466E20(ctx, base);
	// 8267A328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A32C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A338 size=112
    let mut pc: u32 = 0x8267A338;
    'dispatch: loop {
        match pc {
            0x8267A338 => {
    //   block [0x8267A338..0x8267A3A8)
	// 8267A338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A344: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267A348: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A34C: 392A3D08  addi r9, r10, 0x3d08
	ctx.r[9].s64 = ctx.r[10].s64 + 15624;
	// 8267A350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A354: 390B7DA8  addi r8, r11, 0x7da8
	ctx.r[8].s64 = ctx.r[11].s64 + 32168;
	// 8267A358: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8267A35C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 8267A360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A364: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A36C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A370: 386A5DA8  addi r3, r10, 0x5da8
	ctx.r[3].s64 = ctx.r[10].s64 + 23976;
	// 8267A374: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267A378: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267A37C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A38C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267A390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A394: 4BDECA8D  bl 0x82466e20
	ctx.lr = 0x8267A398;
	sub_82466E20(ctx, base);
	// 8267A398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A39C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A3A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A3A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A3A8 size=100
    let mut pc: u32 = 0x8267A3A8;
    'dispatch: loop {
        match pc {
            0x8267A3A8 => {
    //   block [0x8267A3A8..0x8267A40C)
	// 8267A3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A3B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A3B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A3BC: 38AA64F8  addi r5, r10, 0x64f8
	ctx.r[5].s64 = ctx.r[10].s64 + 25848;
	// 8267A3C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A3C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A3C8: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 8267A3CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A3D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A3DC: 386A5DD8  addi r3, r10, 0x5dd8
	ctx.r[3].s64 = ctx.r[10].s64 + 24024;
	// 8267A3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A3E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A3E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267A3EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A3F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267A3F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A3F8: 4BDECA29  bl 0x82466e20
	ctx.lr = 0x8267A3FC;
	sub_82466E20(ctx, base);
	// 8267A3FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A410 size=112
    let mut pc: u32 = 0x8267A410;
    'dispatch: loop {
        match pc {
            0x8267A410 => {
    //   block [0x8267A410..0x8267A480)
	// 8267A410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A41C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A420: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A424: 38AA5DD8  addi r5, r10, 0x5dd8
	ctx.r[5].s64 = ctx.r[10].s64 + 24024;
	// 8267A428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A42C: 390B7DD8  addi r8, r11, 0x7dd8
	ctx.r[8].s64 = ctx.r[11].s64 + 32216;
	// 8267A430: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267A434: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 8267A438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A43C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A448: 386A5E08  addi r3, r10, 0x5e08
	ctx.r[3].s64 = ctx.r[10].s64 + 24072;
	// 8267A44C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267A450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A45C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A46C: 4BDEC9B5  bl 0x82466e20
	ctx.lr = 0x8267A470;
	sub_82466E20(ctx, base);
	// 8267A470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A480 size=116
    let mut pc: u32 = 0x8267A480;
    'dispatch: loop {
        match pc {
            0x8267A480 => {
    //   block [0x8267A480..0x8267A4F4)
	// 8267A480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A48C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8267A490: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8267A494: 390A7E08  addi r8, r10, 0x7e08
	ctx.r[8].s64 = ctx.r[10].s64 + 32264;
	// 8267A498: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A49C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267A4A0: 38AA5DD8  addi r5, r10, 0x5dd8
	ctx.r[5].s64 = ctx.r[10].s64 + 24024;
	// 8267A4A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A4A8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267A4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A4B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A4B4: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 8267A4B8: 396B3D1C  addi r11, r11, 0x3d1c
	ctx.r[11].s64 = ctx.r[11].s64 + 15644;
	// 8267A4BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A4C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A4C4: 386A5E38  addi r3, r10, 0x5e38
	ctx.r[3].s64 = ctx.r[10].s64 + 24120;
	// 8267A4C8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267A4CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A4D0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267A4D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A4D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A4DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A4E0: 4BDEC941  bl 0x82466e20
	ctx.lr = 0x8267A4E4;
	sub_82466E20(ctx, base);
	// 8267A4E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A4E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A4EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A4F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A4F8 size=100
    let mut pc: u32 = 0x8267A4F8;
    'dispatch: loop {
        match pc {
            0x8267A4F8 => {
    //   block [0x8267A4F8..0x8267A55C)
	// 8267A4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A504: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A50C: 38AA5E38  addi r5, r10, 0x5e38
	ctx.r[5].s64 = ctx.r[10].s64 + 24120;
	// 8267A510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A518: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 8267A51C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A52C: 386A5E68  addi r3, r10, 0x5e68
	ctx.r[3].s64 = ctx.r[10].s64 + 24168;
	// 8267A530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A534: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A538: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267A53C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A540: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267A544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A548: 4BDEC8D9  bl 0x82466e20
	ctx.lr = 0x8267A54C;
	sub_82466E20(ctx, base);
	// 8267A54C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267A560 size=24
    let mut pc: u32 = 0x8267A560;
    'dispatch: loop {
        match pc {
            0x8267A560 => {
    //   block [0x8267A560..0x8267A578)
	// 8267A560: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A564: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267A568: 394AE5D0  addi r10, r10, -0x1a30
	ctx.r[10].s64 = ctx.r[10].s64 + -6704;
	// 8267A56C: 816B7EB0  lwz r11, 0x7eb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32432 as u32) ) } as u64;
	// 8267A570: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8267A574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A578 size=116
    let mut pc: u32 = 0x8267A578;
    'dispatch: loop {
        match pc {
            0x8267A578 => {
    //   block [0x8267A578..0x8267A5EC)
	// 8267A578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A584: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267A588: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A58C: 392B3D58  addi r9, r11, 0x3d58
	ctx.r[9].s64 = ctx.r[11].s64 + 15704;
	// 8267A590: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267A594: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A598: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8267A59C: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 8267A5A0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267A5A4: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 8267A5A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A5AC: 396BE5D0  addi r11, r11, -0x1a30
	ctx.r[11].s64 = ctx.r[11].s64 + -6704;
	// 8267A5B0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8267A5B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A5B8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8267A5BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A5C0: 386A5E98  addi r3, r10, 0x5e98
	ctx.r[3].s64 = ctx.r[10].s64 + 24216;
	// 8267A5C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267A5C8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8267A5CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A5D0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8267A5D4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267A5D8: 4BDEC849  bl 0x82466e20
	ctx.lr = 0x8267A5DC;
	sub_82466E20(ctx, base);
	// 8267A5DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A5E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A5E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A5F0 size=116
    let mut pc: u32 = 0x8267A5F0;
    'dispatch: loop {
        match pc {
            0x8267A5F0 => {
    //   block [0x8267A5F0..0x8267A664)
	// 8267A5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A5F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A5FC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267A600: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A604: 392B3D9C  addi r9, r11, 0x3d9c
	ctx.r[9].s64 = ctx.r[11].s64 + 15772;
	// 8267A608: 38AA5DD8  addi r5, r10, 0x5dd8
	ctx.r[5].s64 = ctx.r[10].s64 + 24024;
	// 8267A60C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A610: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8267A614: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8267A618: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A61C: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 8267A620: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A624: 396B7EB8  addi r11, r11, 0x7eb8
	ctx.r[11].s64 = ctx.r[11].s64 + 32440;
	// 8267A628: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8267A62C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A630: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8267A634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A638: 386A5EC8  addi r3, r10, 0x5ec8
	ctx.r[3].s64 = ctx.r[10].s64 + 24264;
	// 8267A63C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267A640: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8267A644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A648: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8267A64C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267A650: 4BDEC7D1  bl 0x82466e20
	ctx.lr = 0x8267A654;
	sub_82466E20(ctx, base);
	// 8267A654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A65C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A668 size=108
    let mut pc: u32 = 0x8267A668;
    'dispatch: loop {
        match pc {
            0x8267A668 => {
    //   block [0x8267A668..0x8267A6D4)
	// 8267A668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A674: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A67C: 38EB7F60  addi r7, r11, 0x7f60
	ctx.r[7].s64 = ctx.r[11].s64 + 32608;
	// 8267A680: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267A684: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 8267A688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A68C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A690: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267A694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A698: 386A5EF8  addi r3, r10, 0x5ef8
	ctx.r[3].s64 = ctx.r[10].s64 + 24312;
	// 8267A69C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267A6A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A6A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A6A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A6AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A6B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A6B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A6B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A6BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267A6C0: 4BDEC761  bl 0x82466e20
	ctx.lr = 0x8267A6C4;
	sub_82466E20(ctx, base);
	// 8267A6C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A6C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A6CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A6D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267A6D8 size=24
    let mut pc: u32 = 0x8267A6D8;
    'dispatch: loop {
        match pc {
            0x8267A6D8 => {
    //   block [0x8267A6D8..0x8267A6F0)
	// 8267A6D8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A6DC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267A6E0: 394AE648  addi r10, r10, -0x19b8
	ctx.r[10].s64 = ctx.r[10].s64 + -6584;
	// 8267A6E4: 816B7FA8  lwz r11, 0x7fa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32680 as u32) ) } as u64;
	// 8267A6E8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8267A6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A6F0 size=116
    let mut pc: u32 = 0x8267A6F0;
    'dispatch: loop {
        match pc {
            0x8267A6F0 => {
    //   block [0x8267A6F0..0x8267A764)
	// 8267A6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A6FC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267A700: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267A704: 390BE648  addi r8, r11, -0x19b8
	ctx.r[8].s64 = ctx.r[11].s64 + -6584;
	// 8267A708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A70C: 392A3E08  addi r9, r10, 0x3e08
	ctx.r[9].s64 = ctx.r[10].s64 + 15880;
	// 8267A710: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A714: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8267A718: 38AA5DD8  addi r5, r10, 0x5dd8
	ctx.r[5].s64 = ctx.r[10].s64 + 24024;
	// 8267A71C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A724: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A72C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A734: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267A738: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 8267A73C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267A740: 386B5F28  addi r3, r11, 0x5f28
	ctx.r[3].s64 = ctx.r[11].s64 + 24360;
	// 8267A744: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8267A748: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A74C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A750: 4BDEC6D1  bl 0x82466e20
	ctx.lr = 0x8267A754;
	sub_82466E20(ctx, base);
	// 8267A754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A75C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A768 size=112
    let mut pc: u32 = 0x8267A768;
    'dispatch: loop {
        match pc {
            0x8267A768 => {
    //   block [0x8267A768..0x8267A7D8)
	// 8267A768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A774: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A778: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A77C: 38AA5DD8  addi r5, r10, 0x5dd8
	ctx.r[5].s64 = ctx.r[10].s64 + 24024;
	// 8267A780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A784: 390B7FB0  addi r8, r11, 0x7fb0
	ctx.r[8].s64 = ctx.r[11].s64 + 32688;
	// 8267A788: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267A78C: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 8267A790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A794: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A798: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A79C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A7A0: 386A5F58  addi r3, r10, 0x5f58
	ctx.r[3].s64 = ctx.r[10].s64 + 24408;
	// 8267A7A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267A7A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A7AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A7B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A7B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A7C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A7C4: 4BDEC65D  bl 0x82466e20
	ctx.lr = 0x8267A7C8;
	sub_82466E20(ctx, base);
	// 8267A7C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A7CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A7D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A7D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267A7D8 size=24
    let mut pc: u32 = 0x8267A7D8;
    'dispatch: loop {
        match pc {
            0x8267A7D8 => {
    //   block [0x8267A7D8..0x8267A7F0)
	// 8267A7D8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A7DC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267A7E0: 394AE7C8  addi r10, r10, -0x1838
	ctx.r[10].s64 = ctx.r[10].s64 + -6200;
	// 8267A7E4: 816B7FE0  lwz r11, 0x7fe0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32736 as u32) ) } as u64;
	// 8267A7E8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8267A7EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A7F0 size=116
    let mut pc: u32 = 0x8267A7F0;
    'dispatch: loop {
        match pc {
            0x8267A7F0 => {
    //   block [0x8267A7F0..0x8267A864)
	// 8267A7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A7F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A7FC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267A800: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267A804: 390BE7C8  addi r8, r11, -0x1838
	ctx.r[8].s64 = ctx.r[11].s64 + -6200;
	// 8267A808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A80C: 392A3E40  addi r9, r10, 0x3e40
	ctx.r[9].s64 = ctx.r[10].s64 + 15936;
	// 8267A810: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A814: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 8267A818: 38AA5EC8  addi r5, r10, 0x5ec8
	ctx.r[5].s64 = ctx.r[10].s64 + 24264;
	// 8267A81C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A824: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A82C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A834: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267A838: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 8267A83C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267A840: 386B5F88  addi r3, r11, 0x5f88
	ctx.r[3].s64 = ctx.r[11].s64 + 24456;
	// 8267A844: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267A848: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A84C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A850: 4BDEC5D1  bl 0x82466e20
	ctx.lr = 0x8267A854;
	sub_82466E20(ctx, base);
	// 8267A854: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A85C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A868 size=112
    let mut pc: u32 = 0x8267A868;
    'dispatch: loop {
        match pc {
            0x8267A868 => {
    //   block [0x8267A868..0x8267A8D8)
	// 8267A868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A874: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A878: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A87C: 38AA5DD8  addi r5, r10, 0x5dd8
	ctx.r[5].s64 = ctx.r[10].s64 + 24024;
	// 8267A880: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A884: 390B7FE4  addi r8, r11, 0x7fe4
	ctx.r[8].s64 = ctx.r[11].s64 + 32740;
	// 8267A888: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267A88C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 8267A890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A894: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A898: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A89C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A8A0: 386A5FB8  addi r3, r10, 0x5fb8
	ctx.r[3].s64 = ctx.r[10].s64 + 24504;
	// 8267A8A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267A8A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A8AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A8B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A8B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A8B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A8BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A8C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A8C4: 4BDEC55D  bl 0x82466e20
	ctx.lr = 0x8267A8C8;
	sub_82466E20(ctx, base);
	// 8267A8C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A8D8 size=100
    let mut pc: u32 = 0x8267A8D8;
    'dispatch: loop {
        match pc {
            0x8267A8D8 => {
    //   block [0x8267A8D8..0x8267A93C)
	// 8267A8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A8E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A8E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A8E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A8EC: 38AA64F8  addi r5, r10, 0x64f8
	ctx.r[5].s64 = ctx.r[10].s64 + 25848;
	// 8267A8F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A8F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A8F8: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 8267A8FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A90C: 386A5FE8  addi r3, r10, 0x5fe8
	ctx.r[3].s64 = ctx.r[10].s64 + 24552;
	// 8267A910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A914: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A918: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267A91C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A920: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267A924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A928: 4BDEC4F9  bl 0x82466e20
	ctx.lr = 0x8267A92C;
	sub_82466E20(ctx, base);
	// 8267A92C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A940 size=108
    let mut pc: u32 = 0x8267A940;
    'dispatch: loop {
        match pc {
            0x8267A940 => {
    //   block [0x8267A940..0x8267A9AC)
	// 8267A940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A94C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267A950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A954: 38EB8000  addi r7, r11, -0x8000
	ctx.r[7].s64 = ctx.r[11].s64 + -32768;
	// 8267A958: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8267A95C: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 8267A960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A964: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A968: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267A96C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A970: 386A6018  addi r3, r10, 0x6018
	ctx.r[3].s64 = ctx.r[10].s64 + 24600;
	// 8267A974: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267A978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A97C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A98C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A994: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267A998: 4BDEC489  bl 0x82466e20
	ctx.lr = 0x8267A99C;
	sub_82466E20(ctx, base);
	// 8267A99C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A9A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A9A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A9A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A9B0 size=112
    let mut pc: u32 = 0x8267A9B0;
    'dispatch: loop {
        match pc {
            0x8267A9B0 => {
    //   block [0x8267A9B0..0x8267AA20)
	// 8267A9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A9B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A9BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A9C0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267A9C4: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267A9C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A9CC: 390B80D8  addi r8, r11, -0x7f28
	ctx.r[8].s64 = ctx.r[11].s64 + -32552;
	// 8267A9D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267A9D4: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 8267A9D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A9DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A9E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A9E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A9E8: 386A6048  addi r3, r10, 0x6048
	ctx.r[3].s64 = ctx.r[10].s64 + 24648;
	// 8267A9EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267A9F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A9F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A9F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A9FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AA00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267AA04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AA08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AA0C: 4BDEC415  bl 0x82466e20
	ctx.lr = 0x8267AA10;
	sub_82466E20(ctx, base);
	// 8267AA10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AA14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AA18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AA1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AA20 size=108
    let mut pc: u32 = 0x8267AA20;
    'dispatch: loop {
        match pc {
            0x8267AA20 => {
    //   block [0x8267AA20..0x8267AA8C)
	// 8267AA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AA2C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267AA30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AA34: 38EB8108  addi r7, r11, -0x7ef8
	ctx.r[7].s64 = ctx.r[11].s64 + -32504;
	// 8267AA38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267AA3C: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 8267AA40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267AA44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AA48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267AA4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AA50: 386A6078  addi r3, r10, 0x6078
	ctx.r[3].s64 = ctx.r[10].s64 + 24696;
	// 8267AA54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267AA58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AA60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267AA64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AA68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267AA6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AA70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AA74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267AA78: 4BDEC3A9  bl 0x82466e20
	ctx.lr = 0x8267AA7C;
	sub_82466E20(ctx, base);
	// 8267AA7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AA80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AA84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AA88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AA90 size=112
    let mut pc: u32 = 0x8267AA90;
    'dispatch: loop {
        match pc {
            0x8267AA90 => {
    //   block [0x8267AA90..0x8267AB00)
	// 8267AA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AA98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AA9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AAA0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267AAA4: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267AAA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AAAC: 390B8138  addi r8, r11, -0x7ec8
	ctx.r[8].s64 = ctx.r[11].s64 + -32456;
	// 8267AAB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267AAB4: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 8267AAB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267AABC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AAC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267AAC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AAC8: 386A60A8  addi r3, r10, 0x60a8
	ctx.r[3].s64 = ctx.r[10].s64 + 24744;
	// 8267AACC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267AAD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AAD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AAD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267AADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AAE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267AAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AAE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AAEC: 4BDEC335  bl 0x82466e20
	ctx.lr = 0x8267AAF0;
	sub_82466E20(ctx, base);
	// 8267AAF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AAFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AB00 size=112
    let mut pc: u32 = 0x8267AB00;
    'dispatch: loop {
        match pc {
            0x8267AB00 => {
    //   block [0x8267AB00..0x8267AB70)
	// 8267AB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AB08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AB0C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267AB10: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8267AB14: 38EA8150  addi r7, r10, -0x7eb0
	ctx.r[7].s64 = ctx.r[10].s64 + -32432;
	// 8267AB18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AB1C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267AB20: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 8267AB24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AB28: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267AB2C: 396B3E54  addi r11, r11, 0x3e54
	ctx.r[11].s64 = ctx.r[11].s64 + 15956;
	// 8267AB30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267AB34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AB38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AB3C: 386A60D8  addi r3, r10, 0x60d8
	ctx.r[3].s64 = ctx.r[10].s64 + 24792;
	// 8267AB40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AB44: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267AB48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AB4C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267AB50: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AB54: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AB58: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267AB5C: 4BDEC2C5  bl 0x82466e20
	ctx.lr = 0x8267AB60;
	sub_82466E20(ctx, base);
	// 8267AB60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AB64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AB68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AB70 size=108
    let mut pc: u32 = 0x8267AB70;
    'dispatch: loop {
        match pc {
            0x8267AB70 => {
    //   block [0x8267AB70..0x8267ABDC)
	// 8267AB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AB7C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267AB80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AB84: 38EB8228  addi r7, r11, -0x7dd8
	ctx.r[7].s64 = ctx.r[11].s64 + -32216;
	// 8267AB88: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267AB8C: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 8267AB90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267AB94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AB98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267AB9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267ABA0: 386A6108  addi r3, r10, 0x6108
	ctx.r[3].s64 = ctx.r[10].s64 + 24840;
	// 8267ABA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267ABA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267ABAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267ABB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267ABB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267ABB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267ABBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267ABC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267ABC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267ABC8: 4BDEC259  bl 0x82466e20
	ctx.lr = 0x8267ABCC;
	sub_82466E20(ctx, base);
	// 8267ABCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267ABD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267ABD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267ABD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267ABE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267ABE0 size=108
    let mut pc: u32 = 0x8267ABE0;
    'dispatch: loop {
        match pc {
            0x8267ABE0 => {
    //   block [0x8267ABE0..0x8267AC4C)
	// 8267ABE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267ABE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267ABE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267ABEC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267ABF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267ABF4: 38EB8240  addi r7, r11, -0x7dc0
	ctx.r[7].s64 = ctx.r[11].s64 + -32192;
	// 8267ABF8: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8267ABFC: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 8267AC00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267AC04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AC08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267AC0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AC10: 386A6138  addi r3, r10, 0x6138
	ctx.r[3].s64 = ctx.r[10].s64 + 24888;
	// 8267AC14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267AC18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AC1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AC20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267AC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AC28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267AC2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AC30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AC34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267AC38: 4BDEC1E9  bl 0x82466e20
	ctx.lr = 0x8267AC3C;
	sub_82466E20(ctx, base);
	// 8267AC3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AC40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AC44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AC48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AC50 size=112
    let mut pc: u32 = 0x8267AC50;
    'dispatch: loop {
        match pc {
            0x8267AC50 => {
    //   block [0x8267AC50..0x8267ACC0)
	// 8267AC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AC58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AC5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AC60: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267AC64: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267AC68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AC6C: 390B8348  addi r8, r11, -0x7cb8
	ctx.r[8].s64 = ctx.r[11].s64 + -31928;
	// 8267AC70: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 8267AC74: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 8267AC78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267AC7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AC80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267AC84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AC88: 386A6168  addi r3, r10, 0x6168
	ctx.r[3].s64 = ctx.r[10].s64 + 24936;
	// 8267AC8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267AC90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AC94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AC98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267AC9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267ACA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267ACA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267ACA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267ACAC: 4BDEC175  bl 0x82466e20
	ctx.lr = 0x8267ACB0;
	sub_82466E20(ctx, base);
	// 8267ACB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267ACB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267ACB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267ACBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267ACC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267ACC0 size=112
    let mut pc: u32 = 0x8267ACC0;
    'dispatch: loop {
        match pc {
            0x8267ACC0 => {
    //   block [0x8267ACC0..0x8267AD30)
	// 8267ACC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267ACC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267ACC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267ACCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267ACD0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267ACD4: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267ACD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267ACDC: 390B8450  addi r8, r11, -0x7bb0
	ctx.r[8].s64 = ctx.r[11].s64 + -31664;
	// 8267ACE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267ACE4: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 8267ACE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267ACEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267ACF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267ACF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267ACF8: 386A6198  addi r3, r10, 0x6198
	ctx.r[3].s64 = ctx.r[10].s64 + 24984;
	// 8267ACFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267AD00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AD04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AD08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267AD0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AD10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267AD14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AD18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AD1C: 4BDEC105  bl 0x82466e20
	ctx.lr = 0x8267AD20;
	sub_82466E20(ctx, base);
	// 8267AD20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AD24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AD28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AD2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AD30 size=116
    let mut pc: u32 = 0x8267AD30;
    'dispatch: loop {
        match pc {
            0x8267AD30 => {
    //   block [0x8267AD30..0x8267ADA4)
	// 8267AD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AD38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AD3C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267AD40: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8267AD44: 390A8468  addi r8, r10, -0x7b98
	ctx.r[8].s64 = ctx.r[10].s64 + -31640;
	// 8267AD48: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AD4C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267AD50: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267AD54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AD58: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267AD5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AD60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267AD64: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 8267AD68: 396B3E84  addi r11, r11, 0x3e84
	ctx.r[11].s64 = ctx.r[11].s64 + 16004;
	// 8267AD6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AD70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AD74: 386A61C8  addi r3, r10, 0x61c8
	ctx.r[3].s64 = ctx.r[10].s64 + 25032;
	// 8267AD78: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267AD7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AD80: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267AD84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AD88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AD8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AD90: 4BDEC091  bl 0x82466e20
	ctx.lr = 0x8267AD94;
	sub_82466E20(ctx, base);
	// 8267AD94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AD98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AD9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267ADA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267ADA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267ADA8 size=108
    let mut pc: u32 = 0x8267ADA8;
    'dispatch: loop {
        match pc {
            0x8267ADA8 => {
    //   block [0x8267ADA8..0x8267AE14)
	// 8267ADA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267ADAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267ADB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267ADB4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267ADB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267ADBC: 38EB84C8  addi r7, r11, -0x7b38
	ctx.r[7].s64 = ctx.r[11].s64 + -31544;
	// 8267ADC0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267ADC4: 388A3224  addi r4, r10, 0x3224
	ctx.r[4].s64 = ctx.r[10].s64 + 12836;
	// 8267ADC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267ADCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267ADD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267ADD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267ADD8: 386A61F8  addi r3, r10, 0x61f8
	ctx.r[3].s64 = ctx.r[10].s64 + 25080;
	// 8267ADDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267ADE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267ADE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267ADE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267ADEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267ADF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267ADF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267ADF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267ADFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267AE00: 4BDEC021  bl 0x82466e20
	ctx.lr = 0x8267AE04;
	sub_82466E20(ctx, base);
	// 8267AE04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AE08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AE0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AE10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AE18 size=108
    let mut pc: u32 = 0x8267AE18;
    'dispatch: loop {
        match pc {
            0x8267AE18 => {
    //   block [0x8267AE18..0x8267AE84)
	// 8267AE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AE24: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267AE28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AE2C: 38EB8510  addi r7, r11, -0x7af0
	ctx.r[7].s64 = ctx.r[11].s64 + -31472;
	// 8267AE30: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267AE34: 388A3244  addi r4, r10, 0x3244
	ctx.r[4].s64 = ctx.r[10].s64 + 12868;
	// 8267AE38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267AE3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AE40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267AE44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AE48: 386A6228  addi r3, r10, 0x6228
	ctx.r[3].s64 = ctx.r[10].s64 + 25128;
	// 8267AE4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267AE50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AE54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AE58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267AE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AE60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267AE64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AE68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AE6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267AE70: 4BDEBFB1  bl 0x82466e20
	ctx.lr = 0x8267AE74;
	sub_82466E20(ctx, base);
	// 8267AE74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AE78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AE7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AE88 size=112
    let mut pc: u32 = 0x8267AE88;
    'dispatch: loop {
        match pc {
            0x8267AE88 => {
    //   block [0x8267AE88..0x8267AEF8)
	// 8267AE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AE94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AE98: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267AE9C: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267AEA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AEA4: 390B8558  addi r8, r11, -0x7aa8
	ctx.r[8].s64 = ctx.r[11].s64 + -31400;
	// 8267AEA8: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8267AEAC: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 8267AEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267AEB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AEB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267AEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AEC0: 386A6258  addi r3, r10, 0x6258
	ctx.r[3].s64 = ctx.r[10].s64 + 25176;
	// 8267AEC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267AEC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267AED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267AEDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AEE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AEE4: 4BDEBF3D  bl 0x82466e20
	ctx.lr = 0x8267AEE8;
	sub_82466E20(ctx, base);
	// 8267AEE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AEF8 size=112
    let mut pc: u32 = 0x8267AEF8;
    'dispatch: loop {
        match pc {
            0x8267AEF8 => {
    //   block [0x8267AEF8..0x8267AF68)
	// 8267AEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AF04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AF08: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267AF0C: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267AF10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AF14: 390B8630  addi r8, r11, -0x79d0
	ctx.r[8].s64 = ctx.r[11].s64 + -31184;
	// 8267AF18: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 8267AF1C: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 8267AF20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267AF24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AF28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267AF2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AF30: 386A6288  addi r3, r10, 0x6288
	ctx.r[3].s64 = ctx.r[10].s64 + 25224;
	// 8267AF34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267AF38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AF3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AF40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267AF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AF48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267AF4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AF50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AF54: 4BDEBECD  bl 0x82466e20
	ctx.lr = 0x8267AF58;
	sub_82466E20(ctx, base);
	// 8267AF58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AF5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AF60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AF64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AF68 size=112
    let mut pc: u32 = 0x8267AF68;
    'dispatch: loop {
        match pc {
            0x8267AF68 => {
    //   block [0x8267AF68..0x8267AFD8)
	// 8267AF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AF70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AF74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AF78: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267AF7C: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267AF80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AF84: 390B8738  addi r8, r11, -0x78c8
	ctx.r[8].s64 = ctx.r[11].s64 + -30920;
	// 8267AF88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267AF8C: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 8267AF90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267AF94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AF98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267AF9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AFA0: 386A62B8  addi r3, r10, 0x62b8
	ctx.r[3].s64 = ctx.r[10].s64 + 25272;
	// 8267AFA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267AFA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AFAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AFB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267AFB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AFB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267AFBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AFC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AFC4: 4BDEBE5D  bl 0x82466e20
	ctx.lr = 0x8267AFC8;
	sub_82466E20(ctx, base);
	// 8267AFC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AFD8 size=112
    let mut pc: u32 = 0x8267AFD8;
    'dispatch: loop {
        match pc {
            0x8267AFD8 => {
    //   block [0x8267AFD8..0x8267B048)
	// 8267AFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AFE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AFE8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267AFEC: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267AFF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AFF4: 390B8750  addi r8, r11, -0x78b0
	ctx.r[8].s64 = ctx.r[11].s64 + -30896;
	// 8267AFF8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267AFFC: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 8267B000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B004: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B00C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B010: 386A62E8  addi r3, r10, 0x62e8
	ctx.r[3].s64 = ctx.r[10].s64 + 25320;
	// 8267B014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B02C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B034: 4BDEBDED  bl 0x82466e20
	ctx.lr = 0x8267B038;
	sub_82466E20(ctx, base);
	// 8267B038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B03C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B048 size=108
    let mut pc: u32 = 0x8267B048;
    'dispatch: loop {
        match pc {
            0x8267B048 => {
    //   block [0x8267B048..0x8267B0B4)
	// 8267B048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B054: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B05C: 38EB8780  addi r7, r11, -0x7880
	ctx.r[7].s64 = ctx.r[11].s64 + -30848;
	// 8267B060: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8267B064: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 8267B068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B06C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B070: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267B074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B078: 386A6318  addi r3, r10, 0x6318
	ctx.r[3].s64 = ctx.r[10].s64 + 25368;
	// 8267B07C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267B080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B08C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B09C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267B0A0: 4BDEBD81  bl 0x82466e20
	ctx.lr = 0x8267B0A4;
	sub_82466E20(ctx, base);
	// 8267B0A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B0B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267B0B8 size=24
    let mut pc: u32 = 0x8267B0B8;
    'dispatch: loop {
        match pc {
            0x8267B0B8 => {
    //   block [0x8267B0B8..0x8267B0D0)
	// 8267B0B8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267B0BC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267B0C0: 394AE8B8  addi r10, r10, -0x1748
	ctx.r[10].s64 = ctx.r[10].s64 + -5960;
	// 8267B0C4: 816B7FFC  lwz r11, 0x7ffc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32764 as u32) ) } as u64;
	// 8267B0C8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8267B0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B0D0 size=116
    let mut pc: u32 = 0x8267B0D0;
    'dispatch: loop {
        match pc {
            0x8267B0D0 => {
    //   block [0x8267B0D0..0x8267B144)
	// 8267B0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B0D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B0DC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B0E0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267B0E4: 390BE8B8  addi r8, r11, -0x1748
	ctx.r[8].s64 = ctx.r[11].s64 + -5960;
	// 8267B0E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B0EC: 392A3EB0  addi r9, r10, 0x3eb0
	ctx.r[9].s64 = ctx.r[10].s64 + 16048;
	// 8267B0F0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B0F4: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 8267B0F8: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267B0FC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B104: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B10C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B114: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267B118: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 8267B11C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267B120: 386B6348  addi r3, r11, 0x6348
	ctx.r[3].s64 = ctx.r[11].s64 + 25416;
	// 8267B124: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267B128: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B12C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B130: 4BDEBCF1  bl 0x82466e20
	ctx.lr = 0x8267B134;
	sub_82466E20(ctx, base);
	// 8267B134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B13C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B148 size=112
    let mut pc: u32 = 0x8267B148;
    'dispatch: loop {
        match pc {
            0x8267B148 => {
    //   block [0x8267B148..0x8267B1B8)
	// 8267B148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B154: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B158: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B15C: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267B160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B164: 390B87F8  addi r8, r11, -0x7808
	ctx.r[8].s64 = ctx.r[11].s64 + -30728;
	// 8267B168: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267B16C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 8267B170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B174: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B178: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B17C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B180: 386A6378  addi r3, r10, 0x6378
	ctx.r[3].s64 = ctx.r[10].s64 + 25464;
	// 8267B184: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B18C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B19C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B1A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B1A4: 4BDEBC7D  bl 0x82466e20
	ctx.lr = 0x8267B1A8;
	sub_82466E20(ctx, base);
	// 8267B1A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B1AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B1B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B1B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B1B8 size=116
    let mut pc: u32 = 0x8267B1B8;
    'dispatch: loop {
        match pc {
            0x8267B1B8 => {
    //   block [0x8267B1B8..0x8267B22C)
	// 8267B1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B1C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B1C4: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267B1C8: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8267B1CC: 390A8828  addi r8, r10, -0x77d8
	ctx.r[8].s64 = ctx.r[10].s64 + -30680;
	// 8267B1D0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B1D4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267B1D8: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267B1DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B1E0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267B1E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B1E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B1EC: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 8267B1F0: 396B3EC4  addi r11, r11, 0x3ec4
	ctx.r[11].s64 = ctx.r[11].s64 + 16068;
	// 8267B1F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B1F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B1FC: 386A63A8  addi r3, r10, 0x63a8
	ctx.r[3].s64 = ctx.r[10].s64 + 25512;
	// 8267B200: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267B204: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B208: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267B20C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B218: 4BDEBC09  bl 0x82466e20
	ctx.lr = 0x8267B21C;
	sub_82466E20(ctx, base);
	// 8267B21C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B230 size=112
    let mut pc: u32 = 0x8267B230;
    'dispatch: loop {
        match pc {
            0x8267B230 => {
    //   block [0x8267B230..0x8267B2A0)
	// 8267B230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B23C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B240: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B244: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267B248: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B24C: 390B88E8  addi r8, r11, -0x7718
	ctx.r[8].s64 = ctx.r[11].s64 + -30488;
	// 8267B250: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267B254: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 8267B258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B25C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B260: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B268: 386A63D8  addi r3, r10, 0x63d8
	ctx.r[3].s64 = ctx.r[10].s64 + 25560;
	// 8267B26C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B270: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B27C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B28C: 4BDEBB95  bl 0x82466e20
	ctx.lr = 0x8267B290;
	sub_82466E20(ctx, base);
	// 8267B290: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B29C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B2A0 size=108
    let mut pc: u32 = 0x8267B2A0;
    'dispatch: loop {
        match pc {
            0x8267B2A0 => {
    //   block [0x8267B2A0..0x8267B30C)
	// 8267B2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B2A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B2AC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B2B4: 38EB8900  addi r7, r11, -0x7700
	ctx.r[7].s64 = ctx.r[11].s64 + -30464;
	// 8267B2B8: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 8267B2BC: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 8267B2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B2C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B2C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267B2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B2D0: 386A6408  addi r3, r10, 0x6408
	ctx.r[3].s64 = ctx.r[10].s64 + 25608;
	// 8267B2D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267B2D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B2DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B2E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B2EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B2F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267B2F8: 4BDEBB29  bl 0x82466e20
	ctx.lr = 0x8267B2FC;
	sub_82466E20(ctx, base);
	// 8267B2FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B310 size=116
    let mut pc: u32 = 0x8267B310;
    'dispatch: loop {
        match pc {
            0x8267B310 => {
    //   block [0x8267B310..0x8267B384)
	// 8267B310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B31C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267B320: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8267B324: 390A8A38  addi r8, r10, -0x75c8
	ctx.r[8].s64 = ctx.r[10].s64 + -30152;
	// 8267B328: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B32C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267B330: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267B334: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B338: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267B33C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B340: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B344: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 8267B348: 396B3EE8  addi r11, r11, 0x3ee8
	ctx.r[11].s64 = ctx.r[11].s64 + 16104;
	// 8267B34C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B350: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B354: 386A6438  addi r3, r10, 0x6438
	ctx.r[3].s64 = ctx.r[10].s64 + 25656;
	// 8267B358: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267B35C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B360: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267B364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B36C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B370: 4BDEBAB1  bl 0x82466e20
	ctx.lr = 0x8267B374;
	sub_82466E20(ctx, base);
	// 8267B374: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B37C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B388 size=112
    let mut pc: u32 = 0x8267B388;
    'dispatch: loop {
        match pc {
            0x8267B388 => {
    //   block [0x8267B388..0x8267B3F8)
	// 8267B388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B394: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B398: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B39C: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267B3A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B3A4: 390B8AE0  addi r8, r11, -0x7520
	ctx.r[8].s64 = ctx.r[11].s64 + -29984;
	// 8267B3A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267B3AC: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 8267B3B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B3B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B3B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B3BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B3C0: 386A6468  addi r3, r10, 0x6468
	ctx.r[3].s64 = ctx.r[10].s64 + 25704;
	// 8267B3C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B3C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B3CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B3D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B3E4: 4BDEBA3D  bl 0x82466e20
	ctx.lr = 0x8267B3E8;
	sub_82466E20(ctx, base);
	// 8267B3E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B3F8 size=112
    let mut pc: u32 = 0x8267B3F8;
    'dispatch: loop {
        match pc {
            0x8267B3F8 => {
    //   block [0x8267B3F8..0x8267B468)
	// 8267B3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B404: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B408: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B40C: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267B410: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B414: 390B8AF8  addi r8, r11, -0x7508
	ctx.r[8].s64 = ctx.r[11].s64 + -29960;
	// 8267B418: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 8267B41C: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 8267B420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B424: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B428: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B42C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B430: 386A6498  addi r3, r10, 0x6498
	ctx.r[3].s64 = ctx.r[10].s64 + 25752;
	// 8267B434: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B43C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B44C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B454: 4BDEB9CD  bl 0x82466e20
	ctx.lr = 0x8267B458;
	sub_82466E20(ctx, base);
	// 8267B458: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B468 size=112
    let mut pc: u32 = 0x8267B468;
    'dispatch: loop {
        match pc {
            0x8267B468 => {
    //   block [0x8267B468..0x8267B4D8)
	// 8267B468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B474: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B478: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B47C: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267B480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B484: 390B8C18  addi r8, r11, -0x73e8
	ctx.r[8].s64 = ctx.r[11].s64 + -29672;
	// 8267B488: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267B48C: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 8267B490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B494: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B4A0: 386A64C8  addi r3, r10, 0x64c8
	ctx.r[3].s64 = ctx.r[10].s64 + 25800;
	// 8267B4A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B4B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B4C4: 4BDEB95D  bl 0x82466e20
	ctx.lr = 0x8267B4C8;
	sub_82466E20(ctx, base);
	// 8267B4C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B4D8 size=116
    let mut pc: u32 = 0x8267B4D8;
    'dispatch: loop {
        match pc {
            0x8267B4D8 => {
    //   block [0x8267B4D8..0x8267B54C)
	// 8267B4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B4E4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B4E8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267B4EC: 390B8C34  addi r8, r11, -0x73cc
	ctx.r[8].s64 = ctx.r[11].s64 + -29644;
	// 8267B4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B4F4: 392A3F20  addi r9, r10, 0x3f20
	ctx.r[9].s64 = ctx.r[10].s64 + 16160;
	// 8267B4F8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B4FC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8267B500: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267B504: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B50C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B51C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267B520: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 8267B524: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267B528: 386B64F8  addi r3, r11, 0x64f8
	ctx.r[3].s64 = ctx.r[11].s64 + 25848;
	// 8267B52C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267B530: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B538: 4BDEB8E9  bl 0x82466e20
	ctx.lr = 0x8267B53C;
	sub_82466E20(ctx, base);
	// 8267B53C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B550 size=100
    let mut pc: u32 = 0x8267B550;
    'dispatch: loop {
        match pc {
            0x8267B550 => {
    //   block [0x8267B550..0x8267B5B4)
	// 8267B550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B55C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B564: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267B568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B570: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 8267B574: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B584: 386A6528  addi r3, r10, 0x6528
	ctx.r[3].s64 = ctx.r[10].s64 + 25896;
	// 8267B588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B58C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B590: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267B594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B598: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267B59C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B5A0: 4BDEB881  bl 0x82466e20
	ctx.lr = 0x8267B5A4;
	sub_82466E20(ctx, base);
	// 8267B5A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B5A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B5AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B5B8 size=112
    let mut pc: u32 = 0x8267B5B8;
    'dispatch: loop {
        match pc {
            0x8267B5B8 => {
    //   block [0x8267B5B8..0x8267B628)
	// 8267B5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B5C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B5C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B5C8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B5CC: 38AA6528  addi r5, r10, 0x6528
	ctx.r[5].s64 = ctx.r[10].s64 + 25896;
	// 8267B5D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B5D4: 390B8C64  addi r8, r11, -0x739c
	ctx.r[8].s64 = ctx.r[11].s64 + -29596;
	// 8267B5D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267B5DC: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 8267B5E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B5E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B5E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B5EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B5F0: 386A6558  addi r3, r10, 0x6558
	ctx.r[3].s64 = ctx.r[10].s64 + 25944;
	// 8267B5F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B5F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B5FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B60C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B614: 4BDEB80D  bl 0x82466e20
	ctx.lr = 0x8267B618;
	sub_82466E20(ctx, base);
	// 8267B618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B61C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B628 size=112
    let mut pc: u32 = 0x8267B628;
    'dispatch: loop {
        match pc {
            0x8267B628 => {
    //   block [0x8267B628..0x8267B698)
	// 8267B628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B634: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B638: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B63C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267B640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B644: 390B8C80  addi r8, r11, -0x7380
	ctx.r[8].s64 = ctx.r[11].s64 + -29568;
	// 8267B648: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267B64C: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 8267B650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B654: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B658: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B660: 386A6588  addi r3, r10, 0x6588
	ctx.r[3].s64 = ctx.r[10].s64 + 25992;
	// 8267B664: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B66C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B674: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B684: 4BDEB79D  bl 0x82466e20
	ctx.lr = 0x8267B688;
	sub_82466E20(ctx, base);
	// 8267B688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B698 size=112
    let mut pc: u32 = 0x8267B698;
    'dispatch: loop {
        match pc {
            0x8267B698 => {
    //   block [0x8267B698..0x8267B708)
	// 8267B698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B6A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B6A8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B6AC: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267B6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B6B4: 390B8CE0  addi r8, r11, -0x7320
	ctx.r[8].s64 = ctx.r[11].s64 + -29472;
	// 8267B6B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267B6BC: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 8267B6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B6C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B6C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B6CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B6D0: 386A65B8  addi r3, r10, 0x65b8
	ctx.r[3].s64 = ctx.r[10].s64 + 26040;
	// 8267B6D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B6F4: 4BDEB72D  bl 0x82466e20
	ctx.lr = 0x8267B6F8;
	sub_82466E20(ctx, base);
	// 8267B6F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B6FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B708 size=112
    let mut pc: u32 = 0x8267B708;
    'dispatch: loop {
        match pc {
            0x8267B708 => {
    //   block [0x8267B708..0x8267B778)
	// 8267B708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B714: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B718: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B71C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267B720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B724: 390B8D28  addi r8, r11, -0x72d8
	ctx.r[8].s64 = ctx.r[11].s64 + -29400;
	// 8267B728: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267B72C: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 8267B730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B734: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B740: 386A65E8  addi r3, r10, 0x65e8
	ctx.r[3].s64 = ctx.r[10].s64 + 26088;
	// 8267B744: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B74C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B75C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B764: 4BDEB6BD  bl 0x82466e20
	ctx.lr = 0x8267B768;
	sub_82466E20(ctx, base);
	// 8267B768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B778 size=112
    let mut pc: u32 = 0x8267B778;
    'dispatch: loop {
        match pc {
            0x8267B778 => {
    //   block [0x8267B778..0x8267B7E8)
	// 8267B778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B784: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B788: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B78C: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267B790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B794: 390B8D58  addi r8, r11, -0x72a8
	ctx.r[8].s64 = ctx.r[11].s64 + -29352;
	// 8267B798: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8267B79C: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 8267B7A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B7A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B7A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B7AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B7B0: 386A6618  addi r3, r10, 0x6618
	ctx.r[3].s64 = ctx.r[10].s64 + 26136;
	// 8267B7B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B7B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B7BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B7C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B7CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B7D4: 4BDEB64D  bl 0x82466e20
	ctx.lr = 0x8267B7D8;
	sub_82466E20(ctx, base);
	// 8267B7D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B7DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B7E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B7E8 size=100
    let mut pc: u32 = 0x8267B7E8;
    'dispatch: loop {
        match pc {
            0x8267B7E8 => {
    //   block [0x8267B7E8..0x8267B84C)
	// 8267B7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B7F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B7F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B7F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B7FC: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267B800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B808: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 8267B80C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B81C: 386A6648  addi r3, r10, 0x6648
	ctx.r[3].s64 = ctx.r[10].s64 + 26184;
	// 8267B820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B824: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B828: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267B82C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B830: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267B834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B838: 4BDEB5E9  bl 0x82466e20
	ctx.lr = 0x8267B83C;
	sub_82466E20(ctx, base);
	// 8267B83C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B850 size=108
    let mut pc: u32 = 0x8267B850;
    'dispatch: loop {
        match pc {
            0x8267B850 => {
    //   block [0x8267B850..0x8267B8BC)
	// 8267B850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B85C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B864: 38EB8DD0  addi r7, r11, -0x7230
	ctx.r[7].s64 = ctx.r[11].s64 + -29232;
	// 8267B868: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267B86C: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 8267B870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B874: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B878: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267B87C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B880: 386A6678  addi r3, r10, 0x6678
	ctx.r[3].s64 = ctx.r[10].s64 + 26232;
	// 8267B884: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267B888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B8A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B8A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267B8A8: 4BDEB579  bl 0x82466e20
	ctx.lr = 0x8267B8AC;
	sub_82466E20(ctx, base);
	// 8267B8AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B8B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B8B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B8C0 size=112
    let mut pc: u32 = 0x8267B8C0;
    'dispatch: loop {
        match pc {
            0x8267B8C0 => {
    //   block [0x8267B8C0..0x8267B930)
	// 8267B8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B8CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B8D0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B8D4: 38AA6648  addi r5, r10, 0x6648
	ctx.r[5].s64 = ctx.r[10].s64 + 26184;
	// 8267B8D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B8DC: 390B8E00  addi r8, r11, -0x7200
	ctx.r[8].s64 = ctx.r[11].s64 + -29184;
	// 8267B8E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267B8E4: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 8267B8E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B8EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B8F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B8F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B8F8: 386A66A8  addi r3, r10, 0x66a8
	ctx.r[3].s64 = ctx.r[10].s64 + 26280;
	// 8267B8FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B90C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B91C: 4BDEB505  bl 0x82466e20
	ctx.lr = 0x8267B920;
	sub_82466E20(ctx, base);
	// 8267B920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B930 size=108
    let mut pc: u32 = 0x8267B930;
    'dispatch: loop {
        match pc {
            0x8267B930 => {
    //   block [0x8267B930..0x8267B99C)
	// 8267B930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B93C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B944: 38EB8E30  addi r7, r11, -0x71d0
	ctx.r[7].s64 = ctx.r[11].s64 + -29136;
	// 8267B948: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267B94C: 388A34C4  addi r4, r10, 0x34c4
	ctx.r[4].s64 = ctx.r[10].s64 + 13508;
	// 8267B950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B954: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267B95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B960: 386A66D8  addi r3, r10, 0x66d8
	ctx.r[3].s64 = ctx.r[10].s64 + 26328;
	// 8267B964: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267B968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267B988: 4BDEB499  bl 0x82466e20
	ctx.lr = 0x8267B98C;
	sub_82466E20(ctx, base);
	// 8267B98C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B9A0 size=112
    let mut pc: u32 = 0x8267B9A0;
    'dispatch: loop {
        match pc {
            0x8267B9A0 => {
    //   block [0x8267B9A0..0x8267BA10)
	// 8267B9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B9A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B9AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B9B0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B9B4: 38AA6648  addi r5, r10, 0x6648
	ctx.r[5].s64 = ctx.r[10].s64 + 26184;
	// 8267B9B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B9BC: 390B8E60  addi r8, r11, -0x71a0
	ctx.r[8].s64 = ctx.r[11].s64 + -29088;
	// 8267B9C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267B9C4: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 8267B9C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B9CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B9D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B9D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B9D8: 386A6708  addi r3, r10, 0x6708
	ctx.r[3].s64 = ctx.r[10].s64 + 26376;
	// 8267B9DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B9E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B9E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B9E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B9EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B9F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B9F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B9F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B9FC: 4BDEB425  bl 0x82466e20
	ctx.lr = 0x8267BA00;
	sub_82466E20(ctx, base);
	// 8267BA00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BA10 size=108
    let mut pc: u32 = 0x8267BA10;
    'dispatch: loop {
        match pc {
            0x8267BA10 => {
    //   block [0x8267BA10..0x8267BA7C)
	// 8267BA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BA1C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BA20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BA24: 38EB8EA8  addi r7, r11, -0x7158
	ctx.r[7].s64 = ctx.r[11].s64 + -29016;
	// 8267BA28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267BA2C: 388A3514  addi r4, r10, 0x3514
	ctx.r[4].s64 = ctx.r[10].s64 + 13588;
	// 8267BA30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BA34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BA38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267BA3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BA40: 386A6738  addi r3, r10, 0x6738
	ctx.r[3].s64 = ctx.r[10].s64 + 26424;
	// 8267BA44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267BA48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BA4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BA50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BA54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BA58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BA5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BA60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BA64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267BA68: 4BDEB3B9  bl 0x82466e20
	ctx.lr = 0x8267BA6C;
	sub_82466E20(ctx, base);
	// 8267BA6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BA70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BA74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BA78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BA80 size=112
    let mut pc: u32 = 0x8267BA80;
    'dispatch: loop {
        match pc {
            0x8267BA80 => {
    //   block [0x8267BA80..0x8267BAF0)
	// 8267BA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BA8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BA90: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BA94: 38AA6648  addi r5, r10, 0x6648
	ctx.r[5].s64 = ctx.r[10].s64 + 26184;
	// 8267BA98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BA9C: 390B8ED8  addi r8, r11, -0x7128
	ctx.r[8].s64 = ctx.r[11].s64 + -28968;
	// 8267BAA0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267BAA4: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 8267BAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BAAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BAB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267BAB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BAB8: 386A6768  addi r3, r10, 0x6768
	ctx.r[3].s64 = ctx.r[10].s64 + 26472;
	// 8267BABC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267BAC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BAC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BAC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BAD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BAD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BAD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BADC: 4BDEB345  bl 0x82466e20
	ctx.lr = 0x8267BAE0;
	sub_82466E20(ctx, base);
	// 8267BAE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BAF0 size=108
    let mut pc: u32 = 0x8267BAF0;
    'dispatch: loop {
        match pc {
            0x8267BAF0 => {
    //   block [0x8267BAF0..0x8267BB5C)
	// 8267BAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BAFC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BB00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BB04: 38EB8F20  addi r7, r11, -0x70e0
	ctx.r[7].s64 = ctx.r[11].s64 + -28896;
	// 8267BB08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267BB0C: 388A3564  addi r4, r10, 0x3564
	ctx.r[4].s64 = ctx.r[10].s64 + 13668;
	// 8267BB10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BB14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BB18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267BB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BB20: 386A6798  addi r3, r10, 0x6798
	ctx.r[3].s64 = ctx.r[10].s64 + 26520;
	// 8267BB24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267BB28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BB2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BB30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BB34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BB38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BB3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BB40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BB44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267BB48: 4BDEB2D9  bl 0x82466e20
	ctx.lr = 0x8267BB4C;
	sub_82466E20(ctx, base);
	// 8267BB4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BB50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BB54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BB58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BB60 size=112
    let mut pc: u32 = 0x8267BB60;
    'dispatch: loop {
        match pc {
            0x8267BB60 => {
    //   block [0x8267BB60..0x8267BBD0)
	// 8267BB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BB68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BB6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BB70: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BB74: 38AA6648  addi r5, r10, 0x6648
	ctx.r[5].s64 = ctx.r[10].s64 + 26184;
	// 8267BB78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BB7C: 390B8F50  addi r8, r11, -0x70b0
	ctx.r[8].s64 = ctx.r[11].s64 + -28848;
	// 8267BB80: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267BB84: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 8267BB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BB8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BB90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267BB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BB98: 386A67C8  addi r3, r10, 0x67c8
	ctx.r[3].s64 = ctx.r[10].s64 + 26568;
	// 8267BB9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267BBA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BBA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BBAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BBB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BBBC: 4BDEB265  bl 0x82466e20
	ctx.lr = 0x8267BBC0;
	sub_82466E20(ctx, base);
	// 8267BBC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BBD0 size=108
    let mut pc: u32 = 0x8267BBD0;
    'dispatch: loop {
        match pc {
            0x8267BBD0 => {
    //   block [0x8267BBD0..0x8267BC3C)
	// 8267BBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BBD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BBDC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BBE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BBE4: 38EB8F98  addi r7, r11, -0x7068
	ctx.r[7].s64 = ctx.r[11].s64 + -28776;
	// 8267BBE8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267BBEC: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 8267BBF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BBF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BBF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267BBFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BC00: 386A67F8  addi r3, r10, 0x67f8
	ctx.r[3].s64 = ctx.r[10].s64 + 26616;
	// 8267BC04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267BC08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BC0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BC10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BC14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BC18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BC1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BC20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BC24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267BC28: 4BDEB1F9  bl 0x82466e20
	ctx.lr = 0x8267BC2C;
	sub_82466E20(ctx, base);
	// 8267BC2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BC30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BC34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BC38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BC40 size=112
    let mut pc: u32 = 0x8267BC40;
    'dispatch: loop {
        match pc {
            0x8267BC40 => {
    //   block [0x8267BC40..0x8267BCB0)
	// 8267BC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BC48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BC4C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267BC50: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BC54: 392A3F90  addi r9, r10, 0x3f90
	ctx.r[9].s64 = ctx.r[10].s64 + 16272;
	// 8267BC58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BC5C: 390B8FF8  addi r8, r11, -0x7008
	ctx.r[8].s64 = ctx.r[11].s64 + -28680;
	// 8267BC60: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8267BC64: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 8267BC68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BC6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BC70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267BC74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BC78: 386A6828  addi r3, r10, 0x6828
	ctx.r[3].s64 = ctx.r[10].s64 + 26664;
	// 8267BC7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267BC80: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267BC84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BC88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BC8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BC90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BC94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267BC98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BC9C: 4BDEB185  bl 0x82466e20
	ctx.lr = 0x8267BCA0;
	sub_82466E20(ctx, base);
	// 8267BCA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BCA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BCA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BCB0 size=108
    let mut pc: u32 = 0x8267BCB0;
    'dispatch: loop {
        match pc {
            0x8267BCB0 => {
    //   block [0x8267BCB0..0x8267BD1C)
	// 8267BCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BCB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BCBC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BCC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BCC4: 38EB90B8  addi r7, r11, -0x6f48
	ctx.r[7].s64 = ctx.r[11].s64 + -28488;
	// 8267BCC8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8267BCCC: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 8267BCD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BCD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BCD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267BCDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BCE0: 386A6858  addi r3, r10, 0x6858
	ctx.r[3].s64 = ctx.r[10].s64 + 26712;
	// 8267BCE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267BCE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BCEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BCF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BCF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BCF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BCFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BD00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BD04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267BD08: 4BDEB119  bl 0x82466e20
	ctx.lr = 0x8267BD0C;
	sub_82466E20(ctx, base);
	// 8267BD0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BD10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BD14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BD18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BD20 size=116
    let mut pc: u32 = 0x8267BD20;
    'dispatch: loop {
        match pc {
            0x8267BD20 => {
    //   block [0x8267BD20..0x8267BD94)
	// 8267BD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BD28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BD2C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267BD30: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 8267BD34: 390A9130  addi r8, r10, -0x6ed0
	ctx.r[8].s64 = ctx.r[10].s64 + -28368;
	// 8267BD38: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BD3C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267BD40: 38AA5DD8  addi r5, r10, 0x5dd8
	ctx.r[5].s64 = ctx.r[10].s64 + 24024;
	// 8267BD44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BD48: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267BD4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BD50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267BD54: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 8267BD58: 396B3FA8  addi r11, r11, 0x3fa8
	ctx.r[11].s64 = ctx.r[11].s64 + 16296;
	// 8267BD5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BD60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BD64: 386A6888  addi r3, r10, 0x6888
	ctx.r[3].s64 = ctx.r[10].s64 + 26760;
	// 8267BD68: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267BD6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BD70: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267BD74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BD78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BD7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BD80: 4BDEB0A1  bl 0x82466e20
	ctx.lr = 0x8267BD84;
	sub_82466E20(ctx, base);
	// 8267BD84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BD88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BD8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BD90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BD98 size=100
    let mut pc: u32 = 0x8267BD98;
    'dispatch: loop {
        match pc {
            0x8267BD98 => {
    //   block [0x8267BD98..0x8267BDFC)
	// 8267BD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BDA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BDA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BDA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BDAC: 38AA5DD8  addi r5, r10, 0x5dd8
	ctx.r[5].s64 = ctx.r[10].s64 + 24024;
	// 8267BDB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BDB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BDB8: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 8267BDBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BDC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BDC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BDC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BDCC: 386A68B8  addi r3, r10, 0x68b8
	ctx.r[3].s64 = ctx.r[10].s64 + 26808;
	// 8267BDD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BDD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BDD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267BDDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BDE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267BDE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BDE8: 4BDEB039  bl 0x82466e20
	ctx.lr = 0x8267BDEC;
	sub_82466E20(ctx, base);
	// 8267BDEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BDF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BDF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BDF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267BE00 size=24
    let mut pc: u32 = 0x8267BE00;
    'dispatch: loop {
        match pc {
            0x8267BE00 => {
    //   block [0x8267BE00..0x8267BE18)
	// 8267BE00: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BE04: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267BE08: 394AE9C0  addi r10, r10, -0x1640
	ctx.r[10].s64 = ctx.r[10].s64 + -5696;
	// 8267BE0C: 816B92CC  lwz r11, -0x6d34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27956 as u32) ) } as u64;
	// 8267BE10: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8267BE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BE18 size=116
    let mut pc: u32 = 0x8267BE18;
    'dispatch: loop {
        match pc {
            0x8267BE18 => {
    //   block [0x8267BE18..0x8267BE8C)
	// 8267BE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BE24: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BE28: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267BE2C: 390BE9C0  addi r8, r11, -0x1640
	ctx.r[8].s64 = ctx.r[11].s64 + -5696;
	// 8267BE30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BE34: 392A4020  addi r9, r10, 0x4020
	ctx.r[9].s64 = ctx.r[10].s64 + 16416;
	// 8267BE38: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BE3C: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 8267BE40: 38AA68B8  addi r5, r10, 0x68b8
	ctx.r[5].s64 = ctx.r[10].s64 + 26808;
	// 8267BE44: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267BE48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BE4C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BE50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BE54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BE58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BE5C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267BE60: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 8267BE64: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267BE68: 386B68E8  addi r3, r11, 0x68e8
	ctx.r[3].s64 = ctx.r[11].s64 + 26856;
	// 8267BE6C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8267BE70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BE74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BE78: 4BDEAFA9  bl 0x82466e20
	ctx.lr = 0x8267BE7C;
	sub_82466E20(ctx, base);
	// 8267BE7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BE80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BE84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BE88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BE90 size=108
    let mut pc: u32 = 0x8267BE90;
    'dispatch: loop {
        match pc {
            0x8267BE90 => {
    //   block [0x8267BE90..0x8267BEFC)
	// 8267BE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BE98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BE9C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BEA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BEA4: 38EB92D0  addi r7, r11, -0x6d30
	ctx.r[7].s64 = ctx.r[11].s64 + -27952;
	// 8267BEA8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267BEAC: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 8267BEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BEB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BEB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267BEBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BEC0: 386A6918  addi r3, r10, 0x6918
	ctx.r[3].s64 = ctx.r[10].s64 + 26904;
	// 8267BEC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267BEC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BEDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BEE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BEE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267BEE8: 4BDEAF39  bl 0x82466e20
	ctx.lr = 0x8267BEEC;
	sub_82466E20(ctx, base);
	// 8267BEEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BEF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BEF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BEF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267BF00 size=24
    let mut pc: u32 = 0x8267BF00;
    'dispatch: loop {
        match pc {
            0x8267BF00 => {
    //   block [0x8267BF00..0x8267BF18)
	// 8267BF00: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BF04: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267BF08: 394AEAE0  addi r10, r10, -0x1520
	ctx.r[10].s64 = ctx.r[10].s64 + -5408;
	// 8267BF0C: 816B92E8  lwz r11, -0x6d18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27928 as u32) ) } as u64;
	// 8267BF10: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8267BF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BF18 size=112
    let mut pc: u32 = 0x8267BF18;
    'dispatch: loop {
        match pc {
            0x8267BF18 => {
    //   block [0x8267BF18..0x8267BF88)
	// 8267BF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BF20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BF24: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267BF28: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BF2C: 392A4078  addi r9, r10, 0x4078
	ctx.r[9].s64 = ctx.r[10].s64 + 16504;
	// 8267BF30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BF34: 390BEAE0  addi r8, r11, -0x1520
	ctx.r[8].s64 = ctx.r[11].s64 + -5408;
	// 8267BF38: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8267BF3C: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 8267BF40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BF44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BF48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267BF4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BF50: 386A6948  addi r3, r10, 0x6948
	ctx.r[3].s64 = ctx.r[10].s64 + 26952;
	// 8267BF54: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267BF58: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267BF5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BF60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BF64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BF68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BF6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267BF70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BF74: 4BDEAEAD  bl 0x82466e20
	ctx.lr = 0x8267BF78;
	sub_82466E20(ctx, base);
	// 8267BF78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BF7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BF80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BF84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BF88 size=108
    let mut pc: u32 = 0x8267BF88;
    'dispatch: loop {
        match pc {
            0x8267BF88 => {
    //   block [0x8267BF88..0x8267BFF4)
	// 8267BF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BF90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BF94: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BF98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BF9C: 38EB92F0  addi r7, r11, -0x6d10
	ctx.r[7].s64 = ctx.r[11].s64 + -27920;
	// 8267BFA0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8267BFA4: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 8267BFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BFAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BFB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267BFB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BFB8: 386A6978  addi r3, r10, 0x6978
	ctx.r[3].s64 = ctx.r[10].s64 + 27000;
	// 8267BFBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267BFC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BFCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BFD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BFDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267BFE0: 4BDEAE41  bl 0x82466e20
	ctx.lr = 0x8267BFE4;
	sub_82466E20(ctx, base);
	// 8267BFE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BFE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BFEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BFF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BFF8 size=112
    let mut pc: u32 = 0x8267BFF8;
    'dispatch: loop {
        match pc {
            0x8267BFF8 => {
    //   block [0x8267BFF8..0x8267C068)
	// 8267BFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C004: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C008: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C00C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267C010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267C014: 390B9368  addi r8, r11, -0x6c98
	ctx.r[8].s64 = ctx.r[11].s64 + -27800;
	// 8267C018: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267C01C: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 8267C020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C024: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267C02C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C030: 386A69A8  addi r3, r10, 0x69a8
	ctx.r[3].s64 = ctx.r[10].s64 + 27048;
	// 8267C034: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267C038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C03C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C04C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C054: 4BDEADCD  bl 0x82466e20
	ctx.lr = 0x8267C058;
	sub_82466E20(ctx, base);
	// 8267C058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C05C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C068 size=108
    let mut pc: u32 = 0x8267C068;
    'dispatch: loop {
        match pc {
            0x8267C068 => {
    //   block [0x8267C068..0x8267C0D4)
	// 8267C068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C074: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267C07C: 38EB9380  addi r7, r11, -0x6c80
	ctx.r[7].s64 = ctx.r[11].s64 + -27776;
	// 8267C080: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267C084: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 8267C088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C08C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C098: 386A69D8  addi r3, r10, 0x69d8
	ctx.r[3].s64 = ctx.r[10].s64 + 27096;
	// 8267C09C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C0A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C0A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C0A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C0AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C0B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C0B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C0B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C0BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C0C0: 4BDEAD61  bl 0x82466e20
	ctx.lr = 0x8267C0C4;
	sub_82466E20(ctx, base);
	// 8267C0C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C0C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C0CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C0D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C0D8 size=108
    let mut pc: u32 = 0x8267C0D8;
    'dispatch: loop {
        match pc {
            0x8267C0D8 => {
    //   block [0x8267C0D8..0x8267C144)
	// 8267C0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C0E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C0E4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C0E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267C0EC: 38EB93E0  addi r7, r11, -0x6c20
	ctx.r[7].s64 = ctx.r[11].s64 + -27680;
	// 8267C0F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267C0F4: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 8267C0F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C0FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C108: 386A6A08  addi r3, r10, 0x6a08
	ctx.r[3].s64 = ctx.r[10].s64 + 27144;
	// 8267C10C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C11C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C12C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C130: 4BDEACF1  bl 0x82466e20
	ctx.lr = 0x8267C134;
	sub_82466E20(ctx, base);
	// 8267C134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C13C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C148 size=116
    let mut pc: u32 = 0x8267C148;
    'dispatch: loop {
        match pc {
            0x8267C148 => {
    //   block [0x8267C148..0x8267C1BC)
	// 8267C148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C154: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C158: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267C15C: 390B9410  addi r8, r11, -0x6bf0
	ctx.r[8].s64 = ctx.r[11].s64 + -27632;
	// 8267C160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C164: 392A40A4  addi r9, r10, 0x40a4
	ctx.r[9].s64 = ctx.r[10].s64 + 16548;
	// 8267C168: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C16C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8267C170: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267C174: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267C178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C17C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267C180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C18C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267C190: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 8267C194: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267C198: 386B6A38  addi r3, r11, 0x6a38
	ctx.r[3].s64 = ctx.r[11].s64 + 27192;
	// 8267C19C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267C1A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C1A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C1A8: 4BDEAC79  bl 0x82466e20
	ctx.lr = 0x8267C1AC;
	sub_82466E20(ctx, base);
	// 8267C1AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C1B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C1B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C1B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C1C0 size=96
    let mut pc: u32 = 0x8267C1C0;
    'dispatch: loop {
        match pc {
            0x8267C1C0 => {
    //   block [0x8267C1C0..0x8267C220)
	// 8267C1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C1C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C1CC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C1D4: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 8267C1D8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C1DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C1E0: 386A6A68  addi r3, r10, 0x6a68
	ctx.r[3].s64 = ctx.r[10].s64 + 27240;
	// 8267C1E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C1E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C1EC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267C1F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C1F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C200: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267C204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C208: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267C20C: 4BDEAC15  bl 0x82466e20
	ctx.lr = 0x8267C210;
	sub_82466E20(ctx, base);
	// 8267C210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C220 size=112
    let mut pc: u32 = 0x8267C220;
    'dispatch: loop {
        match pc {
            0x8267C220 => {
    //   block [0x8267C220..0x8267C290)
	// 8267C220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C22C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C230: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C234: 38AA6A68  addi r5, r10, 0x6a68
	ctx.r[5].s64 = ctx.r[10].s64 + 27240;
	// 8267C238: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C23C: 390B9428  addi r8, r11, -0x6bd8
	ctx.r[8].s64 = ctx.r[11].s64 + -27608;
	// 8267C240: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267C244: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 8267C248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C24C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C250: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267C254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C258: 386A6A98  addi r3, r10, 0x6a98
	ctx.r[3].s64 = ctx.r[10].s64 + 27288;
	// 8267C25C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267C260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C26C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C27C: 4BDEABA5  bl 0x82466e20
	ctx.lr = 0x8267C280;
	sub_82466E20(ctx, base);
	// 8267C280: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C290 size=112
    let mut pc: u32 = 0x8267C290;
    'dispatch: loop {
        match pc {
            0x8267C290 => {
    //   block [0x8267C290..0x8267C300)
	// 8267C290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C29C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267C2A0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C2A4: 392A40C0  addi r9, r10, 0x40c0
	ctx.r[9].s64 = ctx.r[10].s64 + 16576;
	// 8267C2A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C2AC: 390B9460  addi r8, r11, -0x6ba0
	ctx.r[8].s64 = ctx.r[11].s64 + -27552;
	// 8267C2B0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8267C2B4: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 8267C2B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C2BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C2C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267C2C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C2C8: 386A6AC8  addi r3, r10, 0x6ac8
	ctx.r[3].s64 = ctx.r[10].s64 + 27336;
	// 8267C2CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267C2D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267C2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C2D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C2DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C2E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C2E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C2E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C2EC: 4BDEAB35  bl 0x82466e20
	ctx.lr = 0x8267C2F0;
	sub_82466E20(ctx, base);
	// 8267C2F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C300 size=108
    let mut pc: u32 = 0x8267C300;
    'dispatch: loop {
        match pc {
            0x8267C300 => {
    //   block [0x8267C300..0x8267C36C)
	// 8267C300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C30C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C310: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C314: 38EB9508  addi r7, r11, -0x6af8
	ctx.r[7].s64 = ctx.r[11].s64 + -27384;
	// 8267C318: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267C31C: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 8267C320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C324: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C328: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C32C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C330: 386A6AF8  addi r3, r10, 0x6af8
	ctx.r[3].s64 = ctx.r[10].s64 + 27384;
	// 8267C334: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C33C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C358: 4BDEAAC9  bl 0x82466e20
	ctx.lr = 0x8267C35C;
	sub_82466E20(ctx, base);
	// 8267C35C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C370 size=108
    let mut pc: u32 = 0x8267C370;
    'dispatch: loop {
        match pc {
            0x8267C370 => {
    //   block [0x8267C370..0x8267C3DC)
	// 8267C370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C37C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C380: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C384: 38EB9538  addi r7, r11, -0x6ac8
	ctx.r[7].s64 = ctx.r[11].s64 + -27336;
	// 8267C388: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267C38C: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 8267C390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C394: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C398: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C39C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C3A0: 386A6B28  addi r3, r10, 0x6b28
	ctx.r[3].s64 = ctx.r[10].s64 + 27432;
	// 8267C3A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C3A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C3AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C3B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C3B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C3B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C3BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C3C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C3C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C3C8: 4BDEAA59  bl 0x82466e20
	ctx.lr = 0x8267C3CC;
	sub_82466E20(ctx, base);
	// 8267C3CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C3D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C3D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C3D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267C3E0 size=28
    let mut pc: u32 = 0x8267C3E0;
    'dispatch: loop {
        match pc {
            0x8267C3E0 => {
    //   block [0x8267C3E0..0x8267C3FC)
	// 8267C3E0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C3E4: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267C3E8: 394AEB10  addi r10, r10, -0x14f0
	ctx.r[10].s64 = ctx.r[10].s64 + -5360;
	// 8267C3EC: 816B945C  lwz r11, -0x6ba4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27556 as u32) ) } as u64;
	// 8267C3F0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8267C3F4: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8267C3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C400 size=112
    let mut pc: u32 = 0x8267C400;
    'dispatch: loop {
        match pc {
            0x8267C400 => {
    //   block [0x8267C400..0x8267C470)
	// 8267C400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C40C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267C410: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C414: 392A4240  addi r9, r10, 0x4240
	ctx.r[9].s64 = ctx.r[10].s64 + 16960;
	// 8267C418: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C41C: 390BEB10  addi r8, r11, -0x14f0
	ctx.r[8].s64 = ctx.r[11].s64 + -5360;
	// 8267C420: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8267C424: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 8267C428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C42C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267C434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C438: 386A6B58  addi r3, r10, 0x6b58
	ctx.r[3].s64 = ctx.r[10].s64 + 27480;
	// 8267C43C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267C440: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8267C444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C44C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C45C: 4BDEA9C5  bl 0x82466e20
	ctx.lr = 0x8267C460;
	sub_82466E20(ctx, base);
	// 8267C460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C470 size=108
    let mut pc: u32 = 0x8267C470;
    'dispatch: loop {
        match pc {
            0x8267C470 => {
    //   block [0x8267C470..0x8267C4DC)
	// 8267C470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C47C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C480: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C484: 38EB9570  addi r7, r11, -0x6a90
	ctx.r[7].s64 = ctx.r[11].s64 + -27280;
	// 8267C488: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267C48C: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 8267C490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C494: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C49C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C4A0: 386A6B88  addi r3, r10, 0x6b88
	ctx.r[3].s64 = ctx.r[10].s64 + 27528;
	// 8267C4A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C4C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C4C8: 4BDEA959  bl 0x82466e20
	ctx.lr = 0x8267C4CC;
	sub_82466E20(ctx, base);
	// 8267C4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C4E0 size=108
    let mut pc: u32 = 0x8267C4E0;
    'dispatch: loop {
        match pc {
            0x8267C4E0 => {
    //   block [0x8267C4E0..0x8267C54C)
	// 8267C4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C4EC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C4F0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C4F4: 38EB95A0  addi r7, r11, -0x6a60
	ctx.r[7].s64 = ctx.r[11].s64 + -27232;
	// 8267C4F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267C4FC: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 8267C500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C504: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C508: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C50C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C510: 386A6BB8  addi r3, r10, 0x6bb8
	ctx.r[3].s64 = ctx.r[10].s64 + 27576;
	// 8267C514: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C51C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C538: 4BDEA8E9  bl 0x82466e20
	ctx.lr = 0x8267C53C;
	sub_82466E20(ctx, base);
	// 8267C53C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C550 size=108
    let mut pc: u32 = 0x8267C550;
    'dispatch: loop {
        match pc {
            0x8267C550 => {
    //   block [0x8267C550..0x8267C5BC)
	// 8267C550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C55C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C560: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C564: 38EB95D0  addi r7, r11, -0x6a30
	ctx.r[7].s64 = ctx.r[11].s64 + -27184;
	// 8267C568: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267C56C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 8267C570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C574: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C578: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C57C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C580: 386A6BE8  addi r3, r10, 0x6be8
	ctx.r[3].s64 = ctx.r[10].s64 + 27624;
	// 8267C584: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C58C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C59C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C5A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C5A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C5A8: 4BDEA879  bl 0x82466e20
	ctx.lr = 0x8267C5AC;
	sub_82466E20(ctx, base);
	// 8267C5AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C5B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C5B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C5B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267C5C0 size=24
    let mut pc: u32 = 0x8267C5C0;
    'dispatch: loop {
        match pc {
            0x8267C5C0 => {
    //   block [0x8267C5C0..0x8267C5D8)
	// 8267C5C0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C5C4: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267C5C8: 394AEBD0  addi r10, r10, -0x1430
	ctx.r[10].s64 = ctx.r[10].s64 + -5168;
	// 8267C5CC: 816B95E8  lwz r11, -0x6a18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27160 as u32) ) } as u64;
	// 8267C5D0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8267C5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C5D8 size=112
    let mut pc: u32 = 0x8267C5D8;
    'dispatch: loop {
        match pc {
            0x8267C5D8 => {
    //   block [0x8267C5D8..0x8267C648)
	// 8267C5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C5E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C5E4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267C5E8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C5EC: 392A4294  addi r9, r10, 0x4294
	ctx.r[9].s64 = ctx.r[10].s64 + 17044;
	// 8267C5F0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C5F4: 390BEBD0  addi r8, r11, -0x1430
	ctx.r[8].s64 = ctx.r[11].s64 + -5168;
	// 8267C5F8: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8267C5FC: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 8267C600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C604: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267C60C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C610: 386A6C18  addi r3, r10, 0x6c18
	ctx.r[3].s64 = ctx.r[10].s64 + 27672;
	// 8267C614: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267C618: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267C61C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C62C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C634: 4BDEA7ED  bl 0x82466e20
	ctx.lr = 0x8267C638;
	sub_82466E20(ctx, base);
	// 8267C638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C63C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C648 size=112
    let mut pc: u32 = 0x8267C648;
    'dispatch: loop {
        match pc {
            0x8267C648 => {
    //   block [0x8267C648..0x8267C6B8)
	// 8267C648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C654: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267C658: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C65C: 392A42D0  addi r9, r10, 0x42d0
	ctx.r[9].s64 = ctx.r[10].s64 + 17104;
	// 8267C660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267C664: 390B95F8  addi r8, r11, -0x6a08
	ctx.r[8].s64 = ctx.r[11].s64 + -27144;
	// 8267C668: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8267C66C: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 8267C670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C674: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267C67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C680: 386A6C48  addi r3, r10, 0x6c48
	ctx.r[3].s64 = ctx.r[10].s64 + 27720;
	// 8267C684: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267C688: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8267C68C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C69C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C6A4: 4BDEA77D  bl 0x82466e20
	ctx.lr = 0x8267C6A8;
	sub_82466E20(ctx, base);
	// 8267C6A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C6AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C6B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C6B8 size=108
    let mut pc: u32 = 0x8267C6B8;
    'dispatch: loop {
        match pc {
            0x8267C6B8 => {
    //   block [0x8267C6B8..0x8267C724)
	// 8267C6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C6C4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C6C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C6CC: 38EB9640  addi r7, r11, -0x69c0
	ctx.r[7].s64 = ctx.r[11].s64 + -27072;
	// 8267C6D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267C6D4: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 8267C6D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C6DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C6E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C6E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C6E8: 386A6C78  addi r3, r10, 0x6c78
	ctx.r[3].s64 = ctx.r[10].s64 + 27768;
	// 8267C6EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C6F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C6F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C6F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C70C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C710: 4BDEA711  bl 0x82466e20
	ctx.lr = 0x8267C714;
	sub_82466E20(ctx, base);
	// 8267C714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C71C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C728 size=108
    let mut pc: u32 = 0x8267C728;
    'dispatch: loop {
        match pc {
            0x8267C728 => {
    //   block [0x8267C728..0x8267C794)
	// 8267C728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C734: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C738: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C73C: 38EB9670  addi r7, r11, -0x6990
	ctx.r[7].s64 = ctx.r[11].s64 + -27024;
	// 8267C740: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267C744: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 8267C748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C74C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C750: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C758: 386A6CA8  addi r3, r10, 0x6ca8
	ctx.r[3].s64 = ctx.r[10].s64 + 27816;
	// 8267C75C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C76C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C77C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C780: 4BDEA6A1  bl 0x82466e20
	ctx.lr = 0x8267C784;
	sub_82466E20(ctx, base);
	// 8267C784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C78C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C798 size=112
    let mut pc: u32 = 0x8267C798;
    'dispatch: loop {
        match pc {
            0x8267C798 => {
    //   block [0x8267C798..0x8267C808)
	// 8267C798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C7A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C7A4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267C7A8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C7AC: 392A4308  addi r9, r10, 0x4308
	ctx.r[9].s64 = ctx.r[10].s64 + 17160;
	// 8267C7B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267C7B4: 390B96A0  addi r8, r11, -0x6960
	ctx.r[8].s64 = ctx.r[11].s64 + -26976;
	// 8267C7B8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8267C7BC: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 8267C7C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C7C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C7C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267C7CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C7D0: 386A6CD8  addi r3, r10, 0x6cd8
	ctx.r[3].s64 = ctx.r[10].s64 + 27864;
	// 8267C7D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267C7D8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267C7DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C7E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C7E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C7E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C7EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C7F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C7F4: 4BDEA62D  bl 0x82466e20
	ctx.lr = 0x8267C7F8;
	sub_82466E20(ctx, base);
	// 8267C7F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C7FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C808 size=108
    let mut pc: u32 = 0x8267C808;
    'dispatch: loop {
        match pc {
            0x8267C808 => {
    //   block [0x8267C808..0x8267C874)
	// 8267C808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C814: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C818: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C81C: 38EB9700  addi r7, r11, -0x6900
	ctx.r[7].s64 = ctx.r[11].s64 + -26880;
	// 8267C820: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8267C824: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 8267C828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C82C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C830: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C838: 386A6D08  addi r3, r10, 0x6d08
	ctx.r[3].s64 = ctx.r[10].s64 + 27912;
	// 8267C83C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C84C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C85C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C860: 4BDEA5C1  bl 0x82466e20
	ctx.lr = 0x8267C864;
	sub_82466E20(ctx, base);
	// 8267C864: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C86C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C878 size=108
    let mut pc: u32 = 0x8267C878;
    'dispatch: loop {
        match pc {
            0x8267C878 => {
    //   block [0x8267C878..0x8267C8E4)
	// 8267C878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C884: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C888: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C88C: 38EB9808  addi r7, r11, -0x67f8
	ctx.r[7].s64 = ctx.r[11].s64 + -26616;
	// 8267C890: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267C894: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 8267C898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C89C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C8A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C8A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C8A8: 386A6D38  addi r3, r10, 0x6d38
	ctx.r[3].s64 = ctx.r[10].s64 + 27960;
	// 8267C8AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C8B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C8B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C8B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C8BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C8C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C8C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C8C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C8CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C8D0: 4BDEA551  bl 0x82466e20
	ctx.lr = 0x8267C8D4;
	sub_82466E20(ctx, base);
	// 8267C8D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C8D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C8DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C8E8 size=108
    let mut pc: u32 = 0x8267C8E8;
    'dispatch: loop {
        match pc {
            0x8267C8E8 => {
    //   block [0x8267C8E8..0x8267C954)
	// 8267C8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C8F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C8F4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C8F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267C8FC: 38EB9820  addi r7, r11, -0x67e0
	ctx.r[7].s64 = ctx.r[11].s64 + -26592;
	// 8267C900: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8267C904: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 8267C908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C90C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C910: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C918: 386A6D68  addi r3, r10, 0x6d68
	ctx.r[3].s64 = ctx.r[10].s64 + 28008;
	// 8267C91C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C92C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C93C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C940: 4BDEA4E1  bl 0x82466e20
	ctx.lr = 0x8267C944;
	sub_82466E20(ctx, base);
	// 8267C944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C94C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267C958 size=24
    let mut pc: u32 = 0x8267C958;
    'dispatch: loop {
        match pc {
            0x8267C958 => {
    //   block [0x8267C958..0x8267C970)
	// 8267C958: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C95C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267C960: 394AECA8  addi r10, r10, -0x1358
	ctx.r[10].s64 = ctx.r[10].s64 + -4952;
	// 8267C964: 816B98B0  lwz r11, -0x6750(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26448 as u32) ) } as u64;
	// 8267C968: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8267C96C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C970 size=108
    let mut pc: u32 = 0x8267C970;
    'dispatch: loop {
        match pc {
            0x8267C970 => {
    //   block [0x8267C970..0x8267C9DC)
	// 8267C970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C97C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267C984: 38EBECA8  addi r7, r11, -0x1358
	ctx.r[7].s64 = ctx.r[11].s64 + -4952;
	// 8267C988: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267C98C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 8267C990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C994: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C9A0: 386A6D98  addi r3, r10, 0x6d98
	ctx.r[3].s64 = ctx.r[10].s64 + 28056;
	// 8267C9A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C9A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C9B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C9B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C9BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C9C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C9C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C9C8: 4BDEA459  bl 0x82466e20
	ctx.lr = 0x8267C9CC;
	sub_82466E20(ctx, base);
	// 8267C9CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C9D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C9D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267C9E0 size=24
    let mut pc: u32 = 0x8267C9E0;
    'dispatch: loop {
        match pc {
            0x8267C9E0 => {
    //   block [0x8267C9E0..0x8267C9F8)
	// 8267C9E0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C9E4: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267C9E8: 394AECD8  addi r10, r10, -0x1328
	ctx.r[10].s64 = ctx.r[10].s64 + -4904;
	// 8267C9EC: 816B98B0  lwz r11, -0x6750(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26448 as u32) ) } as u64;
	// 8267C9F0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8267C9F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C9F8 size=108
    let mut pc: u32 = 0x8267C9F8;
    'dispatch: loop {
        match pc {
            0x8267C9F8 => {
    //   block [0x8267C9F8..0x8267CA64)
	// 8267C9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CA00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CA04: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CA08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CA0C: 38EBECD8  addi r7, r11, -0x1328
	ctx.r[7].s64 = ctx.r[11].s64 + -4904;
	// 8267CA10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267CA14: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 8267CA18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CA1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CA20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CA24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CA28: 386A6DC8  addi r3, r10, 0x6dc8
	ctx.r[3].s64 = ctx.r[10].s64 + 28104;
	// 8267CA2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CA30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CA34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CA38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CA3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CA40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CA44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CA48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CA4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CA50: 4BDEA3D1  bl 0x82466e20
	ctx.lr = 0x8267CA54;
	sub_82466E20(ctx, base);
	// 8267CA54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CA58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CA5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CA60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CA68 size=108
    let mut pc: u32 = 0x8267CA68;
    'dispatch: loop {
        match pc {
            0x8267CA68 => {
    //   block [0x8267CA68..0x8267CAD4)
	// 8267CA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CA70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CA74: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CA78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CA7C: 38EB9898  addi r7, r11, -0x6768
	ctx.r[7].s64 = ctx.r[11].s64 + -26472;
	// 8267CA80: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267CA84: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 8267CA88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CA8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CA90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CA94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CA98: 386A6DF8  addi r3, r10, 0x6df8
	ctx.r[3].s64 = ctx.r[10].s64 + 28152;
	// 8267CA9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CAA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CAA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CAA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CAAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CAB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CAB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CAB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CAC0: 4BDEA361  bl 0x82466e20
	ctx.lr = 0x8267CAC4;
	sub_82466E20(ctx, base);
	// 8267CAC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CAC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CAD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267CAD8 size=24
    let mut pc: u32 = 0x8267CAD8;
    'dispatch: loop {
        match pc {
            0x8267CAD8 => {
    //   block [0x8267CAD8..0x8267CAF0)
	// 8267CAD8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CADC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267CAE0: 394AED08  addi r10, r10, -0x12f8
	ctx.r[10].s64 = ctx.r[10].s64 + -4856;
	// 8267CAE4: 816B98B0  lwz r11, -0x6750(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26448 as u32) ) } as u64;
	// 8267CAE8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8267CAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CAF0 size=108
    let mut pc: u32 = 0x8267CAF0;
    'dispatch: loop {
        match pc {
            0x8267CAF0 => {
    //   block [0x8267CAF0..0x8267CB5C)
	// 8267CAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CAFC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CB00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CB04: 38EBED08  addi r7, r11, -0x12f8
	ctx.r[7].s64 = ctx.r[11].s64 + -4856;
	// 8267CB08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267CB0C: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 8267CB10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CB14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CB18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CB20: 386A6E28  addi r3, r10, 0x6e28
	ctx.r[3].s64 = ctx.r[10].s64 + 28200;
	// 8267CB24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CB28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CB2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CB30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CB34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CB38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CB3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CB40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CB44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CB48: 4BDEA2D9  bl 0x82466e20
	ctx.lr = 0x8267CB4C;
	sub_82466E20(ctx, base);
	// 8267CB4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CB50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CB54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CB58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CB60 size=112
    let mut pc: u32 = 0x8267CB60;
    'dispatch: loop {
        match pc {
            0x8267CB60 => {
    //   block [0x8267CB60..0x8267CBD0)
	// 8267CB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CB68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CB6C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267CB70: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CB74: 392A434C  addi r9, r10, 0x434c
	ctx.r[9].s64 = ctx.r[10].s64 + 17228;
	// 8267CB78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CB7C: 390B98B4  addi r8, r11, -0x674c
	ctx.r[8].s64 = ctx.r[11].s64 + -26444;
	// 8267CB80: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8267CB84: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 8267CB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CB8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CB90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267CB94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CB98: 386A6E58  addi r3, r10, 0x6e58
	ctx.r[3].s64 = ctx.r[10].s64 + 28248;
	// 8267CB9C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267CBA0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267CBA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CBAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CBB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CBBC: 4BDEA265  bl 0x82466e20
	ctx.lr = 0x8267CBC0;
	sub_82466E20(ctx, base);
	// 8267CBC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CBD0 size=108
    let mut pc: u32 = 0x8267CBD0;
    'dispatch: loop {
        match pc {
            0x8267CBD0 => {
    //   block [0x8267CBD0..0x8267CC3C)
	// 8267CBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CBD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CBDC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CBE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CBE4: 38EB98E4  addi r7, r11, -0x671c
	ctx.r[7].s64 = ctx.r[11].s64 + -26396;
	// 8267CBE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267CBEC: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 8267CBF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CBF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CBF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CBFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CC00: 386A6E88  addi r3, r10, 0x6e88
	ctx.r[3].s64 = ctx.r[10].s64 + 28296;
	// 8267CC04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CC08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CC0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CC10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CC14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CC18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CC1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CC20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CC24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CC28: 4BDEA1F9  bl 0x82466e20
	ctx.lr = 0x8267CC2C;
	sub_82466E20(ctx, base);
	// 8267CC2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CC30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CC34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CC38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CC40 size=108
    let mut pc: u32 = 0x8267CC40;
    'dispatch: loop {
        match pc {
            0x8267CC40 => {
    //   block [0x8267CC40..0x8267CCAC)
	// 8267CC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CC48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CC4C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CC50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CC54: 38EB9914  addi r7, r11, -0x66ec
	ctx.r[7].s64 = ctx.r[11].s64 + -26348;
	// 8267CC58: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267CC5C: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 8267CC60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CC64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CC68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CC6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CC70: 386A6EB8  addi r3, r10, 0x6eb8
	ctx.r[3].s64 = ctx.r[10].s64 + 28344;
	// 8267CC74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CC78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CC80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CC84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CC88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CC8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CC90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CC94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CC98: 4BDEA189  bl 0x82466e20
	ctx.lr = 0x8267CC9C;
	sub_82466E20(ctx, base);
	// 8267CC9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CCA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CCA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CCA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CCB0 size=108
    let mut pc: u32 = 0x8267CCB0;
    'dispatch: loop {
        match pc {
            0x8267CCB0 => {
    //   block [0x8267CCB0..0x8267CD1C)
	// 8267CCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CCB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CCBC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CCC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CCC4: 38EB992C  addi r7, r11, -0x66d4
	ctx.r[7].s64 = ctx.r[11].s64 + -26324;
	// 8267CCC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267CCCC: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 8267CCD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CCD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CCD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CCDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CCE0: 386A6EE8  addi r3, r10, 0x6ee8
	ctx.r[3].s64 = ctx.r[10].s64 + 28392;
	// 8267CCE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CCE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CCEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CCF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CCF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CCF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CCFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CD00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CD04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CD08: 4BDEA119  bl 0x82466e20
	ctx.lr = 0x8267CD0C;
	sub_82466E20(ctx, base);
	// 8267CD0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CD10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CD14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CD18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CD20 size=112
    let mut pc: u32 = 0x8267CD20;
    'dispatch: loop {
        match pc {
            0x8267CD20 => {
    //   block [0x8267CD20..0x8267CD90)
	// 8267CD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CD28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CD2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CD30: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CD34: 38AA6F48  addi r5, r10, 0x6f48
	ctx.r[5].s64 = ctx.r[10].s64 + 28488;
	// 8267CD38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CD3C: 390B995C  addi r8, r11, -0x66a4
	ctx.r[8].s64 = ctx.r[11].s64 + -26276;
	// 8267CD40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267CD44: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 8267CD48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CD4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CD50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267CD54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CD58: 386A6F18  addi r3, r10, 0x6f18
	ctx.r[3].s64 = ctx.r[10].s64 + 28440;
	// 8267CD5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267CD60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CD64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CD68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CD6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CD70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CD74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CD78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CD7C: 4BDEA0A5  bl 0x82466e20
	ctx.lr = 0x8267CD80;
	sub_82466E20(ctx, base);
	// 8267CD80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CD84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CD88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CD8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CD90 size=108
    let mut pc: u32 = 0x8267CD90;
    'dispatch: loop {
        match pc {
            0x8267CD90 => {
    //   block [0x8267CD90..0x8267CDFC)
	// 8267CD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CD98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CD9C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CDA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CDA4: 38EB9974  addi r7, r11, -0x668c
	ctx.r[7].s64 = ctx.r[11].s64 + -26252;
	// 8267CDA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267CDAC: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 8267CDB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CDB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CDB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CDBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CDC0: 386A6F48  addi r3, r10, 0x6f48
	ctx.r[3].s64 = ctx.r[10].s64 + 28488;
	// 8267CDC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CDC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CDCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CDD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CDD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CDD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CDDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CDE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CDE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CDE8: 4BDEA039  bl 0x82466e20
	ctx.lr = 0x8267CDEC;
	sub_82466E20(ctx, base);
	// 8267CDEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CDF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CDF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CDF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CE00 size=108
    let mut pc: u32 = 0x8267CE00;
    'dispatch: loop {
        match pc {
            0x8267CE00 => {
    //   block [0x8267CE00..0x8267CE6C)
	// 8267CE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CE08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CE0C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CE10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CE14: 38EB99A4  addi r7, r11, -0x665c
	ctx.r[7].s64 = ctx.r[11].s64 + -26204;
	// 8267CE18: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267CE1C: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 8267CE20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CE24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CE28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CE2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CE30: 386A6F78  addi r3, r10, 0x6f78
	ctx.r[3].s64 = ctx.r[10].s64 + 28536;
	// 8267CE34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CE38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CE3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CE40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CE44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CE48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CE4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CE50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CE54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CE58: 4BDE9FC9  bl 0x82466e20
	ctx.lr = 0x8267CE5C;
	sub_82466E20(ctx, base);
	// 8267CE5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CE60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CE64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CE68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CE70 size=108
    let mut pc: u32 = 0x8267CE70;
    'dispatch: loop {
        match pc {
            0x8267CE70 => {
    //   block [0x8267CE70..0x8267CEDC)
	// 8267CE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CE78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CE7C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CE80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CE84: 38EB99BC  addi r7, r11, -0x6644
	ctx.r[7].s64 = ctx.r[11].s64 + -26180;
	// 8267CE88: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267CE8C: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 8267CE90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CE94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CE98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CE9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CEA0: 386A6FA8  addi r3, r10, 0x6fa8
	ctx.r[3].s64 = ctx.r[10].s64 + 28584;
	// 8267CEA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CEA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CEAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CEB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CEB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CEB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CEBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CEC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CEC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CEC8: 4BDE9F59  bl 0x82466e20
	ctx.lr = 0x8267CECC;
	sub_82466E20(ctx, base);
	// 8267CECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CEE0 size=108
    let mut pc: u32 = 0x8267CEE0;
    'dispatch: loop {
        match pc {
            0x8267CEE0 => {
    //   block [0x8267CEE0..0x8267CF4C)
	// 8267CEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CEE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CEE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CEEC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CEF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CEF4: 38EB99F0  addi r7, r11, -0x6610
	ctx.r[7].s64 = ctx.r[11].s64 + -26128;
	// 8267CEF8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8267CEFC: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 8267CF00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CF04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CF08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CF0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CF10: 386A6FD8  addi r3, r10, 0x6fd8
	ctx.r[3].s64 = ctx.r[10].s64 + 28632;
	// 8267CF14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CF18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CF1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CF20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CF24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CF28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CF2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CF30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CF34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CF38: 4BDE9EE9  bl 0x82466e20
	ctx.lr = 0x8267CF3C;
	sub_82466E20(ctx, base);
	// 8267CF3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CF40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CF44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CF48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CF50 size=108
    let mut pc: u32 = 0x8267CF50;
    'dispatch: loop {
        match pc {
            0x8267CF50 => {
    //   block [0x8267CF50..0x8267CFBC)
	// 8267CF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CF58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CF5C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CF60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CF64: 38EB9A98  addi r7, r11, -0x6568
	ctx.r[7].s64 = ctx.r[11].s64 + -25960;
	// 8267CF68: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267CF6C: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 8267CF70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CF74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CF78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CF7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CF80: 386A7008  addi r3, r10, 0x7008
	ctx.r[3].s64 = ctx.r[10].s64 + 28680;
	// 8267CF84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CF88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CF8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CF90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CF94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CF98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CF9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CFA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CFA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CFA8: 4BDE9E79  bl 0x82466e20
	ctx.lr = 0x8267CFAC;
	sub_82466E20(ctx, base);
	// 8267CFAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CFB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CFB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CFB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CFC0 size=108
    let mut pc: u32 = 0x8267CFC0;
    'dispatch: loop {
        match pc {
            0x8267CFC0 => {
    //   block [0x8267CFC0..0x8267D02C)
	// 8267CFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CFC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CFC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CFCC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CFD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CFD4: 38EB9AC8  addi r7, r11, -0x6538
	ctx.r[7].s64 = ctx.r[11].s64 + -25912;
	// 8267CFD8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267CFDC: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 8267CFE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CFE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CFE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CFEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CFF0: 386A7038  addi r3, r10, 0x7038
	ctx.r[3].s64 = ctx.r[10].s64 + 28728;
	// 8267CFF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CFF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CFFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D00C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D018: 4BDE9E09  bl 0x82466e20
	ctx.lr = 0x8267D01C;
	sub_82466E20(ctx, base);
	// 8267D01C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D030 size=108
    let mut pc: u32 = 0x8267D030;
    'dispatch: loop {
        match pc {
            0x8267D030 => {
    //   block [0x8267D030..0x8267D09C)
	// 8267D030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D03C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D044: 38EB9AE0  addi r7, r11, -0x6520
	ctx.r[7].s64 = ctx.r[11].s64 + -25888;
	// 8267D048: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267D04C: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 8267D050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D054: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D058: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D05C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D060: 386A7068  addi r3, r10, 0x7068
	ctx.r[3].s64 = ctx.r[10].s64 + 28776;
	// 8267D064: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D06C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D07C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D088: 4BDE9D99  bl 0x82466e20
	ctx.lr = 0x8267D08C;
	sub_82466E20(ctx, base);
	// 8267D08C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D0A0 size=112
    let mut pc: u32 = 0x8267D0A0;
    'dispatch: loop {
        match pc {
            0x8267D0A0 => {
    //   block [0x8267D0A0..0x8267D110)
	// 8267D0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D0A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D0AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D0B0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D0B4: 38AA6EB8  addi r5, r10, 0x6eb8
	ctx.r[5].s64 = ctx.r[10].s64 + 28344;
	// 8267D0B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D0BC: 390B9B10  addi r8, r11, -0x64f0
	ctx.r[8].s64 = ctx.r[11].s64 + -25840;
	// 8267D0C0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8267D0C4: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 8267D0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D0CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D0D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267D0D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D0D8: 386A7098  addi r3, r10, 0x7098
	ctx.r[3].s64 = ctx.r[10].s64 + 28824;
	// 8267D0DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267D0E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D0E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D0E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D0EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D0F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D0F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D0FC: 4BDE9D25  bl 0x82466e20
	ctx.lr = 0x8267D100;
	sub_82466E20(ctx, base);
	// 8267D100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267D110 size=24
    let mut pc: u32 = 0x8267D110;
    'dispatch: loop {
        match pc {
            0x8267D110 => {
    //   block [0x8267D110..0x8267D128)
	// 8267D110: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D114: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267D118: 394AED38  addi r10, r10, -0x12c8
	ctx.r[10].s64 = ctx.r[10].s64 + -4808;
	// 8267D11C: 816B99EC  lwz r11, -0x6614(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26132 as u32) ) } as u64;
	// 8267D120: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8267D124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D128 size=112
    let mut pc: u32 = 0x8267D128;
    'dispatch: loop {
        match pc {
            0x8267D128 => {
    //   block [0x8267D128..0x8267D198)
	// 8267D128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D134: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267D138: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D13C: 392A4378  addi r9, r10, 0x4378
	ctx.r[9].s64 = ctx.r[10].s64 + 17272;
	// 8267D140: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D144: 390BED38  addi r8, r11, -0x12c8
	ctx.r[8].s64 = ctx.r[11].s64 + -4808;
	// 8267D148: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8267D14C: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 8267D150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D154: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D158: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267D15C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D160: 386A70C8  addi r3, r10, 0x70c8
	ctx.r[3].s64 = ctx.r[10].s64 + 28872;
	// 8267D164: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267D168: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267D16C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D17C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D184: 4BDE9C9D  bl 0x82466e20
	ctx.lr = 0x8267D188;
	sub_82466E20(ctx, base);
	// 8267D188: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D18C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D198 size=108
    let mut pc: u32 = 0x8267D198;
    'dispatch: loop {
        match pc {
            0x8267D198 => {
    //   block [0x8267D198..0x8267D204)
	// 8267D198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D1A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D1A4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D1A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D1AC: 38EB9BBC  addi r7, r11, -0x6444
	ctx.r[7].s64 = ctx.r[11].s64 + -25668;
	// 8267D1B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267D1B4: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 8267D1B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D1BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D1C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D1C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D1C8: 386A70F8  addi r3, r10, 0x70f8
	ctx.r[3].s64 = ctx.r[10].s64 + 28920;
	// 8267D1CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D1D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D1D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D1D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D1E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D1E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D1E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D1EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D1F0: 4BDE9C31  bl 0x82466e20
	ctx.lr = 0x8267D1F4;
	sub_82466E20(ctx, base);
	// 8267D1F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D1F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D1FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D208 size=116
    let mut pc: u32 = 0x8267D208;
    'dispatch: loop {
        match pc {
            0x8267D208 => {
    //   block [0x8267D208..0x8267D27C)
	// 8267D208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D214: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D218: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267D21C: 390B9BF0  addi r8, r11, -0x6410
	ctx.r[8].s64 = ctx.r[11].s64 + -25616;
	// 8267D220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D224: 392A43BC  addi r9, r10, 0x43bc
	ctx.r[9].s64 = ctx.r[10].s64 + 17340;
	// 8267D228: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D22C: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8267D230: 38AA6EB8  addi r5, r10, 0x6eb8
	ctx.r[5].s64 = ctx.r[10].s64 + 28344;
	// 8267D234: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267D238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D23C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D24C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267D250: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 8267D254: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267D258: 386B7128  addi r3, r11, 0x7128
	ctx.r[3].s64 = ctx.r[11].s64 + 28968;
	// 8267D25C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267D260: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D268: 4BDE9BB9  bl 0x82466e20
	ctx.lr = 0x8267D26C;
	sub_82466E20(ctx, base);
	// 8267D26C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267D280 size=24
    let mut pc: u32 = 0x8267D280;
    'dispatch: loop {
        match pc {
            0x8267D280 => {
    //   block [0x8267D280..0x8267D298)
	// 8267D280: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D284: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267D288: 394AEDB0  addi r10, r10, -0x1250
	ctx.r[10].s64 = ctx.r[10].s64 + -4688;
	// 8267D28C: 816B9BEC  lwz r11, -0x6414(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25620 as u32) ) } as u64;
	// 8267D290: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8267D294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D298 size=112
    let mut pc: u32 = 0x8267D298;
    'dispatch: loop {
        match pc {
            0x8267D298 => {
    //   block [0x8267D298..0x8267D308)
	// 8267D298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D2A4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267D2A8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D2AC: 392A43F8  addi r9, r10, 0x43f8
	ctx.r[9].s64 = ctx.r[10].s64 + 17400;
	// 8267D2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D2B4: 390BEDB0  addi r8, r11, -0x1250
	ctx.r[8].s64 = ctx.r[11].s64 + -4688;
	// 8267D2B8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8267D2BC: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 8267D2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D2C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D2C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267D2CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D2D0: 386A7158  addi r3, r10, 0x7158
	ctx.r[3].s64 = ctx.r[10].s64 + 29016;
	// 8267D2D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267D2D8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267D2DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D2E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D2EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D2F4: 4BDE9B2D  bl 0x82466e20
	ctx.lr = 0x8267D2F8;
	sub_82466E20(ctx, base);
	// 8267D2F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D308 size=108
    let mut pc: u32 = 0x8267D308;
    'dispatch: loop {
        match pc {
            0x8267D308 => {
    //   block [0x8267D308..0x8267D374)
	// 8267D308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D314: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D31C: 38EB9CB0  addi r7, r11, -0x6350
	ctx.r[7].s64 = ctx.r[11].s64 + -25424;
	// 8267D320: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267D324: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 8267D328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D32C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D330: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D338: 386A7188  addi r3, r10, 0x7188
	ctx.r[3].s64 = ctx.r[10].s64 + 29064;
	// 8267D33C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D34C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D35C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D360: 4BDE9AC1  bl 0x82466e20
	ctx.lr = 0x8267D364;
	sub_82466E20(ctx, base);
	// 8267D364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D36C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D378 size=108
    let mut pc: u32 = 0x8267D378;
    'dispatch: loop {
        match pc {
            0x8267D378 => {
    //   block [0x8267D378..0x8267D3E4)
	// 8267D378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D384: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D38C: 38EB9CC8  addi r7, r11, -0x6338
	ctx.r[7].s64 = ctx.r[11].s64 + -25400;
	// 8267D390: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267D394: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 8267D398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D39C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D3A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D3A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D3A8: 386A71B8  addi r3, r10, 0x71b8
	ctx.r[3].s64 = ctx.r[10].s64 + 29112;
	// 8267D3AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D3B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D3B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D3BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D3C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D3C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D3CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D3D0: 4BDE9A51  bl 0x82466e20
	ctx.lr = 0x8267D3D4;
	sub_82466E20(ctx, base);
	// 8267D3D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D3D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D3DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267D3E8 size=24
    let mut pc: u32 = 0x8267D3E8;
    'dispatch: loop {
        match pc {
            0x8267D3E8 => {
    //   block [0x8267D3E8..0x8267D400)
	// 8267D3E8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D3EC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267D3F0: 394AEDF8  addi r10, r10, -0x1208
	ctx.r[10].s64 = ctx.r[10].s64 + -4616;
	// 8267D3F4: 816B9CF8  lwz r11, -0x6308(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25352 as u32) ) } as u64;
	// 8267D3F8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8267D3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D400 size=112
    let mut pc: u32 = 0x8267D400;
    'dispatch: loop {
        match pc {
            0x8267D400 => {
    //   block [0x8267D400..0x8267D470)
	// 8267D400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D40C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267D410: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D414: 392A4434  addi r9, r10, 0x4434
	ctx.r[9].s64 = ctx.r[10].s64 + 17460;
	// 8267D418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D41C: 390BEDF8  addi r8, r11, -0x1208
	ctx.r[8].s64 = ctx.r[11].s64 + -4616;
	// 8267D420: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8267D424: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 8267D428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D42C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267D434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D438: 386A71E8  addi r3, r10, 0x71e8
	ctx.r[3].s64 = ctx.r[10].s64 + 29160;
	// 8267D43C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267D440: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267D444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D44C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D45C: 4BDE99C5  bl 0x82466e20
	ctx.lr = 0x8267D460;
	sub_82466E20(ctx, base);
	// 8267D460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D470 size=112
    let mut pc: u32 = 0x8267D470;
    'dispatch: loop {
        match pc {
            0x8267D470 => {
    //   block [0x8267D470..0x8267D4E0)
	// 8267D470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D47C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D480: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D484: 38AA6EB8  addi r5, r10, 0x6eb8
	ctx.r[5].s64 = ctx.r[10].s64 + 28344;
	// 8267D488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D48C: 390B9CFC  addi r8, r11, -0x6304
	ctx.r[8].s64 = ctx.r[11].s64 + -25348;
	// 8267D490: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267D494: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 8267D498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D49C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D4A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267D4A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D4A8: 386A7218  addi r3, r10, 0x7218
	ctx.r[3].s64 = ctx.r[10].s64 + 29208;
	// 8267D4AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267D4B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D4B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D4B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D4BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D4C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D4C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D4C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D4CC: 4BDE9955  bl 0x82466e20
	ctx.lr = 0x8267D4D0;
	sub_82466E20(ctx, base);
	// 8267D4D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D4D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D4D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D4DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D4E0 size=108
    let mut pc: u32 = 0x8267D4E0;
    'dispatch: loop {
        match pc {
            0x8267D4E0 => {
    //   block [0x8267D4E0..0x8267D54C)
	// 8267D4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D4EC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D4F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D4F4: 38EB9D2C  addi r7, r11, -0x62d4
	ctx.r[7].s64 = ctx.r[11].s64 + -25300;
	// 8267D4F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267D4FC: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 8267D500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D504: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D508: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D50C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D510: 386A7248  addi r3, r10, 0x7248
	ctx.r[3].s64 = ctx.r[10].s64 + 29256;
	// 8267D514: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D51C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D538: 4BDE98E9  bl 0x82466e20
	ctx.lr = 0x8267D53C;
	sub_82466E20(ctx, base);
	// 8267D53C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D550 size=108
    let mut pc: u32 = 0x8267D550;
    'dispatch: loop {
        match pc {
            0x8267D550 => {
    //   block [0x8267D550..0x8267D5BC)
	// 8267D550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D55C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D564: 38EB9D60  addi r7, r11, -0x62a0
	ctx.r[7].s64 = ctx.r[11].s64 + -25248;
	// 8267D568: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267D56C: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 8267D570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D574: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D578: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D57C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D580: 386A7278  addi r3, r10, 0x7278
	ctx.r[3].s64 = ctx.r[10].s64 + 29304;
	// 8267D584: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D58C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D59C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D5A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D5A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D5A8: 4BDE9879  bl 0x82466e20
	ctx.lr = 0x8267D5AC;
	sub_82466E20(ctx, base);
	// 8267D5AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D5B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D5B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D5B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D5C0 size=108
    let mut pc: u32 = 0x8267D5C0;
    'dispatch: loop {
        match pc {
            0x8267D5C0 => {
    //   block [0x8267D5C0..0x8267D62C)
	// 8267D5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D5C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D5CC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D5D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D5D4: 38EB9DC0  addi r7, r11, -0x6240
	ctx.r[7].s64 = ctx.r[11].s64 + -25152;
	// 8267D5D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267D5DC: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 8267D5E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D5E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D5E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D5F0: 386A72A8  addi r3, r10, 0x72a8
	ctx.r[3].s64 = ctx.r[10].s64 + 29352;
	// 8267D5F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D5F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D60C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D614: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D618: 4BDE9809  bl 0x82466e20
	ctx.lr = 0x8267D61C;
	sub_82466E20(ctx, base);
	// 8267D61C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


