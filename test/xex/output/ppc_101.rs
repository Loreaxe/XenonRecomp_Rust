pub fn sub_82693770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693770 size=112
    let mut pc: u32 = 0x82693770;
    'dispatch: loop {
        match pc {
            0x82693770 => {
    //   block [0x82693770..0x826937E0)
	// 82693770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269377C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693780: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693784: 38AA08A8  addi r5, r10, 0x8a8
	ctx.r[5].s64 = ctx.r[10].s64 + 2216;
	// 82693788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269378C: 390BA0E0  addi r8, r11, -0x5f20
	ctx.r[8].s64 = ctx.r[11].s64 + -24352;
	// 82693790: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82693794: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 82693798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269379C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826937A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826937A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826937A8: 386A0908  addi r3, r10, 0x908
	ctx.r[3].s64 = ctx.r[10].s64 + 2312;
	// 826937AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826937B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826937B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826937B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826937BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826937C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826937C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826937C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826937CC: 4BDD3655  bl 0x82466e20
	ctx.lr = 0x826937D0;
	sub_82466E20(ctx, base);
	// 826937D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826937D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826937D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826937DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826937E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826937E0 size=100
    let mut pc: u32 = 0x826937E0;
    'dispatch: loop {
        match pc {
            0x826937E0 => {
    //   block [0x826937E0..0x82693844)
	// 826937E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826937E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826937E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826937EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826937F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826937F4: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 826937F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826937FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693800: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 82693804: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269380C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693814: 386A0938  addi r3, r10, 0x938
	ctx.r[3].s64 = ctx.r[10].s64 + 2360;
	// 82693818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269381C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693820: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82693824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693828: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269382C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693830: 4BDD35F1  bl 0x82466e20
	ctx.lr = 0x82693834;
	sub_82466E20(ctx, base);
	// 82693834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269383C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693848 size=112
    let mut pc: u32 = 0x82693848;
    'dispatch: loop {
        match pc {
            0x82693848 => {
    //   block [0x82693848..0x826938B8)
	// 82693848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269384C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693854: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693858: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269385C: 38AA0938  addi r5, r10, 0x938
	ctx.r[5].s64 = ctx.r[10].s64 + 2360;
	// 82693860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693864: 390BA0F8  addi r8, r11, -0x5f08
	ctx.r[8].s64 = ctx.r[11].s64 + -24328;
	// 82693868: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8269386C: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 82693870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693874: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269387C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693880: 386A0968  addi r3, r10, 0x968
	ctx.r[3].s64 = ctx.r[10].s64 + 2408;
	// 82693884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82693888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269388C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269389C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826938A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826938A4: 4BDD357D  bl 0x82466e20
	ctx.lr = 0x826938A8;
	sub_82466E20(ctx, base);
	// 826938A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826938AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826938B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826938B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826938B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826938B8 size=108
    let mut pc: u32 = 0x826938B8;
    'dispatch: loop {
        match pc {
            0x826938B8 => {
    //   block [0x826938B8..0x82693924)
	// 826938B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826938BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826938C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826938C4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826938C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826938CC: 38EBA1A0  addi r7, r11, -0x5e60
	ctx.r[7].s64 = ctx.r[11].s64 + -24160;
	// 826938D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826938D4: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 826938D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826938DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826938E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826938E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826938E8: 386A0998  addi r3, r10, 0x998
	ctx.r[3].s64 = ctx.r[10].s64 + 2456;
	// 826938EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826938F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826938F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826938F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826938FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269390C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82693910: 4BDD3511  bl 0x82466e20
	ctx.lr = 0x82693914;
	sub_82466E20(ctx, base);
	// 82693914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269391C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693928 size=112
    let mut pc: u32 = 0x82693928;
    'dispatch: loop {
        match pc {
            0x82693928 => {
    //   block [0x82693928..0x82693998)
	// 82693928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269392C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693934: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693938: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269393C: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82693940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693944: 390BA1D0  addi r8, r11, -0x5e30
	ctx.r[8].s64 = ctx.r[11].s64 + -24112;
	// 82693948: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269394C: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 82693950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693954: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269395C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693960: 386A09C8  addi r3, r10, 0x9c8
	ctx.r[3].s64 = ctx.r[10].s64 + 2504;
	// 82693964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82693968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269396C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269397C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693984: 4BDD349D  bl 0x82466e20
	ctx.lr = 0x82693988;
	sub_82466E20(ctx, base);
	// 82693988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269398C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693998 size=112
    let mut pc: u32 = 0x82693998;
    'dispatch: loop {
        match pc {
            0x82693998 => {
    //   block [0x82693998..0x82693A08)
	// 82693998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269399C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826939A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826939A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826939A8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826939AC: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 826939B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826939B4: 390BA218  addi r8, r11, -0x5de8
	ctx.r[8].s64 = ctx.r[11].s64 + -24040;
	// 826939B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826939BC: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 826939C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826939C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826939C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826939CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826939D0: 386A09F8  addi r3, r10, 0x9f8
	ctx.r[3].s64 = ctx.r[10].s64 + 2552;
	// 826939D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826939D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826939DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826939E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826939E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826939E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826939EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826939F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826939F4: 4BDD342D  bl 0x82466e20
	ctx.lr = 0x826939F8;
	sub_82466E20(ctx, base);
	// 826939F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826939FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693A08 size=100
    let mut pc: u32 = 0x82693A08;
    'dispatch: loop {
        match pc {
            0x82693A08 => {
    //   block [0x82693A08..0x82693A6C)
	// 82693A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693A14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693A1C: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82693A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693A28: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 82693A2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693A30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693A38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693A3C: 386A0A28  addi r3, r10, 0xa28
	ctx.r[3].s64 = ctx.r[10].s64 + 2600;
	// 82693A40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693A44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693A48: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82693A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693A50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82693A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693A58: 4BDD33C9  bl 0x82466e20
	ctx.lr = 0x82693A5C;
	sub_82466E20(ctx, base);
	// 82693A5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693A70 size=112
    let mut pc: u32 = 0x82693A70;
    'dispatch: loop {
        match pc {
            0x82693A70 => {
    //   block [0x82693A70..0x82693AE0)
	// 82693A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693A7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693A80: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693A84: 38AA0A28  addi r5, r10, 0xa28
	ctx.r[5].s64 = ctx.r[10].s64 + 2600;
	// 82693A88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693A8C: 390BA260  addi r8, r11, -0x5da0
	ctx.r[8].s64 = ctx.r[11].s64 + -23968;
	// 82693A90: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82693A94: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 82693A98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693A9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693AA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82693AA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693AA8: 386A0A58  addi r3, r10, 0xa58
	ctx.r[3].s64 = ctx.r[10].s64 + 2648;
	// 82693AAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82693AB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693AC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693ACC: 4BDD3355  bl 0x82466e20
	ctx.lr = 0x82693AD0;
	sub_82466E20(ctx, base);
	// 82693AD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693AE0 size=112
    let mut pc: u32 = 0x82693AE0;
    'dispatch: loop {
        match pc {
            0x82693AE0 => {
    //   block [0x82693AE0..0x82693B50)
	// 82693AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693AEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693AF0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693AF4: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82693AF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693AFC: 390BA2A8  addi r8, r11, -0x5d58
	ctx.r[8].s64 = ctx.r[11].s64 + -23896;
	// 82693B00: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82693B04: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 82693B08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693B0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693B10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82693B14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693B18: 386A0A88  addi r3, r10, 0xa88
	ctx.r[3].s64 = ctx.r[10].s64 + 2696;
	// 82693B1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82693B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693B24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693B28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693B2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693B30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693B38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693B3C: 4BDD32E5  bl 0x82466e20
	ctx.lr = 0x82693B40;
	sub_82466E20(ctx, base);
	// 82693B40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693B50 size=112
    let mut pc: u32 = 0x82693B50;
    'dispatch: loop {
        match pc {
            0x82693B50 => {
    //   block [0x82693B50..0x82693BC0)
	// 82693B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693B5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693B60: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693B64: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82693B68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693B6C: 390BA2C0  addi r8, r11, -0x5d40
	ctx.r[8].s64 = ctx.r[11].s64 + -23872;
	// 82693B70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82693B74: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 82693B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693B7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693B80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82693B84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693B88: 386A0AB8  addi r3, r10, 0xab8
	ctx.r[3].s64 = ctx.r[10].s64 + 2744;
	// 82693B8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82693B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693B94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693B9C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82693BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693BA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693BAC: 4BDD3275  bl 0x82466e20
	ctx.lr = 0x82693BB0;
	sub_82466E20(ctx, base);
	// 82693BB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693BC0 size=112
    let mut pc: u32 = 0x82693BC0;
    'dispatch: loop {
        match pc {
            0x82693BC0 => {
    //   block [0x82693BC0..0x82693C30)
	// 82693BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693BCC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693BD0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693BD4: 38AA0A88  addi r5, r10, 0xa88
	ctx.r[5].s64 = ctx.r[10].s64 + 2696;
	// 82693BD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693BDC: 390BA2D8  addi r8, r11, -0x5d28
	ctx.r[8].s64 = ctx.r[11].s64 + -23848;
	// 82693BE0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82693BE4: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 82693BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693BEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693BF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82693BF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693BF8: 386A0AE8  addi r3, r10, 0xae8
	ctx.r[3].s64 = ctx.r[10].s64 + 2792;
	// 82693BFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82693C00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693C04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693C08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693C10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693C18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693C1C: 4BDD3205  bl 0x82466e20
	ctx.lr = 0x82693C20;
	sub_82466E20(ctx, base);
	// 82693C20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693C30 size=72
    let mut pc: u32 = 0x82693C30;
    'dispatch: loop {
        match pc {
            0x82693C30 => {
    //   block [0x82693C30..0x82693C78)
	// 82693C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693C38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693C3C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82693C40: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82693C44: 38CB6850  addi r6, r11, 0x6850
	ctx.r[6].s64 = ctx.r[11].s64 + 26704;
	// 82693C48: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82693C4C: 388B71F8  addi r4, r11, 0x71f8
	ctx.r[4].s64 = ctx.r[11].s64 + 29176;
	// 82693C50: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82693C54: 386B0B18  addi r3, r11, 0xb18
	ctx.r[3].s64 = ctx.r[11].s64 + 2840;
	// 82693C58: 4BDE7E31  bl 0x8247ba88
	ctx.lr = 0x82693C5C;
	sub_8247BA88(ctx, base);
	// 82693C5C: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82693C60: 386BCE70  addi r3, r11, -0x3190
	ctx.r[3].s64 = ctx.r[11].s64 + -12688;
	// 82693C64: 4BE9EED5  bl 0x82532b38
	ctx.lr = 0x82693C68;
	sub_82532B38(ctx, base);
	// 82693C68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82693C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693C78 size=108
    let mut pc: u32 = 0x82693C78;
    'dispatch: loop {
        match pc {
            0x82693C78 => {
    //   block [0x82693C78..0x82693CE4)
	// 82693C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693C84: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693C88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82693C8C: 38EBABC8  addi r7, r11, -0x5438
	ctx.r[7].s64 = ctx.r[11].s64 + -21560;
	// 82693C90: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82693C94: 388AA590  addi r4, r10, -0x5a70
	ctx.r[4].s64 = ctx.r[10].s64 + -23152;
	// 82693C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693C9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693CA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82693CA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693CA8: 386A0B30  addi r3, r10, 0xb30
	ctx.r[3].s64 = ctx.r[10].s64 + 2864;
	// 82693CAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82693CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693CBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693CCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82693CD0: 4BDD3151  bl 0x82466e20
	ctx.lr = 0x82693CD4;
	sub_82466E20(ctx, base);
	// 82693CD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82693CE8 size=24
    let mut pc: u32 = 0x82693CE8;
    'dispatch: loop {
        match pc {
            0x82693CE8 => {
    //   block [0x82693CE8..0x82693D00)
	// 82693CE8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693CEC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82693CF0: 394A4830  addi r10, r10, 0x4830
	ctx.r[10].s64 = ctx.r[10].s64 + 18480;
	// 82693CF4: 816BAC40  lwz r11, -0x53c0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21440 as u32) ) } as u64;
	// 82693CF8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82693CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693D00 size=112
    let mut pc: u32 = 0x82693D00;
    'dispatch: loop {
        match pc {
            0x82693D00 => {
    //   block [0x82693D00..0x82693D70)
	// 82693D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693D0C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82693D10: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693D14: 392A81A4  addi r9, r10, -0x7e5c
	ctx.r[9].s64 = ctx.r[10].s64 + -32348;
	// 82693D18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82693D1C: 390B4830  addi r8, r11, 0x4830
	ctx.r[8].s64 = ctx.r[11].s64 + 18480;
	// 82693D20: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82693D24: 388AA5A8  addi r4, r10, -0x5a58
	ctx.r[4].s64 = ctx.r[10].s64 + -23128;
	// 82693D28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693D2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693D30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82693D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693D38: 386A0B60  addi r3, r10, 0xb60
	ctx.r[3].s64 = ctx.r[10].s64 + 2912;
	// 82693D3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82693D40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82693D44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693D48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693D50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693D54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82693D58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693D5C: 4BDD30C5  bl 0x82466e20
	ctx.lr = 0x82693D60;
	sub_82466E20(ctx, base);
	// 82693D60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693D70 size=108
    let mut pc: u32 = 0x82693D70;
    'dispatch: loop {
        match pc {
            0x82693D70 => {
    //   block [0x82693D70..0x82693DDC)
	// 82693D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693D7C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693D80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82693D84: 38EBAC44  addi r7, r11, -0x53bc
	ctx.r[7].s64 = ctx.r[11].s64 + -21436;
	// 82693D88: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82693D8C: 388AA5BC  addi r4, r10, -0x5a44
	ctx.r[4].s64 = ctx.r[10].s64 + -23108;
	// 82693D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693D94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693D98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82693D9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693DA0: 386A0B90  addi r3, r10, 0xb90
	ctx.r[3].s64 = ctx.r[10].s64 + 2960;
	// 82693DA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82693DA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693DBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693DC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82693DC8: 4BDD3059  bl 0x82466e20
	ctx.lr = 0x82693DCC;
	sub_82466E20(ctx, base);
	// 82693DCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693DE0 size=108
    let mut pc: u32 = 0x82693DE0;
    'dispatch: loop {
        match pc {
            0x82693DE0 => {
    //   block [0x82693DE0..0x82693E4C)
	// 82693DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693DEC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693DF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82693DF4: 38EBAC74  addi r7, r11, -0x538c
	ctx.r[7].s64 = ctx.r[11].s64 + -21388;
	// 82693DF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82693DFC: 388AA5DC  addi r4, r10, -0x5a24
	ctx.r[4].s64 = ctx.r[10].s64 + -23076;
	// 82693E00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693E04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693E08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82693E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693E10: 386A0BC0  addi r3, r10, 0xbc0
	ctx.r[3].s64 = ctx.r[10].s64 + 3008;
	// 82693E14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82693E18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693E1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693E20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693E28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693E2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693E30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693E34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82693E38: 4BDD2FE9  bl 0x82466e20
	ctx.lr = 0x82693E3C;
	sub_82466E20(ctx, base);
	// 82693E3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693E40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693E44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82693E50 size=24
    let mut pc: u32 = 0x82693E50;
    'dispatch: loop {
        match pc {
            0x82693E50 => {
    //   block [0x82693E50..0x82693E68)
	// 82693E50: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693E54: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82693E58: 394A4890  addi r10, r10, 0x4890
	ctx.r[10].s64 = ctx.r[10].s64 + 18576;
	// 82693E5C: 816BACA4  lwz r11, -0x535c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21340 as u32) ) } as u64;
	// 82693E60: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82693E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693E68 size=116
    let mut pc: u32 = 0x82693E68;
    'dispatch: loop {
        match pc {
            0x82693E68 => {
    //   block [0x82693E68..0x82693EDC)
	// 82693E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693E74: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693E78: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82693E7C: 390B4890  addi r8, r11, 0x4890
	ctx.r[8].s64 = ctx.r[11].s64 + 18576;
	// 82693E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693E84: 392A81E8  addi r9, r10, -0x7e18
	ctx.r[9].s64 = ctx.r[10].s64 + -32280;
	// 82693E88: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693E8C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82693E90: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82693E94: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82693E98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693E9C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82693EA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693EA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693EA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693EAC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82693EB0: 388AA5F0  addi r4, r10, -0x5a10
	ctx.r[4].s64 = ctx.r[10].s64 + -23056;
	// 82693EB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82693EB8: 386B0BF0  addi r3, r11, 0xbf0
	ctx.r[3].s64 = ctx.r[11].s64 + 3056;
	// 82693EBC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82693EC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693EC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693EC8: 4BDD2F59  bl 0x82466e20
	ctx.lr = 0x82693ECC;
	sub_82466E20(ctx, base);
	// 82693ECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693EE0 size=108
    let mut pc: u32 = 0x82693EE0;
    'dispatch: loop {
        match pc {
            0x82693EE0 => {
    //   block [0x82693EE0..0x82693F4C)
	// 82693EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693EEC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693EF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82693EF4: 38EBACA8  addi r7, r11, -0x5358
	ctx.r[7].s64 = ctx.r[11].s64 + -21336;
	// 82693EF8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82693EFC: 388AA608  addi r4, r10, -0x59f8
	ctx.r[4].s64 = ctx.r[10].s64 + -23032;
	// 82693F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693F04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693F08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82693F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693F10: 386A0C20  addi r3, r10, 0xc20
	ctx.r[3].s64 = ctx.r[10].s64 + 3104;
	// 82693F14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82693F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693F24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693F34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82693F38: 4BDD2EE9  bl 0x82466e20
	ctx.lr = 0x82693F3C;
	sub_82466E20(ctx, base);
	// 82693F3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693F50 size=112
    let mut pc: u32 = 0x82693F50;
    'dispatch: loop {
        match pc {
            0x82693F50 => {
    //   block [0x82693F50..0x82693FC0)
	// 82693F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693F58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693F5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693F60: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693F64: 38AA0BF0  addi r5, r10, 0xbf0
	ctx.r[5].s64 = ctx.r[10].s64 + 3056;
	// 82693F68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82693F6C: 390BAD38  addi r8, r11, -0x52c8
	ctx.r[8].s64 = ctx.r[11].s64 + -21192;
	// 82693F70: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82693F74: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 82693F78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693F7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693F80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82693F84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693F88: 386A0C50  addi r3, r10, 0xc50
	ctx.r[3].s64 = ctx.r[10].s64 + 3152;
	// 82693F8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82693F90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693F94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693F98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693FA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693FA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693FAC: 4BDD2E75  bl 0x82466e20
	ctx.lr = 0x82693FB0;
	sub_82466E20(ctx, base);
	// 82693FB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693FC0 size=112
    let mut pc: u32 = 0x82693FC0;
    'dispatch: loop {
        match pc {
            0x82693FC0 => {
    //   block [0x82693FC0..0x82694030)
	// 82693FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693FCC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693FD0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693FD4: 38AA0BF0  addi r5, r10, 0xbf0
	ctx.r[5].s64 = ctx.r[10].s64 + 3056;
	// 82693FD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82693FDC: 390BAE58  addi r8, r11, -0x51a8
	ctx.r[8].s64 = ctx.r[11].s64 + -20904;
	// 82693FE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82693FE4: 388AA664  addi r4, r10, -0x599c
	ctx.r[4].s64 = ctx.r[10].s64 + -22940;
	// 82693FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693FEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693FF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82693FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693FF8: 386A0C80  addi r3, r10, 0xc80
	ctx.r[3].s64 = ctx.r[10].s64 + 3200;
	// 82693FFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82694000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269400C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269401C: 4BDD2E05  bl 0x82466e20
	ctx.lr = 0x82694020;
	sub_82466E20(ctx, base);
	// 82694020: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269402C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82694030 size=44
    let mut pc: u32 = 0x82694030;
    'dispatch: loop {
        match pc {
            0x82694030 => {
    //   block [0x82694030..0x8269405C)
	// 82694030: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694034: 814BAE88  lwz r10, -0x5178(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20856 as u32) ) } as u64;
	// 82694038: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269403C: 396B4920  addi r11, r11, 0x4920
	ctx.r[11].s64 = ctx.r[11].s64 + 18720;
	// 82694040: 914B00C8  stw r10, 0xc8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(200 as u32), ctx.r[10].u32 ) };
	// 82694044: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82694048: 814AAE8C  lwz r10, -0x5174(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20852 as u32) ) } as u64;
	// 8269404C: 914B00E0  stw r10, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[10].u32 ) };
	// 82694050: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 82694054: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 82694058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694060 size=112
    let mut pc: u32 = 0x82694060;
    'dispatch: loop {
        match pc {
            0x82694060 => {
    //   block [0x82694060..0x826940D0)
	// 82694060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269406C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82694070: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694074: 392A823C  addi r9, r10, -0x7dc4
	ctx.r[9].s64 = ctx.r[10].s64 + -32196;
	// 82694078: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269407C: 390B4920  addi r8, r11, 0x4920
	ctx.r[8].s64 = ctx.r[11].s64 + 18720;
	// 82694080: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82694084: 388AB160  addi r4, r10, -0x4ea0
	ctx.r[4].s64 = ctx.r[10].s64 + -20128;
	// 82694088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269408C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694090: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694098: 386A0CB0  addi r3, r10, 0xcb0
	ctx.r[3].s64 = ctx.r[10].s64 + 3248;
	// 8269409C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826940A0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826940A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826940A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826940AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826940B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826940B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826940B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826940BC: 4BDD2D65  bl 0x82466e20
	ctx.lr = 0x826940C0;
	sub_82466E20(ctx, base);
	// 826940C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826940C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826940C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826940CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826940D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826940D0 size=108
    let mut pc: u32 = 0x826940D0;
    'dispatch: loop {
        match pc {
            0x826940D0 => {
    //   block [0x826940D0..0x8269413C)
	// 826940D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826940D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826940D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826940DC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826940E0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826940E4: 38EBAE90  addi r7, r11, -0x5170
	ctx.r[7].s64 = ctx.r[11].s64 + -20848;
	// 826940E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826940EC: 388AB194  addi r4, r10, -0x4e6c
	ctx.r[4].s64 = ctx.r[10].s64 + -20076;
	// 826940F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826940F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826940F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826940FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694100: 386A0CE0  addi r3, r10, 0xce0
	ctx.r[3].s64 = ctx.r[10].s64 + 3296;
	// 82694104: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269410C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269411C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694128: 4BDD2CF9  bl 0x82466e20
	ctx.lr = 0x8269412C;
	sub_82466E20(ctx, base);
	// 8269412C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694140 size=112
    let mut pc: u32 = 0x82694140;
    'dispatch: loop {
        match pc {
            0x82694140 => {
    //   block [0x82694140..0x826941B0)
	// 82694140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269414C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694150: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694154: 38AA0BF0  addi r5, r10, 0xbf0
	ctx.r[5].s64 = ctx.r[10].s64 + 3056;
	// 82694158: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269415C: 390BAEC0  addi r8, r11, -0x5140
	ctx.r[8].s64 = ctx.r[11].s64 + -20800;
	// 82694160: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 82694164: 388AB1CC  addi r4, r10, -0x4e34
	ctx.r[4].s64 = ctx.r[10].s64 + -20020;
	// 82694168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269416C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694178: 386A0D10  addi r3, r10, 0xd10
	ctx.r[3].s64 = ctx.r[10].s64 + 3344;
	// 8269417C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82694180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269418C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269419C: 4BDD2C85  bl 0x82466e20
	ctx.lr = 0x826941A0;
	sub_82466E20(ctx, base);
	// 826941A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826941A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826941A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826941AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826941B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826941B0 size=108
    let mut pc: u32 = 0x826941B0;
    'dispatch: loop {
        match pc {
            0x826941B0 => {
    //   block [0x826941B0..0x8269421C)
	// 826941B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826941B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826941B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826941BC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826941C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826941C4: 38EBAFF8  addi r7, r11, -0x5008
	ctx.r[7].s64 = ctx.r[11].s64 + -20488;
	// 826941C8: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826941CC: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 826941D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826941D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826941D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826941DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826941E0: 386A0D40  addi r3, r10, 0xd40
	ctx.r[3].s64 = ctx.r[10].s64 + 3392;
	// 826941E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826941E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826941EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826941F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826941F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826941F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826941FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694208: 4BDD2C19  bl 0x82466e20
	ctx.lr = 0x8269420C;
	sub_82466E20(ctx, base);
	// 8269420C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694220 size=108
    let mut pc: u32 = 0x82694220;
    'dispatch: loop {
        match pc {
            0x82694220 => {
    //   block [0x82694220..0x8269428C)
	// 82694220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269422C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694230: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694234: 38EBB0E8  addi r7, r11, -0x4f18
	ctx.r[7].s64 = ctx.r[11].s64 + -20248;
	// 82694238: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8269423C: 388AA6B4  addi r4, r10, -0x594c
	ctx.r[4].s64 = ctx.r[10].s64 + -22860;
	// 82694240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694244: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694248: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269424C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694250: 386A0D70  addi r3, r10, 0xd70
	ctx.r[3].s64 = ctx.r[10].s64 + 3440;
	// 82694254: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269425C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269426C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694278: 4BDD2BA9  bl 0x82466e20
	ctx.lr = 0x8269427C;
	sub_82466E20(ctx, base);
	// 8269427C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694290 size=112
    let mut pc: u32 = 0x82694290;
    'dispatch: loop {
        match pc {
            0x82694290 => {
    //   block [0x82694290..0x82694300)
	// 82694290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269429C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826942A0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826942A4: 38AA0BF0  addi r5, r10, 0xbf0
	ctx.r[5].s64 = ctx.r[10].s64 + 3056;
	// 826942A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826942AC: 390BB178  addi r8, r11, -0x4e88
	ctx.r[8].s64 = ctx.r[11].s64 + -20104;
	// 826942B0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826942B4: 388AA6E4  addi r4, r10, -0x591c
	ctx.r[4].s64 = ctx.r[10].s64 + -22812;
	// 826942B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826942BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826942C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826942C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826942C8: 386A0DA0  addi r3, r10, 0xda0
	ctx.r[3].s64 = ctx.r[10].s64 + 3488;
	// 826942CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826942D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826942D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826942D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826942DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826942E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826942E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826942E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826942EC: 4BDD2B35  bl 0x82466e20
	ctx.lr = 0x826942F0;
	sub_82466E20(ctx, base);
	// 826942F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826942F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826942F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826942FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694300 size=108
    let mut pc: u32 = 0x82694300;
    'dispatch: loop {
        match pc {
            0x82694300 => {
    //   block [0x82694300..0x8269436C)
	// 82694300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269430C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694310: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694314: 38EBB268  addi r7, r11, -0x4d98
	ctx.r[7].s64 = ctx.r[11].s64 + -19864;
	// 82694318: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269431C: 388AA700  addi r4, r10, -0x5900
	ctx.r[4].s64 = ctx.r[10].s64 + -22784;
	// 82694320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694324: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694328: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269432C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694330: 386A0DD0  addi r3, r10, 0xdd0
	ctx.r[3].s64 = ctx.r[10].s64 + 3536;
	// 82694334: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269433C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269434C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694358: 4BDD2AC9  bl 0x82466e20
	ctx.lr = 0x8269435C;
	sub_82466E20(ctx, base);
	// 8269435C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694370 size=108
    let mut pc: u32 = 0x82694370;
    'dispatch: loop {
        match pc {
            0x82694370 => {
    //   block [0x82694370..0x826943DC)
	// 82694370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269437C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694380: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694384: 38EBB280  addi r7, r11, -0x4d80
	ctx.r[7].s64 = ctx.r[11].s64 + -19840;
	// 82694388: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269438C: 388AA718  addi r4, r10, -0x58e8
	ctx.r[4].s64 = ctx.r[10].s64 + -22760;
	// 82694390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694394: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694398: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269439C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826943A0: 386A0E00  addi r3, r10, 0xe00
	ctx.r[3].s64 = ctx.r[10].s64 + 3584;
	// 826943A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826943A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826943AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826943B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826943B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826943B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826943BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826943C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826943C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826943C8: 4BDD2A59  bl 0x82466e20
	ctx.lr = 0x826943CC;
	sub_82466E20(ctx, base);
	// 826943CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826943D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826943D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826943D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826943E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826943E0 size=116
    let mut pc: u32 = 0x826943E0;
    'dispatch: loop {
        match pc {
            0x826943E0 => {
    //   block [0x826943E0..0x82694454)
	// 826943E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826943E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826943E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826943EC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826943F0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826943F4: 390BB2E4  addi r8, r11, -0x4d1c
	ctx.r[8].s64 = ctx.r[11].s64 + -19740;
	// 826943F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826943FC: 392A827C  addi r9, r10, -0x7d84
	ctx.r[9].s64 = ctx.r[10].s64 + -32132;
	// 82694400: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694404: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82694408: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269440C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694414: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269441C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694424: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82694428: 388AA728  addi r4, r10, -0x58d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22744;
	// 8269442C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82694430: 386B0E30  addi r3, r11, 0xe30
	ctx.r[3].s64 = ctx.r[11].s64 + 3632;
	// 82694434: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82694438: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269443C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694440: 4BDD29E1  bl 0x82466e20
	ctx.lr = 0x82694444;
	sub_82466E20(ctx, base);
	// 82694444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269444C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694458 size=108
    let mut pc: u32 = 0x82694458;
    'dispatch: loop {
        match pc {
            0x82694458 => {
    //   block [0x82694458..0x826944C4)
	// 82694458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269445C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694464: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694468: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269446C: 38EBB300  addi r7, r11, -0x4d00
	ctx.r[7].s64 = ctx.r[11].s64 + -19712;
	// 82694470: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82694474: 388AA73C  addi r4, r10, -0x58c4
	ctx.r[4].s64 = ctx.r[10].s64 + -22724;
	// 82694478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269447C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694480: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82694484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694488: 386A0E60  addi r3, r10, 0xe60
	ctx.r[3].s64 = ctx.r[10].s64 + 3680;
	// 8269448C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269449C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826944A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826944A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826944A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826944AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826944B0: 4BDD2971  bl 0x82466e20
	ctx.lr = 0x826944B4;
	sub_82466E20(ctx, base);
	// 826944B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826944B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826944BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826944C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826944C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826944C8 size=108
    let mut pc: u32 = 0x826944C8;
    'dispatch: loop {
        match pc {
            0x826944C8 => {
    //   block [0x826944C8..0x82694534)
	// 826944C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826944CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826944D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826944D4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826944D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826944DC: 38EBB348  addi r7, r11, -0x4cb8
	ctx.r[7].s64 = ctx.r[11].s64 + -19640;
	// 826944E0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826944E4: 388AA760  addi r4, r10, -0x58a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22688;
	// 826944E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826944EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826944F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826944F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826944F8: 386A0E90  addi r3, r10, 0xe90
	ctx.r[3].s64 = ctx.r[10].s64 + 3728;
	// 826944FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269450C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269451C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694520: 4BDD2901  bl 0x82466e20
	ctx.lr = 0x82694524;
	sub_82466E20(ctx, base);
	// 82694524: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269452C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694538 size=108
    let mut pc: u32 = 0x82694538;
    'dispatch: loop {
        match pc {
            0x82694538 => {
    //   block [0x82694538..0x826945A4)
	// 82694538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269453C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694544: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694548: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269454C: 38EBB3D8  addi r7, r11, -0x4c28
	ctx.r[7].s64 = ctx.r[11].s64 + -19496;
	// 82694550: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82694554: 388AA784  addi r4, r10, -0x587c
	ctx.r[4].s64 = ctx.r[10].s64 + -22652;
	// 82694558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269455C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694560: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82694564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694568: 386A0EC0  addi r3, r10, 0xec0
	ctx.r[3].s64 = ctx.r[10].s64 + 3776;
	// 8269456C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269457C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269458C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694590: 4BDD2891  bl 0x82466e20
	ctx.lr = 0x82694594;
	sub_82466E20(ctx, base);
	// 82694594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269459C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826945A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826945A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826945A8 size=100
    let mut pc: u32 = 0x826945A8;
    'dispatch: loop {
        match pc {
            0x826945A8 => {
    //   block [0x826945A8..0x8269460C)
	// 826945A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826945AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826945B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826945B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826945B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826945BC: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826945C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826945C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826945C8: 388AA79C  addi r4, r10, -0x5864
	ctx.r[4].s64 = ctx.r[10].s64 + -22628;
	// 826945CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826945D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826945D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826945D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826945DC: 386A0EF0  addi r3, r10, 0xef0
	ctx.r[3].s64 = ctx.r[10].s64 + 3824;
	// 826945E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826945E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826945E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826945EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826945F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826945F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826945F8: 4BDD2829  bl 0x82466e20
	ctx.lr = 0x826945FC;
	sub_82466E20(ctx, base);
	// 826945FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694610 size=112
    let mut pc: u32 = 0x82694610;
    'dispatch: loop {
        match pc {
            0x82694610 => {
    //   block [0x82694610..0x82694680)
	// 82694610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269461C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694620: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694624: 38AA0EF0  addi r5, r10, 0xef0
	ctx.r[5].s64 = ctx.r[10].s64 + 3824;
	// 82694628: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269462C: 390BB468  addi r8, r11, -0x4b98
	ctx.r[8].s64 = ctx.r[11].s64 + -19352;
	// 82694630: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82694634: 388AA7B8  addi r4, r10, -0x5848
	ctx.r[4].s64 = ctx.r[10].s64 + -22600;
	// 82694638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269463C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694648: 386A0F20  addi r3, r10, 0xf20
	ctx.r[3].s64 = ctx.r[10].s64 + 3872;
	// 8269464C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82694650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269465C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269466C: 4BDD27B5  bl 0x82466e20
	ctx.lr = 0x82694670;
	sub_82466E20(ctx, base);
	// 82694670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269467C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694680 size=108
    let mut pc: u32 = 0x82694680;
    'dispatch: loop {
        match pc {
            0x82694680 => {
    //   block [0x82694680..0x826946EC)
	// 82694680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269468C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694690: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694694: 38EBB4C8  addi r7, r11, -0x4b38
	ctx.r[7].s64 = ctx.r[11].s64 + -19256;
	// 82694698: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269469C: 388AA7DC  addi r4, r10, -0x5824
	ctx.r[4].s64 = ctx.r[10].s64 + -22564;
	// 826946A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826946A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826946A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826946AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826946B0: 386A0F50  addi r3, r10, 0xf50
	ctx.r[3].s64 = ctx.r[10].s64 + 3920;
	// 826946B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826946B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826946BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826946C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826946C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826946C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826946CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826946D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826946D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826946D8: 4BDD2749  bl 0x82466e20
	ctx.lr = 0x826946DC;
	sub_82466E20(ctx, base);
	// 826946DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826946E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826946E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826946E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826946F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826946F0 size=108
    let mut pc: u32 = 0x826946F0;
    'dispatch: loop {
        match pc {
            0x826946F0 => {
    //   block [0x826946F0..0x8269475C)
	// 826946F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826946F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826946F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826946FC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694700: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694704: 38EBB4F8  addi r7, r11, -0x4b08
	ctx.r[7].s64 = ctx.r[11].s64 + -19208;
	// 82694708: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269470C: 388AA7E4  addi r4, r10, -0x581c
	ctx.r[4].s64 = ctx.r[10].s64 + -22556;
	// 82694710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694714: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694718: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269471C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694720: 386A0F80  addi r3, r10, 0xf80
	ctx.r[3].s64 = ctx.r[10].s64 + 3968;
	// 82694724: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269472C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269473C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694744: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694748: 4BDD26D9  bl 0x82466e20
	ctx.lr = 0x8269474C;
	sub_82466E20(ctx, base);
	// 8269474C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694760 size=108
    let mut pc: u32 = 0x82694760;
    'dispatch: loop {
        match pc {
            0x82694760 => {
    //   block [0x82694760..0x826947CC)
	// 82694760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269476C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694770: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694774: 38EBB558  addi r7, r11, -0x4aa8
	ctx.r[7].s64 = ctx.r[11].s64 + -19112;
	// 82694778: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8269477C: 388AA7F8  addi r4, r10, -0x5808
	ctx.r[4].s64 = ctx.r[10].s64 + -22536;
	// 82694780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694784: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694788: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269478C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694790: 386A0FB0  addi r3, r10, 0xfb0
	ctx.r[3].s64 = ctx.r[10].s64 + 4016;
	// 82694794: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269479C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826947A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826947A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826947A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826947AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826947B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826947B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826947B8: 4BDD2669  bl 0x82466e20
	ctx.lr = 0x826947BC;
	sub_82466E20(ctx, base);
	// 826947BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826947C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826947C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826947C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826947D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826947D0 size=112
    let mut pc: u32 = 0x826947D0;
    'dispatch: loop {
        match pc {
            0x826947D0 => {
    //   block [0x826947D0..0x82694840)
	// 826947D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826947D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826947D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826947DC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826947E0: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826947E4: 38EAB5D0  addi r7, r10, -0x4a30
	ctx.r[7].s64 = ctx.r[10].s64 + -18992;
	// 826947E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826947EC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826947F0: 388AA804  addi r4, r10, -0x57fc
	ctx.r[4].s64 = ctx.r[10].s64 + -22524;
	// 826947F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826947F8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826947FC: 396B8290  addi r11, r11, -0x7d70
	ctx.r[11].s64 = ctx.r[11].s64 + -32112;
	// 82694800: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82694804: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694808: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269480C: 386A0FE0  addi r3, r10, 0xfe0
	ctx.r[3].s64 = ctx.r[10].s64 + 4064;
	// 82694810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694814: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82694818: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269481C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82694820: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694824: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694828: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269482C: 4BDD25F5  bl 0x82466e20
	ctx.lr = 0x82694830;
	sub_82466E20(ctx, base);
	// 82694830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269483C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694840 size=96
    let mut pc: u32 = 0x82694840;
    'dispatch: loop {
        match pc {
            0x82694840 => {
    //   block [0x82694840..0x826948A0)
	// 82694840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269484C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694854: 388AA82C  addi r4, r10, -0x57d4
	ctx.r[4].s64 = ctx.r[10].s64 + -22484;
	// 82694858: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269485C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694860: 386A1010  addi r3, r10, 0x1010
	ctx.r[3].s64 = ctx.r[10].s64 + 4112;
	// 82694864: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269486C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269487C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694880: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82694884: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694888: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269488C: 4BDD2595  bl 0x82466e20
	ctx.lr = 0x82694890;
	sub_82466E20(ctx, base);
	// 82694890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269489C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826948A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826948A0 size=112
    let mut pc: u32 = 0x826948A0;
    'dispatch: loop {
        match pc {
            0x826948A0 => {
    //   block [0x826948A0..0x82694910)
	// 826948A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826948A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826948A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826948AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826948B0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826948B4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826948B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826948BC: 390BB6F0  addi r8, r11, -0x4910
	ctx.r[8].s64 = ctx.r[11].s64 + -18704;
	// 826948C0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826948C4: 388AA848  addi r4, r10, -0x57b8
	ctx.r[4].s64 = ctx.r[10].s64 + -22456;
	// 826948C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826948CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826948D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826948D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826948D8: 386A1040  addi r3, r10, 0x1040
	ctx.r[3].s64 = ctx.r[10].s64 + 4160;
	// 826948DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826948E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826948E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826948E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826948EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826948F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826948F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826948F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826948FC: 4BDD2525  bl 0x82466e20
	ctx.lr = 0x82694900;
	sub_82466E20(ctx, base);
	// 82694900: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269490C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82694910 size=24
    let mut pc: u32 = 0x82694910;
    'dispatch: loop {
        match pc {
            0x82694910 => {
    //   block [0x82694910..0x82694928)
	// 82694910: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694914: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82694918: 394A4A40  addi r10, r10, 0x4a40
	ctx.r[10].s64 = ctx.r[10].s64 + 19008;
	// 8269491C: 816BB2FC  lwz r11, -0x4d04(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19716 as u32) ) } as u64;
	// 82694920: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82694924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694928 size=116
    let mut pc: u32 = 0x82694928;
    'dispatch: loop {
        match pc {
            0x82694928 => {
    //   block [0x82694928..0x8269499C)
	// 82694928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269492C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694934: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694938: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269493C: 390B4A40  addi r8, r11, 0x4a40
	ctx.r[8].s64 = ctx.r[11].s64 + 19008;
	// 82694940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694944: 392A830C  addi r9, r10, -0x7cf4
	ctx.r[9].s64 = ctx.r[10].s64 + -31988;
	// 82694948: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269494C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82694950: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82694954: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269495C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82694960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269496C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82694970: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 82694974: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82694978: 386B1070  addi r3, r11, 0x1070
	ctx.r[3].s64 = ctx.r[11].s64 + 4208;
	// 8269497C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82694980: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694988: 4BDD2499  bl 0x82466e20
	ctx.lr = 0x8269498C;
	sub_82466E20(ctx, base);
	// 8269498C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826949A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826949A0 size=24
    let mut pc: u32 = 0x826949A0;
    'dispatch: loop {
        match pc {
            0x826949A0 => {
    //   block [0x826949A0..0x826949B8)
	// 826949A0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826949A4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826949A8: 394A4B18  addi r10, r10, 0x4b18
	ctx.r[10].s64 = ctx.r[10].s64 + 19224;
	// 826949AC: 816BB758  lwz r11, -0x48a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18600 as u32) ) } as u64;
	// 826949B0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826949B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826949B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826949B8 size=116
    let mut pc: u32 = 0x826949B8;
    'dispatch: loop {
        match pc {
            0x826949B8 => {
    //   block [0x826949B8..0x82694A2C)
	// 826949B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826949BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826949C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826949C4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826949C8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826949CC: 390B4B18  addi r8, r11, 0x4b18
	ctx.r[8].s64 = ctx.r[11].s64 + 19224;
	// 826949D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826949D4: 392A83D0  addi r9, r10, -0x7c30
	ctx.r[9].s64 = ctx.r[10].s64 + -31792;
	// 826949D8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826949DC: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 826949E0: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 826949E4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826949E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826949EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826949F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826949F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826949F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826949FC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82694A00: 388AA870  addi r4, r10, -0x5790
	ctx.r[4].s64 = ctx.r[10].s64 + -22416;
	// 82694A04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82694A08: 386B10A0  addi r3, r11, 0x10a0
	ctx.r[3].s64 = ctx.r[11].s64 + 4256;
	// 82694A0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82694A10: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694A18: 4BDD2409  bl 0x82466e20
	ctx.lr = 0x82694A1C;
	sub_82466E20(ctx, base);
	// 82694A1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694A30 size=112
    let mut pc: u32 = 0x82694A30;
    'dispatch: loop {
        match pc {
            0x82694A30 => {
    //   block [0x82694A30..0x82694AA0)
	// 82694A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694A38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694A3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694A40: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694A44: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82694A48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694A4C: 390BB760  addi r8, r11, -0x48a0
	ctx.r[8].s64 = ctx.r[11].s64 + -18592;
	// 82694A50: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82694A54: 388AA884  addi r4, r10, -0x577c
	ctx.r[4].s64 = ctx.r[10].s64 + -22396;
	// 82694A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694A5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694A60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694A68: 386A10D0  addi r3, r10, 0x10d0
	ctx.r[3].s64 = ctx.r[10].s64 + 4304;
	// 82694A6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82694A70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694A78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694A7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694A80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694A84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694A88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694A8C: 4BDD2395  bl 0x82466e20
	ctx.lr = 0x82694A90;
	sub_82466E20(ctx, base);
	// 82694A90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694AA0 size=112
    let mut pc: u32 = 0x82694AA0;
    'dispatch: loop {
        match pc {
            0x82694AA0 => {
    //   block [0x82694AA0..0x82694B10)
	// 82694AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694AAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694AB0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694AB4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82694AB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694ABC: 390BB7A8  addi r8, r11, -0x4858
	ctx.r[8].s64 = ctx.r[11].s64 + -18520;
	// 82694AC0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82694AC4: 388AA89C  addi r4, r10, -0x5764
	ctx.r[4].s64 = ctx.r[10].s64 + -22372;
	// 82694AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694ACC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694AD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694AD8: 386A1100  addi r3, r10, 0x1100
	ctx.r[3].s64 = ctx.r[10].s64 + 4352;
	// 82694ADC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82694AE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694AE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694AE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694AEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694AF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694AF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694AF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694AFC: 4BDD2325  bl 0x82466e20
	ctx.lr = 0x82694B00;
	sub_82466E20(ctx, base);
	// 82694B00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694B04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694B08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694B10 size=108
    let mut pc: u32 = 0x82694B10;
    'dispatch: loop {
        match pc {
            0x82694B10 => {
    //   block [0x82694B10..0x82694B7C)
	// 82694B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694B1C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694B20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82694B24: 38EBB7F0  addi r7, r11, -0x4810
	ctx.r[7].s64 = ctx.r[11].s64 + -18448;
	// 82694B28: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82694B2C: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 82694B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694B34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694B38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82694B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694B40: 386A1130  addi r3, r10, 0x1130
	ctx.r[3].s64 = ctx.r[10].s64 + 4400;
	// 82694B44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694B68: 4BDD22B9  bl 0x82466e20
	ctx.lr = 0x82694B6C;
	sub_82466E20(ctx, base);
	// 82694B6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694B80 size=112
    let mut pc: u32 = 0x82694B80;
    'dispatch: loop {
        match pc {
            0x82694B80 => {
    //   block [0x82694B80..0x82694BF0)
	// 82694B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694B8C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82694B90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82694B94: 392B8404  addi r9, r11, -0x7bfc
	ctx.r[9].s64 = ctx.r[11].s64 + -31740;
	// 82694B98: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 82694B9C: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82694BA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694BA4: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 82694BA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694BAC: 396BB898  addi r11, r11, -0x4768
	ctx.r[11].s64 = ctx.r[11].s64 + -18280;
	// 82694BB0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82694BB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694BB8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82694BBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694BC0: 386A1160  addi r3, r10, 0x1160
	ctx.r[3].s64 = ctx.r[10].s64 + 4448;
	// 82694BC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82694BC8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82694BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694BD0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82694BD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694BD8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82694BDC: 4BDD2245  bl 0x82466e20
	ctx.lr = 0x82694BE0;
	sub_82466E20(ctx, base);
	// 82694BE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694BF0 size=112
    let mut pc: u32 = 0x82694BF0;
    'dispatch: loop {
        match pc {
            0x82694BF0 => {
    //   block [0x82694BF0..0x82694C60)
	// 82694BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694BFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694C00: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694C04: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82694C08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82694C0C: 390BB9E8  addi r8, r11, -0x4618
	ctx.r[8].s64 = ctx.r[11].s64 + -17944;
	// 82694C10: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82694C14: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 82694C18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694C1C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694C20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694C24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694C28: 386A1190  addi r3, r10, 0x1190
	ctx.r[3].s64 = ctx.r[10].s64 + 4496;
	// 82694C2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82694C30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694C34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694C3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694C44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694C4C: 4BDD21D5  bl 0x82466e20
	ctx.lr = 0x82694C50;
	sub_82466E20(ctx, base);
	// 82694C50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694C60 size=112
    let mut pc: u32 = 0x82694C60;
    'dispatch: loop {
        match pc {
            0x82694C60 => {
    //   block [0x82694C60..0x82694CD0)
	// 82694C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694C68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694C6C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694C70: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694C74: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82694C78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82694C7C: 390BBA90  addi r8, r11, -0x4570
	ctx.r[8].s64 = ctx.r[11].s64 + -17776;
	// 82694C80: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82694C84: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 82694C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694C8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694C90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694C94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694C98: 386A11C0  addi r3, r10, 0x11c0
	ctx.r[3].s64 = ctx.r[10].s64 + 4544;
	// 82694C9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82694CA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694CA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694CA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694CAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694CB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694CB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694CB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694CBC: 4BDD2165  bl 0x82466e20
	ctx.lr = 0x82694CC0;
	sub_82466E20(ctx, base);
	// 82694CC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694CD0 size=112
    let mut pc: u32 = 0x82694CD0;
    'dispatch: loop {
        match pc {
            0x82694CD0 => {
    //   block [0x82694CD0..0x82694D40)
	// 82694CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694CDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694CE0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694CE4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82694CE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82694CEC: 390BBB20  addi r8, r11, -0x44e0
	ctx.r[8].s64 = ctx.r[11].s64 + -17632;
	// 82694CF0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82694CF4: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 82694CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694CFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694D00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694D08: 386A11F0  addi r3, r10, 0x11f0
	ctx.r[3].s64 = ctx.r[10].s64 + 4592;
	// 82694D0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82694D10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694D14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694D18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694D1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694D20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694D28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694D2C: 4BDD20F5  bl 0x82466e20
	ctx.lr = 0x82694D30;
	sub_82466E20(ctx, base);
	// 82694D30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694D40 size=116
    let mut pc: u32 = 0x82694D40;
    'dispatch: loop {
        match pc {
            0x82694D40 => {
    //   block [0x82694D40..0x82694DB4)
	// 82694D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694D4C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82694D50: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82694D54: 390ABBC8  addi r8, r10, -0x4438
	ctx.r[8].s64 = ctx.r[10].s64 + -17464;
	// 82694D58: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694D5C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82694D60: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82694D64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694D68: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82694D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694D70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694D74: 388AA8B8  addi r4, r10, -0x5748
	ctx.r[4].s64 = ctx.r[10].s64 + -22344;
	// 82694D78: 396B8458  addi r11, r11, -0x7ba8
	ctx.r[11].s64 = ctx.r[11].s64 + -31656;
	// 82694D7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694D80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694D84: 386A1220  addi r3, r10, 0x1220
	ctx.r[3].s64 = ctx.r[10].s64 + 4640;
	// 82694D88: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82694D8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694D90: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82694D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694D98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694DA0: 4BDD2081  bl 0x82466e20
	ctx.lr = 0x82694DA4;
	sub_82466E20(ctx, base);
	// 82694DA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694DA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694DAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694DB8 size=108
    let mut pc: u32 = 0x82694DB8;
    'dispatch: loop {
        match pc {
            0x82694DB8 => {
    //   block [0x82694DB8..0x82694E24)
	// 82694DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694DC4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694DC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82694DCC: 38EBBC58  addi r7, r11, -0x43a8
	ctx.r[7].s64 = ctx.r[11].s64 + -17320;
	// 82694DD0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82694DD4: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 82694DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694DDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694DE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82694DE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694DE8: 386A1250  addi r3, r10, 0x1250
	ctx.r[3].s64 = ctx.r[10].s64 + 4688;
	// 82694DEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694DF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694E0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694E10: 4BDD2011  bl 0x82466e20
	ctx.lr = 0x82694E14;
	sub_82466E20(ctx, base);
	// 82694E14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694E18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694E1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694E28 size=112
    let mut pc: u32 = 0x82694E28;
    'dispatch: loop {
        match pc {
            0x82694E28 => {
    //   block [0x82694E28..0x82694E98)
	// 82694E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694E34: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82694E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82694E3C: 392B8518  addi r9, r11, -0x7ae8
	ctx.r[9].s64 = ctx.r[11].s64 + -31464;
	// 82694E40: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82694E44: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82694E48: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694E4C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 82694E50: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694E54: 396BBD1C  addi r11, r11, -0x42e4
	ctx.r[11].s64 = ctx.r[11].s64 + -17124;
	// 82694E58: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82694E5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694E60: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82694E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694E68: 386A1280  addi r3, r10, 0x1280
	ctx.r[3].s64 = ctx.r[10].s64 + 4736;
	// 82694E6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82694E70: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82694E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694E78: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82694E7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694E80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82694E84: 4BDD1F9D  bl 0x82466e20
	ctx.lr = 0x82694E88;
	sub_82466E20(ctx, base);
	// 82694E88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694E98 size=100
    let mut pc: u32 = 0x82694E98;
    'dispatch: loop {
        match pc {
            0x82694E98 => {
    //   block [0x82694E98..0x82694EFC)
	// 82694E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694EA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694EAC: 38AA1C70  addi r5, r10, 0x1c70
	ctx.r[5].s64 = ctx.r[10].s64 + 7280;
	// 82694EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82694EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694EB8: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 82694EBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694ECC: 386A12B0  addi r3, r10, 0x12b0
	ctx.r[3].s64 = ctx.r[10].s64 + 4784;
	// 82694ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694ED4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694ED8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82694EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694EE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82694EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694EE8: 4BDD1F39  bl 0x82466e20
	ctx.lr = 0x82694EEC;
	sub_82466E20(ctx, base);
	// 82694EEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82694F00 size=24
    let mut pc: u32 = 0x82694F00;
    'dispatch: loop {
        match pc {
            0x82694F00 => {
    //   block [0x82694F00..0x82694F18)
	// 82694F00: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694F04: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82694F08: 394A4C50  addi r10, r10, 0x4c50
	ctx.r[10].s64 = ctx.r[10].s64 + 19536;
	// 82694F0C: 816BBD50  lwz r11, -0x42b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17072 as u32) ) } as u64;
	// 82694F10: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82694F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694F18 size=108
    let mut pc: u32 = 0x82694F18;
    'dispatch: loop {
        match pc {
            0x82694F18 => {
    //   block [0x82694F18..0x82694F84)
	// 82694F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694F24: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694F28: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82694F2C: 38EB4C50  addi r7, r11, 0x4c50
	ctx.r[7].s64 = ctx.r[11].s64 + 19536;
	// 82694F30: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82694F34: 388AB204  addi r4, r10, -0x4dfc
	ctx.r[4].s64 = ctx.r[10].s64 + -19964;
	// 82694F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694F3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694F40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82694F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694F48: 386A12E0  addi r3, r10, 0x12e0
	ctx.r[3].s64 = ctx.r[10].s64 + 4832;
	// 82694F4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694F5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694F6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694F70: 4BDD1EB1  bl 0x82466e20
	ctx.lr = 0x82694F74;
	sub_82466E20(ctx, base);
	// 82694F74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694F88 size=108
    let mut pc: u32 = 0x82694F88;
    'dispatch: loop {
        match pc {
            0x82694F88 => {
    //   block [0x82694F88..0x82694FF4)
	// 82694F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694F94: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694F98: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82694F9C: 38EBBD58  addi r7, r11, -0x42a8
	ctx.r[7].s64 = ctx.r[11].s64 + -17064;
	// 82694FA0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82694FA4: 388AB224  addi r4, r10, -0x4ddc
	ctx.r[4].s64 = ctx.r[10].s64 + -19932;
	// 82694FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694FAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694FB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82694FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694FB8: 386A1310  addi r3, r10, 0x1310
	ctx.r[3].s64 = ctx.r[10].s64 + 4880;
	// 82694FBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694FDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694FE0: 4BDD1E41  bl 0x82466e20
	ctx.lr = 0x82694FE4;
	sub_82466E20(ctx, base);
	// 82694FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694FF8 size=108
    let mut pc: u32 = 0x82694FF8;
    'dispatch: loop {
        match pc {
            0x82694FF8 => {
    //   block [0x82694FF8..0x82695064)
	// 82694FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695004: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695008: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269500C: 38EBBDA0  addi r7, r11, -0x4260
	ctx.r[7].s64 = ctx.r[11].s64 + -16992;
	// 82695010: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82695014: 388AB248  addi r4, r10, -0x4db8
	ctx.r[4].s64 = ctx.r[10].s64 + -19896;
	// 82695018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269501C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695020: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82695024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695028: 386A1340  addi r3, r10, 0x1340
	ctx.r[3].s64 = ctx.r[10].s64 + 4928;
	// 8269502C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82695030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269503C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269504C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82695050: 4BDD1DD1  bl 0x82466e20
	ctx.lr = 0x82695054;
	sub_82466E20(ctx, base);
	// 82695054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269505C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695068 size=108
    let mut pc: u32 = 0x82695068;
    'dispatch: loop {
        match pc {
            0x82695068 => {
    //   block [0x82695068..0x826950D4)
	// 82695068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269506C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695074: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695078: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269507C: 38EBBDD0  addi r7, r11, -0x4230
	ctx.r[7].s64 = ctx.r[11].s64 + -16944;
	// 82695080: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82695084: 388AB264  addi r4, r10, -0x4d9c
	ctx.r[4].s64 = ctx.r[10].s64 + -19868;
	// 82695088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269508C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82695094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695098: 386A1370  addi r3, r10, 0x1370
	ctx.r[3].s64 = ctx.r[10].s64 + 4976;
	// 8269509C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826950A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826950A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826950A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826950AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826950B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826950B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826950B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826950BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826950C0: 4BDD1D61  bl 0x82466e20
	ctx.lr = 0x826950C4;
	sub_82466E20(ctx, base);
	// 826950C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826950C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826950CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826950D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826950D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826950D8 size=100
    let mut pc: u32 = 0x826950D8;
    'dispatch: loop {
        match pc {
            0x826950D8 => {
    //   block [0x826950D8..0x8269513C)
	// 826950D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826950DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826950E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826950E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826950E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826950EC: 38AA1370  addi r5, r10, 0x1370
	ctx.r[5].s64 = ctx.r[10].s64 + 4976;
	// 826950F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826950F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826950F8: 388AA8D0  addi r4, r10, -0x5730
	ctx.r[4].s64 = ctx.r[10].s64 + -22320;
	// 826950FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82695104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269510C: 386A13A0  addi r3, r10, 0x13a0
	ctx.r[3].s64 = ctx.r[10].s64 + 5024;
	// 82695110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695114: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695118: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269511C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695120: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82695124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695128: 4BDD1CF9  bl 0x82466e20
	ctx.lr = 0x8269512C;
	sub_82466E20(ctx, base);
	// 8269512C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695140 size=112
    let mut pc: u32 = 0x82695140;
    'dispatch: loop {
        match pc {
            0x82695140 => {
    //   block [0x82695140..0x826951B0)
	// 82695140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269514C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82695150: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695154: 392A85A8  addi r9, r10, -0x7a58
	ctx.r[9].s64 = ctx.r[10].s64 + -31320;
	// 82695158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269515C: 390BBE00  addi r8, r11, -0x4200
	ctx.r[8].s64 = ctx.r[11].s64 + -16896;
	// 82695160: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82695164: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 82695168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269516C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695178: 386A13D0  addi r3, r10, 0x13d0
	ctx.r[3].s64 = ctx.r[10].s64 + 5072;
	// 8269517C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82695180: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82695184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269518C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82695198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269519C: 4BDD1C85  bl 0x82466e20
	ctx.lr = 0x826951A0;
	sub_82466E20(ctx, base);
	// 826951A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826951A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826951A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826951AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826951B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826951B0 size=112
    let mut pc: u32 = 0x826951B0;
    'dispatch: loop {
        match pc {
            0x826951B0 => {
    //   block [0x826951B0..0x82695220)
	// 826951B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826951B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826951B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826951BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826951C0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826951C4: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 826951C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826951CC: 390BBE48  addi r8, r11, -0x41b8
	ctx.r[8].s64 = ctx.r[11].s64 + -16824;
	// 826951D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826951D4: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 826951D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826951DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826951E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826951E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826951E8: 386A1400  addi r3, r10, 0x1400
	ctx.r[3].s64 = ctx.r[10].s64 + 5120;
	// 826951EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826951F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826951F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826951F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826951FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269520C: 4BDD1C15  bl 0x82466e20
	ctx.lr = 0x82695210;
	sub_82466E20(ctx, base);
	// 82695210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269521C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695220 size=116
    let mut pc: u32 = 0x82695220;
    'dispatch: loop {
        match pc {
            0x82695220 => {
    //   block [0x82695220..0x82695294)
	// 82695220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269522C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695230: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82695234: 390ABE78  addi r8, r10, -0x4188
	ctx.r[8].s64 = ctx.r[10].s64 + -16776;
	// 82695238: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269523C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695240: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 82695244: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695248: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269524C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695250: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695254: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 82695258: 396B85D0  addi r11, r11, -0x7a30
	ctx.r[11].s64 = ctx.r[11].s64 + -31280;
	// 8269525C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695260: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695264: 386A1430  addi r3, r10, 0x1430
	ctx.r[3].s64 = ctx.r[10].s64 + 5168;
	// 82695268: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269526C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695270: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269527C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695280: 4BDD1BA1  bl 0x82466e20
	ctx.lr = 0x82695284;
	sub_82466E20(ctx, base);
	// 82695284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269528C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695298 size=100
    let mut pc: u32 = 0x82695298;
    'dispatch: loop {
        match pc {
            0x82695298 => {
    //   block [0x82695298..0x826952FC)
	// 82695298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269529C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826952A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826952A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826952A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826952AC: 38AA1430  addi r5, r10, 0x1430
	ctx.r[5].s64 = ctx.r[10].s64 + 5168;
	// 826952B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826952B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826952B8: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826952BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826952C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826952C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826952C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826952CC: 386A1460  addi r3, r10, 0x1460
	ctx.r[3].s64 = ctx.r[10].s64 + 5216;
	// 826952D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826952D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826952D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826952DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826952E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826952E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826952E8: 4BDD1B39  bl 0x82466e20
	ctx.lr = 0x826952EC;
	sub_82466E20(ctx, base);
	// 826952EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826952F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826952F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826952F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695300 size=116
    let mut pc: u32 = 0x82695300;
    'dispatch: loop {
        match pc {
            0x82695300 => {
    //   block [0x82695300..0x82695374)
	// 82695300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269530C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695310: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695314: 392B8624  addi r9, r11, -0x79dc
	ctx.r[9].s64 = ctx.r[11].s64 + -31196;
	// 82695318: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 8269531C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695320: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82695324: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 82695328: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269532C: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 82695330: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695334: 396BBF20  addi r11, r11, -0x40e0
	ctx.r[11].s64 = ctx.r[11].s64 + -16608;
	// 82695338: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8269533C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695340: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82695344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695348: 386A1490  addi r3, r10, 0x1490
	ctx.r[3].s64 = ctx.r[10].s64 + 5264;
	// 8269534C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82695350: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82695354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695358: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8269535C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82695360: 4BDD1AC1  bl 0x82466e20
	ctx.lr = 0x82695364;
	sub_82466E20(ctx, base);
	// 82695364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269536C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82695378 size=24
    let mut pc: u32 = 0x82695378;
    'dispatch: loop {
        match pc {
            0x82695378 => {
    //   block [0x82695378..0x82695390)
	// 82695378: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269537C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695380: 394A4CE0  addi r10, r10, 0x4ce0
	ctx.r[10].s64 = ctx.r[10].s64 + 19680;
	// 82695384: 816BC070  lwz r11, -0x3f90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16272 as u32) ) } as u64;
	// 82695388: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8269538C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695390 size=116
    let mut pc: u32 = 0x82695390;
    'dispatch: loop {
        match pc {
            0x82695390 => {
    //   block [0x82695390..0x82695404)
	// 82695390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269539C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826953A0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826953A4: 392B8690  addi r9, r11, -0x7970
	ctx.r[9].s64 = ctx.r[11].s64 + -31088;
	// 826953A8: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826953AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826953B0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826953B4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 826953B8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826953BC: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 826953C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826953C4: 396B4CE0  addi r11, r11, 0x4ce0
	ctx.r[11].s64 = ctx.r[11].s64 + 19680;
	// 826953C8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826953CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826953D0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826953D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826953D8: 386A14C0  addi r3, r10, 0x14c0
	ctx.r[3].s64 = ctx.r[10].s64 + 5312;
	// 826953DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826953E0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826953E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826953E8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826953EC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826953F0: 4BDD1A31  bl 0x82466e20
	ctx.lr = 0x826953F4;
	sub_82466E20(ctx, base);
	// 826953F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826953F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826953FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695408 size=108
    let mut pc: u32 = 0x82695408;
    'dispatch: loop {
        match pc {
            0x82695408 => {
    //   block [0x82695408..0x82695474)
	// 82695408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269540C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695414: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269541C: 38EBC078  addi r7, r11, -0x3f88
	ctx.r[7].s64 = ctx.r[11].s64 + -16264;
	// 82695420: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82695424: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 82695428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269542C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695430: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82695434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695438: 386A14F0  addi r3, r10, 0x14f0
	ctx.r[3].s64 = ctx.r[10].s64 + 5360;
	// 8269543C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82695440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269544C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269545C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82695460: 4BDD19C1  bl 0x82466e20
	ctx.lr = 0x82695464;
	sub_82466E20(ctx, base);
	// 82695464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269546C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82695478 size=24
    let mut pc: u32 = 0x82695478;
    'dispatch: loop {
        match pc {
            0x82695478 => {
    //   block [0x82695478..0x82695490)
	// 82695478: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269547C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695480: 394A4DA0  addi r10, r10, 0x4da0
	ctx.r[10].s64 = ctx.r[10].s64 + 19872;
	// 82695484: 816BC074  lwz r11, -0x3f8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16268 as u32) ) } as u64;
	// 82695488: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8269548C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695490 size=116
    let mut pc: u32 = 0x82695490;
    'dispatch: loop {
        match pc {
            0x82695490 => {
    //   block [0x82695490..0x82695504)
	// 82695490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269549C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826954A0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826954A4: 390B4DA0  addi r8, r11, 0x4da0
	ctx.r[8].s64 = ctx.r[11].s64 + 19872;
	// 826954A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826954AC: 392A8710  addi r9, r10, -0x78f0
	ctx.r[9].s64 = ctx.r[10].s64 + -30960;
	// 826954B0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826954B4: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 826954B8: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 826954BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826954C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826954C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826954C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826954CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826954D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826954D4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826954D8: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826954DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826954E0: 386B1520  addi r3, r11, 0x1520
	ctx.r[3].s64 = ctx.r[11].s64 + 5408;
	// 826954E4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826954E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826954EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826954F0: 4BDD1931  bl 0x82466e20
	ctx.lr = 0x826954F4;
	sub_82466E20(ctx, base);
	// 826954F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826954F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826954FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695508 size=112
    let mut pc: u32 = 0x82695508;
    'dispatch: loop {
        match pc {
            0x82695508 => {
    //   block [0x82695508..0x82695578)
	// 82695508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269550C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695514: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695518: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269551C: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 82695520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695524: 390BC0F4  addi r8, r11, -0x3f0c
	ctx.r[8].s64 = ctx.r[11].s64 + -16140;
	// 82695528: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269552C: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 82695530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82695534: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695538: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269553C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695540: 386A1550  addi r3, r10, 0x1550
	ctx.r[3].s64 = ctx.r[10].s64 + 5456;
	// 82695544: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82695548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269554C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82695554: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269555C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695564: 4BDD18BD  bl 0x82466e20
	ctx.lr = 0x82695568;
	sub_82466E20(ctx, base);
	// 82695568: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269556C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82695578 size=24
    let mut pc: u32 = 0x82695578;
    'dispatch: loop {
        match pc {
            0x82695578 => {
    //   block [0x82695578..0x82695590)
	// 82695578: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269557C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695580: 394A4F38  addi r10, r10, 0x4f38
	ctx.r[10].s64 = ctx.r[10].s64 + 20280;
	// 82695584: 816BC124  lwz r11, -0x3edc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16092 as u32) ) } as u64;
	// 82695588: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 8269558C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695590 size=116
    let mut pc: u32 = 0x82695590;
    'dispatch: loop {
        match pc {
            0x82695590 => {
    //   block [0x82695590..0x82695604)
	// 82695590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269559C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826955A0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826955A4: 392B8748  addi r9, r11, -0x78b8
	ctx.r[9].s64 = ctx.r[11].s64 + -30904;
	// 826955A8: 38AA1490  addi r5, r10, 0x1490
	ctx.r[5].s64 = ctx.r[10].s64 + 5264;
	// 826955AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826955B0: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826955B4: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 826955B8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826955BC: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 826955C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826955C4: 396B4F38  addi r11, r11, 0x4f38
	ctx.r[11].s64 = ctx.r[11].s64 + 20280;
	// 826955C8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826955CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826955D0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826955D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826955D8: 386A1580  addi r3, r10, 0x1580
	ctx.r[3].s64 = ctx.r[10].s64 + 5504;
	// 826955DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826955E0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826955E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826955E8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826955EC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826955F0: 4BDD1831  bl 0x82466e20
	ctx.lr = 0x826955F4;
	sub_82466E20(ctx, base);
	// 826955F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826955F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826955FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695608 size=116
    let mut pc: u32 = 0x82695608;
    'dispatch: loop {
        match pc {
            0x82695608 => {
    //   block [0x82695608..0x8269567C)
	// 82695608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269560C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695614: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695618: 38E0000F  li r7, 0xf
	ctx.r[7].s64 = 15;
	// 8269561C: 390AC128  addi r8, r10, -0x3ed8
	ctx.r[8].s64 = ctx.r[10].s64 + -16088;
	// 82695620: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695624: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695628: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 8269562C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82695630: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82695634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695638: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269563C: 388AB294  addi r4, r10, -0x4d6c
	ctx.r[4].s64 = ctx.r[10].s64 + -19820;
	// 82695640: 396B87B8  addi r11, r11, -0x7848
	ctx.r[11].s64 = ctx.r[11].s64 + -30792;
	// 82695644: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695648: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269564C: 386A15B0  addi r3, r10, 0x15b0
	ctx.r[3].s64 = ctx.r[10].s64 + 5552;
	// 82695650: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82695654: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695658: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269565C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695668: 4BDD17B9  bl 0x82466e20
	ctx.lr = 0x8269566C;
	sub_82466E20(ctx, base);
	// 8269566C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695680 size=116
    let mut pc: u32 = 0x82695680;
    'dispatch: loop {
        match pc {
            0x82695680 => {
    //   block [0x82695680..0x826956F4)
	// 82695680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269568C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695690: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82695694: 390AC290  addi r8, r10, -0x3d70
	ctx.r[8].s64 = ctx.r[10].s64 + -15728;
	// 82695698: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269569C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826956A0: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826956A4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826956A8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826956AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826956B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826956B4: 388AB2BC  addi r4, r10, -0x4d44
	ctx.r[4].s64 = ctx.r[10].s64 + -19780;
	// 826956B8: 396B8834  addi r11, r11, -0x77cc
	ctx.r[11].s64 = ctx.r[11].s64 + -30668;
	// 826956BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826956C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826956C4: 386A15E0  addi r3, r10, 0x15e0
	ctx.r[3].s64 = ctx.r[10].s64 + 5600;
	// 826956C8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826956CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826956D0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826956D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826956D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826956DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826956E0: 4BDD1741  bl 0x82466e20
	ctx.lr = 0x826956E4;
	sub_82466E20(ctx, base);
	// 826956E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826956E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826956EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826956F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826956F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826956F8 size=112
    let mut pc: u32 = 0x826956F8;
    'dispatch: loop {
        match pc {
            0x826956F8 => {
    //   block [0x826956F8..0x82695768)
	// 826956F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826956FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695704: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695708: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269570C: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 82695710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695714: 390BC2D8  addi r8, r11, -0x3d28
	ctx.r[8].s64 = ctx.r[11].s64 + -15656;
	// 82695718: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269571C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 82695720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82695724: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269572C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695730: 386A1610  addi r3, r10, 0x1610
	ctx.r[3].s64 = ctx.r[10].s64 + 5648;
	// 82695734: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82695738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269573C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82695744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269574C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695754: 4BDD16CD  bl 0x82466e20
	ctx.lr = 0x82695758;
	sub_82466E20(ctx, base);
	// 82695758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269575C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695768 size=100
    let mut pc: u32 = 0x82695768;
    'dispatch: loop {
        match pc {
            0x82695768 => {
    //   block [0x82695768..0x826957CC)
	// 82695768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269576C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695774: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269577C: 38AA1C70  addi r5, r10, 0x1c70
	ctx.r[5].s64 = ctx.r[10].s64 + 7280;
	// 82695780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695788: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 8269578C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82695794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269579C: 386A1640  addi r3, r10, 0x1640
	ctx.r[3].s64 = ctx.r[10].s64 + 5696;
	// 826957A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826957A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826957A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826957AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826957B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826957B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826957B8: 4BDD1669  bl 0x82466e20
	ctx.lr = 0x826957BC;
	sub_82466E20(ctx, base);
	// 826957BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826957C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826957C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826957C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826957D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826957D0 size=112
    let mut pc: u32 = 0x826957D0;
    'dispatch: loop {
        match pc {
            0x826957D0 => {
    //   block [0x826957D0..0x82695840)
	// 826957D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826957D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826957D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826957DC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826957E0: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826957E4: 38EAC2F0  addi r7, r10, -0x3d10
	ctx.r[7].s64 = ctx.r[10].s64 + -15632;
	// 826957E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826957EC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826957F0: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 826957F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826957F8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826957FC: 396B8860  addi r11, r11, -0x77a0
	ctx.r[11].s64 = ctx.r[11].s64 + -30624;
	// 82695800: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82695804: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695808: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269580C: 386A1670  addi r3, r10, 0x1670
	ctx.r[3].s64 = ctx.r[10].s64 + 5744;
	// 82695810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695814: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82695818: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269581C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695820: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695824: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695828: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269582C: 4BDD15F5  bl 0x82466e20
	ctx.lr = 0x82695830;
	sub_82466E20(ctx, base);
	// 82695830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269583C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695840 size=112
    let mut pc: u32 = 0x82695840;
    'dispatch: loop {
        match pc {
            0x82695840 => {
    //   block [0x82695840..0x826958B0)
	// 82695840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269584C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695850: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695854: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82695858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269585C: 390BC428  addi r8, r11, -0x3bd8
	ctx.r[8].s64 = ctx.r[11].s64 + -15320;
	// 82695860: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82695864: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 82695868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269586C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695878: 386A16A0  addi r3, r10, 0x16a0
	ctx.r[3].s64 = ctx.r[10].s64 + 5792;
	// 8269587C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82695880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269588C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269589C: 4BDD1585  bl 0x82466e20
	ctx.lr = 0x826958A0;
	sub_82466E20(ctx, base);
	// 826958A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826958A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826958A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826958AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826958B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826958B0 size=112
    let mut pc: u32 = 0x826958B0;
    'dispatch: loop {
        match pc {
            0x826958B0 => {
    //   block [0x826958B0..0x82695920)
	// 826958B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826958B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826958B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826958BC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826958C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826958C4: 38EAC458  addi r7, r10, -0x3ba8
	ctx.r[7].s64 = ctx.r[10].s64 + -15272;
	// 826958C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826958CC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826958D0: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 826958D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826958D8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826958DC: 396B88B4  addi r11, r11, -0x774c
	ctx.r[11].s64 = ctx.r[11].s64 + -30540;
	// 826958E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826958E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826958E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826958EC: 386A16D0  addi r3, r10, 0x16d0
	ctx.r[3].s64 = ctx.r[10].s64 + 5840;
	// 826958F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826958F4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826958F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826958FC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695900: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695904: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695908: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269590C: 4BDD1515  bl 0x82466e20
	ctx.lr = 0x82695910;
	sub_82466E20(ctx, base);
	// 82695910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269591C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695920 size=112
    let mut pc: u32 = 0x82695920;
    'dispatch: loop {
        match pc {
            0x82695920 => {
    //   block [0x82695920..0x82695990)
	// 82695920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269592C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695930: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695934: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82695938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269593C: 390BC488  addi r8, r11, -0x3b78
	ctx.r[8].s64 = ctx.r[11].s64 + -15224;
	// 82695940: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82695944: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 82695948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269594C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695958: 386A1700  addi r3, r10, 0x1700
	ctx.r[3].s64 = ctx.r[10].s64 + 5888;
	// 8269595C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82695960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269596C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269597C: 4BDD14A5  bl 0x82466e20
	ctx.lr = 0x82695980;
	sub_82466E20(ctx, base);
	// 82695980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269598C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695990 size=108
    let mut pc: u32 = 0x82695990;
    'dispatch: loop {
        match pc {
            0x82695990 => {
    //   block [0x82695990..0x826959FC)
	// 82695990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269599C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826959A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826959A4: 38EBC4A0  addi r7, r11, -0x3b60
	ctx.r[7].s64 = ctx.r[11].s64 + -15200;
	// 826959A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826959AC: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 826959B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826959B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826959B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826959BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826959C0: 386A1730  addi r3, r10, 0x1730
	ctx.r[3].s64 = ctx.r[10].s64 + 5936;
	// 826959C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826959C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826959CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826959D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826959D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826959D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826959DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826959E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826959E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826959E8: 4BDD1439  bl 0x82466e20
	ctx.lr = 0x826959EC;
	sub_82466E20(ctx, base);
	// 826959EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826959F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826959F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826959F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695A00 size=112
    let mut pc: u32 = 0x82695A00;
    'dispatch: loop {
        match pc {
            0x82695A00 => {
    //   block [0x82695A00..0x82695A70)
	// 82695A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695A0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695A10: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695A14: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82695A18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695A1C: 390BC4B8  addi r8, r11, -0x3b48
	ctx.r[8].s64 = ctx.r[11].s64 + -15176;
	// 82695A20: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82695A24: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 82695A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82695A2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695A30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695A38: 386A1760  addi r3, r10, 0x1760
	ctx.r[3].s64 = ctx.r[10].s64 + 5984;
	// 82695A3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82695A40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82695A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695A5C: 4BDD13C5  bl 0x82466e20
	ctx.lr = 0x82695A60;
	sub_82466E20(ctx, base);
	// 82695A60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695A70 size=112
    let mut pc: u32 = 0x82695A70;
    'dispatch: loop {
        match pc {
            0x82695A70 => {
    //   block [0x82695A70..0x82695AE0)
	// 82695A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695A7C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695A80: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82695A84: 38EAC4D0  addi r7, r10, -0x3b30
	ctx.r[7].s64 = ctx.r[10].s64 + -15152;
	// 82695A88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695A8C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695A90: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 82695A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695A98: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82695A9C: 396B88C0  addi r11, r11, -0x7740
	ctx.r[11].s64 = ctx.r[11].s64 + -30528;
	// 82695AA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82695AA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695AA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695AAC: 386A1790  addi r3, r10, 0x1790
	ctx.r[3].s64 = ctx.r[10].s64 + 6032;
	// 82695AB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695AB4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82695AB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695ABC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695AC0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695AC4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695AC8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82695ACC: 4BDD1355  bl 0x82466e20
	ctx.lr = 0x82695AD0;
	sub_82466E20(ctx, base);
	// 82695AD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695AE0 size=112
    let mut pc: u32 = 0x82695AE0;
    'dispatch: loop {
        match pc {
            0x82695AE0 => {
    //   block [0x82695AE0..0x82695B50)
	// 82695AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695AEC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695AF0: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 82695AF4: 38EAC5A8  addi r7, r10, -0x3a58
	ctx.r[7].s64 = ctx.r[10].s64 + -14936;
	// 82695AF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695AFC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695B00: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 82695B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695B08: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82695B0C: 396B8900  addi r11, r11, -0x7700
	ctx.r[11].s64 = ctx.r[11].s64 + -30464;
	// 82695B10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82695B14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695B18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695B1C: 386A17C0  addi r3, r10, 0x17c0
	ctx.r[3].s64 = ctx.r[10].s64 + 6080;
	// 82695B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695B24: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82695B28: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695B2C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695B30: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695B34: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695B38: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82695B3C: 4BDD12E5  bl 0x82466e20
	ctx.lr = 0x82695B40;
	sub_82466E20(ctx, base);
	// 82695B40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695B50 size=108
    let mut pc: u32 = 0x82695B50;
    'dispatch: loop {
        match pc {
            0x82695B50 => {
    //   block [0x82695B50..0x82695BBC)
	// 82695B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695B5C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695B64: 38EBC710  addi r7, r11, -0x38f0
	ctx.r[7].s64 = ctx.r[11].s64 + -14576;
	// 82695B68: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82695B6C: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 82695B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82695B74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695B78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82695B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695B80: 386A17F0  addi r3, r10, 0x17f0
	ctx.r[3].s64 = ctx.r[10].s64 + 6128;
	// 82695B84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82695B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695B8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82695B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695BA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82695BA8: 4BDD1279  bl 0x82466e20
	ctx.lr = 0x82695BAC;
	sub_82466E20(ctx, base);
	// 82695BAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695BB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695BB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695BC0 size=116
    let mut pc: u32 = 0x82695BC0;
    'dispatch: loop {
        match pc {
            0x82695BC0 => {
    //   block [0x82695BC0..0x82695C34)
	// 82695BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695BCC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695BD0: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82695BD4: 390AC788  addi r8, r10, -0x3878
	ctx.r[8].s64 = ctx.r[10].s64 + -14456;
	// 82695BD8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695BDC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695BE0: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82695BE4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695BE8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82695BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695BF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695BF4: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 82695BF8: 396B89A8  addi r11, r11, -0x7658
	ctx.r[11].s64 = ctx.r[11].s64 + -30296;
	// 82695BFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695C00: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695C04: 386A1820  addi r3, r10, 0x1820
	ctx.r[3].s64 = ctx.r[10].s64 + 6176;
	// 82695C08: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82695C0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695C10: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695C18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695C20: 4BDD1201  bl 0x82466e20
	ctx.lr = 0x82695C24;
	sub_82466E20(ctx, base);
	// 82695C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695C38 size=116
    let mut pc: u32 = 0x82695C38;
    'dispatch: loop {
        match pc {
            0x82695C38 => {
    //   block [0x82695C38..0x82695CAC)
	// 82695C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695C44: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695C48: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82695C4C: 390AC8A8  addi r8, r10, -0x3758
	ctx.r[8].s64 = ctx.r[10].s64 + -14168;
	// 82695C50: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695C54: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695C58: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82695C5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695C60: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82695C64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695C6C: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 82695C70: 396B89E0  addi r11, r11, -0x7620
	ctx.r[11].s64 = ctx.r[11].s64 + -30240;
	// 82695C74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695C78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695C7C: 386A1850  addi r3, r10, 0x1850
	ctx.r[3].s64 = ctx.r[10].s64 + 6224;
	// 82695C80: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82695C84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695C88: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695C98: 4BDD1189  bl 0x82466e20
	ctx.lr = 0x82695C9C;
	sub_82466E20(ctx, base);
	// 82695C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695CB0 size=108
    let mut pc: u32 = 0x82695CB0;
    'dispatch: loop {
        match pc {
            0x82695CB0 => {
    //   block [0x82695CB0..0x82695D1C)
	// 82695CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695CBC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695CC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82695CC4: 38EBC908  addi r7, r11, -0x36f8
	ctx.r[7].s64 = ctx.r[11].s64 + -14072;
	// 82695CC8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82695CCC: 388AA8F8  addi r4, r10, -0x5708
	ctx.r[4].s64 = ctx.r[10].s64 + -22280;
	// 82695CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82695CD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82695CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695CE0: 386A1880  addi r3, r10, 0x1880
	ctx.r[3].s64 = ctx.r[10].s64 + 6272;
	// 82695CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82695CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82695CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82695D08: 4BDD1119  bl 0x82466e20
	ctx.lr = 0x82695D0C;
	sub_82466E20(ctx, base);
	// 82695D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695D20 size=112
    let mut pc: u32 = 0x82695D20;
    'dispatch: loop {
        match pc {
            0x82695D20 => {
    //   block [0x82695D20..0x82695D90)
	// 82695D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695D2C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695D30: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82695D34: 38EAC9B0  addi r7, r10, -0x3650
	ctx.r[7].s64 = ctx.r[10].s64 + -13904;
	// 82695D38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82695D3C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695D40: 388AA910  addi r4, r10, -0x56f0
	ctx.r[4].s64 = ctx.r[10].s64 + -22256;
	// 82695D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695D48: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82695D4C: 396B89F8  addi r11, r11, -0x7608
	ctx.r[11].s64 = ctx.r[11].s64 + -30216;
	// 82695D50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82695D54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695D58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695D5C: 386A18B0  addi r3, r10, 0x18b0
	ctx.r[3].s64 = ctx.r[10].s64 + 6320;
	// 82695D60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695D64: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82695D68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695D6C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695D70: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695D74: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695D78: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82695D7C: 4BDD10A5  bl 0x82466e20
	ctx.lr = 0x82695D80;
	sub_82466E20(ctx, base);
	// 82695D80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695D90 size=112
    let mut pc: u32 = 0x82695D90;
    'dispatch: loop {
        match pc {
            0x82695D90 => {
    //   block [0x82695D90..0x82695E00)
	// 82695D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695D9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695DA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695DA4: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82695DA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695DAC: 390BCA88  addi r8, r11, -0x3578
	ctx.r[8].s64 = ctx.r[11].s64 + -13688;
	// 82695DB0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82695DB4: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 82695DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82695DBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695DC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695DC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695DC8: 386A18E0  addi r3, r10, 0x18e0
	ctx.r[3].s64 = ctx.r[10].s64 + 6368;
	// 82695DCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82695DD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695DD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82695DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695DE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695DE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695DEC: 4BDD1035  bl 0x82466e20
	ctx.lr = 0x82695DF0;
	sub_82466E20(ctx, base);
	// 82695DF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695DF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695DF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82695E00 size=24
    let mut pc: u32 = 0x82695E00;
    'dispatch: loop {
        match pc {
            0x82695E00 => {
    //   block [0x82695E00..0x82695E18)
	// 82695E00: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695E04: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695E08: 394A50A0  addi r10, r10, 0x50a0
	ctx.r[10].s64 = ctx.r[10].s64 + 20640;
	// 82695E0C: 816BCAD0  lwz r11, -0x3530(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13616 as u32) ) } as u64;
	// 82695E10: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82695E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695E18 size=116
    let mut pc: u32 = 0x82695E18;
    'dispatch: loop {
        match pc {
            0x82695E18 => {
    //   block [0x82695E18..0x82695E8C)
	// 82695E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695E24: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695E28: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695E2C: 392B8A68  addi r9, r11, -0x7598
	ctx.r[9].s64 = ctx.r[11].s64 + -30104;
	// 82695E30: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82695E34: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82695E38: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 82695E3C: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 82695E40: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695E44: 388A7B20  addi r4, r10, 0x7b20
	ctx.r[4].s64 = ctx.r[10].s64 + 31520;
	// 82695E48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695E4C: 396B50A0  addi r11, r11, 0x50a0
	ctx.r[11].s64 = ctx.r[11].s64 + 20640;
	// 82695E50: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82695E54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695E58: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82695E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695E60: 386A1910  addi r3, r10, 0x1910
	ctx.r[3].s64 = ctx.r[10].s64 + 6416;
	// 82695E64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82695E68: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82695E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695E70: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82695E74: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82695E78: 4BDD0FA9  bl 0x82466e20
	ctx.lr = 0x82695E7C;
	sub_82466E20(ctx, base);
	// 82695E7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695E90 size=116
    let mut pc: u32 = 0x82695E90;
    'dispatch: loop {
        match pc {
            0x82695E90 => {
    //   block [0x82695E90..0x82695F04)
	// 82695E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695E9C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695EA0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82695EA4: 390ACAD8  addi r8, r10, -0x3528
	ctx.r[8].s64 = ctx.r[10].s64 + -13608;
	// 82695EA8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695EAC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695EB0: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82695EB4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82695EB8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82695EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695EC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695EC4: 388AB2FC  addi r4, r10, -0x4d04
	ctx.r[4].s64 = ctx.r[10].s64 + -19716;
	// 82695EC8: 396B8AC4  addi r11, r11, -0x753c
	ctx.r[11].s64 = ctx.r[11].s64 + -30012;
	// 82695ECC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695ED0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695ED4: 386A1940  addi r3, r10, 0x1940
	ctx.r[3].s64 = ctx.r[10].s64 + 6464;
	// 82695ED8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82695EDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695EE0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695EF0: 4BDD0F31  bl 0x82466e20
	ctx.lr = 0x82695EF4;
	sub_82466E20(ctx, base);
	// 82695EF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695F08 size=116
    let mut pc: u32 = 0x82695F08;
    'dispatch: loop {
        match pc {
            0x82695F08 => {
    //   block [0x82695F08..0x82695F7C)
	// 82695F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695F14: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695F18: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 82695F1C: 390ACB20  addi r8, r10, -0x34e0
	ctx.r[8].s64 = ctx.r[10].s64 + -13536;
	// 82695F20: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695F24: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695F28: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82695F2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695F30: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82695F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695F38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695F3C: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 82695F40: 396B8AD8  addi r11, r11, -0x7528
	ctx.r[11].s64 = ctx.r[11].s64 + -29992;
	// 82695F44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695F48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695F4C: 386A1970  addi r3, r10, 0x1970
	ctx.r[3].s64 = ctx.r[10].s64 + 6512;
	// 82695F50: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82695F54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695F58: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695F5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695F68: 4BDD0EB9  bl 0x82466e20
	ctx.lr = 0x82695F6C;
	sub_82466E20(ctx, base);
	// 82695F6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695F80 size=112
    let mut pc: u32 = 0x82695F80;
    'dispatch: loop {
        match pc {
            0x82695F80 => {
    //   block [0x82695F80..0x82695FF0)
	// 82695F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695F8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695F90: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695F94: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82695F98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82695F9C: 390BCC58  addi r8, r11, -0x33a8
	ctx.r[8].s64 = ctx.r[11].s64 + -13224;
	// 82695FA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82695FA4: 388AA928  addi r4, r10, -0x56d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22232;
	// 82695FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82695FAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695FB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695FB8: 386A19A0  addi r3, r10, 0x19a0
	ctx.r[3].s64 = ctx.r[10].s64 + 6560;
	// 82695FBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82695FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695FC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82695FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695FDC: 4BDD0E45  bl 0x82466e20
	ctx.lr = 0x82695FE0;
	sub_82466E20(ctx, base);
	// 82695FE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695FF0 size=112
    let mut pc: u32 = 0x82695FF0;
    'dispatch: loop {
        match pc {
            0x82695FF0 => {
    //   block [0x82695FF0..0x82696060)
	// 82695FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695FFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696000: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696004: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82696008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269600C: 390BCC70  addi r8, r11, -0x3390
	ctx.r[8].s64 = ctx.r[11].s64 + -13200;
	// 82696010: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82696014: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 82696018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269601C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696028: 386A19D0  addi r3, r10, 0x19d0
	ctx.r[3].s64 = ctx.r[10].s64 + 6608;
	// 8269602C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82696030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269603C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269604C: 4BDD0DD5  bl 0x82466e20
	ctx.lr = 0x82696050;
	sub_82466E20(ctx, base);
	// 82696050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269605C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696060 size=112
    let mut pc: u32 = 0x82696060;
    'dispatch: loop {
        match pc {
            0x82696060 => {
    //   block [0x82696060..0x826960D0)
	// 82696060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269606C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696070: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696074: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82696078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269607C: 390BCC88  addi r8, r11, -0x3378
	ctx.r[8].s64 = ctx.r[11].s64 + -13176;
	// 82696080: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82696084: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 82696088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269608C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696090: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696098: 386A1A00  addi r3, r10, 0x1a00
	ctx.r[3].s64 = ctx.r[10].s64 + 6656;
	// 8269609C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826960A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826960A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826960A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826960AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826960B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826960B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826960B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826960BC: 4BDD0D65  bl 0x82466e20
	ctx.lr = 0x826960C0;
	sub_82466E20(ctx, base);
	// 826960C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826960C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826960C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826960CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826960D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826960D0 size=112
    let mut pc: u32 = 0x826960D0;
    'dispatch: loop {
        match pc {
            0x826960D0 => {
    //   block [0x826960D0..0x82696140)
	// 826960D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826960D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826960D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826960DC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826960E0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826960E4: 38EACCB8  addi r7, r10, -0x3348
	ctx.r[7].s64 = ctx.r[10].s64 + -13128;
	// 826960E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826960EC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826960F0: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 826960F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826960F8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826960FC: 396B8B50  addi r11, r11, -0x74b0
	ctx.r[11].s64 = ctx.r[11].s64 + -29872;
	// 82696100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82696104: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696108: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269610C: 386A1A30  addi r3, r10, 0x1a30
	ctx.r[3].s64 = ctx.r[10].s64 + 6704;
	// 82696110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696114: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82696118: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269611C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82696120: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696124: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696128: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269612C: 4BDD0CF5  bl 0x82466e20
	ctx.lr = 0x82696130;
	sub_82466E20(ctx, base);
	// 82696130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269613C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696140 size=112
    let mut pc: u32 = 0x82696140;
    'dispatch: loop {
        match pc {
            0x82696140 => {
    //   block [0x82696140..0x826961B0)
	// 82696140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269614C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696150: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696154: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82696158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269615C: 390BCD30  addi r8, r11, -0x32d0
	ctx.r[8].s64 = ctx.r[11].s64 + -13008;
	// 82696160: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82696164: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 82696168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269616C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696178: 386A1A60  addi r3, r10, 0x1a60
	ctx.r[3].s64 = ctx.r[10].s64 + 6752;
	// 8269617C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82696180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269618C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269619C: 4BDD0C85  bl 0x82466e20
	ctx.lr = 0x826961A0;
	sub_82466E20(ctx, base);
	// 826961A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826961A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826961A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826961AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826961B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826961B0 size=24
    let mut pc: u32 = 0x826961B0;
    'dispatch: loop {
        match pc {
            0x826961B0 => {
    //   block [0x826961B0..0x826961C8)
	// 826961B0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826961B4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826961B8: 394A51A8  addi r10, r10, 0x51a8
	ctx.r[10].s64 = ctx.r[10].s64 + 20904;
	// 826961BC: 816BCAD4  lwz r11, -0x352c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13612 as u32) ) } as u64;
	// 826961C0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826961C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826961C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826961C8 size=116
    let mut pc: u32 = 0x826961C8;
    'dispatch: loop {
        match pc {
            0x826961C8 => {
    //   block [0x826961C8..0x8269623C)
	// 826961C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826961CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826961D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826961D4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826961D8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826961DC: 390B51A8  addi r8, r11, 0x51a8
	ctx.r[8].s64 = ctx.r[11].s64 + 20904;
	// 826961E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826961E4: 392A8B90  addi r9, r10, -0x7470
	ctx.r[9].s64 = ctx.r[10].s64 + -29808;
	// 826961E8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826961EC: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826961F0: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 826961F4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826961F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826961FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269620C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82696210: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 82696214: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82696218: 386B1A90  addi r3, r11, 0x1a90
	ctx.r[3].s64 = ctx.r[11].s64 + 6800;
	// 8269621C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82696220: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696228: 4BDD0BF9  bl 0x82466e20
	ctx.lr = 0x8269622C;
	sub_82466E20(ctx, base);
	// 8269622C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696240 size=116
    let mut pc: u32 = 0x82696240;
    'dispatch: loop {
        match pc {
            0x82696240 => {
    //   block [0x82696240..0x826962B4)
	// 82696240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269624C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82696250: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82696254: 390ACD60  addi r8, r10, -0x32a0
	ctx.r[8].s64 = ctx.r[10].s64 + -12960;
	// 82696258: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269625C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82696260: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82696264: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696268: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269626C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696274: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 82696278: 396B8BA4  addi r11, r11, -0x745c
	ctx.r[11].s64 = ctx.r[11].s64 + -29788;
	// 8269627C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696280: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696284: 386A1AC0  addi r3, r10, 0x1ac0
	ctx.r[3].s64 = ctx.r[10].s64 + 6848;
	// 82696288: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269628C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696290: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82696294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269629C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826962A0: 4BDD0B81  bl 0x82466e20
	ctx.lr = 0x826962A4;
	sub_82466E20(ctx, base);
	// 826962A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826962A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826962AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826962B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826962B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826962B8 size=112
    let mut pc: u32 = 0x826962B8;
    'dispatch: loop {
        match pc {
            0x826962B8 => {
    //   block [0x826962B8..0x82696328)
	// 826962B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826962BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826962C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826962C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826962C8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826962CC: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 826962D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826962D4: 390BCE20  addi r8, r11, -0x31e0
	ctx.r[8].s64 = ctx.r[11].s64 + -12768;
	// 826962D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826962DC: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 826962E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826962E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826962E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826962EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826962F0: 386A1AF0  addi r3, r10, 0x1af0
	ctx.r[3].s64 = ctx.r[10].s64 + 6896;
	// 826962F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826962F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826962FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269630C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696314: 4BDD0B0D  bl 0x82466e20
	ctx.lr = 0x82696318;
	sub_82466E20(ctx, base);
	// 82696318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269631C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696328 size=112
    let mut pc: u32 = 0x82696328;
    'dispatch: loop {
        match pc {
            0x82696328 => {
    //   block [0x82696328..0x82696398)
	// 82696328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269632C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696334: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82696338: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269633C: 38EACE38  addi r7, r10, -0x31c8
	ctx.r[7].s64 = ctx.r[10].s64 + -12744;
	// 82696340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696344: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82696348: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 8269634C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696350: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82696354: 396B8BC8  addi r11, r11, -0x7438
	ctx.r[11].s64 = ctx.r[11].s64 + -29752;
	// 82696358: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269635C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696360: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696364: 386A1B20  addi r3, r10, 0x1b20
	ctx.r[3].s64 = ctx.r[10].s64 + 6944;
	// 82696368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269636C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82696370: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696374: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82696378: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269637C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696380: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696384: 4BDD0A9D  bl 0x82466e20
	ctx.lr = 0x82696388;
	sub_82466E20(ctx, base);
	// 82696388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269638C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696398 size=112
    let mut pc: u32 = 0x82696398;
    'dispatch: loop {
        match pc {
            0x82696398 => {
    //   block [0x82696398..0x82696408)
	// 82696398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269639C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826963A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826963A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826963A8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826963AC: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 826963B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826963B4: 390BCE68  addi r8, r11, -0x3198
	ctx.r[8].s64 = ctx.r[11].s64 + -12696;
	// 826963B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826963BC: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826963C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826963C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826963C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826963CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826963D0: 386A1B50  addi r3, r10, 0x1b50
	ctx.r[3].s64 = ctx.r[10].s64 + 6992;
	// 826963D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826963D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826963DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826963E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826963E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826963E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826963EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826963F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826963F4: 4BDD0A2D  bl 0x82466e20
	ctx.lr = 0x826963F8;
	sub_82466E20(ctx, base);
	// 826963F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826963FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696408 size=116
    let mut pc: u32 = 0x82696408;
    'dispatch: loop {
        match pc {
            0x82696408 => {
    //   block [0x82696408..0x8269647C)
	// 82696408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269640C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696414: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82696418: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8269641C: 390ACE80  addi r8, r10, -0x3180
	ctx.r[8].s64 = ctx.r[10].s64 + -12672;
	// 82696420: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696424: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82696428: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 8269642C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696430: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82696434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269643C: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 82696440: 396B8BD4  addi r11, r11, -0x742c
	ctx.r[11].s64 = ctx.r[11].s64 + -29740;
	// 82696444: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696448: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269644C: 386A1B80  addi r3, r10, 0x1b80
	ctx.r[3].s64 = ctx.r[10].s64 + 7040;
	// 82696450: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82696454: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696458: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269645C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696468: 4BDD09B9  bl 0x82466e20
	ctx.lr = 0x8269646C;
	sub_82466E20(ctx, base);
	// 8269646C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696480 size=112
    let mut pc: u32 = 0x82696480;
    'dispatch: loop {
        match pc {
            0x82696480 => {
    //   block [0x82696480..0x826964F0)
	// 82696480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269648C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696490: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696494: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82696498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269649C: 390BCF28  addi r8, r11, -0x30d8
	ctx.r[8].s64 = ctx.r[11].s64 + -12504;
	// 826964A0: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 826964A4: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 826964A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826964AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826964B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826964B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826964B8: 386A1BB0  addi r3, r10, 0x1bb0
	ctx.r[3].s64 = ctx.r[10].s64 + 7088;
	// 826964BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826964C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826964C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826964C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826964CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826964D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826964D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826964D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826964DC: 4BDD0945  bl 0x82466e20
	ctx.lr = 0x826964E0;
	sub_82466E20(ctx, base);
	// 826964E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826964E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826964E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826964EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826964F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826964F0 size=24
    let mut pc: u32 = 0x826964F0;
    'dispatch: loop {
        match pc {
            0x826964F0 => {
    //   block [0x826964F0..0x82696508)
	// 826964F0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826964F4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826964F8: 394A52B0  addi r10, r10, 0x52b0
	ctx.r[10].s64 = ctx.r[10].s64 + 21168;
	// 826964FC: 816BD060  lwz r11, -0x2fa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12192 as u32) ) } as u64;
	// 82696500: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82696504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696508 size=116
    let mut pc: u32 = 0x82696508;
    'dispatch: loop {
        match pc {
            0x82696508 => {
    //   block [0x82696508..0x8269657C)
	// 82696508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269650C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696514: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82696518: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269651C: 392B8C04  addi r9, r11, -0x73fc
	ctx.r[9].s64 = ctx.r[11].s64 + -29692;
	// 82696520: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82696524: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82696528: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8269652C: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 82696530: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696534: 388A7B04  addi r4, r10, 0x7b04
	ctx.r[4].s64 = ctx.r[10].s64 + 31492;
	// 82696538: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269653C: 396B52B0  addi r11, r11, 0x52b0
	ctx.r[11].s64 = ctx.r[11].s64 + 21168;
	// 82696540: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82696544: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696548: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269654C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696550: 386A1BE0  addi r3, r10, 0x1be0
	ctx.r[3].s64 = ctx.r[10].s64 + 7136;
	// 82696554: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82696558: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8269655C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696560: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82696564: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82696568: 4BDD08B9  bl 0x82466e20
	ctx.lr = 0x8269656C;
	sub_82466E20(ctx, base);
	// 8269656C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696580 size=112
    let mut pc: u32 = 0x82696580;
    'dispatch: loop {
        match pc {
            0x82696580 => {
    //   block [0x82696580..0x826965F0)
	// 82696580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269658C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696590: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696594: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82696598: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269659C: 390BD064  addi r8, r11, -0x2f9c
	ctx.r[8].s64 = ctx.r[11].s64 + -12188;
	// 826965A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826965A4: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 826965A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826965AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826965B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826965B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826965B8: 386A1C10  addi r3, r10, 0x1c10
	ctx.r[3].s64 = ctx.r[10].s64 + 7184;
	// 826965BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826965C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826965C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826965C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826965CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826965D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826965D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826965D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826965DC: 4BDD0845  bl 0x82466e20
	ctx.lr = 0x826965E0;
	sub_82466E20(ctx, base);
	// 826965E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826965E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826965E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826965EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826965F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826965F0 size=24
    let mut pc: u32 = 0x826965F0;
    'dispatch: loop {
        match pc {
            0x826965F0 => {
    //   block [0x826965F0..0x82696608)
	// 826965F0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826965F4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826965F8: 394A5388  addi r10, r10, 0x5388
	ctx.r[10].s64 = ctx.r[10].s64 + 21384;
	// 826965FC: 816BD07C  lwz r11, -0x2f84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12164 as u32) ) } as u64;
	// 82696600: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82696604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696608 size=116
    let mut pc: u32 = 0x82696608;
    'dispatch: loop {
        match pc {
            0x82696608 => {
    //   block [0x82696608..0x8269667C)
	// 82696608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269660C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696614: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82696618: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269661C: 392B8CA0  addi r9, r11, -0x7360
	ctx.r[9].s64 = ctx.r[11].s64 + -29536;
	// 82696620: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82696624: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82696628: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8269662C: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82696630: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696634: 388AB34C  addi r4, r10, -0x4cb4
	ctx.r[4].s64 = ctx.r[10].s64 + -19636;
	// 82696638: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269663C: 396B5388  addi r11, r11, 0x5388
	ctx.r[11].s64 = ctx.r[11].s64 + 21384;
	// 82696640: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82696644: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696648: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269664C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696650: 386A1C40  addi r3, r10, 0x1c40
	ctx.r[3].s64 = ctx.r[10].s64 + 7232;
	// 82696654: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82696658: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8269665C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696660: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82696664: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82696668: 4BDD07B9  bl 0x82466e20
	ctx.lr = 0x8269666C;
	sub_82466E20(ctx, base);
	// 8269666C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696680 size=116
    let mut pc: u32 = 0x82696680;
    'dispatch: loop {
        match pc {
            0x82696680 => {
    //   block [0x82696680..0x826966F4)
	// 82696680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269668C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696690: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82696694: 390BD084  addi r8, r11, -0x2f7c
	ctx.r[8].s64 = ctx.r[11].s64 + -12156;
	// 82696698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269669C: 392A8CFC  addi r9, r10, -0x7304
	ctx.r[9].s64 = ctx.r[10].s64 + -29444;
	// 826966A0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826966A4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826966A8: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826966AC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826966B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826966B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826966B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826966BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826966C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826966C4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826966C8: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 826966CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826966D0: 386B1C70  addi r3, r11, 0x1c70
	ctx.r[3].s64 = ctx.r[11].s64 + 7280;
	// 826966D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826966D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826966DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826966E0: 4BDD0741  bl 0x82466e20
	ctx.lr = 0x826966E4;
	sub_82466E20(ctx, base);
	// 826966E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826966E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826966EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826966F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826966F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826966F8 size=100
    let mut pc: u32 = 0x826966F8;
    'dispatch: loop {
        match pc {
            0x826966F8 => {
    //   block [0x826966F8..0x8269675C)
	// 826966F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826966FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696704: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269670C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82696710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696718: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 8269671C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269672C: 386A1CA0  addi r3, r10, 0x1ca0
	ctx.r[3].s64 = ctx.r[10].s64 + 7328;
	// 82696730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696734: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696738: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269673C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696740: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82696744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696748: 4BDD06D9  bl 0x82466e20
	ctx.lr = 0x8269674C;
	sub_82466E20(ctx, base);
	// 8269674C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696760 size=112
    let mut pc: u32 = 0x82696760;
    'dispatch: loop {
        match pc {
            0x82696760 => {
    //   block [0x82696760..0x826967D0)
	// 82696760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269676C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696770: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696774: 38AA1CA0  addi r5, r10, 0x1ca0
	ctx.r[5].s64 = ctx.r[10].s64 + 7328;
	// 82696778: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269677C: 390BD0B4  addi r8, r11, -0x2f4c
	ctx.r[8].s64 = ctx.r[11].s64 + -12108;
	// 82696780: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82696784: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 82696788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269678C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696798: 386A1CD0  addi r3, r10, 0x1cd0
	ctx.r[3].s64 = ctx.r[10].s64 + 7376;
	// 8269679C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826967A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826967A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826967A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826967AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826967B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826967B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826967B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826967BC: 4BDD0665  bl 0x82466e20
	ctx.lr = 0x826967C0;
	sub_82466E20(ctx, base);
	// 826967C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826967C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826967C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826967CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826967D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826967D0 size=112
    let mut pc: u32 = 0x826967D0;
    'dispatch: loop {
        match pc {
            0x826967D0 => {
    //   block [0x826967D0..0x82696840)
	// 826967D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826967D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826967D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826967DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826967E0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826967E4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826967E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826967EC: 390BD0D0  addi r8, r11, -0x2f30
	ctx.r[8].s64 = ctx.r[11].s64 + -12080;
	// 826967F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826967F4: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 826967F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826967FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696808: 386A1D00  addi r3, r10, 0x1d00
	ctx.r[3].s64 = ctx.r[10].s64 + 7424;
	// 8269680C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82696810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269681C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269682C: 4BDD05F5  bl 0x82466e20
	ctx.lr = 0x82696830;
	sub_82466E20(ctx, base);
	// 82696830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269683C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696840 size=112
    let mut pc: u32 = 0x82696840;
    'dispatch: loop {
        match pc {
            0x82696840 => {
    //   block [0x82696840..0x826968B0)
	// 82696840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269684C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696850: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696854: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82696858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269685C: 390BD118  addi r8, r11, -0x2ee8
	ctx.r[8].s64 = ctx.r[11].s64 + -12008;
	// 82696860: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82696864: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 82696868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269686C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696878: 386A1D30  addi r3, r10, 0x1d30
	ctx.r[3].s64 = ctx.r[10].s64 + 7472;
	// 8269687C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82696880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269688C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269689C: 4BDD0585  bl 0x82466e20
	ctx.lr = 0x826968A0;
	sub_82466E20(ctx, base);
	// 826968A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826968A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826968A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826968AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826968B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826968B0 size=116
    let mut pc: u32 = 0x826968B0;
    'dispatch: loop {
        match pc {
            0x826968B0 => {
    //   block [0x826968B0..0x82696924)
	// 826968B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826968B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826968B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826968BC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826968C0: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826968C4: 390AD1C0  addi r8, r10, -0x2e40
	ctx.r[8].s64 = ctx.r[10].s64 + -11840;
	// 826968C8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826968CC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826968D0: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 826968D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826968D8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826968DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826968E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826968E4: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 826968E8: 396B8D10  addi r11, r11, -0x72f0
	ctx.r[11].s64 = ctx.r[11].s64 + -29424;
	// 826968EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826968F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826968F4: 386A1D60  addi r3, r10, 0x1d60
	ctx.r[3].s64 = ctx.r[10].s64 + 7520;
	// 826968F8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826968FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696900: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82696904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269690C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696910: 4BDD0511  bl 0x82466e20
	ctx.lr = 0x82696914;
	sub_82466E20(ctx, base);
	// 82696914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269691C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696928 size=112
    let mut pc: u32 = 0x82696928;
    'dispatch: loop {
        match pc {
            0x82696928 => {
    //   block [0x82696928..0x82696998)
	// 82696928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269692C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696934: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696938: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269693C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82696940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696944: 390BD2C8  addi r8, r11, -0x2d38
	ctx.r[8].s64 = ctx.r[11].s64 + -11576;
	// 82696948: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269694C: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 82696950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696954: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269695C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696960: 386A1D90  addi r3, r10, 0x1d90
	ctx.r[3].s64 = ctx.r[10].s64 + 7568;
	// 82696964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82696968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269696C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269697C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696984: 4BDD049D  bl 0x82466e20
	ctx.lr = 0x82696988;
	sub_82466E20(ctx, base);
	// 82696988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269698C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696998 size=100
    let mut pc: u32 = 0x82696998;
    'dispatch: loop {
        match pc {
            0x82696998 => {
    //   block [0x82696998..0x826969FC)
	// 82696998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269699C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826969A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826969A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826969A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826969AC: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826969B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826969B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826969B8: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 826969BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826969C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826969C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826969C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826969CC: 386A1DC0  addi r3, r10, 0x1dc0
	ctx.r[3].s64 = ctx.r[10].s64 + 7616;
	// 826969D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826969D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826969D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826969DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826969E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826969E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826969E8: 4BDD0439  bl 0x82466e20
	ctx.lr = 0x826969EC;
	sub_82466E20(ctx, base);
	// 826969EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826969F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826969F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826969F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696A00 size=108
    let mut pc: u32 = 0x82696A00;
    'dispatch: loop {
        match pc {
            0x82696A00 => {
    //   block [0x82696A00..0x82696A6C)
	// 82696A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696A0C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696A14: 38EBD2F8  addi r7, r11, -0x2d08
	ctx.r[7].s64 = ctx.r[11].s64 + -11528;
	// 82696A18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82696A1C: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 82696A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696A24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696A28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82696A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696A30: 386A1DF0  addi r3, r10, 0x1df0
	ctx.r[3].s64 = ctx.r[10].s64 + 7664;
	// 82696A34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82696A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696A3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696A54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696A58: 4BDD03C9  bl 0x82466e20
	ctx.lr = 0x82696A5C;
	sub_82466E20(ctx, base);
	// 82696A5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696A70 size=112
    let mut pc: u32 = 0x82696A70;
    'dispatch: loop {
        match pc {
            0x82696A70 => {
    //   block [0x82696A70..0x82696AE0)
	// 82696A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696A7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696A80: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696A84: 38AA1DC0  addi r5, r10, 0x1dc0
	ctx.r[5].s64 = ctx.r[10].s64 + 7616;
	// 82696A88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696A8C: 390BD328  addi r8, r11, -0x2cd8
	ctx.r[8].s64 = ctx.r[11].s64 + -11480;
	// 82696A90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82696A94: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 82696A98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696A9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696AA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696AA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696AA8: 386A1E20  addi r3, r10, 0x1e20
	ctx.r[3].s64 = ctx.r[10].s64 + 7712;
	// 82696AAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82696AB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696AC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696ACC: 4BDD0355  bl 0x82466e20
	ctx.lr = 0x82696AD0;
	sub_82466E20(ctx, base);
	// 82696AD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696AE0 size=108
    let mut pc: u32 = 0x82696AE0;
    'dispatch: loop {
        match pc {
            0x82696AE0 => {
    //   block [0x82696AE0..0x82696B4C)
	// 82696AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696AEC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696AF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82696AF4: 38EBD358  addi r7, r11, -0x2ca8
	ctx.r[7].s64 = ctx.r[11].s64 + -11432;
	// 82696AF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82696AFC: 388AA93C  addi r4, r10, -0x56c4
	ctx.r[4].s64 = ctx.r[10].s64 + -22212;
	// 82696B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696B04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696B08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82696B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696B10: 386A1E50  addi r3, r10, 0x1e50
	ctx.r[3].s64 = ctx.r[10].s64 + 7760;
	// 82696B14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82696B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696B1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696B34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696B38: 4BDD02E9  bl 0x82466e20
	ctx.lr = 0x82696B3C;
	sub_82466E20(ctx, base);
	// 82696B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696B50 size=116
    let mut pc: u32 = 0x82696B50;
    'dispatch: loop {
        match pc {
            0x82696B50 => {
    //   block [0x82696B50..0x82696BC4)
	// 82696B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696B5C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82696B60: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82696B64: 390AD388  addi r8, r10, -0x2c78
	ctx.r[8].s64 = ctx.r[10].s64 + -11384;
	// 82696B68: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696B6C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82696B70: 38AA1DC0  addi r5, r10, 0x1dc0
	ctx.r[5].s64 = ctx.r[10].s64 + 7616;
	// 82696B74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696B78: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82696B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696B80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696B84: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 82696B88: 396B8D44  addi r11, r11, -0x72bc
	ctx.r[11].s64 = ctx.r[11].s64 + -29372;
	// 82696B8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696B90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696B94: 386A1E80  addi r3, r10, 0x1e80
	ctx.r[3].s64 = ctx.r[10].s64 + 7808;
	// 82696B98: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82696B9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696BA0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82696BA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696BB0: 4BDD0271  bl 0x82466e20
	ctx.lr = 0x82696BB4;
	sub_82466E20(ctx, base);
	// 82696BB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696BC8 size=108
    let mut pc: u32 = 0x82696BC8;
    'dispatch: loop {
        match pc {
            0x82696BC8 => {
    //   block [0x82696BC8..0x82696C34)
	// 82696BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696BD4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696BD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82696BDC: 38EBD3D0  addi r7, r11, -0x2c30
	ctx.r[7].s64 = ctx.r[11].s64 + -11312;
	// 82696BE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82696BE4: 388AA960  addi r4, r10, -0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22176;
	// 82696BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696BEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696BF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82696BF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696BF8: 386A1EB0  addi r3, r10, 0x1eb0
	ctx.r[3].s64 = ctx.r[10].s64 + 7856;
	// 82696BFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82696C00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696C08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696C10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696C18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696C1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696C20: 4BDD0201  bl 0x82466e20
	ctx.lr = 0x82696C24;
	sub_82466E20(ctx, base);
	// 82696C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696C38 size=116
    let mut pc: u32 = 0x82696C38;
    'dispatch: loop {
        match pc {
            0x82696C38 => {
    //   block [0x82696C38..0x82696CAC)
	// 82696C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696C44: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82696C48: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82696C4C: 390AD400  addi r8, r10, -0x2c00
	ctx.r[8].s64 = ctx.r[10].s64 + -11264;
	// 82696C50: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696C54: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82696C58: 38AA1DC0  addi r5, r10, 0x1dc0
	ctx.r[5].s64 = ctx.r[10].s64 + 7616;
	// 82696C5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696C60: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82696C64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696C6C: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 82696C70: 396B8D54  addi r11, r11, -0x72ac
	ctx.r[11].s64 = ctx.r[11].s64 + -29356;
	// 82696C74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696C78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696C7C: 386A1EE0  addi r3, r10, 0x1ee0
	ctx.r[3].s64 = ctx.r[10].s64 + 7904;
	// 82696C80: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82696C84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696C88: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82696C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696C98: 4BDD0189  bl 0x82466e20
	ctx.lr = 0x82696C9C;
	sub_82466E20(ctx, base);
	// 82696C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696CB0 size=108
    let mut pc: u32 = 0x82696CB0;
    'dispatch: loop {
        match pc {
            0x82696CB0 => {
    //   block [0x82696CB0..0x82696D1C)
	// 82696CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696CBC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696CC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82696CC4: 38EBD448  addi r7, r11, -0x2bb8
	ctx.r[7].s64 = ctx.r[11].s64 + -11192;
	// 82696CC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82696CCC: 388AA984  addi r4, r10, -0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + -22140;
	// 82696CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696CD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82696CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696CE0: 386A1F10  addi r3, r10, 0x1f10
	ctx.r[3].s64 = ctx.r[10].s64 + 7952;
	// 82696CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82696CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696D08: 4BDD0119  bl 0x82466e20
	ctx.lr = 0x82696D0C;
	sub_82466E20(ctx, base);
	// 82696D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696D20 size=116
    let mut pc: u32 = 0x82696D20;
    'dispatch: loop {
        match pc {
            0x82696D20 => {
    //   block [0x82696D20..0x82696D94)
	// 82696D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696D2C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82696D30: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82696D34: 390AD478  addi r8, r10, -0x2b88
	ctx.r[8].s64 = ctx.r[10].s64 + -11144;
	// 82696D38: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696D3C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82696D40: 38AA1DC0  addi r5, r10, 0x1dc0
	ctx.r[5].s64 = ctx.r[10].s64 + 7616;
	// 82696D44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696D48: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82696D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696D50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696D54: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 82696D58: 396B8D64  addi r11, r11, -0x729c
	ctx.r[11].s64 = ctx.r[11].s64 + -29340;
	// 82696D5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696D60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696D64: 386A1F40  addi r3, r10, 0x1f40
	ctx.r[3].s64 = ctx.r[10].s64 + 8000;
	// 82696D68: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82696D6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696D70: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82696D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696D78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696D80: 4BDD00A1  bl 0x82466e20
	ctx.lr = 0x82696D84;
	sub_82466E20(ctx, base);
	// 82696D84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696D88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696D8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696D90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696D98 size=108
    let mut pc: u32 = 0x82696D98;
    'dispatch: loop {
        match pc {
            0x82696D98 => {
    //   block [0x82696D98..0x82696E04)
	// 82696D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696DA4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696DA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696DAC: 38EBD4C0  addi r7, r11, -0x2b40
	ctx.r[7].s64 = ctx.r[11].s64 + -11072;
	// 82696DB0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82696DB4: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 82696DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696DBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696DC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82696DC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696DC8: 386A1F70  addi r3, r10, 0x1f70
	ctx.r[3].s64 = ctx.r[10].s64 + 8048;
	// 82696DCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82696DD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696DD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696DE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696DE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696DEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696DF0: 4BDD0031  bl 0x82466e20
	ctx.lr = 0x82696DF4;
	sub_82466E20(ctx, base);
	// 82696DF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696DF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696DFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82696E08 size=24
    let mut pc: u32 = 0x82696E08;
    'dispatch: loop {
        match pc {
            0x82696E08 => {
    //   block [0x82696E08..0x82696E20)
	// 82696E08: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696E0C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82696E10: 394A5418  addi r10, r10, 0x5418
	ctx.r[10].s64 = ctx.r[10].s64 + 21528;
	// 82696E14: 816BD520  lwz r11, -0x2ae0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10976 as u32) ) } as u64;
	// 82696E18: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 82696E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696E20 size=112
    let mut pc: u32 = 0x82696E20;
    'dispatch: loop {
        match pc {
            0x82696E20 => {
    //   block [0x82696E20..0x82696E90)
	// 82696E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696E2C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82696E30: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696E34: 392A8E30  addi r9, r10, -0x71d0
	ctx.r[9].s64 = ctx.r[10].s64 + -29136;
	// 82696E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696E3C: 390B5418  addi r8, r11, 0x5418
	ctx.r[8].s64 = ctx.r[11].s64 + 21528;
	// 82696E40: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 82696E44: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 82696E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696E4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696E50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696E54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696E58: 386A1FA0  addi r3, r10, 0x1fa0
	ctx.r[3].s64 = ctx.r[10].s64 + 8096;
	// 82696E5C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82696E60: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82696E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696E74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696E7C: 4BDCFFA5  bl 0x82466e20
	ctx.lr = 0x82696E80;
	sub_82466E20(ctx, base);
	// 82696E80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696E90 size=108
    let mut pc: u32 = 0x82696E90;
    'dispatch: loop {
        match pc {
            0x82696E90 => {
    //   block [0x82696E90..0x82696EFC)
	// 82696E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696E9C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696EA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696EA4: 38EBD528  addi r7, r11, -0x2ad8
	ctx.r[7].s64 = ctx.r[11].s64 + -10968;
	// 82696EA8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82696EAC: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 82696EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696EB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696EB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82696EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696EC0: 386A1FD0  addi r3, r10, 0x1fd0
	ctx.r[3].s64 = ctx.r[10].s64 + 8144;
	// 82696EC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82696EC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696EE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696EE8: 4BDCFF39  bl 0x82466e20
	ctx.lr = 0x82696EEC;
	sub_82466E20(ctx, base);
	// 82696EEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696F00 size=108
    let mut pc: u32 = 0x82696F00;
    'dispatch: loop {
        match pc {
            0x82696F00 => {
    //   block [0x82696F00..0x82696F6C)
	// 82696F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696F0C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696F10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82696F14: 38EBD5A0  addi r7, r11, -0x2a60
	ctx.r[7].s64 = ctx.r[11].s64 + -10848;
	// 82696F18: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82696F1C: 388AA9B8  addi r4, r10, -0x5648
	ctx.r[4].s64 = ctx.r[10].s64 + -22088;
	// 82696F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696F24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696F28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82696F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696F30: 386A2000  addi r3, r10, 0x2000
	ctx.r[3].s64 = ctx.r[10].s64 + 8192;
	// 82696F34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82696F38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696F3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696F40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696F48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696F50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696F54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696F58: 4BDCFEC9  bl 0x82466e20
	ctx.lr = 0x82696F5C;
	sub_82466E20(ctx, base);
	// 82696F5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696F70 size=108
    let mut pc: u32 = 0x82696F70;
    'dispatch: loop {
        match pc {
            0x82696F70 => {
    //   block [0x82696F70..0x82696FDC)
	// 82696F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696F7C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696F84: 38EBD600  addi r7, r11, -0x2a00
	ctx.r[7].s64 = ctx.r[11].s64 + -10752;
	// 82696F88: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82696F8C: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 82696F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696F94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696F98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82696F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696FA0: 386A2030  addi r3, r10, 0x2030
	ctx.r[3].s64 = ctx.r[10].s64 + 8240;
	// 82696FA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82696FA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696FC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696FC8: 4BDCFE59  bl 0x82466e20
	ctx.lr = 0x82696FCC;
	sub_82466E20(ctx, base);
	// 82696FCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82696FE0 size=24
    let mut pc: u32 = 0x82696FE0;
    'dispatch: loop {
        match pc {
            0x82696FE0 => {
    //   block [0x82696FE0..0x82696FF8)
	// 82696FE0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696FE4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82696FE8: 394A5520  addi r10, r10, 0x5520
	ctx.r[10].s64 = ctx.r[10].s64 + 21792;
	// 82696FEC: 816BD0CC  lwz r11, -0x2f34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12084 as u32) ) } as u64;
	// 82696FF0: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 82696FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696FF8 size=116
    let mut pc: u32 = 0x82696FF8;
    'dispatch: loop {
        match pc {
            0x82696FF8 => {
    //   block [0x82696FF8..0x8269706C)
	// 82696FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697004: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82697008: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269700C: 392B8D94  addi r9, r11, -0x726c
	ctx.r[9].s64 = ctx.r[11].s64 + -29292;
	// 82697010: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 82697014: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82697018: 38E900C4  addi r7, r9, 0xc4
	ctx.r[7].s64 = ctx.r[9].s64 + 196;
	// 8269701C: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 82697020: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697024: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 82697028: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269702C: 396B5520  addi r11, r11, 0x5520
	ctx.r[11].s64 = ctx.r[11].s64 + 21792;
	// 82697030: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82697034: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697038: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269703C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697040: 386A2060  addi r3, r10, 0x2060
	ctx.r[3].s64 = ctx.r[10].s64 + 8288;
	// 82697044: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82697048: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8269704C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697050: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82697054: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82697058: 4BDCFDC9  bl 0x82466e20
	ctx.lr = 0x8269705C;
	sub_82466E20(ctx, base);
	// 8269705C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82697070 size=36
    let mut pc: u32 = 0x82697070;
    'dispatch: loop {
        match pc {
            0x82697070 => {
    //   block [0x82697070..0x82697094)
	// 82697070: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697074: 814BD6A8  lwz r10, -0x2958(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10584 as u32) ) } as u64;
	// 82697078: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269707C: 396B57F0  addi r11, r11, 0x57f0
	ctx.r[11].s64 = ctx.r[11].s64 + 22512;
	// 82697080: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82697084: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82697088: 814AD6AC  lwz r10, -0x2954(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-10580 as u32) ) } as u64;
	// 8269708C: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82697090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697098 size=116
    let mut pc: u32 = 0x82697098;
    'dispatch: loop {
        match pc {
            0x82697098 => {
    //   block [0x82697098..0x8269710C)
	// 82697098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269709C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826970A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826970A4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826970A8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826970AC: 390B57F0  addi r8, r11, 0x57f0
	ctx.r[8].s64 = ctx.r[11].s64 + 22512;
	// 826970B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826970B4: 392A8F18  addi r9, r10, -0x70e8
	ctx.r[9].s64 = ctx.r[10].s64 + -28904;
	// 826970B8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826970BC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826970C0: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 826970C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826970C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826970CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826970D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826970D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826970D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826970DC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826970E0: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 826970E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826970E8: 386B2090  addi r3, r11, 0x2090
	ctx.r[3].s64 = ctx.r[11].s64 + 8336;
	// 826970EC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826970F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826970F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826970F8: 4BDCFD29  bl 0x82466e20
	ctx.lr = 0x826970FC;
	sub_82466E20(ctx, base);
	// 826970FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82697110 size=24
    let mut pc: u32 = 0x82697110;
    'dispatch: loop {
        match pc {
            0x82697110 => {
    //   block [0x82697110..0x82697128)
	// 82697110: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697114: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82697118: 394A5820  addi r10, r10, 0x5820
	ctx.r[10].s64 = ctx.r[10].s64 + 22560;
	// 8269711C: 816BD6B4  lwz r11, -0x294c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10572 as u32) ) } as u64;
	// 82697120: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82697124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697128 size=116
    let mut pc: u32 = 0x82697128;
    'dispatch: loop {
        match pc {
            0x82697128 => {
    //   block [0x82697128..0x8269719C)
	// 82697128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269712C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697134: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697138: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269713C: 390B5820  addi r8, r11, 0x5820
	ctx.r[8].s64 = ctx.r[11].s64 + 22560;
	// 82697140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697144: 392A8F70  addi r9, r10, -0x7090
	ctx.r[9].s64 = ctx.r[10].s64 + -28816;
	// 82697148: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269714C: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 82697150: 38AA2090  addi r5, r10, 0x2090
	ctx.r[5].s64 = ctx.r[10].s64 + 8336;
	// 82697154: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269715C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82697160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269716C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82697170: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 82697174: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82697178: 386B20C0  addi r3, r11, 0x20c0
	ctx.r[3].s64 = ctx.r[11].s64 + 8384;
	// 8269717C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82697180: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697188: 4BDCFC99  bl 0x82466e20
	ctx.lr = 0x8269718C;
	sub_82466E20(ctx, base);
	// 8269718C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826971A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826971A0 size=116
    let mut pc: u32 = 0x826971A0;
    'dispatch: loop {
        match pc {
            0x826971A0 => {
    //   block [0x826971A0..0x82697214)
	// 826971A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826971A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826971A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826971AC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826971B0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826971B4: 390BD6C0  addi r8, r11, -0x2940
	ctx.r[8].s64 = ctx.r[11].s64 + -10560;
	// 826971B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826971BC: 392A8FB8  addi r9, r10, -0x7048
	ctx.r[9].s64 = ctx.r[10].s64 + -28744;
	// 826971C0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826971C4: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826971C8: 38AA2090  addi r5, r10, 0x2090
	ctx.r[5].s64 = ctx.r[10].s64 + 8336;
	// 826971CC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826971D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826971D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826971D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826971DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826971E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826971E4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826971E8: 388AAAE8  addi r4, r10, -0x5518
	ctx.r[4].s64 = ctx.r[10].s64 + -21784;
	// 826971EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826971F0: 386B20F0  addi r3, r11, 0x20f0
	ctx.r[3].s64 = ctx.r[11].s64 + 8432;
	// 826971F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826971F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826971FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697200: 4BDCFC21  bl 0x82466e20
	ctx.lr = 0x82697204;
	sub_82466E20(ctx, base);
	// 82697204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269720C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697218 size=112
    let mut pc: u32 = 0x82697218;
    'dispatch: loop {
        match pc {
            0x82697218 => {
    //   block [0x82697218..0x82697288)
	// 82697218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269721C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697224: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82697228: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8269722C: 38EAD7C8  addi r7, r10, -0x2838
	ctx.r[7].s64 = ctx.r[10].s64 + -10296;
	// 82697230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82697234: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82697238: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 8269723C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697240: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82697244: 396B8FCC  addi r11, r11, -0x7034
	ctx.r[11].s64 = ctx.r[11].s64 + -28724;
	// 82697248: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269724C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697250: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697254: 386A2120  addi r3, r10, 0x2120
	ctx.r[3].s64 = ctx.r[10].s64 + 8480;
	// 82697258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269725C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82697260: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697264: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82697268: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269726C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697270: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697274: 4BDCFBAD  bl 0x82466e20
	ctx.lr = 0x82697278;
	sub_82466E20(ctx, base);
	// 82697278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269727C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697288 size=112
    let mut pc: u32 = 0x82697288;
    'dispatch: loop {
        match pc {
            0x82697288 => {
    //   block [0x82697288..0x826972F8)
	// 82697288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269728C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697294: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697298: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269729C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826972A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826972A4: 390BD858  addi r8, r11, -0x27a8
	ctx.r[8].s64 = ctx.r[11].s64 + -10152;
	// 826972A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826972AC: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 826972B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826972B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826972B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826972BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826972C0: 386A2150  addi r3, r10, 0x2150
	ctx.r[3].s64 = ctx.r[10].s64 + 8528;
	// 826972C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826972C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826972CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826972D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826972D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826972D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826972DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826972E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826972E4: 4BDCFB3D  bl 0x82466e20
	ctx.lr = 0x826972E8;
	sub_82466E20(ctx, base);
	// 826972E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826972EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826972F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826972F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826972F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826972F8 size=24
    let mut pc: u32 = 0x826972F8;
    'dispatch: loop {
        match pc {
            0x826972F8 => {
    //   block [0x826972F8..0x82697310)
	// 826972F8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826972FC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82697300: 394A5958  addi r10, r10, 0x5958
	ctx.r[10].s64 = ctx.r[10].s64 + 22872;
	// 82697304: 816BD6BC  lwz r11, -0x2944(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10564 as u32) ) } as u64;
	// 82697308: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8269730C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697310 size=112
    let mut pc: u32 = 0x82697310;
    'dispatch: loop {
        match pc {
            0x82697310 => {
    //   block [0x82697310..0x82697380)
	// 82697310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269731C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82697320: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697324: 392A9018  addi r9, r10, -0x6fe8
	ctx.r[9].s64 = ctx.r[10].s64 + -28648;
	// 82697328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269732C: 390B5958  addi r8, r11, 0x5958
	ctx.r[8].s64 = ctx.r[11].s64 + 22872;
	// 82697330: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82697334: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 82697338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269733C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697340: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697348: 386A2180  addi r3, r10, 0x2180
	ctx.r[3].s64 = ctx.r[10].s64 + 8576;
	// 8269734C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82697350: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82697354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269735C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269736C: 4BDCFAB5  bl 0x82466e20
	ctx.lr = 0x82697370;
	sub_82466E20(ctx, base);
	// 82697370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269737C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697380 size=108
    let mut pc: u32 = 0x82697380;
    'dispatch: loop {
        match pc {
            0x82697380 => {
    //   block [0x82697380..0x826973EC)
	// 82697380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269738C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82697394: 38EBD878  addi r7, r11, -0x2788
	ctx.r[7].s64 = ctx.r[11].s64 + -10120;
	// 82697398: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269739C: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 826973A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826973A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826973A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826973AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826973B0: 386A21B0  addi r3, r10, 0x21b0
	ctx.r[3].s64 = ctx.r[10].s64 + 8624;
	// 826973B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826973B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826973BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826973C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826973C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826973C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826973CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826973D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826973D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826973D8: 4BDCFA49  bl 0x82466e20
	ctx.lr = 0x826973DC;
	sub_82466E20(ctx, base);
	// 826973DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826973E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826973E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826973E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826973F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826973F0 size=108
    let mut pc: u32 = 0x826973F0;
    'dispatch: loop {
        match pc {
            0x826973F0 => {
    //   block [0x826973F0..0x8269745C)
	// 826973F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826973F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826973F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826973FC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82697404: 38EBD8D8  addi r7, r11, -0x2728
	ctx.r[7].s64 = ctx.r[11].s64 + -10024;
	// 82697408: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269740C: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 82697410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697414: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697418: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269741C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697420: 386A21E0  addi r3, r10, 0x21e0
	ctx.r[3].s64 = ctx.r[10].s64 + 8672;
	// 82697424: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82697428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269742C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269743C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697448: 4BDCF9D9  bl 0x82466e20
	ctx.lr = 0x8269744C;
	sub_82466E20(ctx, base);
	// 8269744C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697460 size=116
    let mut pc: u32 = 0x82697460;
    'dispatch: loop {
        match pc {
            0x82697460 => {
    //   block [0x82697460..0x826974D4)
	// 82697460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269746C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697470: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82697474: 390BD908  addi r8, r11, -0x26f8
	ctx.r[8].s64 = ctx.r[11].s64 + -9976;
	// 82697478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269747C: 392A9044  addi r9, r10, -0x6fbc
	ctx.r[9].s64 = ctx.r[10].s64 + -28604;
	// 82697480: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697484: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82697488: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269748C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697494: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82697498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269749C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826974A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826974A4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826974A8: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 826974AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826974B0: 386B2210  addi r3, r11, 0x2210
	ctx.r[3].s64 = ctx.r[11].s64 + 8720;
	// 826974B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826974B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826974BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826974C0: 4BDCF961  bl 0x82466e20
	ctx.lr = 0x826974C4;
	sub_82466E20(ctx, base);
	// 826974C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826974C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826974CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826974D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826974D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826974D8 size=108
    let mut pc: u32 = 0x826974D8;
    'dispatch: loop {
        match pc {
            0x826974D8 => {
    //   block [0x826974D8..0x82697544)
	// 826974D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826974DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826974E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826974E4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826974E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826974EC: 38EBD920  addi r7, r11, -0x26e0
	ctx.r[7].s64 = ctx.r[11].s64 + -9952;
	// 826974F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826974F4: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 826974F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826974FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697500: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82697504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697508: 386A2240  addi r3, r10, 0x2240
	ctx.r[3].s64 = ctx.r[10].s64 + 8768;
	// 8269750C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82697510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269751C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269752C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697530: 4BDCF8F1  bl 0x82466e20
	ctx.lr = 0x82697534;
	sub_82466E20(ctx, base);
	// 82697534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269753C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697548 size=112
    let mut pc: u32 = 0x82697548;
    'dispatch: loop {
        match pc {
            0x82697548 => {
    //   block [0x82697548..0x826975B8)
	// 82697548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269754C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697554: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697558: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269755C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82697560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82697564: 390BD938  addi r8, r11, -0x26c8
	ctx.r[8].s64 = ctx.r[11].s64 + -9928;
	// 82697568: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8269756C: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 82697570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697574: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697578: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269757C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697580: 386A2270  addi r3, r10, 0x2270
	ctx.r[3].s64 = ctx.r[10].s64 + 8816;
	// 82697584: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82697588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269758C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697594: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269759C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826975A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826975A4: 4BDCF87D  bl 0x82466e20
	ctx.lr = 0x826975A8;
	sub_82466E20(ctx, base);
	// 826975A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826975AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826975B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826975B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826975B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826975B8 size=108
    let mut pc: u32 = 0x826975B8;
    'dispatch: loop {
        match pc {
            0x826975B8 => {
    //   block [0x826975B8..0x82697624)
	// 826975B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826975BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826975C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826975C4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826975C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826975CC: 38EBDA10  addi r7, r11, -0x25f0
	ctx.r[7].s64 = ctx.r[11].s64 + -9712;
	// 826975D0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826975D4: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 826975D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826975DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826975E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826975E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826975E8: 386A22A0  addi r3, r10, 0x22a0
	ctx.r[3].s64 = ctx.r[10].s64 + 8864;
	// 826975EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826975F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826975F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826975F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826975FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269760C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697610: 4BDCF811  bl 0x82466e20
	ctx.lr = 0x82697614;
	sub_82466E20(ctx, base);
	// 82697614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269761C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697628 size=108
    let mut pc: u32 = 0x82697628;
    'dispatch: loop {
        match pc {
            0x82697628 => {
    //   block [0x82697628..0x82697694)
	// 82697628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269762C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697634: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269763C: 38EBDA88  addi r7, r11, -0x2578
	ctx.r[7].s64 = ctx.r[11].s64 + -9592;
	// 82697640: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82697644: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 82697648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269764C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697650: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82697654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697658: 386A22D0  addi r3, r10, 0x22d0
	ctx.r[3].s64 = ctx.r[10].s64 + 8912;
	// 8269765C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82697660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269766C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269767C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697680: 4BDCF7A1  bl 0x82466e20
	ctx.lr = 0x82697684;
	sub_82466E20(ctx, base);
	// 82697684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269768C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697698 size=116
    let mut pc: u32 = 0x82697698;
    'dispatch: loop {
        match pc {
            0x82697698 => {
    //   block [0x82697698..0x8269770C)
	// 82697698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269769C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826976A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826976A4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826976A8: 38E00017  li r7, 0x17
	ctx.r[7].s64 = 23;
	// 826976AC: 390ADAD0  addi r8, r10, -0x2530
	ctx.r[8].s64 = ctx.r[10].s64 + -9520;
	// 826976B0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826976B4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826976B8: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826976BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826976C0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826976C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826976C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826976CC: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 826976D0: 396B9058  addi r11, r11, -0x6fa8
	ctx.r[11].s64 = ctx.r[11].s64 + -28584;
	// 826976D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826976D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826976DC: 386A2300  addi r3, r10, 0x2300
	ctx.r[3].s64 = ctx.r[10].s64 + 8960;
	// 826976E0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826976E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826976E8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826976EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826976F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826976F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826976F8: 4BDCF729  bl 0x82466e20
	ctx.lr = 0x826976FC;
	sub_82466E20(ctx, base);
	// 826976FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697710 size=112
    let mut pc: u32 = 0x82697710;
    'dispatch: loop {
        match pc {
            0x82697710 => {
    //   block [0x82697710..0x82697780)
	// 82697710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269771C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697720: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697724: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 82697728: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269772C: 390BDCF8  addi r8, r11, -0x2308
	ctx.r[8].s64 = ctx.r[11].s64 + -8968;
	// 82697730: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82697734: 388AB374  addi r4, r10, -0x4c8c
	ctx.r[4].s64 = ctx.r[10].s64 + -19596;
	// 82697738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269773C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697740: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697748: 386A2330  addi r3, r10, 0x2330
	ctx.r[3].s64 = ctx.r[10].s64 + 9008;
	// 8269774C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82697750: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269775C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269776C: 4BDCF6B5  bl 0x82466e20
	ctx.lr = 0x82697770;
	sub_82466E20(ctx, base);
	// 82697770: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269777C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82697780 size=76
    let mut pc: u32 = 0x82697780;
    'dispatch: loop {
        match pc {
            0x82697780 => {
    //   block [0x82697780..0x826977CC)
	// 82697780: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697784: 814BD874  lwz r10, -0x278c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10124 as u32) ) } as u64;
	// 82697788: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269778C: 396B5988  addi r11, r11, 0x5988
	ctx.r[11].s64 = ctx.r[11].s64 + 22920;
	// 82697790: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 82697794: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 82697798: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 8269779C: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826977A0: 914B0158  stw r10, 0x158(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 826977A4: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 826977A8: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826977AC: 814ADD10  lwz r10, -0x22f0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8944 as u32) ) } as u64;
	// 826977B0: 914B0188  stw r10, 0x188(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 826977B4: 914B01A0  stw r10, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[10].u32 ) };
	// 826977B8: 914B01B8  stw r10, 0x1b8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 826977BC: 914B01D0  stw r10, 0x1d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 826977C0: 914B01E8  stw r10, 0x1e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), ctx.r[10].u32 ) };
	// 826977C4: 914B0200  stw r10, 0x200(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(512 as u32), ctx.r[10].u32 ) };
	// 826977C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826977D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826977D0 size=108
    let mut pc: u32 = 0x826977D0;
    'dispatch: loop {
        match pc {
            0x826977D0 => {
    //   block [0x826977D0..0x8269783C)
	// 826977D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826977D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826977D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826977DC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826977E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826977E4: 38EB5988  addi r7, r11, 0x5988
	ctx.r[7].s64 = ctx.r[11].s64 + 22920;
	// 826977E8: 3900001A  li r8, 0x1a
	ctx.r[8].s64 = 26;
	// 826977EC: 388AABE8  addi r4, r10, -0x5418
	ctx.r[4].s64 = ctx.r[10].s64 + -21528;
	// 826977F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826977F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826977F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826977FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697800: 386A2360  addi r3, r10, 0x2360
	ctx.r[3].s64 = ctx.r[10].s64 + 9056;
	// 82697804: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82697808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269780C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269781C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697828: 4BDCF5F9  bl 0x82466e20
	ctx.lr = 0x8269782C;
	sub_82466E20(ctx, base);
	// 8269782C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82697840 size=76
    let mut pc: u32 = 0x82697840;
    'dispatch: loop {
        match pc {
            0x82697840 => {
    //   block [0x82697840..0x8269788C)
	// 82697840: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697844: 814BD874  lwz r10, -0x278c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10124 as u32) ) } as u64;
	// 82697848: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269784C: 396B5BF8  addi r11, r11, 0x5bf8
	ctx.r[11].s64 = ctx.r[11].s64 + 23544;
	// 82697850: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 82697854: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 82697858: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 8269785C: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 82697860: 914B0158  stw r10, 0x158(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 82697864: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 82697868: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269786C: 814ADD10  lwz r10, -0x22f0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8944 as u32) ) } as u64;
	// 82697870: 914B0188  stw r10, 0x188(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 82697874: 914B01A0  stw r10, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[10].u32 ) };
	// 82697878: 914B01B8  stw r10, 0x1b8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 8269787C: 914B01D0  stw r10, 0x1d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 82697880: 914B01E8  stw r10, 0x1e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), ctx.r[10].u32 ) };
	// 82697884: 914B0200  stw r10, 0x200(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(512 as u32), ctx.r[10].u32 ) };
	// 82697888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697890 size=116
    let mut pc: u32 = 0x82697890;
    'dispatch: loop {
        match pc {
            0x82697890 => {
    //   block [0x82697890..0x82697904)
	// 82697890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269789C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826978A0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826978A4: 390B5BF8  addi r8, r11, 0x5bf8
	ctx.r[8].s64 = ctx.r[11].s64 + 23544;
	// 826978A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826978AC: 392A90F4  addi r9, r10, -0x6f0c
	ctx.r[9].s64 = ctx.r[10].s64 + -28428;
	// 826978B0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826978B4: 38E00028  li r7, 0x28
	ctx.r[7].s64 = 40;
	// 826978B8: 38AA1610  addi r5, r10, 0x1610
	ctx.r[5].s64 = ctx.r[10].s64 + 5648;
	// 826978BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826978C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826978C4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826978C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826978CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826978D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826978D4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826978D8: 388AACDC  addi r4, r10, -0x5324
	ctx.r[4].s64 = ctx.r[10].s64 + -21284;
	// 826978DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826978E0: 386B2390  addi r3, r11, 0x2390
	ctx.r[3].s64 = ctx.r[11].s64 + 9104;
	// 826978E4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826978E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826978EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826978F0: 4BDCF531  bl 0x82466e20
	ctx.lr = 0x826978F4;
	sub_82466E20(ctx, base);
	// 826978F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826978F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826978FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697908 size=112
    let mut pc: u32 = 0x82697908;
    'dispatch: loop {
        match pc {
            0x82697908 => {
    //   block [0x82697908..0x82697978)
	// 82697908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269790C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697914: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697918: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269791C: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 82697920: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697924: 390BDD18  addi r8, r11, -0x22e8
	ctx.r[8].s64 = ctx.r[11].s64 + -8936;
	// 82697928: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269792C: 388AACF4  addi r4, r10, -0x530c
	ctx.r[4].s64 = ctx.r[10].s64 + -21260;
	// 82697930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697934: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269793C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697940: 386A23C0  addi r3, r10, 0x23c0
	ctx.r[3].s64 = ctx.r[10].s64 + 9152;
	// 82697944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82697948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269794C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269795C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697964: 4BDCF4BD  bl 0x82466e20
	ctx.lr = 0x82697968;
	sub_82466E20(ctx, base);
	// 82697968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269796C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697978 size=108
    let mut pc: u32 = 0x82697978;
    'dispatch: loop {
        match pc {
            0x82697978 => {
    //   block [0x82697978..0x826979E4)
	// 82697978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269797C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697984: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697988: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269798C: 38EBDD60  addi r7, r11, -0x22a0
	ctx.r[7].s64 = ctx.r[11].s64 + -8864;
	// 82697990: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82697994: 388AAD10  addi r4, r10, -0x52f0
	ctx.r[4].s64 = ctx.r[10].s64 + -21232;
	// 82697998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269799C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826979A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826979A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826979A8: 386A23F0  addi r3, r10, 0x23f0
	ctx.r[3].s64 = ctx.r[10].s64 + 9200;
	// 826979AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826979B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826979B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826979B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826979BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826979C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826979C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826979C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826979CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826979D0: 4BDCF451  bl 0x82466e20
	ctx.lr = 0x826979D4;
	sub_82466E20(ctx, base);
	// 826979D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826979D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826979DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826979E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826979E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826979E8 size=108
    let mut pc: u32 = 0x826979E8;
    'dispatch: loop {
        match pc {
            0x826979E8 => {
    //   block [0x826979E8..0x82697A54)
	// 826979E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826979EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826979F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826979F4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826979F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826979FC: 38EBDDA8  addi r7, r11, -0x2258
	ctx.r[7].s64 = ctx.r[11].s64 + -8792;
	// 82697A00: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82697A04: 388AAD38  addi r4, r10, -0x52c8
	ctx.r[4].s64 = ctx.r[10].s64 + -21192;
	// 82697A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697A0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697A10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82697A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697A18: 386A2420  addi r3, r10, 0x2420
	ctx.r[3].s64 = ctx.r[10].s64 + 9248;
	// 82697A1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82697A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697A3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697A40: 4BDCF3E1  bl 0x82466e20
	ctx.lr = 0x82697A44;
	sub_82466E20(ctx, base);
	// 82697A44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697A58 size=116
    let mut pc: u32 = 0x82697A58;
    'dispatch: loop {
        match pc {
            0x82697A58 => {
    //   block [0x82697A58..0x82697ACC)
	// 82697A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697A64: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82697A68: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82697A6C: 390ADDF0  addi r8, r10, -0x2210
	ctx.r[8].s64 = ctx.r[10].s64 + -8720;
	// 82697A70: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697A74: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82697A78: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 82697A7C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697A80: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82697A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697A88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697A8C: 388AAD60  addi r4, r10, -0x52a0
	ctx.r[4].s64 = ctx.r[10].s64 + -21152;
	// 82697A90: 396B911C  addi r11, r11, -0x6ee4
	ctx.r[11].s64 = ctx.r[11].s64 + -28388;
	// 82697A94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697A98: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697A9C: 386A2450  addi r3, r10, 0x2450
	ctx.r[3].s64 = ctx.r[10].s64 + 9296;
	// 82697AA0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82697AA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697AA8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82697AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697AB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697AB8: 4BDCF369  bl 0x82466e20
	ctx.lr = 0x82697ABC;
	sub_82466E20(ctx, base);
	// 82697ABC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697AC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697AC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697AC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697AD0 size=116
    let mut pc: u32 = 0x82697AD0;
    'dispatch: loop {
        match pc {
            0x82697AD0 => {
    //   block [0x82697AD0..0x82697B44)
	// 82697AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697AD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697ADC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82697AE0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82697AE4: 390ADE98  addi r8, r10, -0x2168
	ctx.r[8].s64 = ctx.r[10].s64 + -8552;
	// 82697AE8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697AEC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82697AF0: 38AA2450  addi r5, r10, 0x2450
	ctx.r[5].s64 = ctx.r[10].s64 + 9296;
	// 82697AF4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697AF8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82697AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697B00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697B04: 388AAD7C  addi r4, r10, -0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + -21124;
	// 82697B08: 396B9140  addi r11, r11, -0x6ec0
	ctx.r[11].s64 = ctx.r[11].s64 + -28352;
	// 82697B0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697B10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697B14: 386A2480  addi r3, r10, 0x2480
	ctx.r[3].s64 = ctx.r[10].s64 + 9344;
	// 82697B18: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82697B1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697B20: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82697B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697B30: 4BDCF2F1  bl 0x82466e20
	ctx.lr = 0x82697B34;
	sub_82466E20(ctx, base);
	// 82697B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82697B48 size=36
    let mut pc: u32 = 0x82697B48;
    'dispatch: loop {
        match pc {
            0x82697B48 => {
    //   block [0x82697B48..0x82697B6C)
	// 82697B48: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697B4C: 814BDD14  lwz r10, -0x22ec(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8940 as u32) ) } as u64;
	// 82697B50: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697B54: 396B5FB8  addi r11, r11, 0x5fb8
	ctx.r[11].s64 = ctx.r[11].s64 + 24504;
	// 82697B58: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82697B5C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82697B60: 814ADEE0  lwz r10, -0x2120(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8480 as u32) ) } as u64;
	// 82697B64: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82697B68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697B70 size=116
    let mut pc: u32 = 0x82697B70;
    'dispatch: loop {
        match pc {
            0x82697B70 => {
    //   block [0x82697B70..0x82697BE4)
	// 82697B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697B7C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697B80: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82697B84: 390B5FB8  addi r8, r11, 0x5fb8
	ctx.r[8].s64 = ctx.r[11].s64 + 24504;
	// 82697B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697B8C: 392A9180  addi r9, r10, -0x6e80
	ctx.r[9].s64 = ctx.r[10].s64 + -28288;
	// 82697B90: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697B94: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 82697B98: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82697B9C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697BA4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697BB4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82697BB8: 388AAE18  addi r4, r10, -0x51e8
	ctx.r[4].s64 = ctx.r[10].s64 + -20968;
	// 82697BBC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82697BC0: 386B24B0  addi r3, r11, 0x24b0
	ctx.r[3].s64 = ctx.r[11].s64 + 9392;
	// 82697BC4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82697BC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697BD0: 4BDCF251  bl 0x82466e20
	ctx.lr = 0x82697BD4;
	sub_82466E20(ctx, base);
	// 82697BD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697BE8 size=112
    let mut pc: u32 = 0x82697BE8;
    'dispatch: loop {
        match pc {
            0x82697BE8 => {
    //   block [0x82697BE8..0x82697C58)
	// 82697BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697BF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697BF8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697BFC: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82697C00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697C04: 390BDEE8  addi r8, r11, -0x2118
	ctx.r[8].s64 = ctx.r[11].s64 + -8472;
	// 82697C08: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82697C0C: 388AAE30  addi r4, r10, -0x51d0
	ctx.r[4].s64 = ctx.r[10].s64 + -20944;
	// 82697C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697C14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697C18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697C20: 386A24E0  addi r3, r10, 0x24e0
	ctx.r[3].s64 = ctx.r[10].s64 + 9440;
	// 82697C24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82697C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697C30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697C38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697C40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697C44: 4BDCF1DD  bl 0x82466e20
	ctx.lr = 0x82697C48;
	sub_82466E20(ctx, base);
	// 82697C48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697C58 size=108
    let mut pc: u32 = 0x82697C58;
    'dispatch: loop {
        match pc {
            0x82697C58 => {
    //   block [0x82697C58..0x82697CC4)
	// 82697C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697C64: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697C68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697C6C: 38EBDFA8  addi r7, r11, -0x2058
	ctx.r[7].s64 = ctx.r[11].s64 + -8280;
	// 82697C70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82697C74: 388AAE50  addi r4, r10, -0x51b0
	ctx.r[4].s64 = ctx.r[10].s64 + -20912;
	// 82697C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697C7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697C80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82697C84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697C88: 386A2510  addi r3, r10, 0x2510
	ctx.r[3].s64 = ctx.r[10].s64 + 9488;
	// 82697C8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82697C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697CAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697CB0: 4BDCF171  bl 0x82466e20
	ctx.lr = 0x82697CB4;
	sub_82466E20(ctx, base);
	// 82697CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697CC8 size=112
    let mut pc: u32 = 0x82697CC8;
    'dispatch: loop {
        match pc {
            0x82697CC8 => {
    //   block [0x82697CC8..0x82697D38)
	// 82697CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697CD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697CD8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697CDC: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82697CE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697CE4: 390BDFD8  addi r8, r11, -0x2028
	ctx.r[8].s64 = ctx.r[11].s64 + -8232;
	// 82697CE8: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 82697CEC: 388AAE6C  addi r4, r10, -0x5194
	ctx.r[4].s64 = ctx.r[10].s64 + -20884;
	// 82697CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697CF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697CF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697D00: 386A2540  addi r3, r10, 0x2540
	ctx.r[3].s64 = ctx.r[10].s64 + 9536;
	// 82697D04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82697D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697D24: 4BDCF0FD  bl 0x82466e20
	ctx.lr = 0x82697D28;
	sub_82466E20(ctx, base);
	// 82697D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697D38 size=112
    let mut pc: u32 = 0x82697D38;
    'dispatch: loop {
        match pc {
            0x82697D38 => {
    //   block [0x82697D38..0x82697DA8)
	// 82697D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697D44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697D48: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697D4C: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82697D50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697D54: 390BE0E0  addi r8, r11, -0x1f20
	ctx.r[8].s64 = ctx.r[11].s64 + -7968;
	// 82697D58: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 82697D5C: 388AAE80  addi r4, r10, -0x5180
	ctx.r[4].s64 = ctx.r[10].s64 + -20864;
	// 82697D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697D64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697D70: 386A2570  addi r3, r10, 0x2570
	ctx.r[3].s64 = ctx.r[10].s64 + 9584;
	// 82697D74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82697D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697D94: 4BDCF08D  bl 0x82466e20
	ctx.lr = 0x82697D98;
	sub_82466E20(ctx, base);
	// 82697D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697DA8 size=112
    let mut pc: u32 = 0x82697DA8;
    'dispatch: loop {
        match pc {
            0x82697DA8 => {
    //   block [0x82697DA8..0x82697E18)
	// 82697DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697DB4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82697DB8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82697DBC: 38EAE218  addi r7, r10, -0x1de8
	ctx.r[7].s64 = ctx.r[10].s64 + -7656;
	// 82697DC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697DC4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82697DC8: 388A7AE8  addi r4, r10, 0x7ae8
	ctx.r[4].s64 = ctx.r[10].s64 + 31464;
	// 82697DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697DD0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82697DD4: 396B91CC  addi r11, r11, -0x6e34
	ctx.r[11].s64 = ctx.r[11].s64 + -28212;
	// 82697DD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82697DDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697DE0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697DE4: 386A25A0  addi r3, r10, 0x25a0
	ctx.r[3].s64 = ctx.r[10].s64 + 9632;
	// 82697DE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697DEC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82697DF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697DF4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82697DF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697DFC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697E00: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697E04: 4BDCF01D  bl 0x82466e20
	ctx.lr = 0x82697E08;
	sub_82466E20(ctx, base);
	// 82697E08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697E18 size=116
    let mut pc: u32 = 0x82697E18;
    'dispatch: loop {
        match pc {
            0x82697E18 => {
    //   block [0x82697E18..0x82697E8C)
	// 82697E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697E24: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82697E28: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697E2C: 392B91B8  addi r9, r11, -0x6e48
	ctx.r[9].s64 = ctx.r[11].s64 + -28232;
	// 82697E30: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82697E34: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697E38: 38E90038  addi r7, r9, 0x38
	ctx.r[7].s64 = ctx.r[9].s64 + 56;
	// 82697E3C: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82697E40: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697E44: 388AAE98  addi r4, r10, -0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + -20840;
	// 82697E48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697E4C: 396BE290  addi r11, r11, -0x1d70
	ctx.r[11].s64 = ctx.r[11].s64 + -7536;
	// 82697E50: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82697E54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697E58: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82697E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697E60: 386A25D0  addi r3, r10, 0x25d0
	ctx.r[3].s64 = ctx.r[10].s64 + 9680;
	// 82697E64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82697E68: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82697E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697E70: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82697E74: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82697E78: 4BDCEFA9  bl 0x82466e20
	ctx.lr = 0x82697E7C;
	sub_82466E20(ctx, base);
	// 82697E7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697E90 size=112
    let mut pc: u32 = 0x82697E90;
    'dispatch: loop {
        match pc {
            0x82697E90 => {
    //   block [0x82697E90..0x82697F00)
	// 82697E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697E9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697EA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697EA4: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82697EA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697EAC: 390BE578  addi r8, r11, -0x1a88
	ctx.r[8].s64 = ctx.r[11].s64 + -6792;
	// 82697EB0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82697EB4: 388AAEB0  addi r4, r10, -0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + -20816;
	// 82697EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697EBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697EC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697EC8: 386A2600  addi r3, r10, 0x2600
	ctx.r[3].s64 = ctx.r[10].s64 + 9728;
	// 82697ECC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82697ED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697ED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697EE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697EEC: 4BDCEF35  bl 0x82466e20
	ctx.lr = 0x82697EF0;
	sub_82466E20(ctx, base);
	// 82697EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697F00 size=116
    let mut pc: u32 = 0x82697F00;
    'dispatch: loop {
        match pc {
            0x82697F00 => {
    //   block [0x82697F00..0x82697F74)
	// 82697F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697F0C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82697F10: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82697F14: 390AE5C0  addi r8, r10, -0x1a40
	ctx.r[8].s64 = ctx.r[10].s64 + -6720;
	// 82697F18: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697F1C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82697F20: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82697F24: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697F28: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82697F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697F30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697F34: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 82697F38: 396B92D0  addi r11, r11, -0x6d30
	ctx.r[11].s64 = ctx.r[11].s64 + -27952;
	// 82697F3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697F40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697F44: 386A2630  addi r3, r10, 0x2630
	ctx.r[3].s64 = ctx.r[10].s64 + 9776;
	// 82697F48: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82697F4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697F50: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82697F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697F60: 4BDCEEC1  bl 0x82466e20
	ctx.lr = 0x82697F64;
	sub_82466E20(ctx, base);
	// 82697F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697F78 size=112
    let mut pc: u32 = 0x82697F78;
    'dispatch: loop {
        match pc {
            0x82697F78 => {
    //   block [0x82697F78..0x82697FE8)
	// 82697F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697F84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697F88: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697F8C: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82697F90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697F94: 390BE608  addi r8, r11, -0x19f8
	ctx.r[8].s64 = ctx.r[11].s64 + -6648;
	// 82697F98: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82697F9C: 388AAEF4  addi r4, r10, -0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + -20748;
	// 82697FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697FA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697FB0: 386A2660  addi r3, r10, 0x2660
	ctx.r[3].s64 = ctx.r[10].s64 + 9824;
	// 82697FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82697FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697FD4: 4BDCEE4D  bl 0x82466e20
	ctx.lr = 0x82697FD8;
	sub_82466E20(ctx, base);
	// 82697FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697FE8 size=112
    let mut pc: u32 = 0x82697FE8;
    'dispatch: loop {
        match pc {
            0x82697FE8 => {
    //   block [0x82697FE8..0x82698058)
	// 82697FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697FF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697FF8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697FFC: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82698000: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82698004: 390BE698  addi r8, r11, -0x1968
	ctx.r[8].s64 = ctx.r[11].s64 + -6504;
	// 82698008: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8269800C: 388AAF08  addi r4, r10, -0x50f8
	ctx.r[4].s64 = ctx.r[10].s64 + -20728;
	// 82698010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698014: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269801C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698020: 386A2690  addi r3, r10, 0x2690
	ctx.r[3].s64 = ctx.r[10].s64 + 9872;
	// 82698024: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82698028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269802C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269803C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698044: 4BDCEDDD  bl 0x82466e20
	ctx.lr = 0x82698048;
	sub_82466E20(ctx, base);
	// 82698048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269804C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82698058 size=28
    let mut pc: u32 = 0x82698058;
    'dispatch: loop {
        match pc {
            0x82698058 => {
    //   block [0x82698058..0x82698074)
	// 82698058: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269805C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82698060: 394A60C0  addi r10, r10, 0x60c0
	ctx.r[10].s64 = ctx.r[10].s64 + 24768;
	// 82698064: 816BE710  lwz r11, -0x18f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6384 as u32) ) } as u64;
	// 82698068: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8269806C: 916A0098  stw r11, 0x98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82698070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698078 size=116
    let mut pc: u32 = 0x82698078;
    'dispatch: loop {
        match pc {
            0x82698078 => {
    //   block [0x82698078..0x826980EC)
	// 82698078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269807C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698084: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82698088: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269808C: 392B92FC  addi r9, r11, -0x6d04
	ctx.r[9].s64 = ctx.r[11].s64 + -27908;
	// 82698090: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82698094: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82698098: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8269809C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 826980A0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826980A4: 388AAF24  addi r4, r10, -0x50dc
	ctx.r[4].s64 = ctx.r[10].s64 + -20700;
	// 826980A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826980AC: 396B60C0  addi r11, r11, 0x60c0
	ctx.r[11].s64 = ctx.r[11].s64 + 24768;
	// 826980B0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826980B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826980B8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826980BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826980C0: 386A26C0  addi r3, r10, 0x26c0
	ctx.r[3].s64 = ctx.r[10].s64 + 9920;
	// 826980C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826980C8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826980CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826980D0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826980D4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826980D8: 4BDCED49  bl 0x82466e20
	ctx.lr = 0x826980DC;
	sub_82466E20(ctx, base);
	// 826980DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826980E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826980E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826980E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826980F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826980F0 size=112
    let mut pc: u32 = 0x826980F0;
    'dispatch: loop {
        match pc {
            0x826980F0 => {
    //   block [0x826980F0..0x82698160)
	// 826980F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826980F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826980F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826980FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698100: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698104: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82698108: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269810C: 390BE718  addi r8, r11, -0x18e8
	ctx.r[8].s64 = ctx.r[11].s64 + -6376;
	// 82698110: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82698114: 388AAF40  addi r4, r10, -0x50c0
	ctx.r[4].s64 = ctx.r[10].s64 + -20672;
	// 82698118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269811C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698120: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698128: 386A26F0  addi r3, r10, 0x26f0
	ctx.r[3].s64 = ctx.r[10].s64 + 9968;
	// 8269812C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82698130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269813C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269814C: 4BDCECD5  bl 0x82466e20
	ctx.lr = 0x82698150;
	sub_82466E20(ctx, base);
	// 82698150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269815C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82698160 size=24
    let mut pc: u32 = 0x82698160;
    'dispatch: loop {
        match pc {
            0x82698160 => {
    //   block [0x82698160..0x82698178)
	// 82698160: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698164: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82698168: 394A6180  addi r10, r10, 0x6180
	ctx.r[10].s64 = ctx.r[10].s64 + 24960;
	// 8269816C: 816BE714  lwz r11, -0x18ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6380 as u32) ) } as u64;
	// 82698170: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82698174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698178 size=116
    let mut pc: u32 = 0x82698178;
    'dispatch: loop {
        match pc {
            0x82698178 => {
    //   block [0x82698178..0x826981EC)
	// 82698178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269817C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698184: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698188: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269818C: 390B6180  addi r8, r11, 0x6180
	ctx.r[8].s64 = ctx.r[11].s64 + 24960;
	// 82698190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698194: 392A9368  addi r9, r10, -0x6c98
	ctx.r[9].s64 = ctx.r[10].s64 + -27800;
	// 82698198: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269819C: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826981A0: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 826981A4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826981A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826981AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826981B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826981B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826981B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826981BC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826981C0: 388AAF78  addi r4, r10, -0x5088
	ctx.r[4].s64 = ctx.r[10].s64 + -20616;
	// 826981C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826981C8: 386B2720  addi r3, r11, 0x2720
	ctx.r[3].s64 = ctx.r[11].s64 + 10016;
	// 826981CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826981D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826981D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826981D8: 4BDCEC49  bl 0x82466e20
	ctx.lr = 0x826981DC;
	sub_82466E20(ctx, base);
	// 826981DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826981E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826981E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826981E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826981F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826981F0 size=112
    let mut pc: u32 = 0x826981F0;
    'dispatch: loop {
        match pc {
            0x826981F0 => {
    //   block [0x826981F0..0x82698260)
	// 826981F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826981F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826981F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826981FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698200: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698204: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82698208: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269820C: 390BE790  addi r8, r11, -0x1870
	ctx.r[8].s64 = ctx.r[11].s64 + -6256;
	// 82698210: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82698214: 388AAF98  addi r4, r10, -0x5068
	ctx.r[4].s64 = ctx.r[10].s64 + -20584;
	// 82698218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269821C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698228: 386A2750  addi r3, r10, 0x2750
	ctx.r[3].s64 = ctx.r[10].s64 + 10064;
	// 8269822C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82698230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269823C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269824C: 4BDCEBD5  bl 0x82466e20
	ctx.lr = 0x82698250;
	sub_82466E20(ctx, base);
	// 82698250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269825C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698260 size=112
    let mut pc: u32 = 0x82698260;
    'dispatch: loop {
        match pc {
            0x82698260 => {
    //   block [0x82698260..0x826982D0)
	// 82698260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269826C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698270: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698274: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82698278: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269827C: 390BE880  addi r8, r11, -0x1780
	ctx.r[8].s64 = ctx.r[11].s64 + -6016;
	// 82698280: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82698284: 388AAFB4  addi r4, r10, -0x504c
	ctx.r[4].s64 = ctx.r[10].s64 + -20556;
	// 82698288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269828C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698298: 386A2780  addi r3, r10, 0x2780
	ctx.r[3].s64 = ctx.r[10].s64 + 10112;
	// 8269829C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826982A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826982A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826982A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826982AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826982B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826982B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826982B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826982BC: 4BDCEB65  bl 0x82466e20
	ctx.lr = 0x826982C0;
	sub_82466E20(ctx, base);
	// 826982C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826982C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826982C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826982CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826982D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826982D0 size=24
    let mut pc: u32 = 0x826982D0;
    'dispatch: loop {
        match pc {
            0x826982D0 => {
    //   block [0x826982D0..0x826982E8)
	// 826982D0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826982D4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826982D8: 394A6228  addi r10, r10, 0x6228
	ctx.r[10].s64 = ctx.r[10].s64 + 25128;
	// 826982DC: 816BE8E0  lwz r11, -0x1720(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5920 as u32) ) } as u64;
	// 826982E0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826982E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826982E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826982E8 size=116
    let mut pc: u32 = 0x826982E8;
    'dispatch: loop {
        match pc {
            0x826982E8 => {
    //   block [0x826982E8..0x8269835C)
	// 826982E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826982EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826982F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826982F4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826982F8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826982FC: 390B6228  addi r8, r11, 0x6228
	ctx.r[8].s64 = ctx.r[11].s64 + 25128;
	// 82698300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698304: 392A93B4  addi r9, r10, -0x6c4c
	ctx.r[9].s64 = ctx.r[10].s64 + -27724;
	// 82698308: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269830C: 38E00019  li r7, 0x19
	ctx.r[7].s64 = 25;
	// 82698310: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82698314: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269831C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82698320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269832C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82698330: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 82698334: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82698338: 386B27B0  addi r3, r11, 0x27b0
	ctx.r[3].s64 = ctx.r[11].s64 + 10160;
	// 8269833C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82698340: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698348: 4BDCEAD9  bl 0x82466e20
	ctx.lr = 0x8269834C;
	sub_82466E20(ctx, base);
	// 8269834C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698360 size=116
    let mut pc: u32 = 0x82698360;
    'dispatch: loop {
        match pc {
            0x82698360 => {
    //   block [0x82698360..0x826983D4)
	// 82698360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269836C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82698370: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698374: 392B93EC  addi r9, r11, -0x6c14
	ctx.r[9].s64 = ctx.r[11].s64 + -27668;
	// 82698378: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 8269837C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82698380: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82698384: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 82698388: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269838C: 388AB168  addi r4, r10, -0x4e98
	ctx.r[4].s64 = ctx.r[10].s64 + -20120;
	// 82698390: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698394: 396BE8F0  addi r11, r11, -0x1710
	ctx.r[11].s64 = ctx.r[11].s64 + -5904;
	// 82698398: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8269839C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826983A0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826983A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826983A8: 386A27E0  addi r3, r10, 0x27e0
	ctx.r[3].s64 = ctx.r[10].s64 + 10208;
	// 826983AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826983B0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826983B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826983B8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826983BC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826983C0: 4BDCEA61  bl 0x82466e20
	ctx.lr = 0x826983C4;
	sub_82466E20(ctx, base);
	// 826983C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826983C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826983CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826983D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826983D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826983D8 size=112
    let mut pc: u32 = 0x826983D8;
    'dispatch: loop {
        match pc {
            0x826983D8 => {
    //   block [0x826983D8..0x82698448)
	// 826983D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826983DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826983E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826983E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826983E8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826983EC: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826983F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826983F4: 390BEAB8  addi r8, r11, -0x1548
	ctx.r[8].s64 = ctx.r[11].s64 + -5448;
	// 826983F8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826983FC: 388AB188  addi r4, r10, -0x4e78
	ctx.r[4].s64 = ctx.r[10].s64 + -20088;
	// 82698400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698404: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269840C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698410: 386A2810  addi r3, r10, 0x2810
	ctx.r[3].s64 = ctx.r[10].s64 + 10256;
	// 82698414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82698418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269841C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269842C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698434: 4BDCE9ED  bl 0x82466e20
	ctx.lr = 0x82698438;
	sub_82466E20(ctx, base);
	// 82698438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269843C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82698448 size=48
    let mut pc: u32 = 0x82698448;
    'dispatch: loop {
        match pc {
            0x82698448 => {
    //   block [0x82698448..0x82698478)
	// 82698448: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269844C: 814BEB30  lwz r10, -0x14d0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5328 as u32) ) } as u64;
	// 82698450: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698454: 396B6480  addi r11, r11, 0x6480
	ctx.r[11].s64 = ctx.r[11].s64 + 25728;
	// 82698458: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8269845C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82698460: 814AEB34  lwz r10, -0x14cc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-5324 as u32) ) } as u64;
	// 82698464: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 82698468: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269846C: 814AE8EC  lwz r10, -0x1714(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-5908 as u32) ) } as u64;
	// 82698470: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 82698474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698478 size=116
    let mut pc: u32 = 0x82698478;
    'dispatch: loop {
        match pc {
            0x82698478 => {
    //   block [0x82698478..0x826984EC)
	// 82698478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269847C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698484: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698488: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269848C: 390B6480  addi r8, r11, 0x6480
	ctx.r[8].s64 = ctx.r[11].s64 + 25728;
	// 82698490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698494: 392A94A8  addi r9, r10, -0x6b58
	ctx.r[9].s64 = ctx.r[10].s64 + -27480;
	// 82698498: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269849C: 38E00016  li r7, 0x16
	ctx.r[7].s64 = 22;
	// 826984A0: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 826984A4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826984A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826984AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826984B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826984B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826984B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826984BC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826984C0: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826984C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826984C8: 386B2840  addi r3, r11, 0x2840
	ctx.r[3].s64 = ctx.r[11].s64 + 10304;
	// 826984CC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826984D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826984D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826984D8: 4BDCE949  bl 0x82466e20
	ctx.lr = 0x826984DC;
	sub_82466E20(ctx, base);
	// 826984DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826984E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826984E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826984E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826984F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826984F0 size=116
    let mut pc: u32 = 0x826984F0;
    'dispatch: loop {
        match pc {
            0x826984F0 => {
    //   block [0x826984F0..0x82698564)
	// 826984F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826984F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826984F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826984FC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82698500: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82698504: 390AEB38  addi r8, r10, -0x14c8
	ctx.r[8].s64 = ctx.r[10].s64 + -5320;
	// 82698508: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269850C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82698510: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82698514: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82698518: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269851C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698520: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698524: 388AB2C4  addi r4, r10, -0x4d3c
	ctx.r[4].s64 = ctx.r[10].s64 + -19772;
	// 82698528: 396B94E4  addi r11, r11, -0x6b1c
	ctx.r[11].s64 = ctx.r[11].s64 + -27420;
	// 8269852C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698530: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698534: 386A2870  addi r3, r10, 0x2870
	ctx.r[3].s64 = ctx.r[10].s64 + 10352;
	// 82698538: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269853C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698540: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82698544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269854C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698550: 4BDCE8D1  bl 0x82466e20
	ctx.lr = 0x82698554;
	sub_82466E20(ctx, base);
	// 82698554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269855C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698568 size=116
    let mut pc: u32 = 0x82698568;
    'dispatch: loop {
        match pc {
            0x82698568 => {
    //   block [0x82698568..0x826985DC)
	// 82698568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269856C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698574: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82698578: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8269857C: 390AEB80  addi r8, r10, -0x1480
	ctx.r[8].s64 = ctx.r[10].s64 + -5248;
	// 82698580: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698584: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82698588: 38AA1CA0  addi r5, r10, 0x1ca0
	ctx.r[5].s64 = ctx.r[10].s64 + 7328;
	// 8269858C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82698590: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82698594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269859C: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 826985A0: 396B94F4  addi r11, r11, -0x6b0c
	ctx.r[11].s64 = ctx.r[11].s64 + -27404;
	// 826985A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826985A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826985AC: 386A28A0  addi r3, r10, 0x28a0
	ctx.r[3].s64 = ctx.r[10].s64 + 10400;
	// 826985B0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826985B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826985B8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826985BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826985C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826985C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826985C8: 4BDCE859  bl 0x82466e20
	ctx.lr = 0x826985CC;
	sub_82466E20(ctx, base);
	// 826985CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826985D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826985D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826985D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826985E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826985E0 size=116
    let mut pc: u32 = 0x826985E0;
    'dispatch: loop {
        match pc {
            0x826985E0 => {
    //   block [0x826985E0..0x82698654)
	// 826985E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826985E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826985E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826985EC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826985F0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826985F4: 390AEBF8  addi r8, r10, -0x1408
	ctx.r[8].s64 = ctx.r[10].s64 + -5128;
	// 826985F8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826985FC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82698600: 38AA20C0  addi r5, r10, 0x20c0
	ctx.r[5].s64 = ctx.r[10].s64 + 8384;
	// 82698604: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82698608: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269860C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698614: 388AB2F4  addi r4, r10, -0x4d0c
	ctx.r[4].s64 = ctx.r[10].s64 + -19724;
	// 82698618: 396B950C  addi r11, r11, -0x6af4
	ctx.r[11].s64 = ctx.r[11].s64 + -27380;
	// 8269861C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698620: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698624: 386A28D0  addi r3, r10, 0x28d0
	ctx.r[3].s64 = ctx.r[10].s64 + 10448;
	// 82698628: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269862C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698630: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82698634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269863C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698640: 4BDCE7E1  bl 0x82466e20
	ctx.lr = 0x82698644;
	sub_82466E20(ctx, base);
	// 82698644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269864C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698658 size=108
    let mut pc: u32 = 0x82698658;
    'dispatch: loop {
        match pc {
            0x82698658 => {
    //   block [0x82698658..0x826986C4)
	// 82698658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269865C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698664: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698668: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269866C: 38EBEC70  addi r7, r11, -0x1390
	ctx.r[7].s64 = ctx.r[11].s64 + -5008;
	// 82698670: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82698674: 388AB318  addi r4, r10, -0x4ce8
	ctx.r[4].s64 = ctx.r[10].s64 + -19688;
	// 82698678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269867C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698680: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698688: 386A2900  addi r3, r10, 0x2900
	ctx.r[3].s64 = ctx.r[10].s64 + 10496;
	// 8269868C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269869C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826986A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826986A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826986A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826986AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826986B0: 4BDCE771  bl 0x82466e20
	ctx.lr = 0x826986B4;
	sub_82466E20(ctx, base);
	// 826986B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826986B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826986BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826986C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826986C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826986C8 size=108
    let mut pc: u32 = 0x826986C8;
    'dispatch: loop {
        match pc {
            0x826986C8 => {
    //   block [0x826986C8..0x82698734)
	// 826986C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826986CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826986D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826986D4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826986D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826986DC: 38EBECB8  addi r7, r11, -0x1348
	ctx.r[7].s64 = ctx.r[11].s64 + -4936;
	// 826986E0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826986E4: 388AB344  addi r4, r10, -0x4cbc
	ctx.r[4].s64 = ctx.r[10].s64 + -19644;
	// 826986E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826986EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826986F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826986F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826986F8: 386A2930  addi r3, r10, 0x2930
	ctx.r[3].s64 = ctx.r[10].s64 + 10544;
	// 826986FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269870C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269871C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698720: 4BDCE701  bl 0x82466e20
	ctx.lr = 0x82698724;
	sub_82466E20(ctx, base);
	// 82698724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269872C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698738 size=108
    let mut pc: u32 = 0x82698738;
    'dispatch: loop {
        match pc {
            0x82698738 => {
    //   block [0x82698738..0x826987A4)
	// 82698738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269873C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698744: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698748: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269874C: 38EBED00  addi r7, r11, -0x1300
	ctx.r[7].s64 = ctx.r[11].s64 + -4864;
	// 82698750: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82698754: 388AB36C  addi r4, r10, -0x4c94
	ctx.r[4].s64 = ctx.r[10].s64 + -19604;
	// 82698758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269875C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698760: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698768: 386A2960  addi r3, r10, 0x2960
	ctx.r[3].s64 = ctx.r[10].s64 + 10592;
	// 8269876C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269877C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269878C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698790: 4BDCE691  bl 0x82466e20
	ctx.lr = 0x82698794;
	sub_82466E20(ctx, base);
	// 82698794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269879C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826987A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826987A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826987A8 size=108
    let mut pc: u32 = 0x826987A8;
    'dispatch: loop {
        match pc {
            0x826987A8 => {
    //   block [0x826987A8..0x82698814)
	// 826987A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826987AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826987B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826987B4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826987B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826987BC: 38EBED48  addi r7, r11, -0x12b8
	ctx.r[7].s64 = ctx.r[11].s64 + -4792;
	// 826987C0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826987C4: 388AB398  addi r4, r10, -0x4c68
	ctx.r[4].s64 = ctx.r[10].s64 + -19560;
	// 826987C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826987CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826987D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826987D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826987D8: 386A2990  addi r3, r10, 0x2990
	ctx.r[3].s64 = ctx.r[10].s64 + 10640;
	// 826987DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826987E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826987E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826987E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826987EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826987F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826987F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826987F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826987FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698800: 4BDCE621  bl 0x82466e20
	ctx.lr = 0x82698804;
	sub_82466E20(ctx, base);
	// 82698804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269880C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698818 size=108
    let mut pc: u32 = 0x82698818;
    'dispatch: loop {
        match pc {
            0x82698818 => {
    //   block [0x82698818..0x82698884)
	// 82698818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269881C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698824: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698828: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8269882C: 38EBEDF0  addi r7, r11, -0x1210
	ctx.r[7].s64 = ctx.r[11].s64 + -4624;
	// 82698830: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82698834: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 82698838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269883C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698840: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698848: 386A29C0  addi r3, r10, 0x29c0
	ctx.r[3].s64 = ctx.r[10].s64 + 10688;
	// 8269884C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269885C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269886C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698870: 4BDCE5B1  bl 0x82466e20
	ctx.lr = 0x82698874;
	sub_82466E20(ctx, base);
	// 82698874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269887C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698888 size=112
    let mut pc: u32 = 0x82698888;
    'dispatch: loop {
        match pc {
            0x82698888 => {
    //   block [0x82698888..0x826988F8)
	// 82698888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269888C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698894: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82698898: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269889C: 392A954C  addi r9, r10, -0x6ab4
	ctx.r[9].s64 = ctx.r[10].s64 + -27316;
	// 826988A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826988A4: 390BEE28  addi r8, r11, -0x11d8
	ctx.r[8].s64 = ctx.r[11].s64 + -4568;
	// 826988A8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826988AC: 388A8198  addi r4, r10, -0x7e68
	ctx.r[4].s64 = ctx.r[10].s64 + -32360;
	// 826988B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826988B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826988B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826988BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826988C0: 386A29F0  addi r3, r10, 0x29f0
	ctx.r[3].s64 = ctx.r[10].s64 + 10736;
	// 826988C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826988C8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826988CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826988D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826988D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826988D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826988DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826988E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826988E4: 4BDCE53D  bl 0x82466e20
	ctx.lr = 0x826988E8;
	sub_82466E20(ctx, base);
	// 826988E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826988EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826988F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826988F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826988F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826988F8 size=108
    let mut pc: u32 = 0x826988F8;
    'dispatch: loop {
        match pc {
            0x826988F8 => {
    //   block [0x826988F8..0x82698964)
	// 826988F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826988FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698904: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269890C: 38EBEE70  addi r7, r11, -0x1190
	ctx.r[7].s64 = ctx.r[11].s64 + -4496;
	// 82698910: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82698914: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 82698918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269891C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698920: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698928: 386A2A20  addi r3, r10, 0x2a20
	ctx.r[3].s64 = ctx.r[10].s64 + 10784;
	// 8269892C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269893C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269894C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698950: 4BDCE4D1  bl 0x82466e20
	ctx.lr = 0x82698954;
	sub_82466E20(ctx, base);
	// 82698954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269895C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698968 size=108
    let mut pc: u32 = 0x82698968;
    'dispatch: loop {
        match pc {
            0x82698968 => {
    //   block [0x82698968..0x826989D4)
	// 82698968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269896C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698974: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698978: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8269897C: 38EBEEE8  addi r7, r11, -0x1118
	ctx.r[7].s64 = ctx.r[11].s64 + -4376;
	// 82698980: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82698984: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 82698988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269898C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698998: 386A2A50  addi r3, r10, 0x2a50
	ctx.r[3].s64 = ctx.r[10].s64 + 10832;
	// 8269899C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826989A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826989A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826989A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826989AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826989B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826989B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826989B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826989BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826989C0: 4BDCE461  bl 0x82466e20
	ctx.lr = 0x826989C4;
	sub_82466E20(ctx, base);
	// 826989C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826989C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826989CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826989D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826989D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826989D8 size=108
    let mut pc: u32 = 0x826989D8;
    'dispatch: loop {
        match pc {
            0x826989D8 => {
    //   block [0x826989D8..0x82698A44)
	// 826989D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826989DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826989E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826989E4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826989E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826989EC: 38EBEF18  addi r7, r11, -0x10e8
	ctx.r[7].s64 = ctx.r[11].s64 + -4328;
	// 826989F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826989F4: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 826989F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826989FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698A00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698A08: 386A2A80  addi r3, r10, 0x2a80
	ctx.r[3].s64 = ctx.r[10].s64 + 10880;
	// 82698A0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698A14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698A2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698A30: 4BDCE3F1  bl 0x82466e20
	ctx.lr = 0x82698A34;
	sub_82466E20(ctx, base);
	// 82698A34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82698A48 size=24
    let mut pc: u32 = 0x82698A48;
    'dispatch: loop {
        match pc {
            0x82698A48 => {
    //   block [0x82698A48..0x82698A60)
	// 82698A48: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698A4C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82698A50: 394A6690  addi r10, r10, 0x6690
	ctx.r[10].s64 = ctx.r[10].s64 + 26256;
	// 82698A54: 816BEF30  lwz r11, -0x10d0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4304 as u32) ) } as u64;
	// 82698A58: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82698A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698A60 size=112
    let mut pc: u32 = 0x82698A60;
    'dispatch: loop {
        match pc {
            0x82698A60 => {
    //   block [0x82698A60..0x82698AD0)
	// 82698A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698A68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698A6C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82698A70: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698A74: 392A958C  addi r9, r10, -0x6a74
	ctx.r[9].s64 = ctx.r[10].s64 + -27252;
	// 82698A78: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698A7C: 390B6690  addi r8, r11, 0x6690
	ctx.r[8].s64 = ctx.r[11].s64 + 26256;
	// 82698A80: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82698A84: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 82698A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698A8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698A90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698A98: 386A2AB0  addi r3, r10, 0x2ab0
	ctx.r[3].s64 = ctx.r[10].s64 + 10928;
	// 82698A9C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82698AA0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82698AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698AA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698AB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698AB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698AB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698ABC: 4BDCE365  bl 0x82466e20
	ctx.lr = 0x82698AC0;
	sub_82466E20(ctx, base);
	// 82698AC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698AC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698AC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698ACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698AD0 size=96
    let mut pc: u32 = 0x82698AD0;
    'dispatch: loop {
        match pc {
            0x82698AD0 => {
    //   block [0x82698AD0..0x82698B30)
	// 82698AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698AD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698ADC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698AE4: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 82698AE8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698AF0: 386A2AE0  addi r3, r10, 0x2ae0
	ctx.r[3].s64 = ctx.r[10].s64 + 10976;
	// 82698AF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698AFC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82698B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698B04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698B10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82698B14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698B18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82698B1C: 4BDCE305  bl 0x82466e20
	ctx.lr = 0x82698B20;
	sub_82466E20(ctx, base);
	// 82698B20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698B30 size=112
    let mut pc: u32 = 0x82698B30;
    'dispatch: loop {
        match pc {
            0x82698B30 => {
    //   block [0x82698B30..0x82698BA0)
	// 82698B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698B3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698B40: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698B44: 38AA2AE0  addi r5, r10, 0x2ae0
	ctx.r[5].s64 = ctx.r[10].s64 + 10976;
	// 82698B48: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698B4C: 390BEF34  addi r8, r11, -0x10cc
	ctx.r[8].s64 = ctx.r[11].s64 + -4300;
	// 82698B50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82698B54: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 82698B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698B5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698B60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698B68: 386A2B10  addi r3, r10, 0x2b10
	ctx.r[3].s64 = ctx.r[10].s64 + 11024;
	// 82698B6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82698B70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698B74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698B7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698B8C: 4BDCE295  bl 0x82466e20
	ctx.lr = 0x82698B90;
	sub_82466E20(ctx, base);
	// 82698B90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82698BA0 size=24
    let mut pc: u32 = 0x82698BA0;
    'dispatch: loop {
        match pc {
            0x82698BA0 => {
    //   block [0x82698BA0..0x82698BB8)
	// 82698BA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698BA4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82698BA8: 394A6768  addi r10, r10, 0x6768
	ctx.r[10].s64 = ctx.r[10].s64 + 26472;
	// 82698BAC: 816BEF68  lwz r11, -0x1098(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4248 as u32) ) } as u64;
	// 82698BB0: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82698BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698BB8 size=112
    let mut pc: u32 = 0x82698BB8;
    'dispatch: loop {
        match pc {
            0x82698BB8 => {
    //   block [0x82698BB8..0x82698C28)
	// 82698BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698BC4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82698BC8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698BCC: 392A95B8  addi r9, r10, -0x6a48
	ctx.r[9].s64 = ctx.r[10].s64 + -27208;
	// 82698BD0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698BD4: 390B6768  addi r8, r11, 0x6768
	ctx.r[8].s64 = ctx.r[11].s64 + 26472;
	// 82698BD8: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 82698BDC: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 82698BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698BE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698BE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698BF0: 386A2B40  addi r3, r10, 0x2b40
	ctx.r[3].s64 = ctx.r[10].s64 + 11072;
	// 82698BF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82698BF8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82698BFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698C04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698C0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698C14: 4BDCE20D  bl 0x82466e20
	ctx.lr = 0x82698C18;
	sub_82466E20(ctx, base);
	// 82698C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698C28 size=108
    let mut pc: u32 = 0x82698C28;
    'dispatch: loop {
        match pc {
            0x82698C28 => {
    //   block [0x82698C28..0x82698C94)
	// 82698C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698C34: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698C38: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698C3C: 38EBEF70  addi r7, r11, -0x1090
	ctx.r[7].s64 = ctx.r[11].s64 + -4240;
	// 82698C40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82698C44: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 82698C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698C4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698C50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698C54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698C58: 386A2B70  addi r3, r10, 0x2b70
	ctx.r[3].s64 = ctx.r[10].s64 + 11120;
	// 82698C5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698C60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698C68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698C70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698C74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698C78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698C7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698C80: 4BDCE1A1  bl 0x82466e20
	ctx.lr = 0x82698C84;
	sub_82466E20(ctx, base);
	// 82698C84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82698C98 size=24
    let mut pc: u32 = 0x82698C98;
    'dispatch: loop {
        match pc {
            0x82698C98 => {
    //   block [0x82698C98..0x82698CB0)
	// 82698C98: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698C9C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82698CA0: 394A6858  addi r10, r10, 0x6858
	ctx.r[10].s64 = ctx.r[10].s64 + 26712;
	// 82698CA4: 816BEF6C  lwz r11, -0x1094(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4244 as u32) ) } as u64;
	// 82698CA8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82698CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698CB0 size=112
    let mut pc: u32 = 0x82698CB0;
    'dispatch: loop {
        match pc {
            0x82698CB0 => {
    //   block [0x82698CB0..0x82698D20)
	// 82698CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698CBC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82698CC0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698CC4: 392A95E8  addi r9, r10, -0x6a18
	ctx.r[9].s64 = ctx.r[10].s64 + -27160;
	// 82698CC8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698CCC: 390B6858  addi r8, r11, 0x6858
	ctx.r[8].s64 = ctx.r[11].s64 + 26712;
	// 82698CD0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82698CD4: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 82698CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698CDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698CE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698CE8: 386A2BA0  addi r3, r10, 0x2ba0
	ctx.r[3].s64 = ctx.r[10].s64 + 11168;
	// 82698CEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82698CF0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82698CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698CF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698D00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698D08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698D0C: 4BDCE115  bl 0x82466e20
	ctx.lr = 0x82698D10;
	sub_82466E20(ctx, base);
	// 82698D10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82698D20 size=40
    let mut pc: u32 = 0x82698D20;
    'dispatch: loop {
        match pc {
            0x82698D20 => {
    //   block [0x82698D20..0x82698D48)
	// 82698D20: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698D24: 814BEFA0  lwz r10, -0x1060(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4192 as u32) ) } as u64;
	// 82698D28: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698D2C: 396B68B8  addi r11, r11, 0x68b8
	ctx.r[11].s64 = ctx.r[11].s64 + 26808;
	// 82698D30: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82698D34: 914B0068  stw r10, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82698D38: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82698D3C: 814AEFA4  lwz r10, -0x105c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4188 as u32) ) } as u64;
	// 82698D40: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 82698D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698D48 size=112
    let mut pc: u32 = 0x82698D48;
    'dispatch: loop {
        match pc {
            0x82698D48 => {
    //   block [0x82698D48..0x82698DB8)
	// 82698D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698D54: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82698D58: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698D5C: 392A9760  addi r9, r10, -0x68a0
	ctx.r[9].s64 = ctx.r[10].s64 + -26784;
	// 82698D60: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698D64: 390B68B8  addi r8, r11, 0x68b8
	ctx.r[8].s64 = ctx.r[11].s64 + 26808;
	// 82698D68: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82698D6C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 82698D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698D74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698D78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698D80: 386A2BD0  addi r3, r10, 0x2bd0
	ctx.r[3].s64 = ctx.r[10].s64 + 11216;
	// 82698D84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82698D88: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82698D8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698D9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698DA4: 4BDCE07D  bl 0x82466e20
	ctx.lr = 0x82698DA8;
	sub_82466E20(ctx, base);
	// 82698DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698DB8 size=108
    let mut pc: u32 = 0x82698DB8;
    'dispatch: loop {
        match pc {
            0x82698DB8 => {
    //   block [0x82698DB8..0x82698E24)
	// 82698DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698DC4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698DC8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698DCC: 38EBEFAC  addi r7, r11, -0x1054
	ctx.r[7].s64 = ctx.r[11].s64 + -4180;
	// 82698DD0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82698DD4: 388A6E48  addi r4, r10, 0x6e48
	ctx.r[4].s64 = ctx.r[10].s64 + 28232;
	// 82698DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698DDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698DE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698DE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698DE8: 386A2C00  addi r3, r10, 0x2c00
	ctx.r[3].s64 = ctx.r[10].s64 + 11264;
	// 82698DEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698DF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698E0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698E10: 4BDCE011  bl 0x82466e20
	ctx.lr = 0x82698E14;
	sub_82466E20(ctx, base);
	// 82698E14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698E18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698E1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698E28 size=108
    let mut pc: u32 = 0x82698E28;
    'dispatch: loop {
        match pc {
            0x82698E28 => {
    //   block [0x82698E28..0x82698E94)
	// 82698E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698E34: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698E38: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698E3C: 38EBEFDC  addi r7, r11, -0x1024
	ctx.r[7].s64 = ctx.r[11].s64 + -4132;
	// 82698E40: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82698E44: 388A6E64  addi r4, r10, 0x6e64
	ctx.r[4].s64 = ctx.r[10].s64 + 28260;
	// 82698E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698E4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698E50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698E58: 386A2C30  addi r3, r10, 0x2c30
	ctx.r[3].s64 = ctx.r[10].s64 + 11312;
	// 82698E5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698E7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698E80: 4BDCDFA1  bl 0x82466e20
	ctx.lr = 0x82698E84;
	sub_82466E20(ctx, base);
	// 82698E84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698E98 size=108
    let mut pc: u32 = 0x82698E98;
    'dispatch: loop {
        match pc {
            0x82698E98 => {
    //   block [0x82698E98..0x82698F04)
	// 82698E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698EA4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698EA8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698EAC: 38EBEFF4  addi r7, r11, -0x100c
	ctx.r[7].s64 = ctx.r[11].s64 + -4108;
	// 82698EB0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82698EB4: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 82698EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698EBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698EC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698EC8: 386A2C60  addi r3, r10, 0x2c60
	ctx.r[3].s64 = ctx.r[10].s64 + 11360;
	// 82698ECC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698ED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698ED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698ED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698EE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698EEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698EF0: 4BDCDF31  bl 0x82466e20
	ctx.lr = 0x82698EF4;
	sub_82466E20(ctx, base);
	// 82698EF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698F08 size=108
    let mut pc: u32 = 0x82698F08;
    'dispatch: loop {
        match pc {
            0x82698F08 => {
    //   block [0x82698F08..0x82698F74)
	// 82698F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698F14: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698F18: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698F1C: 38EBF028  addi r7, r11, -0xfd8
	ctx.r[7].s64 = ctx.r[11].s64 + -4056;
	// 82698F20: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82698F24: 388A7BB8  addi r4, r10, 0x7bb8
	ctx.r[4].s64 = ctx.r[10].s64 + 31672;
	// 82698F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698F2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698F30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698F38: 386A2C90  addi r3, r10, 0x2c90
	ctx.r[3].s64 = ctx.r[10].s64 + 11408;
	// 82698F3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698F5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698F60: 4BDCDEC1  bl 0x82466e20
	ctx.lr = 0x82698F64;
	sub_82466E20(ctx, base);
	// 82698F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698F78 size=108
    let mut pc: u32 = 0x82698F78;
    'dispatch: loop {
        match pc {
            0x82698F78 => {
    //   block [0x82698F78..0x82698FE4)
	// 82698F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698F84: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698F88: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698F8C: 38EBF0B8  addi r7, r11, -0xf48
	ctx.r[7].s64 = ctx.r[11].s64 + -3912;
	// 82698F90: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82698F94: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 82698F98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698F9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698FA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698FA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698FA8: 386A2CC0  addi r3, r10, 0x2cc0
	ctx.r[3].s64 = ctx.r[10].s64 + 11456;
	// 82698FAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698FB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698FB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698FC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698FC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698FC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698FCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698FD0: 4BDCDE51  bl 0x82466e20
	ctx.lr = 0x82698FD4;
	sub_82466E20(ctx, base);
	// 82698FD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698FD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698FDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698FE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698FE8 size=108
    let mut pc: u32 = 0x82698FE8;
    'dispatch: loop {
        match pc {
            0x82698FE8 => {
    //   block [0x82698FE8..0x82699054)
	// 82698FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698FF4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698FF8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698FFC: 38EBF0D0  addi r7, r11, -0xf30
	ctx.r[7].s64 = ctx.r[11].s64 + -3888;
	// 82699000: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82699004: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 82699008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269900C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699010: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699018: 386A2CF0  addi r3, r10, 0x2cf0
	ctx.r[3].s64 = ctx.r[10].s64 + 11504;
	// 8269901C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699020: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269902C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269903C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699040: 4BDCDDE1  bl 0x82466e20
	ctx.lr = 0x82699044;
	sub_82466E20(ctx, base);
	// 82699044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269904C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699058 size=112
    let mut pc: u32 = 0x82699058;
    'dispatch: loop {
        match pc {
            0x82699058 => {
    //   block [0x82699058..0x826990C8)
	// 82699058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269905C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699064: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82699068: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269906C: 392A97B4  addi r9, r10, -0x684c
	ctx.r[9].s64 = ctx.r[10].s64 + -26700;
	// 82699070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699074: 390BF100  addi r8, r11, -0xf00
	ctx.r[8].s64 = ctx.r[11].s64 + -3840;
	// 82699078: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8269907C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 82699080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699084: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699088: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269908C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699090: 386A2D20  addi r3, r10, 0x2d20
	ctx.r[3].s64 = ctx.r[10].s64 + 11552;
	// 82699094: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82699098: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269909C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826990A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826990A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826990A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826990AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826990B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826990B4: 4BDCDD6D  bl 0x82466e20
	ctx.lr = 0x826990B8;
	sub_82466E20(ctx, base);
	// 826990B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826990BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826990C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826990C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826990C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826990C8 size=108
    let mut pc: u32 = 0x826990C8;
    'dispatch: loop {
        match pc {
            0x826990C8 => {
    //   block [0x826990C8..0x82699134)
	// 826990C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826990CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826990D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826990D4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826990D8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826990DC: 38EBF178  addi r7, r11, -0xe88
	ctx.r[7].s64 = ctx.r[11].s64 + -3720;
	// 826990E0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826990E4: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 826990E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826990EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826990F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826990F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826990F8: 386A2D50  addi r3, r10, 0x2d50
	ctx.r[3].s64 = ctx.r[10].s64 + 11600;
	// 826990FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269910C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269911C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699120: 4BDCDD01  bl 0x82466e20
	ctx.lr = 0x82699124;
	sub_82466E20(ctx, base);
	// 82699124: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269912C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82699138 size=24
    let mut pc: u32 = 0x82699138;
    'dispatch: loop {
        match pc {
            0x82699138 => {
    //   block [0x82699138..0x82699150)
	// 82699138: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269913C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82699140: 394A6990  addi r10, r10, 0x6990
	ctx.r[10].s64 = ctx.r[10].s64 + 27024;
	// 82699144: 816BF268  lwz r11, -0xd98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3480 as u32) ) } as u64;
	// 82699148: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8269914C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699150 size=108
    let mut pc: u32 = 0x82699150;
    'dispatch: loop {
        match pc {
            0x82699150 => {
    //   block [0x82699150..0x826991BC)
	// 82699150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269915C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699164: 38EB6990  addi r7, r11, 0x6990
	ctx.r[7].s64 = ctx.r[11].s64 + 27024;
	// 82699168: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269916C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 82699170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699174: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699178: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269917C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699180: 386A2D80  addi r3, r10, 0x2d80
	ctx.r[3].s64 = ctx.r[10].s64 + 11648;
	// 82699184: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269918C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269919C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826991A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826991A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826991A8: 4BDCDC79  bl 0x82466e20
	ctx.lr = 0x826991AC;
	sub_82466E20(ctx, base);
	// 826991AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826991B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826991B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826991B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826991C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826991C0 size=24
    let mut pc: u32 = 0x826991C0;
    'dispatch: loop {
        match pc {
            0x826991C0 => {
    //   block [0x826991C0..0x826991D8)
	// 826991C0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826991C4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826991C8: 394A69C0  addi r10, r10, 0x69c0
	ctx.r[10].s64 = ctx.r[10].s64 + 27072;
	// 826991CC: 816BF268  lwz r11, -0xd98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3480 as u32) ) } as u64;
	// 826991D0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826991D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826991D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826991D8 size=108
    let mut pc: u32 = 0x826991D8;
    'dispatch: loop {
        match pc {
            0x826991D8 => {
    //   block [0x826991D8..0x82699244)
	// 826991D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826991DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826991E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826991E4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826991E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826991EC: 38EB69C0  addi r7, r11, 0x69c0
	ctx.r[7].s64 = ctx.r[11].s64 + 27072;
	// 826991F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826991F4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 826991F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826991FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699200: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699208: 386A2DB0  addi r3, r10, 0x2db0
	ctx.r[3].s64 = ctx.r[10].s64 + 11696;
	// 8269920C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269921C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269922C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699230: 4BDCDBF1  bl 0x82466e20
	ctx.lr = 0x82699234;
	sub_82466E20(ctx, base);
	// 82699234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269923C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699248 size=108
    let mut pc: u32 = 0x82699248;
    'dispatch: loop {
        match pc {
            0x82699248 => {
    //   block [0x82699248..0x826992B4)
	// 82699248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269924C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699254: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269925C: 38EBF250  addi r7, r11, -0xdb0
	ctx.r[7].s64 = ctx.r[11].s64 + -3504;
	// 82699260: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82699264: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 82699268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269926C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699270: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699278: 386A2DE0  addi r3, r10, 0x2de0
	ctx.r[3].s64 = ctx.r[10].s64 + 11744;
	// 8269927C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269928C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269929C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826992A0: 4BDCDB81  bl 0x82466e20
	ctx.lr = 0x826992A4;
	sub_82466E20(ctx, base);
	// 826992A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826992A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826992AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826992B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826992B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826992B8 size=24
    let mut pc: u32 = 0x826992B8;
    'dispatch: loop {
        match pc {
            0x826992B8 => {
    //   block [0x826992B8..0x826992D0)
	// 826992B8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826992BC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826992C0: 394A69F0  addi r10, r10, 0x69f0
	ctx.r[10].s64 = ctx.r[10].s64 + 27120;
	// 826992C4: 816BF268  lwz r11, -0xd98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3480 as u32) ) } as u64;
	// 826992C8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826992CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826992D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826992D0 size=108
    let mut pc: u32 = 0x826992D0;
    'dispatch: loop {
        match pc {
            0x826992D0 => {
    //   block [0x826992D0..0x8269933C)
	// 826992D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826992D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826992D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826992DC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826992E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826992E4: 38EB69F0  addi r7, r11, 0x69f0
	ctx.r[7].s64 = ctx.r[11].s64 + 27120;
	// 826992E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826992EC: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 826992F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826992F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826992F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826992FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699300: 386A2E10  addi r3, r10, 0x2e10
	ctx.r[3].s64 = ctx.r[10].s64 + 11792;
	// 82699304: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269930C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269931C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699324: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699328: 4BDCDAF9  bl 0x82466e20
	ctx.lr = 0x8269932C;
	sub_82466E20(ctx, base);
	// 8269932C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699340 size=112
    let mut pc: u32 = 0x82699340;
    'dispatch: loop {
        match pc {
            0x82699340 => {
    //   block [0x82699340..0x826993B0)
	// 82699340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269934C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82699350: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699354: 392A97F8  addi r9, r10, -0x6808
	ctx.r[9].s64 = ctx.r[10].s64 + -26632;
	// 82699358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269935C: 390BF26C  addi r8, r11, -0xd94
	ctx.r[8].s64 = ctx.r[11].s64 + -3476;
	// 82699360: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82699364: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 82699368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269936C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699370: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82699374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699378: 386A2E40  addi r3, r10, 0x2e40
	ctx.r[3].s64 = ctx.r[10].s64 + 11840;
	// 8269937C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82699380: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82699384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269938C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269939C: 4BDCDA85  bl 0x82466e20
	ctx.lr = 0x826993A0;
	sub_82466E20(ctx, base);
	// 826993A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826993A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826993A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826993AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826993B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826993B0 size=108
    let mut pc: u32 = 0x826993B0;
    'dispatch: loop {
        match pc {
            0x826993B0 => {
    //   block [0x826993B0..0x8269941C)
	// 826993B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826993B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826993B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826993BC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826993C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826993C4: 38EBF29C  addi r7, r11, -0xd64
	ctx.r[7].s64 = ctx.r[11].s64 + -3428;
	// 826993C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826993CC: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 826993D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826993D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826993D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826993DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826993E0: 386A2E70  addi r3, r10, 0x2e70
	ctx.r[3].s64 = ctx.r[10].s64 + 11888;
	// 826993E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826993E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826993EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826993F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826993F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826993F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826993FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699404: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699408: 4BDCDA19  bl 0x82466e20
	ctx.lr = 0x8269940C;
	sub_82466E20(ctx, base);
	// 8269940C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699420 size=108
    let mut pc: u32 = 0x82699420;
    'dispatch: loop {
        match pc {
            0x82699420 => {
    //   block [0x82699420..0x8269948C)
	// 82699420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269942C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699434: 38EBF2CC  addi r7, r11, -0xd34
	ctx.r[7].s64 = ctx.r[11].s64 + -3380;
	// 82699438: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269943C: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 82699440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699444: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699448: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269944C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699450: 386A2EA0  addi r3, r10, 0x2ea0
	ctx.r[3].s64 = ctx.r[10].s64 + 11936;
	// 82699454: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269945C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269946C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699478: 4BDCD9A9  bl 0x82466e20
	ctx.lr = 0x8269947C;
	sub_82466E20(ctx, base);
	// 8269947C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699490 size=108
    let mut pc: u32 = 0x82699490;
    'dispatch: loop {
        match pc {
            0x82699490 => {
    //   block [0x82699490..0x826994FC)
	// 82699490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269949C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826994A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826994A4: 38EBF2E4  addi r7, r11, -0xd1c
	ctx.r[7].s64 = ctx.r[11].s64 + -3356;
	// 826994A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826994AC: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826994B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826994B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826994B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826994BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826994C0: 386A2ED0  addi r3, r10, 0x2ed0
	ctx.r[3].s64 = ctx.r[10].s64 + 11984;
	// 826994C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826994C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826994CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826994D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826994D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826994D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826994DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826994E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826994E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826994E8: 4BDCD939  bl 0x82466e20
	ctx.lr = 0x826994EC;
	sub_82466E20(ctx, base);
	// 826994EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826994F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826994F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826994F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699500 size=112
    let mut pc: u32 = 0x82699500;
    'dispatch: loop {
        match pc {
            0x82699500 => {
    //   block [0x82699500..0x82699570)
	// 82699500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269950C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699510: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699514: 38AA2F30  addi r5, r10, 0x2f30
	ctx.r[5].s64 = ctx.r[10].s64 + 12080;
	// 82699518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269951C: 390BF314  addi r8, r11, -0xcec
	ctx.r[8].s64 = ctx.r[11].s64 + -3308;
	// 82699520: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82699524: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 82699528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269952C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82699534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699538: 386A2F00  addi r3, r10, 0x2f00
	ctx.r[3].s64 = ctx.r[10].s64 + 12032;
	// 8269953C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82699540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269954C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269955C: 4BDCD8C5  bl 0x82466e20
	ctx.lr = 0x82699560;
	sub_82466E20(ctx, base);
	// 82699560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269956C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699570 size=108
    let mut pc: u32 = 0x82699570;
    'dispatch: loop {
        match pc {
            0x82699570 => {
    //   block [0x82699570..0x826995DC)
	// 82699570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269957C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699584: 38EBF32C  addi r7, r11, -0xcd4
	ctx.r[7].s64 = ctx.r[11].s64 + -3284;
	// 82699588: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269958C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 82699590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699594: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699598: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269959C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826995A0: 386A2F30  addi r3, r10, 0x2f30
	ctx.r[3].s64 = ctx.r[10].s64 + 12080;
	// 826995A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826995A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826995AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826995B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826995B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826995B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826995BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826995C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826995C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826995C8: 4BDCD859  bl 0x82466e20
	ctx.lr = 0x826995CC;
	sub_82466E20(ctx, base);
	// 826995CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826995D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826995D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826995D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826995E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826995E0 size=108
    let mut pc: u32 = 0x826995E0;
    'dispatch: loop {
        match pc {
            0x826995E0 => {
    //   block [0x826995E0..0x8269964C)
	// 826995E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826995E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826995E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826995EC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826995F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826995F4: 38EBF35C  addi r7, r11, -0xca4
	ctx.r[7].s64 = ctx.r[11].s64 + -3236;
	// 826995F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826995FC: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 82699600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699604: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269960C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699610: 386A2F60  addi r3, r10, 0x2f60
	ctx.r[3].s64 = ctx.r[10].s64 + 12128;
	// 82699614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269961C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269962C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699638: 4BDCD7E9  bl 0x82466e20
	ctx.lr = 0x8269963C;
	sub_82466E20(ctx, base);
	// 8269963C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699650 size=108
    let mut pc: u32 = 0x82699650;
    'dispatch: loop {
        match pc {
            0x82699650 => {
    //   block [0x82699650..0x826996BC)
	// 82699650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269965C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699664: 38EBF374  addi r7, r11, -0xc8c
	ctx.r[7].s64 = ctx.r[11].s64 + -3212;
	// 82699668: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269966C: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 82699670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699674: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269967C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699680: 386A2F90  addi r3, r10, 0x2f90
	ctx.r[3].s64 = ctx.r[10].s64 + 12176;
	// 82699684: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269968C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269969C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826996A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826996A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826996A8: 4BDCD779  bl 0x82466e20
	ctx.lr = 0x826996AC;
	sub_82466E20(ctx, base);
	// 826996AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826996B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826996B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826996B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826996C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826996C0 size=108
    let mut pc: u32 = 0x826996C0;
    'dispatch: loop {
        match pc {
            0x826996C0 => {
    //   block [0x826996C0..0x8269972C)
	// 826996C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826996C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826996C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826996CC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826996D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826996D4: 38EBF3A8  addi r7, r11, -0xc58
	ctx.r[7].s64 = ctx.r[11].s64 + -3160;
	// 826996D8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826996DC: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826996E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826996E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826996E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826996EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826996F0: 386A2FC0  addi r3, r10, 0x2fc0
	ctx.r[3].s64 = ctx.r[10].s64 + 12224;
	// 826996F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826996F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826996FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269970C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699718: 4BDCD709  bl 0x82466e20
	ctx.lr = 0x8269971C;
	sub_82466E20(ctx, base);
	// 8269971C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699730 size=108
    let mut pc: u32 = 0x82699730;
    'dispatch: loop {
        match pc {
            0x82699730 => {
    //   block [0x82699730..0x8269979C)
	// 82699730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269973C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699744: 38EBF450  addi r7, r11, -0xbb0
	ctx.r[7].s64 = ctx.r[11].s64 + -2992;
	// 82699748: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269974C: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 82699750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699754: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699758: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269975C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699760: 386A2FF0  addi r3, r10, 0x2ff0
	ctx.r[3].s64 = ctx.r[10].s64 + 12272;
	// 82699764: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269976C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269977C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699784: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699788: 4BDCD699  bl 0x82466e20
	ctx.lr = 0x8269978C;
	sub_82466E20(ctx, base);
	// 8269978C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826997A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826997A0 size=108
    let mut pc: u32 = 0x826997A0;
    'dispatch: loop {
        match pc {
            0x826997A0 => {
    //   block [0x826997A0..0x8269980C)
	// 826997A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826997A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826997A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826997AC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826997B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826997B4: 38EBF480  addi r7, r11, -0xb80
	ctx.r[7].s64 = ctx.r[11].s64 + -2944;
	// 826997B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826997BC: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826997C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826997C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826997C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826997CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826997D0: 386A3020  addi r3, r10, 0x3020
	ctx.r[3].s64 = ctx.r[10].s64 + 12320;
	// 826997D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826997D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826997DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826997E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826997E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826997E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826997EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826997F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826997F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826997F8: 4BDCD629  bl 0x82466e20
	ctx.lr = 0x826997FC;
	sub_82466E20(ctx, base);
	// 826997FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699810 size=108
    let mut pc: u32 = 0x82699810;
    'dispatch: loop {
        match pc {
            0x82699810 => {
    //   block [0x82699810..0x8269987C)
	// 82699810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269981C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699824: 38EBF498  addi r7, r11, -0xb68
	ctx.r[7].s64 = ctx.r[11].s64 + -2920;
	// 82699828: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269982C: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 82699830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699834: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699838: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269983C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699840: 386A3050  addi r3, r10, 0x3050
	ctx.r[3].s64 = ctx.r[10].s64 + 12368;
	// 82699844: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269984C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269985C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699864: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699868: 4BDCD5B9  bl 0x82466e20
	ctx.lr = 0x8269986C;
	sub_82466E20(ctx, base);
	// 8269986C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699880 size=112
    let mut pc: u32 = 0x82699880;
    'dispatch: loop {
        match pc {
            0x82699880 => {
    //   block [0x82699880..0x826998F0)
	// 82699880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269988C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699890: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699894: 38AA2EA0  addi r5, r10, 0x2ea0
	ctx.r[5].s64 = ctx.r[10].s64 + 11936;
	// 82699898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269989C: 390BF4C8  addi r8, r11, -0xb38
	ctx.r[8].s64 = ctx.r[11].s64 + -2872;
	// 826998A0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826998A4: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826998A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826998AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826998B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826998B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826998B8: 386A3080  addi r3, r10, 0x3080
	ctx.r[3].s64 = ctx.r[10].s64 + 12416;
	// 826998BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826998C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826998C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826998C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826998CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826998D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826998D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826998D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826998DC: 4BDCD545  bl 0x82466e20
	ctx.lr = 0x826998E0;
	sub_82466E20(ctx, base);
	// 826998E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826998E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826998E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826998EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826998F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826998F0 size=24
    let mut pc: u32 = 0x826998F0;
    'dispatch: loop {
        match pc {
            0x826998F0 => {
    //   block [0x826998F0..0x82699908)
	// 826998F0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826998F4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826998F8: 394A6A20  addi r10, r10, 0x6a20
	ctx.r[10].s64 = ctx.r[10].s64 + 27168;
	// 826998FC: 816BF3A4  lwz r11, -0xc5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3164 as u32) ) } as u64;
	// 82699900: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82699904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699908 size=112
    let mut pc: u32 = 0x82699908;
    'dispatch: loop {
        match pc {
            0x82699908 => {
    //   block [0x82699908..0x82699978)
	// 82699908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269990C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699914: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82699918: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269991C: 392A9824  addi r9, r10, -0x67dc
	ctx.r[9].s64 = ctx.r[10].s64 + -26588;
	// 82699920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699924: 390B6A20  addi r8, r11, 0x6a20
	ctx.r[8].s64 = ctx.r[11].s64 + 27168;
	// 82699928: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8269992C: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 82699930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699934: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269993C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699940: 386A30B0  addi r3, r10, 0x30b0
	ctx.r[3].s64 = ctx.r[10].s64 + 12464;
	// 82699944: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82699948: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269994C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269995C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699964: 4BDCD4BD  bl 0x82466e20
	ctx.lr = 0x82699968;
	sub_82466E20(ctx, base);
	// 82699968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269996C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699978 size=108
    let mut pc: u32 = 0x82699978;
    'dispatch: loop {
        match pc {
            0x82699978 => {
    //   block [0x82699978..0x826999E4)
	// 82699978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269997C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699984: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269998C: 38EBF574  addi r7, r11, -0xa8c
	ctx.r[7].s64 = ctx.r[11].s64 + -2700;
	// 82699990: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82699994: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 82699998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269999C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826999A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826999A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826999A8: 386A30E0  addi r3, r10, 0x30e0
	ctx.r[3].s64 = ctx.r[10].s64 + 12512;
	// 826999AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826999B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826999B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826999B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826999BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826999C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826999C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826999C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826999CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826999D0: 4BDCD451  bl 0x82466e20
	ctx.lr = 0x826999D4;
	sub_82466E20(ctx, base);
	// 826999D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826999D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826999DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826999E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826999E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826999E8 size=116
    let mut pc: u32 = 0x826999E8;
    'dispatch: loop {
        match pc {
            0x826999E8 => {
    //   block [0x826999E8..0x82699A5C)
	// 826999E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826999EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826999F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826999F4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826999F8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826999FC: 390BF5A8  addi r8, r11, -0xa58
	ctx.r[8].s64 = ctx.r[11].s64 + -2648;
	// 82699A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699A04: 392A9868  addi r9, r10, -0x6798
	ctx.r[9].s64 = ctx.r[10].s64 + -26520;
	// 82699A08: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699A0C: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82699A10: 38AA2EA0  addi r5, r10, 0x2ea0
	ctx.r[5].s64 = ctx.r[10].s64 + 11936;
	// 82699A14: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82699A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699A1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699A2C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82699A30: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 82699A34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82699A38: 386B3110  addi r3, r11, 0x3110
	ctx.r[3].s64 = ctx.r[11].s64 + 12560;
	// 82699A3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82699A40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699A44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699A48: 4BDCD3D9  bl 0x82466e20
	ctx.lr = 0x82699A4C;
	sub_82466E20(ctx, base);
	// 82699A4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699A50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699A54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82699A60 size=24
    let mut pc: u32 = 0x82699A60;
    'dispatch: loop {
        match pc {
            0x82699A60 => {
    //   block [0x82699A60..0x82699A78)
	// 82699A60: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699A64: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82699A68: 394A6A98  addi r10, r10, 0x6a98
	ctx.r[10].s64 = ctx.r[10].s64 + 27288;
	// 82699A6C: 816BF5A4  lwz r11, -0xa5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2652 as u32) ) } as u64;
	// 82699A70: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82699A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699A78 size=112
    let mut pc: u32 = 0x82699A78;
    'dispatch: loop {
        match pc {
            0x82699A78 => {
    //   block [0x82699A78..0x82699AE8)
	// 82699A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699A84: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82699A88: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699A8C: 392A98A4  addi r9, r10, -0x675c
	ctx.r[9].s64 = ctx.r[10].s64 + -26460;
	// 82699A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699A94: 390B6A98  addi r8, r11, 0x6a98
	ctx.r[8].s64 = ctx.r[11].s64 + 27288;
	// 82699A98: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82699A9C: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 82699AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699AA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82699AAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699AB0: 386A3140  addi r3, r10, 0x3140
	ctx.r[3].s64 = ctx.r[10].s64 + 12608;
	// 82699AB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82699AB8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82699ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699ACC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699AD4: 4BDCD34D  bl 0x82466e20
	ctx.lr = 0x82699AD8;
	sub_82466E20(ctx, base);
	// 82699AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


