pub fn sub_826E0680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0680 size=108
    let mut pc: u32 = 0x826E0680;
    'dispatch: loop {
        match pc {
            0x826E0680 => {
    //   block [0x826E0680..0x826E06EC)
	// 826E0680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E068C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0690: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0694: 38EBC598  addi r7, r11, -0x3a68
	ctx.r[7].s64 = ctx.r[11].s64 + -14952;
	// 826E0698: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E069C: 388AA700  addi r4, r10, -0x5900
	ctx.r[4].s64 = ctx.r[10].s64 + -22784;
	// 826E06A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E06A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E06A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E06AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E06B0: 386A0D6C  addi r3, r10, 0xd6c
	ctx.r[3].s64 = ctx.r[10].s64 + 3436;
	// 826E06B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E06B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E06BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E06C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E06C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E06C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E06CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E06D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E06D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E06D8: 4BD86749  bl 0x82466e20
	ctx.lr = 0x826E06DC;
	sub_82466E20(ctx, base);
	// 826E06DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E06E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E06E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E06E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E06F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E06F0 size=108
    let mut pc: u32 = 0x826E06F0;
    'dispatch: loop {
        match pc {
            0x826E06F0 => {
    //   block [0x826E06F0..0x826E075C)
	// 826E06F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E06F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E06F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E06FC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0700: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0704: 38EBC5B0  addi r7, r11, -0x3a50
	ctx.r[7].s64 = ctx.r[11].s64 + -14928;
	// 826E0708: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E070C: 388AA718  addi r4, r10, -0x58e8
	ctx.r[4].s64 = ctx.r[10].s64 + -22760;
	// 826E0710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0714: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0718: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E071C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0720: 386A0D9C  addi r3, r10, 0xd9c
	ctx.r[3].s64 = ctx.r[10].s64 + 3484;
	// 826E0724: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E072C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E073C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0744: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0748: 4BD866D9  bl 0x82466e20
	ctx.lr = 0x826E074C;
	sub_82466E20(ctx, base);
	// 826E074C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0760 size=116
    let mut pc: u32 = 0x826E0760;
    'dispatch: loop {
        match pc {
            0x826E0760 => {
    //   block [0x826E0760..0x826E07D4)
	// 826E0760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E076C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0770: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E0774: 390BC614  addi r8, r11, -0x39ec
	ctx.r[8].s64 = ctx.r[11].s64 + -14828;
	// 826E0778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E077C: 392A6954  addi r9, r10, 0x6954
	ctx.r[9].s64 = ctx.r[10].s64 + 26964;
	// 826E0780: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0784: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826E0788: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E078C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E0790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0794: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E079C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E07A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E07A4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E07A8: 388AA728  addi r4, r10, -0x58d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22744;
	// 826E07AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E07B0: 386B0DCC  addi r3, r11, 0xdcc
	ctx.r[3].s64 = ctx.r[11].s64 + 3532;
	// 826E07B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E07B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E07BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E07C0: 4BD86661  bl 0x82466e20
	ctx.lr = 0x826E07C4;
	sub_82466E20(ctx, base);
	// 826E07C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E07C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E07CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E07D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E07D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E07D8 size=108
    let mut pc: u32 = 0x826E07D8;
    'dispatch: loop {
        match pc {
            0x826E07D8 => {
    //   block [0x826E07D8..0x826E0844)
	// 826E07D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E07DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E07E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E07E4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E07E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E07EC: 38EBC630  addi r7, r11, -0x39d0
	ctx.r[7].s64 = ctx.r[11].s64 + -14800;
	// 826E07F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826E07F4: 388AA73C  addi r4, r10, -0x58c4
	ctx.r[4].s64 = ctx.r[10].s64 + -22724;
	// 826E07F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E07FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0800: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E0804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0808: 386A0DFC  addi r3, r10, 0xdfc
	ctx.r[3].s64 = ctx.r[10].s64 + 3580;
	// 826E080C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E081C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E082C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0830: 4BD865F1  bl 0x82466e20
	ctx.lr = 0x826E0834;
	sub_82466E20(ctx, base);
	// 826E0834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E083C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0848 size=108
    let mut pc: u32 = 0x826E0848;
    'dispatch: loop {
        match pc {
            0x826E0848 => {
    //   block [0x826E0848..0x826E08B4)
	// 826E0848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E084C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0854: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0858: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E085C: 38EBC678  addi r7, r11, -0x3988
	ctx.r[7].s64 = ctx.r[11].s64 + -14728;
	// 826E0860: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826E0864: 388AA760  addi r4, r10, -0x58a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22688;
	// 826E0868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E086C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0870: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E0874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0878: 386A0E2C  addi r3, r10, 0xe2c
	ctx.r[3].s64 = ctx.r[10].s64 + 3628;
	// 826E087C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E088C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E089C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E08A0: 4BD86581  bl 0x82466e20
	ctx.lr = 0x826E08A4;
	sub_82466E20(ctx, base);
	// 826E08A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E08A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E08AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E08B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E08B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E08B8 size=108
    let mut pc: u32 = 0x826E08B8;
    'dispatch: loop {
        match pc {
            0x826E08B8 => {
    //   block [0x826E08B8..0x826E0924)
	// 826E08B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E08BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E08C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E08C4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E08C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E08CC: 38EBC708  addi r7, r11, -0x38f8
	ctx.r[7].s64 = ctx.r[11].s64 + -14584;
	// 826E08D0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826E08D4: 388AA784  addi r4, r10, -0x587c
	ctx.r[4].s64 = ctx.r[10].s64 + -22652;
	// 826E08D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E08DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E08E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E08E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E08E8: 386A0E5C  addi r3, r10, 0xe5c
	ctx.r[3].s64 = ctx.r[10].s64 + 3676;
	// 826E08EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E08F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E08F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E08F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E08FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E090C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0910: 4BD86511  bl 0x82466e20
	ctx.lr = 0x826E0914;
	sub_82466E20(ctx, base);
	// 826E0914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E091C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0928 size=100
    let mut pc: u32 = 0x826E0928;
    'dispatch: loop {
        match pc {
            0x826E0928 => {
    //   block [0x826E0928..0x826E098C)
	// 826E0928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E092C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0934: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E093C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E0940: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0948: 388AA79C  addi r4, r10, -0x5864
	ctx.r[4].s64 = ctx.r[10].s64 + -22628;
	// 826E094C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E095C: 386A0E8C  addi r3, r10, 0xe8c
	ctx.r[3].s64 = ctx.r[10].s64 + 3724;
	// 826E0960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0964: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0968: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E096C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0970: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E0974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0978: 4BD864A9  bl 0x82466e20
	ctx.lr = 0x826E097C;
	sub_82466E20(ctx, base);
	// 826E097C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0990 size=112
    let mut pc: u32 = 0x826E0990;
    'dispatch: loop {
        match pc {
            0x826E0990 => {
    //   block [0x826E0990..0x826E0A00)
	// 826E0990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E099C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E09A0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E09A4: 38AA0E8C  addi r5, r10, 0xe8c
	ctx.r[5].s64 = ctx.r[10].s64 + 3724;
	// 826E09A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E09AC: 390BC798  addi r8, r11, -0x3868
	ctx.r[8].s64 = ctx.r[11].s64 + -14440;
	// 826E09B0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826E09B4: 388AA7B8  addi r4, r10, -0x5848
	ctx.r[4].s64 = ctx.r[10].s64 + -22600;
	// 826E09B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E09BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E09C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E09C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E09C8: 386A0EBC  addi r3, r10, 0xebc
	ctx.r[3].s64 = ctx.r[10].s64 + 3772;
	// 826E09CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E09D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E09D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E09D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E09DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E09E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E09E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E09E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E09EC: 4BD86435  bl 0x82466e20
	ctx.lr = 0x826E09F0;
	sub_82466E20(ctx, base);
	// 826E09F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E09F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E09F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E09FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0A00 size=108
    let mut pc: u32 = 0x826E0A00;
    'dispatch: loop {
        match pc {
            0x826E0A00 => {
    //   block [0x826E0A00..0x826E0A6C)
	// 826E0A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0A0C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0A10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0A14: 38EBC7F8  addi r7, r11, -0x3808
	ctx.r[7].s64 = ctx.r[11].s64 + -14344;
	// 826E0A18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E0A1C: 388AA7DC  addi r4, r10, -0x5824
	ctx.r[4].s64 = ctx.r[10].s64 + -22564;
	// 826E0A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0A24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0A28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E0A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0A30: 386A0EEC  addi r3, r10, 0xeec
	ctx.r[3].s64 = ctx.r[10].s64 + 3820;
	// 826E0A34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0A3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0A54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0A58: 4BD863C9  bl 0x82466e20
	ctx.lr = 0x826E0A5C;
	sub_82466E20(ctx, base);
	// 826E0A5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0A70 size=108
    let mut pc: u32 = 0x826E0A70;
    'dispatch: loop {
        match pc {
            0x826E0A70 => {
    //   block [0x826E0A70..0x826E0ADC)
	// 826E0A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0A7C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0A80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0A84: 38EBC828  addi r7, r11, -0x37d8
	ctx.r[7].s64 = ctx.r[11].s64 + -14296;
	// 826E0A88: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E0A8C: 388AA7E4  addi r4, r10, -0x581c
	ctx.r[4].s64 = ctx.r[10].s64 + -22556;
	// 826E0A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0A94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0A98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E0A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0AA0: 386A0F1C  addi r3, r10, 0xf1c
	ctx.r[3].s64 = ctx.r[10].s64 + 3868;
	// 826E0AA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0AC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0AC8: 4BD86359  bl 0x82466e20
	ctx.lr = 0x826E0ACC;
	sub_82466E20(ctx, base);
	// 826E0ACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0AE0 size=108
    let mut pc: u32 = 0x826E0AE0;
    'dispatch: loop {
        match pc {
            0x826E0AE0 => {
    //   block [0x826E0AE0..0x826E0B4C)
	// 826E0AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0AEC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0AF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0AF4: 38EBC888  addi r7, r11, -0x3778
	ctx.r[7].s64 = ctx.r[11].s64 + -14200;
	// 826E0AF8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E0AFC: 388AA7F8  addi r4, r10, -0x5808
	ctx.r[4].s64 = ctx.r[10].s64 + -22536;
	// 826E0B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0B04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0B08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E0B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0B10: 386A0F4C  addi r3, r10, 0xf4c
	ctx.r[3].s64 = ctx.r[10].s64 + 3916;
	// 826E0B14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0B1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0B34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0B38: 4BD862E9  bl 0x82466e20
	ctx.lr = 0x826E0B3C;
	sub_82466E20(ctx, base);
	// 826E0B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0B50 size=112
    let mut pc: u32 = 0x826E0B50;
    'dispatch: loop {
        match pc {
            0x826E0B50 => {
    //   block [0x826E0B50..0x826E0BC0)
	// 826E0B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0B5C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E0B60: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826E0B64: 38EAC8E8  addi r7, r10, -0x3718
	ctx.r[7].s64 = ctx.r[10].s64 + -14104;
	// 826E0B68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0B6C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E0B70: 388AA804  addi r4, r10, -0x57fc
	ctx.r[4].s64 = ctx.r[10].s64 + -22524;
	// 826E0B74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0B78: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0B7C: 396B6968  addi r11, r11, 0x6968
	ctx.r[11].s64 = ctx.r[11].s64 + 26984;
	// 826E0B80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E0B84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0B88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0B8C: 386A0F7C  addi r3, r10, 0xf7c
	ctx.r[3].s64 = ctx.r[10].s64 + 3964;
	// 826E0B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0B94: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E0B98: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0B9C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E0BA0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0BA4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0BA8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0BAC: 4BD86275  bl 0x82466e20
	ctx.lr = 0x826E0BB0;
	sub_82466E20(ctx, base);
	// 826E0BB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0BC0 size=96
    let mut pc: u32 = 0x826E0BC0;
    'dispatch: loop {
        match pc {
            0x826E0BC0 => {
    //   block [0x826E0BC0..0x826E0C20)
	// 826E0BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0BCC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0BD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0BD4: 388AA82C  addi r4, r10, -0x57d4
	ctx.r[4].s64 = ctx.r[10].s64 + -22484;
	// 826E0BD8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0BDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0BE0: 386A0FAC  addi r3, r10, 0xfac
	ctx.r[3].s64 = ctx.r[10].s64 + 4012;
	// 826E0BE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0BEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0BF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0BFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0C00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E0C04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0C08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E0C0C: 4BD86215  bl 0x82466e20
	ctx.lr = 0x826E0C10;
	sub_82466E20(ctx, base);
	// 826E0C10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0C14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0C18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0C1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0C20 size=112
    let mut pc: u32 = 0x826E0C20;
    'dispatch: loop {
        match pc {
            0x826E0C20 => {
    //   block [0x826E0C20..0x826E0C90)
	// 826E0C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0C28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0C2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0C30: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0C34: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E0C38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0C3C: 390BCA08  addi r8, r11, -0x35f8
	ctx.r[8].s64 = ctx.r[11].s64 + -13816;
	// 826E0C40: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826E0C44: 388AA848  addi r4, r10, -0x57b8
	ctx.r[4].s64 = ctx.r[10].s64 + -22456;
	// 826E0C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0C4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0C50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E0C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0C58: 386A0FDC  addi r3, r10, 0xfdc
	ctx.r[3].s64 = ctx.r[10].s64 + 4060;
	// 826E0C5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E0C60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0C64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0C68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0C6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0C70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0C74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0C78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0C7C: 4BD861A5  bl 0x82466e20
	ctx.lr = 0x826E0C80;
	sub_82466E20(ctx, base);
	// 826E0C80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E0C90 size=24
    let mut pc: u32 = 0x826E0C90;
    'dispatch: loop {
        match pc {
            0x826E0C90 => {
    //   block [0x826E0C90..0x826E0CA8)
	// 826E0C90: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0C94: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E0C98: 394A5988  addi r10, r10, 0x5988
	ctx.r[10].s64 = ctx.r[10].s64 + 22920;
	// 826E0C9C: 816BC62C  lwz r11, -0x39d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14804 as u32) ) } as u64;
	// 826E0CA0: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 826E0CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0CA8 size=116
    let mut pc: u32 = 0x826E0CA8;
    'dispatch: loop {
        match pc {
            0x826E0CA8 => {
    //   block [0x826E0CA8..0x826E0D1C)
	// 826E0CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0CB4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0CB8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E0CBC: 390B5988  addi r8, r11, 0x5988
	ctx.r[8].s64 = ctx.r[11].s64 + 22920;
	// 826E0CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0CC4: 392A69E4  addi r9, r10, 0x69e4
	ctx.r[9].s64 = ctx.r[10].s64 + 27108;
	// 826E0CC8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0CCC: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826E0CD0: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E0CD4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E0CD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0CDC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E0CE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0CE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0CEC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E0CF0: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 826E0CF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E0CF8: 386B100C  addi r3, r11, 0x100c
	ctx.r[3].s64 = ctx.r[11].s64 + 4108;
	// 826E0CFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E0D00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0D04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0D08: 4BD86119  bl 0x82466e20
	ctx.lr = 0x826E0D0C;
	sub_82466E20(ctx, base);
	// 826E0D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E0D20 size=24
    let mut pc: u32 = 0x826E0D20;
    'dispatch: loop {
        match pc {
            0x826E0D20 => {
    //   block [0x826E0D20..0x826E0D38)
	// 826E0D20: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0D24: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E0D28: 394A5A60  addi r10, r10, 0x5a60
	ctx.r[10].s64 = ctx.r[10].s64 + 23136;
	// 826E0D2C: 816BCA70  lwz r11, -0x3590(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13712 as u32) ) } as u64;
	// 826E0D30: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826E0D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0D38 size=116
    let mut pc: u32 = 0x826E0D38;
    'dispatch: loop {
        match pc {
            0x826E0D38 => {
    //   block [0x826E0D38..0x826E0DAC)
	// 826E0D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0D44: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0D48: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E0D4C: 390B5A60  addi r8, r11, 0x5a60
	ctx.r[8].s64 = ctx.r[11].s64 + 23136;
	// 826E0D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0D54: 392A6A98  addi r9, r10, 0x6a98
	ctx.r[9].s64 = ctx.r[10].s64 + 27288;
	// 826E0D58: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0D5C: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 826E0D60: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E0D64: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E0D68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0D6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0D70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0D78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0D7C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E0D80: 388AA870  addi r4, r10, -0x5790
	ctx.r[4].s64 = ctx.r[10].s64 + -22416;
	// 826E0D84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E0D88: 386B103C  addi r3, r11, 0x103c
	ctx.r[3].s64 = ctx.r[11].s64 + 4156;
	// 826E0D8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E0D90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0D94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0D98: 4BD86089  bl 0x82466e20
	ctx.lr = 0x826E0D9C;
	sub_82466E20(ctx, base);
	// 826E0D9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0DB0 size=112
    let mut pc: u32 = 0x826E0DB0;
    'dispatch: loop {
        match pc {
            0x826E0DB0 => {
    //   block [0x826E0DB0..0x826E0E20)
	// 826E0DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0DB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0DBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0DC0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0DC4: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E0DC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0DCC: 390BCA78  addi r8, r11, -0x3588
	ctx.r[8].s64 = ctx.r[11].s64 + -13704;
	// 826E0DD0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E0DD4: 388AA884  addi r4, r10, -0x577c
	ctx.r[4].s64 = ctx.r[10].s64 + -22396;
	// 826E0DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0DDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0DE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E0DE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0DE8: 386A106C  addi r3, r10, 0x106c
	ctx.r[3].s64 = ctx.r[10].s64 + 4204;
	// 826E0DEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E0DF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0E0C: 4BD86015  bl 0x82466e20
	ctx.lr = 0x826E0E10;
	sub_82466E20(ctx, base);
	// 826E0E10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0E20 size=112
    let mut pc: u32 = 0x826E0E20;
    'dispatch: loop {
        match pc {
            0x826E0E20 => {
    //   block [0x826E0E20..0x826E0E90)
	// 826E0E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0E2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0E30: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0E34: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E0E38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E0E3C: 390BCAC0  addi r8, r11, -0x3540
	ctx.r[8].s64 = ctx.r[11].s64 + -13632;
	// 826E0E40: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E0E44: 388AA89C  addi r4, r10, -0x5764
	ctx.r[4].s64 = ctx.r[10].s64 + -22372;
	// 826E0E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0E4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0E50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E0E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0E58: 386A109C  addi r3, r10, 0x109c
	ctx.r[3].s64 = ctx.r[10].s64 + 4252;
	// 826E0E5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E0E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0E7C: 4BD85FA5  bl 0x82466e20
	ctx.lr = 0x826E0E80;
	sub_82466E20(ctx, base);
	// 826E0E80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0E90 size=108
    let mut pc: u32 = 0x826E0E90;
    'dispatch: loop {
        match pc {
            0x826E0E90 => {
    //   block [0x826E0E90..0x826E0EFC)
	// 826E0E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0E9C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0EA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E0EA4: 38EBCB08  addi r7, r11, -0x34f8
	ctx.r[7].s64 = ctx.r[11].s64 + -13560;
	// 826E0EA8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E0EAC: 388A2DD8  addi r4, r10, 0x2dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 11736;
	// 826E0EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0EB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0EB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E0EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0EC0: 386A10CC  addi r3, r10, 0x10cc
	ctx.r[3].s64 = ctx.r[10].s64 + 4300;
	// 826E0EC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0EC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0EE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0EE8: 4BD85F39  bl 0x82466e20
	ctx.lr = 0x826E0EEC;
	sub_82466E20(ctx, base);
	// 826E0EEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0F00 size=108
    let mut pc: u32 = 0x826E0F00;
    'dispatch: loop {
        match pc {
            0x826E0F00 => {
    //   block [0x826E0F00..0x826E0F6C)
	// 826E0F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0F0C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0F10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E0F14: 38EBCB68  addi r7, r11, -0x3498
	ctx.r[7].s64 = ctx.r[11].s64 + -13464;
	// 826E0F18: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826E0F1C: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 826E0F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E0F24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0F28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E0F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E0F30: 386A10FC  addi r3, r10, 0x10fc
	ctx.r[3].s64 = ctx.r[10].s64 + 4348;
	// 826E0F34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E0F38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E0F3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0F40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E0F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0F48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E0F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0F50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E0F54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0F58: 4BD85EC9  bl 0x82466e20
	ctx.lr = 0x826E0F5C;
	sub_82466E20(ctx, base);
	// 826E0F5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0F70 size=112
    let mut pc: u32 = 0x826E0F70;
    'dispatch: loop {
        match pc {
            0x826E0F70 => {
    //   block [0x826E0F70..0x826E0FE0)
	// 826E0F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0F7C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E0F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E0F84: 392B6ACC  addi r9, r11, 0x6acc
	ctx.r[9].s64 = ctx.r[11].s64 + 27340;
	// 826E0F88: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826E0F8C: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826E0F90: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E0F94: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 826E0F98: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E0F9C: 396BCC10  addi r11, r11, -0x33f0
	ctx.r[11].s64 = ctx.r[11].s64 + -13296;
	// 826E0FA0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826E0FA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0FA8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826E0FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E0FB0: 386A112C  addi r3, r10, 0x112c
	ctx.r[3].s64 = ctx.r[10].s64 + 4396;
	// 826E0FB4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E0FB8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826E0FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E0FC0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826E0FC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E0FC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E0FCC: 4BD85E55  bl 0x82466e20
	ctx.lr = 0x826E0FD0;
	sub_82466E20(ctx, base);
	// 826E0FD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E0FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E0FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E0FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E0FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E0FE0 size=116
    let mut pc: u32 = 0x826E0FE0;
    'dispatch: loop {
        match pc {
            0x826E0FE0 => {
    //   block [0x826E0FE0..0x826E1054)
	// 826E0FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E0FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E0FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E0FEC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E0FF0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826E0FF4: 390ACD60  addi r8, r10, -0x32a0
	ctx.r[8].s64 = ctx.r[10].s64 + -12960;
	// 826E0FF8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E0FFC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1000: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E1004: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E1008: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E100C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1010: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1014: 388AA8B8  addi r4, r10, -0x5748
	ctx.r[4].s64 = ctx.r[10].s64 + -22344;
	// 826E1018: 396B6B20  addi r11, r11, 0x6b20
	ctx.r[11].s64 = ctx.r[11].s64 + 27424;
	// 826E101C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1020: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1024: 386A115C  addi r3, r10, 0x115c
	ctx.r[3].s64 = ctx.r[10].s64 + 4444;
	// 826E1028: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E102C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1030: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E1034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E103C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1040: 4BD85DE1  bl 0x82466e20
	ctx.lr = 0x826E1044;
	sub_82466E20(ctx, base);
	// 826E1044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E104C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1058 size=112
    let mut pc: u32 = 0x826E1058;
    'dispatch: loop {
        match pc {
            0x826E1058 => {
    //   block [0x826E1058..0x826E10C8)
	// 826E1058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E105C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1064: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1068: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E106C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E1070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1074: 390BCDF0  addi r8, r11, -0x3210
	ctx.r[8].s64 = ctx.r[11].s64 + -12816;
	// 826E1078: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826E107C: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 826E1080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1084: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1088: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E108C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1090: 386A118C  addi r3, r10, 0x118c
	ctx.r[3].s64 = ctx.r[10].s64 + 4492;
	// 826E1094: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E1098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E109C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E10A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E10A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E10A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E10AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E10B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E10B4: 4BD85D6D  bl 0x82466e20
	ctx.lr = 0x826E10B8;
	sub_82466E20(ctx, base);
	// 826E10B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E10BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E10C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E10C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E10C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E10C8 size=112
    let mut pc: u32 = 0x826E10C8;
    'dispatch: loop {
        match pc {
            0x826E10C8 => {
    //   block [0x826E10C8..0x826E1138)
	// 826E10C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E10CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E10D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E10D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E10D8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E10DC: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E10E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E10E4: 390BCE98  addi r8, r11, -0x3168
	ctx.r[8].s64 = ctx.r[11].s64 + -12648;
	// 826E10E8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826E10EC: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 826E10F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E10F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E10F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E10FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1100: 386A11BC  addi r3, r10, 0x11bc
	ctx.r[3].s64 = ctx.r[10].s64 + 4540;
	// 826E1104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E1108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E110C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E111C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1124: 4BD85CFD  bl 0x82466e20
	ctx.lr = 0x826E1128;
	sub_82466E20(ctx, base);
	// 826E1128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E112C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1138 size=112
    let mut pc: u32 = 0x826E1138;
    'dispatch: loop {
        match pc {
            0x826E1138 => {
    //   block [0x826E1138..0x826E11A8)
	// 826E1138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E113C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1144: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1148: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E114C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E1150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1154: 390BCF28  addi r8, r11, -0x30d8
	ctx.r[8].s64 = ctx.r[11].s64 + -12504;
	// 826E1158: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826E115C: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 826E1160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1164: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E116C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1170: 386A11EC  addi r3, r10, 0x11ec
	ctx.r[3].s64 = ctx.r[10].s64 + 4588;
	// 826E1174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E1178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E117C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E118C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1194: 4BD85C8D  bl 0x82466e20
	ctx.lr = 0x826E1198;
	sub_82466E20(ctx, base);
	// 826E1198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E119C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E11A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E11A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E11A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E11A8 size=108
    let mut pc: u32 = 0x826E11A8;
    'dispatch: loop {
        match pc {
            0x826E11A8 => {
    //   block [0x826E11A8..0x826E1214)
	// 826E11A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E11AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E11B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E11B4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E11B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E11BC: 38EBCFD0  addi r7, r11, -0x3030
	ctx.r[7].s64 = ctx.r[11].s64 + -12336;
	// 826E11C0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826E11C4: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 826E11C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E11CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E11D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E11D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E11D8: 386A121C  addi r3, r10, 0x121c
	ctx.r[3].s64 = ctx.r[10].s64 + 4636;
	// 826E11DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E11E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E11E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E11E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E11EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E11F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E11F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E11F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E11FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1200: 4BD85C21  bl 0x82466e20
	ctx.lr = 0x826E1204;
	sub_82466E20(ctx, base);
	// 826E1204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E120C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1218 size=112
    let mut pc: u32 = 0x826E1218;
    'dispatch: loop {
        match pc {
            0x826E1218 => {
    //   block [0x826E1218..0x826E1288)
	// 826E1218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E121C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1224: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E1228: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E122C: 392A6BE0  addi r9, r10, 0x6be0
	ctx.r[9].s64 = ctx.r[10].s64 + 27616;
	// 826E1230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1234: 390BD07C  addi r8, r11, -0x2f84
	ctx.r[8].s64 = ctx.r[11].s64 + -12164;
	// 826E1238: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826E123C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 826E1240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1244: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E124C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1250: 386A124C  addi r3, r10, 0x124c
	ctx.r[3].s64 = ctx.r[10].s64 + 4684;
	// 826E1254: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E1258: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E125C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E126C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1274: 4BD85BAD  bl 0x82466e20
	ctx.lr = 0x826E1278;
	sub_82466E20(ctx, base);
	// 826E1278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E127C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1288 size=100
    let mut pc: u32 = 0x826E1288;
    'dispatch: loop {
        match pc {
            0x826E1288 => {
    //   block [0x826E1288..0x826E12EC)
	// 826E1288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E128C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1294: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E129C: 38AA1ABC  addi r5, r10, 0x1abc
	ctx.r[5].s64 = ctx.r[10].s64 + 6844;
	// 826E12A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E12A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E12A8: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 826E12AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E12B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E12B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E12B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E12BC: 386A127C  addi r3, r10, 0x127c
	ctx.r[3].s64 = ctx.r[10].s64 + 4732;
	// 826E12C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E12C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E12C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E12CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E12D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E12D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E12D8: 4BD85B49  bl 0x82466e20
	ctx.lr = 0x826E12DC;
	sub_82466E20(ctx, base);
	// 826E12DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E12E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E12E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E12E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E12F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E12F0 size=24
    let mut pc: u32 = 0x826E12F0;
    'dispatch: loop {
        match pc {
            0x826E12F0 => {
    //   block [0x826E12F0..0x826E1308)
	// 826E12F0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E12F4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E12F8: 394A5B80  addi r10, r10, 0x5b80
	ctx.r[10].s64 = ctx.r[10].s64 + 23424;
	// 826E12FC: 816BD0B0  lwz r11, -0x2f50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12112 as u32) ) } as u64;
	// 826E1300: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826E1304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1308 size=108
    let mut pc: u32 = 0x826E1308;
    'dispatch: loop {
        match pc {
            0x826E1308 => {
    //   block [0x826E1308..0x826E1374)
	// 826E1308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E130C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1314: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1318: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E131C: 38EB5B80  addi r7, r11, 0x5b80
	ctx.r[7].s64 = ctx.r[11].s64 + 23424;
	// 826E1320: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826E1324: 388AA8D0  addi r4, r10, -0x5730
	ctx.r[4].s64 = ctx.r[10].s64 + -22320;
	// 826E1328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E132C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1330: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E1334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1338: 386A12AC  addi r3, r10, 0x12ac
	ctx.r[3].s64 = ctx.r[10].s64 + 4780;
	// 826E133C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E134C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E135C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1360: 4BD85AC1  bl 0x82466e20
	ctx.lr = 0x826E1364;
	sub_82466E20(ctx, base);
	// 826E1364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E136C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1378 size=112
    let mut pc: u32 = 0x826E1378;
    'dispatch: loop {
        match pc {
            0x826E1378 => {
    //   block [0x826E1378..0x826E13E8)
	// 826E1378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E137C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1384: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E1388: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E138C: 392A6C60  addi r9, r10, 0x6c60
	ctx.r[9].s64 = ctx.r[10].s64 + 27744;
	// 826E1390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1394: 390BD0B8  addi r8, r11, -0x2f48
	ctx.r[8].s64 = ctx.r[11].s64 + -12104;
	// 826E1398: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826E139C: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 826E13A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E13A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E13A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E13AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E13B0: 386A12DC  addi r3, r10, 0x12dc
	ctx.r[3].s64 = ctx.r[10].s64 + 4828;
	// 826E13B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E13B8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E13BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E13C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E13C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E13C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E13CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E13D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E13D4: 4BD85A4D  bl 0x82466e20
	ctx.lr = 0x826E13D8;
	sub_82466E20(ctx, base);
	// 826E13D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E13DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E13E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E13E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E13E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E13E8 size=112
    let mut pc: u32 = 0x826E13E8;
    'dispatch: loop {
        match pc {
            0x826E13E8 => {
    //   block [0x826E13E8..0x826E1458)
	// 826E13E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E13EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E13F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E13F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E13F8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E13FC: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E1400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1404: 390BD100  addi r8, r11, -0x2f00
	ctx.r[8].s64 = ctx.r[11].s64 + -12032;
	// 826E1408: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E140C: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 826E1410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1414: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1418: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E141C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1420: 386A130C  addi r3, r10, 0x130c
	ctx.r[3].s64 = ctx.r[10].s64 + 4876;
	// 826E1424: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E1428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E142C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E143C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1444: 4BD859DD  bl 0x82466e20
	ctx.lr = 0x826E1448;
	sub_82466E20(ctx, base);
	// 826E1448: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E144C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1458 size=116
    let mut pc: u32 = 0x826E1458;
    'dispatch: loop {
        match pc {
            0x826E1458 => {
    //   block [0x826E1458..0x826E14CC)
	// 826E1458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E145C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1464: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1468: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826E146C: 390AD130  addi r8, r10, -0x2ed0
	ctx.r[8].s64 = ctx.r[10].s64 + -11984;
	// 826E1470: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1474: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1478: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E147C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1480: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E1484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1488: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E148C: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 826E1490: 396B6C88  addi r11, r11, 0x6c88
	ctx.r[11].s64 = ctx.r[11].s64 + 27784;
	// 826E1494: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1498: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E149C: 386A133C  addi r3, r10, 0x133c
	ctx.r[3].s64 = ctx.r[10].s64 + 4924;
	// 826E14A0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E14A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E14A8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E14AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E14B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E14B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E14B8: 4BD85969  bl 0x82466e20
	ctx.lr = 0x826E14BC;
	sub_82466E20(ctx, base);
	// 826E14BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E14C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E14C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E14C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E14D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E14D0 size=100
    let mut pc: u32 = 0x826E14D0;
    'dispatch: loop {
        match pc {
            0x826E14D0 => {
    //   block [0x826E14D0..0x826E1534)
	// 826E14D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E14D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E14D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E14DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E14E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E14E4: 38AA133C  addi r5, r10, 0x133c
	ctx.r[5].s64 = ctx.r[10].s64 + 4924;
	// 826E14E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E14EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E14F0: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826E14F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E14F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E14FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1504: 386A136C  addi r3, r10, 0x136c
	ctx.r[3].s64 = ctx.r[10].s64 + 4972;
	// 826E1508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E150C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1510: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E1514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1518: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E151C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1520: 4BD85901  bl 0x82466e20
	ctx.lr = 0x826E1524;
	sub_82466E20(ctx, base);
	// 826E1524: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E152C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E1538 size=24
    let mut pc: u32 = 0x826E1538;
    'dispatch: loop {
        match pc {
            0x826E1538 => {
    //   block [0x826E1538..0x826E1550)
	// 826E1538: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E153C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1540: 394A5BC8  addi r10, r10, 0x5bc8
	ctx.r[10].s64 = ctx.r[10].s64 + 23496;
	// 826E1544: 816BD0B4  lwz r11, -0x2f4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12108 as u32) ) } as u64;
	// 826E1548: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826E154C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1550 size=116
    let mut pc: u32 = 0x826E1550;
    'dispatch: loop {
        match pc {
            0x826E1550 => {
    //   block [0x826E1550..0x826E15C4)
	// 826E1550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E155C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1560: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1564: 392B6CC4  addi r9, r11, 0x6cc4
	ctx.r[9].s64 = ctx.r[11].s64 + 27844;
	// 826E1568: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E156C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1570: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826E1574: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826E1578: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E157C: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 826E1580: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1584: 396B5BC8  addi r11, r11, 0x5bc8
	ctx.r[11].s64 = ctx.r[11].s64 + 23496;
	// 826E1588: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826E158C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1590: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826E1594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1598: 386A139C  addi r3, r10, 0x139c
	ctx.r[3].s64 = ctx.r[10].s64 + 5020;
	// 826E159C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E15A0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826E15A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E15A8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826E15AC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E15B0: 4BD85871  bl 0x82466e20
	ctx.lr = 0x826E15B4;
	sub_82466E20(ctx, base);
	// 826E15B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E15B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E15BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E15C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E15C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E15C8 size=116
    let mut pc: u32 = 0x826E15C8;
    'dispatch: loop {
        match pc {
            0x826E15C8 => {
    //   block [0x826E15C8..0x826E163C)
	// 826E15C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E15CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E15D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E15D4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E15D8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E15DC: 392B6D20  addi r9, r11, 0x6d20
	ctx.r[9].s64 = ctx.r[11].s64 + 27936;
	// 826E15E0: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E15E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E15E8: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826E15EC: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826E15F0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E15F4: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 826E15F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E15FC: 396BD1E0  addi r11, r11, -0x2e20
	ctx.r[11].s64 = ctx.r[11].s64 + -11808;
	// 826E1600: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826E1604: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1608: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826E160C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1610: 386A13CC  addi r3, r10, 0x13cc
	ctx.r[3].s64 = ctx.r[10].s64 + 5068;
	// 826E1614: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E1618: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826E161C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1620: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826E1624: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E1628: 4BD857F9  bl 0x82466e20
	ctx.lr = 0x826E162C;
	sub_82466E20(ctx, base);
	// 826E162C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1640 size=108
    let mut pc: u32 = 0x826E1640;
    'dispatch: loop {
        match pc {
            0x826E1640 => {
    //   block [0x826E1640..0x826E16AC)
	// 826E1640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E164C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1654: 38EBD330  addi r7, r11, -0x2cd0
	ctx.r[7].s64 = ctx.r[11].s64 + -11472;
	// 826E1658: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826E165C: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 826E1660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1664: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1668: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E166C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1670: 386A13FC  addi r3, r10, 0x13fc
	ctx.r[3].s64 = ctx.r[10].s64 + 5116;
	// 826E1674: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E167C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E168C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1698: 4BD85789  bl 0x82466e20
	ctx.lr = 0x826E169C;
	sub_82466E20(ctx, base);
	// 826E169C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E16A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E16A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E16A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E16B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E16B0 size=24
    let mut pc: u32 = 0x826E16B0;
    'dispatch: loop {
        match pc {
            0x826E16B0 => {
    //   block [0x826E16B0..0x826E16C8)
	// 826E16B0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E16B4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E16B8: 394A5C70  addi r10, r10, 0x5c70
	ctx.r[10].s64 = ctx.r[10].s64 + 23664;
	// 826E16BC: 816BD1DC  lwz r11, -0x2e24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11812 as u32) ) } as u64;
	// 826E16C0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826E16C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E16C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E16C8 size=116
    let mut pc: u32 = 0x826E16C8;
    'dispatch: loop {
        match pc {
            0x826E16C8 => {
    //   block [0x826E16C8..0x826E173C)
	// 826E16C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E16CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E16D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E16D4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E16D8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E16DC: 390B5C70  addi r8, r11, 0x5c70
	ctx.r[8].s64 = ctx.r[11].s64 + 23664;
	// 826E16E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E16E4: 392A6DB8  addi r9, r10, 0x6db8
	ctx.r[9].s64 = ctx.r[10].s64 + 28088;
	// 826E16E8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E16EC: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 826E16F0: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E16F4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E16F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E16FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E170C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E1710: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826E1714: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E1718: 386B142C  addi r3, r11, 0x142c
	ctx.r[3].s64 = ctx.r[11].s64 + 5164;
	// 826E171C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E1720: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1728: 4BD856F9  bl 0x82466e20
	ctx.lr = 0x826E172C;
	sub_82466E20(ctx, base);
	// 826E172C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1740 size=112
    let mut pc: u32 = 0x826E1740;
    'dispatch: loop {
        match pc {
            0x826E1740 => {
    //   block [0x826E1740..0x826E17B0)
	// 826E1740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E174C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1750: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1754: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E1758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E175C: 390BD3AC  addi r8, r11, -0x2c54
	ctx.r[8].s64 = ctx.r[11].s64 + -11348;
	// 826E1760: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E1764: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 826E1768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E176C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1770: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1778: 386A145C  addi r3, r10, 0x145c
	ctx.r[3].s64 = ctx.r[10].s64 + 5212;
	// 826E177C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E1780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E178C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E179C: 4BD85685  bl 0x82466e20
	ctx.lr = 0x826E17A0;
	sub_82466E20(ctx, base);
	// 826E17A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E17A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E17A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E17AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E17B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E17B0 size=24
    let mut pc: u32 = 0x826E17B0;
    'dispatch: loop {
        match pc {
            0x826E17B0 => {
    //   block [0x826E17B0..0x826E17C8)
	// 826E17B0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E17B4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E17B8: 394A5E08  addi r10, r10, 0x5e08
	ctx.r[10].s64 = ctx.r[10].s64 + 24072;
	// 826E17BC: 816BD3DC  lwz r11, -0x2c24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11300 as u32) ) } as u64;
	// 826E17C0: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 826E17C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E17C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E17C8 size=116
    let mut pc: u32 = 0x826E17C8;
    'dispatch: loop {
        match pc {
            0x826E17C8 => {
    //   block [0x826E17C8..0x826E183C)
	// 826E17C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E17CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E17D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E17D4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E17D8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E17DC: 392B6DF0  addi r9, r11, 0x6df0
	ctx.r[9].s64 = ctx.r[11].s64 + 28144;
	// 826E17E0: 38AA13CC  addi r5, r10, 0x13cc
	ctx.r[5].s64 = ctx.r[10].s64 + 5068;
	// 826E17E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E17E8: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826E17EC: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 826E17F0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E17F4: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 826E17F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E17FC: 396B5E08  addi r11, r11, 0x5e08
	ctx.r[11].s64 = ctx.r[11].s64 + 24072;
	// 826E1800: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826E1804: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1808: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826E180C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1810: 386A148C  addi r3, r10, 0x148c
	ctx.r[3].s64 = ctx.r[10].s64 + 5260;
	// 826E1814: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E1818: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826E181C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1820: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826E1824: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E1828: 4BD855F9  bl 0x82466e20
	ctx.lr = 0x826E182C;
	sub_82466E20(ctx, base);
	// 826E182C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1840 size=112
    let mut pc: u32 = 0x826E1840;
    'dispatch: loop {
        match pc {
            0x826E1840 => {
    //   block [0x826E1840..0x826E18B0)
	// 826E1840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E184C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1850: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1854: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E1858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E185C: 390BD3E0  addi r8, r11, -0x2c20
	ctx.r[8].s64 = ctx.r[11].s64 + -11296;
	// 826E1860: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E1864: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 826E1868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E186C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1878: 386A14BC  addi r3, r10, 0x14bc
	ctx.r[3].s64 = ctx.r[10].s64 + 5308;
	// 826E187C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E1880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E188C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E189C: 4BD85585  bl 0x82466e20
	ctx.lr = 0x826E18A0;
	sub_82466E20(ctx, base);
	// 826E18A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E18A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E18A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E18AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E18B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E18B0 size=100
    let mut pc: u32 = 0x826E18B0;
    'dispatch: loop {
        match pc {
            0x826E18B0 => {
    //   block [0x826E18B0..0x826E1914)
	// 826E18B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E18B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E18B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E18BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E18C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E18C4: 38AA1ABC  addi r5, r10, 0x1abc
	ctx.r[5].s64 = ctx.r[10].s64 + 6844;
	// 826E18C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E18CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E18D0: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 826E18D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E18D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E18DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E18E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E18E4: 386A14EC  addi r3, r10, 0x14ec
	ctx.r[3].s64 = ctx.r[10].s64 + 5356;
	// 826E18E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E18EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E18F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E18F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E18F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E18FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1900: 4BD85521  bl 0x82466e20
	ctx.lr = 0x826E1904;
	sub_82466E20(ctx, base);
	// 826E1904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E190C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1918 size=112
    let mut pc: u32 = 0x826E1918;
    'dispatch: loop {
        match pc {
            0x826E1918 => {
    //   block [0x826E1918..0x826E1988)
	// 826E1918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E191C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1924: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1928: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826E192C: 38EAD3F8  addi r7, r10, -0x2c08
	ctx.r[7].s64 = ctx.r[10].s64 + -11272;
	// 826E1930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1934: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1938: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 826E193C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1940: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1944: 396B6E60  addi r11, r11, 0x6e60
	ctx.r[11].s64 = ctx.r[11].s64 + 28256;
	// 826E1948: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E194C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1950: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1954: 386A151C  addi r3, r10, 0x151c
	ctx.r[3].s64 = ctx.r[10].s64 + 5404;
	// 826E1958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E195C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E1960: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1964: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E1968: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E196C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1970: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1974: 4BD854AD  bl 0x82466e20
	ctx.lr = 0x826E1978;
	sub_82466E20(ctx, base);
	// 826E1978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E197C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1988 size=112
    let mut pc: u32 = 0x826E1988;
    'dispatch: loop {
        match pc {
            0x826E1988 => {
    //   block [0x826E1988..0x826E19F8)
	// 826E1988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E198C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1994: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1998: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E199C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E19A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E19A4: 390BD530  addi r8, r11, -0x2ad0
	ctx.r[8].s64 = ctx.r[11].s64 + -10960;
	// 826E19A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E19AC: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 826E19B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E19B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E19B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E19BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E19C0: 386A154C  addi r3, r10, 0x154c
	ctx.r[3].s64 = ctx.r[10].s64 + 5452;
	// 826E19C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E19C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E19CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E19D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E19D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E19D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E19DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E19E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E19E4: 4BD8543D  bl 0x82466e20
	ctx.lr = 0x826E19E8;
	sub_82466E20(ctx, base);
	// 826E19E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E19EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E19F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E19F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E19F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E19F8 size=112
    let mut pc: u32 = 0x826E19F8;
    'dispatch: loop {
        match pc {
            0x826E19F8 => {
    //   block [0x826E19F8..0x826E1A68)
	// 826E19F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E19FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1A04: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1A08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E1A0C: 38EAD560  addi r7, r10, -0x2aa0
	ctx.r[7].s64 = ctx.r[10].s64 + -10912;
	// 826E1A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1A14: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1A18: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 826E1A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1A20: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1A24: 396B6EB4  addi r11, r11, 0x6eb4
	ctx.r[11].s64 = ctx.r[11].s64 + 28340;
	// 826E1A28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E1A2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1A30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1A34: 386A157C  addi r3, r10, 0x157c
	ctx.r[3].s64 = ctx.r[10].s64 + 5500;
	// 826E1A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1A3C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E1A40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1A44: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E1A48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1A4C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1A50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1A54: 4BD853CD  bl 0x82466e20
	ctx.lr = 0x826E1A58;
	sub_82466E20(ctx, base);
	// 826E1A58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1A68 size=112
    let mut pc: u32 = 0x826E1A68;
    'dispatch: loop {
        match pc {
            0x826E1A68 => {
    //   block [0x826E1A68..0x826E1AD8)
	// 826E1A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1A74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1A78: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1A7C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E1A80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1A84: 390BD590  addi r8, r11, -0x2a70
	ctx.r[8].s64 = ctx.r[11].s64 + -10864;
	// 826E1A88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E1A8C: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 826E1A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1A94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1A98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1AA0: 386A15AC  addi r3, r10, 0x15ac
	ctx.r[3].s64 = ctx.r[10].s64 + 5548;
	// 826E1AA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E1AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1AB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1AC4: 4BD8535D  bl 0x82466e20
	ctx.lr = 0x826E1AC8;
	sub_82466E20(ctx, base);
	// 826E1AC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1AD8 size=108
    let mut pc: u32 = 0x826E1AD8;
    'dispatch: loop {
        match pc {
            0x826E1AD8 => {
    //   block [0x826E1AD8..0x826E1B44)
	// 826E1AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1AE4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1AE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1AEC: 38EBD5A8  addi r7, r11, -0x2a58
	ctx.r[7].s64 = ctx.r[11].s64 + -10840;
	// 826E1AF0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E1AF4: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 826E1AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1AFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1B00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E1B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1B08: 386A15DC  addi r3, r10, 0x15dc
	ctx.r[3].s64 = ctx.r[10].s64 + 5596;
	// 826E1B0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1B10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1B18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1B20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1B24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1B2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1B30: 4BD852F1  bl 0x82466e20
	ctx.lr = 0x826E1B34;
	sub_82466E20(ctx, base);
	// 826E1B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1B48 size=112
    let mut pc: u32 = 0x826E1B48;
    'dispatch: loop {
        match pc {
            0x826E1B48 => {
    //   block [0x826E1B48..0x826E1BB8)
	// 826E1B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1B54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1B58: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1B5C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E1B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1B64: 390BD5C0  addi r8, r11, -0x2a40
	ctx.r[8].s64 = ctx.r[11].s64 + -10816;
	// 826E1B68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E1B6C: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 826E1B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1B74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1B78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1B80: 386A160C  addi r3, r10, 0x160c
	ctx.r[3].s64 = ctx.r[10].s64 + 5644;
	// 826E1B84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E1B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1BA4: 4BD8527D  bl 0x82466e20
	ctx.lr = 0x826E1BA8;
	sub_82466E20(ctx, base);
	// 826E1BA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1BB8 size=112
    let mut pc: u32 = 0x826E1BB8;
    'dispatch: loop {
        match pc {
            0x826E1BB8 => {
    //   block [0x826E1BB8..0x826E1C28)
	// 826E1BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1BC4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1BC8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826E1BCC: 38EAD5D8  addi r7, r10, -0x2a28
	ctx.r[7].s64 = ctx.r[10].s64 + -10792;
	// 826E1BD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1BD4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1BD8: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 826E1BDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1BE0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1BE4: 396B6EC0  addi r11, r11, 0x6ec0
	ctx.r[11].s64 = ctx.r[11].s64 + 28352;
	// 826E1BE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E1BEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1BF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1BF4: 386A163C  addi r3, r10, 0x163c
	ctx.r[3].s64 = ctx.r[10].s64 + 5692;
	// 826E1BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1BFC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E1C00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1C04: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E1C08: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1C0C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1C10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1C14: 4BD8520D  bl 0x82466e20
	ctx.lr = 0x826E1C18;
	sub_82466E20(ctx, base);
	// 826E1C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1C28 size=112
    let mut pc: u32 = 0x826E1C28;
    'dispatch: loop {
        match pc {
            0x826E1C28 => {
    //   block [0x826E1C28..0x826E1C98)
	// 826E1C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1C34: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1C38: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 826E1C3C: 38EAD6B0  addi r7, r10, -0x2950
	ctx.r[7].s64 = ctx.r[10].s64 + -10576;
	// 826E1C40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1C44: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1C48: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 826E1C4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1C50: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1C54: 396B6F00  addi r11, r11, 0x6f00
	ctx.r[11].s64 = ctx.r[11].s64 + 28416;
	// 826E1C58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E1C5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1C60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1C64: 386A166C  addi r3, r10, 0x166c
	ctx.r[3].s64 = ctx.r[10].s64 + 5740;
	// 826E1C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1C6C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E1C70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1C74: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E1C78: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1C7C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1C80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1C84: 4BD8519D  bl 0x82466e20
	ctx.lr = 0x826E1C88;
	sub_82466E20(ctx, base);
	// 826E1C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1C98 size=108
    let mut pc: u32 = 0x826E1C98;
    'dispatch: loop {
        match pc {
            0x826E1C98 => {
    //   block [0x826E1C98..0x826E1D04)
	// 826E1C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1CA4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1CA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1CAC: 38EBD800  addi r7, r11, -0x2800
	ctx.r[7].s64 = ctx.r[11].s64 + -10240;
	// 826E1CB0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E1CB4: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 826E1CB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1CBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1CC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E1CC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1CC8: 386A169C  addi r3, r10, 0x169c
	ctx.r[3].s64 = ctx.r[10].s64 + 5788;
	// 826E1CCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1CD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1CD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1CE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1CE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1CEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1CF0: 4BD85131  bl 0x82466e20
	ctx.lr = 0x826E1CF4;
	sub_82466E20(ctx, base);
	// 826E1CF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1D08 size=116
    let mut pc: u32 = 0x826E1D08;
    'dispatch: loop {
        match pc {
            0x826E1D08 => {
    //   block [0x826E1D08..0x826E1D7C)
	// 826E1D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1D14: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1D18: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 826E1D1C: 390AD860  addi r8, r10, -0x27a0
	ctx.r[8].s64 = ctx.r[10].s64 + -10144;
	// 826E1D20: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1D24: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1D28: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E1D2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1D30: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E1D34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1D38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1D3C: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 826E1D40: 396B6FA0  addi r11, r11, 0x6fa0
	ctx.r[11].s64 = ctx.r[11].s64 + 28576;
	// 826E1D44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1D48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1D4C: 386A16CC  addi r3, r10, 0x16cc
	ctx.r[3].s64 = ctx.r[10].s64 + 5836;
	// 826E1D50: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E1D54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1D58: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E1D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1D60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1D68: 4BD850B9  bl 0x82466e20
	ctx.lr = 0x826E1D6C;
	sub_82466E20(ctx, base);
	// 826E1D6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1D80 size=116
    let mut pc: u32 = 0x826E1D80;
    'dispatch: loop {
        match pc {
            0x826E1D80 => {
    //   block [0x826E1D80..0x826E1DF4)
	// 826E1D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1D8C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1D90: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826E1D94: 390AD980  addi r8, r10, -0x2680
	ctx.r[8].s64 = ctx.r[10].s64 + -9856;
	// 826E1D98: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1D9C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1DA0: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E1DA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1DA8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E1DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1DB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1DB4: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 826E1DB8: 396B6FDC  addi r11, r11, 0x6fdc
	ctx.r[11].s64 = ctx.r[11].s64 + 28636;
	// 826E1DBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1DC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1DC4: 386A16FC  addi r3, r10, 0x16fc
	ctx.r[3].s64 = ctx.r[10].s64 + 5884;
	// 826E1DC8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E1DCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1DD0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E1DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1DE0: 4BD85041  bl 0x82466e20
	ctx.lr = 0x826E1DE4;
	sub_82466E20(ctx, base);
	// 826E1DE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1DF8 size=108
    let mut pc: u32 = 0x826E1DF8;
    'dispatch: loop {
        match pc {
            0x826E1DF8 => {
    //   block [0x826E1DF8..0x826E1E64)
	// 826E1DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1E04: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1E08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E1E0C: 38EBD9E0  addi r7, r11, -0x2620
	ctx.r[7].s64 = ctx.r[11].s64 + -9760;
	// 826E1E10: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826E1E14: 388AA8F8  addi r4, r10, -0x5708
	ctx.r[4].s64 = ctx.r[10].s64 + -22280;
	// 826E1E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1E1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1E20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E1E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1E28: 386A172C  addi r3, r10, 0x172c
	ctx.r[3].s64 = ctx.r[10].s64 + 5932;
	// 826E1E2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1E34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1E4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1E50: 4BD84FD1  bl 0x82466e20
	ctx.lr = 0x826E1E54;
	sub_82466E20(ctx, base);
	// 826E1E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1E68 size=112
    let mut pc: u32 = 0x826E1E68;
    'dispatch: loop {
        match pc {
            0x826E1E68 => {
    //   block [0x826E1E68..0x826E1ED8)
	// 826E1E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1E74: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1E78: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826E1E7C: 38EADA88  addi r7, r10, -0x2578
	ctx.r[7].s64 = ctx.r[10].s64 + -9592;
	// 826E1E80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E1E84: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1E88: 388AA910  addi r4, r10, -0x56f0
	ctx.r[4].s64 = ctx.r[10].s64 + -22256;
	// 826E1E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1E90: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E1E94: 396B6FF0  addi r11, r11, 0x6ff0
	ctx.r[11].s64 = ctx.r[11].s64 + 28656;
	// 826E1E98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E1E9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1EA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1EA4: 386A175C  addi r3, r10, 0x175c
	ctx.r[3].s64 = ctx.r[10].s64 + 5980;
	// 826E1EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1EAC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E1EB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1EB4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E1EB8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1EBC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1EC0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E1EC4: 4BD84F5D  bl 0x82466e20
	ctx.lr = 0x826E1EC8;
	sub_82466E20(ctx, base);
	// 826E1EC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1ED8 size=112
    let mut pc: u32 = 0x826E1ED8;
    'dispatch: loop {
        match pc {
            0x826E1ED8 => {
    //   block [0x826E1ED8..0x826E1F48)
	// 826E1ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1EE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1EE8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E1EEC: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E1EF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E1EF4: 390BDB00  addi r8, r11, -0x2500
	ctx.r[8].s64 = ctx.r[11].s64 + -9472;
	// 826E1EF8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E1EFC: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 826E1F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E1F04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1F08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1F10: 386A178C  addi r3, r10, 0x178c
	ctx.r[3].s64 = ctx.r[10].s64 + 6028;
	// 826E1F14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E1F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E1F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E1F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1F34: 4BD84EED  bl 0x82466e20
	ctx.lr = 0x826E1F38;
	sub_82466E20(ctx, base);
	// 826E1F38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1F48 size=116
    let mut pc: u32 = 0x826E1F48;
    'dispatch: loop {
        match pc {
            0x826E1F48 => {
    //   block [0x826E1F48..0x826E1FBC)
	// 826E1F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1F54: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1F58: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826E1F5C: 390ADB48  addi r8, r10, -0x24b8
	ctx.r[8].s64 = ctx.r[10].s64 + -9400;
	// 826E1F60: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1F64: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1F68: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E1F6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E1F70: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E1F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1F78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1F7C: 388A7B20  addi r4, r10, 0x7b20
	ctx.r[4].s64 = ctx.r[10].s64 + 31520;
	// 826E1F80: 396B7010  addi r11, r11, 0x7010
	ctx.r[11].s64 = ctx.r[11].s64 + 28688;
	// 826E1F84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1F88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E1F8C: 386A17BC  addi r3, r10, 0x17bc
	ctx.r[3].s64 = ctx.r[10].s64 + 6076;
	// 826E1F90: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E1F94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E1F98: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E1F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E1FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E1FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E1FA8: 4BD84E79  bl 0x82466e20
	ctx.lr = 0x826E1FAC;
	sub_82466E20(ctx, base);
	// 826E1FAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E1FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E1FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E1FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E1FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E1FC0 size=116
    let mut pc: u32 = 0x826E1FC0;
    'dispatch: loop {
        match pc {
            0x826E1FC0 => {
    //   block [0x826E1FC0..0x826E2034)
	// 826E1FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E1FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E1FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E1FCC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E1FD0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826E1FD4: 390ADC50  addi r8, r10, -0x23b0
	ctx.r[8].s64 = ctx.r[10].s64 + -9136;
	// 826E1FD8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E1FDC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E1FE0: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E1FE4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E1FE8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E1FEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E1FF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E1FF4: 388AB2FC  addi r4, r10, -0x4d04
	ctx.r[4].s64 = ctx.r[10].s64 + -19716;
	// 826E1FF8: 396B7058  addi r11, r11, 0x7058
	ctx.r[11].s64 = ctx.r[11].s64 + 28760;
	// 826E1FFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2000: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2004: 386A17EC  addi r3, r10, 0x17ec
	ctx.r[3].s64 = ctx.r[10].s64 + 6124;
	// 826E2008: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E200C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2010: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E2014: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E201C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2020: 4BD84E01  bl 0x82466e20
	ctx.lr = 0x826E2024;
	sub_82466E20(ctx, base);
	// 826E2024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E202C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2038 size=116
    let mut pc: u32 = 0x826E2038;
    'dispatch: loop {
        match pc {
            0x826E2038 => {
    //   block [0x826E2038..0x826E20AC)
	// 826E2038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E203C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2044: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E2048: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 826E204C: 390ADC98  addi r8, r10, -0x2368
	ctx.r[8].s64 = ctx.r[10].s64 + -9064;
	// 826E2050: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2054: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E2058: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E205C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2060: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E2064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E206C: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 826E2070: 396B7068  addi r11, r11, 0x7068
	ctx.r[11].s64 = ctx.r[11].s64 + 28776;
	// 826E2074: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2078: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E207C: 386A181C  addi r3, r10, 0x181c
	ctx.r[3].s64 = ctx.r[10].s64 + 6172;
	// 826E2080: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E2084: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2088: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E208C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2098: 4BD84D89  bl 0x82466e20
	ctx.lr = 0x826E209C;
	sub_82466E20(ctx, base);
	// 826E209C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E20A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E20A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E20A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E20B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E20B0 size=100
    let mut pc: u32 = 0x826E20B0;
    'dispatch: loop {
        match pc {
            0x826E20B0 => {
    //   block [0x826E20B0..0x826E2114)
	// 826E20B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E20B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E20B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E20BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E20C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E20C4: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E20C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E20CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E20D0: 388AA928  addi r4, r10, -0x56d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22232;
	// 826E20D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E20D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E20DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E20E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E20E4: 386A184C  addi r3, r10, 0x184c
	ctx.r[3].s64 = ctx.r[10].s64 + 6220;
	// 826E20E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E20EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E20F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E20F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E20F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E20FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2100: 4BD84D21  bl 0x82466e20
	ctx.lr = 0x826E2104;
	sub_82466E20(ctx, base);
	// 826E2104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E210C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2118 size=112
    let mut pc: u32 = 0x826E2118;
    'dispatch: loop {
        match pc {
            0x826E2118 => {
    //   block [0x826E2118..0x826E2188)
	// 826E2118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E211C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2124: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2128: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E212C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E2130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2134: 390BDDD0  addi r8, r11, -0x2230
	ctx.r[8].s64 = ctx.r[11].s64 + -8752;
	// 826E2138: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E213C: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 826E2140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2144: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2148: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E214C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2150: 386A187C  addi r3, r10, 0x187c
	ctx.r[3].s64 = ctx.r[10].s64 + 6268;
	// 826E2154: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E215C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E216C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2174: 4BD84CAD  bl 0x82466e20
	ctx.lr = 0x826E2178;
	sub_82466E20(ctx, base);
	// 826E2178: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E217C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2188 size=112
    let mut pc: u32 = 0x826E2188;
    'dispatch: loop {
        match pc {
            0x826E2188 => {
    //   block [0x826E2188..0x826E21F8)
	// 826E2188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E218C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2194: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2198: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E219C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E21A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E21A4: 390BDDE8  addi r8, r11, -0x2218
	ctx.r[8].s64 = ctx.r[11].s64 + -8728;
	// 826E21A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E21AC: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 826E21B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E21B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E21B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E21BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E21C0: 386A18AC  addi r3, r10, 0x18ac
	ctx.r[3].s64 = ctx.r[10].s64 + 6316;
	// 826E21C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E21C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E21CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E21D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E21D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E21D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E21DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E21E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E21E4: 4BD84C3D  bl 0x82466e20
	ctx.lr = 0x826E21E8;
	sub_82466E20(ctx, base);
	// 826E21E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E21EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E21F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E21F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E21F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E21F8 size=112
    let mut pc: u32 = 0x826E21F8;
    'dispatch: loop {
        match pc {
            0x826E21F8 => {
    //   block [0x826E21F8..0x826E2268)
	// 826E21F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E21FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2204: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E2208: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826E220C: 38EADE18  addi r7, r10, -0x21e8
	ctx.r[7].s64 = ctx.r[10].s64 + -8680;
	// 826E2210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2214: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E2218: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 826E221C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2220: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E2224: 396B70E0  addi r11, r11, 0x70e0
	ctx.r[11].s64 = ctx.r[11].s64 + 28896;
	// 826E2228: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E222C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2230: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2234: 386A18DC  addi r3, r10, 0x18dc
	ctx.r[3].s64 = ctx.r[10].s64 + 6364;
	// 826E2238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E223C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E2240: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2244: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E2248: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E224C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2250: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2254: 4BD84BCD  bl 0x82466e20
	ctx.lr = 0x826E2258;
	sub_82466E20(ctx, base);
	// 826E2258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E225C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2268 size=112
    let mut pc: u32 = 0x826E2268;
    'dispatch: loop {
        match pc {
            0x826E2268 => {
    //   block [0x826E2268..0x826E22D8)
	// 826E2268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E226C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2274: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2278: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E227C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E2280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2284: 390BDE90  addi r8, r11, -0x2170
	ctx.r[8].s64 = ctx.r[11].s64 + -8560;
	// 826E2288: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E228C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 826E2290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2294: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2298: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E229C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E22A0: 386A190C  addi r3, r10, 0x190c
	ctx.r[3].s64 = ctx.r[10].s64 + 6412;
	// 826E22A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E22A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E22AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E22B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E22B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E22B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E22BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E22C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E22C4: 4BD84B5D  bl 0x82466e20
	ctx.lr = 0x826E22C8;
	sub_82466E20(ctx, base);
	// 826E22C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E22CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E22D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E22D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E22D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E22D8 size=24
    let mut pc: u32 = 0x826E22D8;
    'dispatch: loop {
        match pc {
            0x826E22D8 => {
    //   block [0x826E22D8..0x826E22F0)
	// 826E22D8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E22DC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E22E0: 394A5F70  addi r10, r10, 0x5f70
	ctx.r[10].s64 = ctx.r[10].s64 + 24432;
	// 826E22E4: 816BDEC0  lwz r11, -0x2140(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8512 as u32) ) } as u64;
	// 826E22E8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826E22EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E22F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E22F0 size=116
    let mut pc: u32 = 0x826E22F0;
    'dispatch: loop {
        match pc {
            0x826E22F0 => {
    //   block [0x826E22F0..0x826E2364)
	// 826E22F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E22F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E22F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E22FC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2300: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E2304: 390B5F70  addi r8, r11, 0x5f70
	ctx.r[8].s64 = ctx.r[11].s64 + 24432;
	// 826E2308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E230C: 392A7120  addi r9, r10, 0x7120
	ctx.r[9].s64 = ctx.r[10].s64 + 28960;
	// 826E2310: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2314: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826E2318: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E231C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E2320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2324: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E232C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2334: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E2338: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 826E233C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E2340: 386B193C  addi r3, r11, 0x193c
	ctx.r[3].s64 = ctx.r[11].s64 + 6460;
	// 826E2344: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E2348: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E234C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2350: 4BD84AD1  bl 0x82466e20
	ctx.lr = 0x826E2354;
	sub_82466E20(ctx, base);
	// 826E2354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E235C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2368 size=116
    let mut pc: u32 = 0x826E2368;
    'dispatch: loop {
        match pc {
            0x826E2368 => {
    //   block [0x826E2368..0x826E23DC)
	// 826E2368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E236C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2374: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E2378: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826E237C: 390ADEC8  addi r8, r10, -0x2138
	ctx.r[8].s64 = ctx.r[10].s64 + -8504;
	// 826E2380: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2384: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E2388: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E238C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2390: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E2394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E239C: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 826E23A0: 396B7134  addi r11, r11, 0x7134
	ctx.r[11].s64 = ctx.r[11].s64 + 28980;
	// 826E23A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E23A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E23AC: 386A196C  addi r3, r10, 0x196c
	ctx.r[3].s64 = ctx.r[10].s64 + 6508;
	// 826E23B0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E23B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E23B8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E23BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E23C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E23C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E23C8: 4BD84A59  bl 0x82466e20
	ctx.lr = 0x826E23CC;
	sub_82466E20(ctx, base);
	// 826E23CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E23D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E23D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E23D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E23E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E23E0 size=112
    let mut pc: u32 = 0x826E23E0;
    'dispatch: loop {
        match pc {
            0x826E23E0 => {
    //   block [0x826E23E0..0x826E2450)
	// 826E23E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E23E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E23E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E23EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E23F0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E23F4: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E23F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E23FC: 390BDF88  addi r8, r11, -0x2078
	ctx.r[8].s64 = ctx.r[11].s64 + -8312;
	// 826E2400: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E2404: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 826E2408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E240C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2410: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E2414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2418: 386A199C  addi r3, r10, 0x199c
	ctx.r[3].s64 = ctx.r[10].s64 + 6556;
	// 826E241C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E242C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E243C: 4BD849E5  bl 0x82466e20
	ctx.lr = 0x826E2440;
	sub_82466E20(ctx, base);
	// 826E2440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E244C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2450 size=112
    let mut pc: u32 = 0x826E2450;
    'dispatch: loop {
        match pc {
            0x826E2450 => {
    //   block [0x826E2450..0x826E24C0)
	// 826E2450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E245C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E2460: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E2464: 38EADFA0  addi r7, r10, -0x2060
	ctx.r[7].s64 = ctx.r[10].s64 + -8288;
	// 826E2468: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E246C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E2470: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826E2474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2478: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E247C: 396B7158  addi r11, r11, 0x7158
	ctx.r[11].s64 = ctx.r[11].s64 + 29016;
	// 826E2480: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E2484: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2488: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E248C: 386A19CC  addi r3, r10, 0x19cc
	ctx.r[3].s64 = ctx.r[10].s64 + 6604;
	// 826E2490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2494: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E2498: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E249C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E24A0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E24A4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E24A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E24AC: 4BD84975  bl 0x82466e20
	ctx.lr = 0x826E24B0;
	sub_82466E20(ctx, base);
	// 826E24B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E24B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E24B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E24BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E24C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E24C0 size=112
    let mut pc: u32 = 0x826E24C0;
    'dispatch: loop {
        match pc {
            0x826E24C0 => {
    //   block [0x826E24C0..0x826E2530)
	// 826E24C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E24C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E24C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E24CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E24D0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E24D4: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E24D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E24DC: 390BDFD0  addi r8, r11, -0x2030
	ctx.r[8].s64 = ctx.r[11].s64 + -8240;
	// 826E24E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E24E4: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826E24E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E24EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E24F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E24F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E24F8: 386A19FC  addi r3, r10, 0x19fc
	ctx.r[3].s64 = ctx.r[10].s64 + 6652;
	// 826E24FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E250C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E251C: 4BD84905  bl 0x82466e20
	ctx.lr = 0x826E2520;
	sub_82466E20(ctx, base);
	// 826E2520: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E252C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2530 size=116
    let mut pc: u32 = 0x826E2530;
    'dispatch: loop {
        match pc {
            0x826E2530 => {
    //   block [0x826E2530..0x826E25A4)
	// 826E2530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E253C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E2540: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826E2544: 390ADFE8  addi r8, r10, -0x2018
	ctx.r[8].s64 = ctx.r[10].s64 + -8216;
	// 826E2548: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E254C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E2550: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E2554: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2558: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E255C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2560: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E2564: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 826E2568: 396B7164  addi r11, r11, 0x7164
	ctx.r[11].s64 = ctx.r[11].s64 + 29028;
	// 826E256C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2570: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2574: 386A1A2C  addi r3, r10, 0x1a2c
	ctx.r[3].s64 = ctx.r[10].s64 + 6700;
	// 826E2578: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E257C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2580: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E2584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E258C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2590: 4BD84891  bl 0x82466e20
	ctx.lr = 0x826E2594;
	sub_82466E20(ctx, base);
	// 826E2594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E259C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E25A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E25A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E25A8 size=112
    let mut pc: u32 = 0x826E25A8;
    'dispatch: loop {
        match pc {
            0x826E25A8 => {
    //   block [0x826E25A8..0x826E2618)
	// 826E25A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E25AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E25B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E25B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E25B8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E25BC: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E25C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E25C4: 390BE090  addi r8, r11, -0x1f70
	ctx.r[8].s64 = ctx.r[11].s64 + -8048;
	// 826E25C8: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 826E25CC: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 826E25D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E25D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E25D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E25DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E25E0: 386A1A5C  addi r3, r10, 0x1a5c
	ctx.r[3].s64 = ctx.r[10].s64 + 6748;
	// 826E25E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E25E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E25EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E25F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E25F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E25F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E25FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2604: 4BD8481D  bl 0x82466e20
	ctx.lr = 0x826E2608;
	sub_82466E20(ctx, base);
	// 826E2608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E260C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2618 size=112
    let mut pc: u32 = 0x826E2618;
    'dispatch: loop {
        match pc {
            0x826E2618 => {
    //   block [0x826E2618..0x826E2688)
	// 826E2618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E261C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2624: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2628: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E262C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E2630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2634: 390BE1C8  addi r8, r11, -0x1e38
	ctx.r[8].s64 = ctx.r[11].s64 + -7736;
	// 826E2638: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E263C: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 826E2640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2644: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2648: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E264C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2650: 386A1A8C  addi r3, r10, 0x1a8c
	ctx.r[3].s64 = ctx.r[10].s64 + 6796;
	// 826E2654: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E265C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E266C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2674: 4BD847AD  bl 0x82466e20
	ctx.lr = 0x826E2678;
	sub_82466E20(ctx, base);
	// 826E2678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E267C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2688 size=116
    let mut pc: u32 = 0x826E2688;
    'dispatch: loop {
        match pc {
            0x826E2688 => {
    //   block [0x826E2688..0x826E26FC)
	// 826E2688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E268C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2694: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2698: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E269C: 390BE1E0  addi r8, r11, -0x1e20
	ctx.r[8].s64 = ctx.r[11].s64 + -7712;
	// 826E26A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E26A4: 392A719C  addi r9, r10, 0x719c
	ctx.r[9].s64 = ctx.r[10].s64 + 29084;
	// 826E26A8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E26AC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826E26B0: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E26B4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E26B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E26BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E26C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E26C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E26C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E26CC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E26D0: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 826E26D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E26D8: 386B1ABC  addi r3, r11, 0x1abc
	ctx.r[3].s64 = ctx.r[11].s64 + 6844;
	// 826E26DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E26E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E26E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E26E8: 4BD84739  bl 0x82466e20
	ctx.lr = 0x826E26EC;
	sub_82466E20(ctx, base);
	// 826E26EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E26F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E26F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E26F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2700 size=100
    let mut pc: u32 = 0x826E2700;
    'dispatch: loop {
        match pc {
            0x826E2700 => {
    //   block [0x826E2700..0x826E2764)
	// 826E2700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E270C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2714: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E2718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E271C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2720: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 826E2724: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E272C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2734: 386A1AEC  addi r3, r10, 0x1aec
	ctx.r[3].s64 = ctx.r[10].s64 + 6892;
	// 826E2738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E273C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2740: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E2744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2748: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E274C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2750: 4BD846D1  bl 0x82466e20
	ctx.lr = 0x826E2754;
	sub_82466E20(ctx, base);
	// 826E2754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E275C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2768 size=112
    let mut pc: u32 = 0x826E2768;
    'dispatch: loop {
        match pc {
            0x826E2768 => {
    //   block [0x826E2768..0x826E27D8)
	// 826E2768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E276C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2774: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2778: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E277C: 38AA1AEC  addi r5, r10, 0x1aec
	ctx.r[5].s64 = ctx.r[10].s64 + 6892;
	// 826E2780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2784: 390BE210  addi r8, r11, -0x1df0
	ctx.r[8].s64 = ctx.r[11].s64 + -7664;
	// 826E2788: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E278C: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 826E2790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2794: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2798: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E279C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E27A0: 386A1B1C  addi r3, r10, 0x1b1c
	ctx.r[3].s64 = ctx.r[10].s64 + 6940;
	// 826E27A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E27A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E27AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E27B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E27B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E27B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E27BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E27C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E27C4: 4BD8465D  bl 0x82466e20
	ctx.lr = 0x826E27C8;
	sub_82466E20(ctx, base);
	// 826E27C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E27CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E27D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E27D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E27D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E27D8 size=112
    let mut pc: u32 = 0x826E27D8;
    'dispatch: loop {
        match pc {
            0x826E27D8 => {
    //   block [0x826E27D8..0x826E2848)
	// 826E27D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E27DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E27E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E27E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E27E8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E27EC: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E27F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E27F4: 390BE228  addi r8, r11, -0x1dd8
	ctx.r[8].s64 = ctx.r[11].s64 + -7640;
	// 826E27F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E27FC: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 826E2800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2804: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E280C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2810: 386A1B4C  addi r3, r10, 0x1b4c
	ctx.r[3].s64 = ctx.r[10].s64 + 6988;
	// 826E2814: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E281C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E282C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2834: 4BD845ED  bl 0x82466e20
	ctx.lr = 0x826E2838;
	sub_82466E20(ctx, base);
	// 826E2838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E283C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2848 size=112
    let mut pc: u32 = 0x826E2848;
    'dispatch: loop {
        match pc {
            0x826E2848 => {
    //   block [0x826E2848..0x826E28B8)
	// 826E2848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E284C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2854: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2858: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E285C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E2860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2864: 390BE270  addi r8, r11, -0x1d90
	ctx.r[8].s64 = ctx.r[11].s64 + -7568;
	// 826E2868: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826E286C: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 826E2870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2874: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E287C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2880: 386A1B7C  addi r3, r10, 0x1b7c
	ctx.r[3].s64 = ctx.r[10].s64 + 7036;
	// 826E2884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E288C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E289C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E28A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E28A4: 4BD8457D  bl 0x82466e20
	ctx.lr = 0x826E28A8;
	sub_82466E20(ctx, base);
	// 826E28A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E28AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E28B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E28B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E28B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E28B8 size=116
    let mut pc: u32 = 0x826E28B8;
    'dispatch: loop {
        match pc {
            0x826E28B8 => {
    //   block [0x826E28B8..0x826E292C)
	// 826E28B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E28BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E28C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E28C4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E28C8: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826E28CC: 390AE318  addi r8, r10, -0x1ce8
	ctx.r[8].s64 = ctx.r[10].s64 + -7400;
	// 826E28D0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E28D4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E28D8: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E28DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E28E0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E28E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E28E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E28EC: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 826E28F0: 396B71B0  addi r11, r11, 0x71b0
	ctx.r[11].s64 = ctx.r[11].s64 + 29104;
	// 826E28F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E28F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E28FC: 386A1BAC  addi r3, r10, 0x1bac
	ctx.r[3].s64 = ctx.r[10].s64 + 7084;
	// 826E2900: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E2904: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2908: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E290C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2918: 4BD84509  bl 0x82466e20
	ctx.lr = 0x826E291C;
	sub_82466E20(ctx, base);
	// 826E291C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2930 size=112
    let mut pc: u32 = 0x826E2930;
    'dispatch: loop {
        match pc {
            0x826E2930 => {
    //   block [0x826E2930..0x826E29A0)
	// 826E2930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E293C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2940: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2944: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E2948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E294C: 390BE3D8  addi r8, r11, -0x1c28
	ctx.r[8].s64 = ctx.r[11].s64 + -7208;
	// 826E2950: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E2954: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 826E2958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E295C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E2964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2968: 386A1BDC  addi r3, r10, 0x1bdc
	ctx.r[3].s64 = ctx.r[10].s64 + 7132;
	// 826E296C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E297C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E298C: 4BD84495  bl 0x82466e20
	ctx.lr = 0x826E2990;
	sub_82466E20(ctx, base);
	// 826E2990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E299C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E29A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E29A0 size=100
    let mut pc: u32 = 0x826E29A0;
    'dispatch: loop {
        match pc {
            0x826E29A0 => {
    //   block [0x826E29A0..0x826E2A04)
	// 826E29A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E29A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E29A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E29AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E29B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E29B4: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E29B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E29BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E29C0: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 826E29C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E29C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E29CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E29D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E29D4: 386A1C0C  addi r3, r10, 0x1c0c
	ctx.r[3].s64 = ctx.r[10].s64 + 7180;
	// 826E29D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E29DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E29E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E29E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E29E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E29EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E29F0: 4BD84431  bl 0x82466e20
	ctx.lr = 0x826E29F4;
	sub_82466E20(ctx, base);
	// 826E29F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E29F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E29FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2A08 size=108
    let mut pc: u32 = 0x826E2A08;
    'dispatch: loop {
        match pc {
            0x826E2A08 => {
    //   block [0x826E2A08..0x826E2A74)
	// 826E2A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2A14: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2A18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2A1C: 38EBE408  addi r7, r11, -0x1bf8
	ctx.r[7].s64 = ctx.r[11].s64 + -7160;
	// 826E2A20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E2A24: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 826E2A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2A2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2A30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E2A34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2A38: 386A1C3C  addi r3, r10, 0x1c3c
	ctx.r[3].s64 = ctx.r[10].s64 + 7228;
	// 826E2A3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E2A40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2A44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2A5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2A60: 4BD843C1  bl 0x82466e20
	ctx.lr = 0x826E2A64;
	sub_82466E20(ctx, base);
	// 826E2A64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2A78 size=112
    let mut pc: u32 = 0x826E2A78;
    'dispatch: loop {
        match pc {
            0x826E2A78 => {
    //   block [0x826E2A78..0x826E2AE8)
	// 826E2A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2A84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2A88: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2A8C: 38AA1C0C  addi r5, r10, 0x1c0c
	ctx.r[5].s64 = ctx.r[10].s64 + 7180;
	// 826E2A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2A94: 390BE438  addi r8, r11, -0x1bc8
	ctx.r[8].s64 = ctx.r[11].s64 + -7112;
	// 826E2A98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E2A9C: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 826E2AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2AA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E2AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2AB0: 386A1C6C  addi r3, r10, 0x1c6c
	ctx.r[3].s64 = ctx.r[10].s64 + 7276;
	// 826E2AB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2AD4: 4BD8434D  bl 0x82466e20
	ctx.lr = 0x826E2AD8;
	sub_82466E20(ctx, base);
	// 826E2AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2AE8 size=108
    let mut pc: u32 = 0x826E2AE8;
    'dispatch: loop {
        match pc {
            0x826E2AE8 => {
    //   block [0x826E2AE8..0x826E2B54)
	// 826E2AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2AF4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2AF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E2AFC: 38EBE468  addi r7, r11, -0x1b98
	ctx.r[7].s64 = ctx.r[11].s64 + -7064;
	// 826E2B00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E2B04: 388AA93C  addi r4, r10, -0x56c4
	ctx.r[4].s64 = ctx.r[10].s64 + -22212;
	// 826E2B08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2B0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2B10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E2B14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2B18: 386A1C9C  addi r3, r10, 0x1c9c
	ctx.r[3].s64 = ctx.r[10].s64 + 7324;
	// 826E2B1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E2B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2B28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2B30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2B38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2B3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2B40: 4BD842E1  bl 0x82466e20
	ctx.lr = 0x826E2B44;
	sub_82466E20(ctx, base);
	// 826E2B44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2B58 size=112
    let mut pc: u32 = 0x826E2B58;
    'dispatch: loop {
        match pc {
            0x826E2B58 => {
    //   block [0x826E2B58..0x826E2BC8)
	// 826E2B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2B64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2B68: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2B6C: 38AA1C0C  addi r5, r10, 0x1c0c
	ctx.r[5].s64 = ctx.r[10].s64 + 7180;
	// 826E2B70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2B74: 390BE498  addi r8, r11, -0x1b68
	ctx.r[8].s64 = ctx.r[11].s64 + -7016;
	// 826E2B78: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E2B7C: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 826E2B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2B84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2B88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E2B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2B90: 386A1CCC  addi r3, r10, 0x1ccc
	ctx.r[3].s64 = ctx.r[10].s64 + 7372;
	// 826E2B94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2BB4: 4BD8426D  bl 0x82466e20
	ctx.lr = 0x826E2BB8;
	sub_82466E20(ctx, base);
	// 826E2BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2BC8 size=108
    let mut pc: u32 = 0x826E2BC8;
    'dispatch: loop {
        match pc {
            0x826E2BC8 => {
    //   block [0x826E2BC8..0x826E2C34)
	// 826E2BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2BD4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2BD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E2BDC: 38EBE4E0  addi r7, r11, -0x1b20
	ctx.r[7].s64 = ctx.r[11].s64 + -6944;
	// 826E2BE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E2BE4: 388AA960  addi r4, r10, -0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22176;
	// 826E2BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2BEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2BF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E2BF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2BF8: 386A1CFC  addi r3, r10, 0x1cfc
	ctx.r[3].s64 = ctx.r[10].s64 + 7420;
	// 826E2BFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E2C00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2C08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2C10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2C18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2C1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2C20: 4BD84201  bl 0x82466e20
	ctx.lr = 0x826E2C24;
	sub_82466E20(ctx, base);
	// 826E2C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2C38 size=112
    let mut pc: u32 = 0x826E2C38;
    'dispatch: loop {
        match pc {
            0x826E2C38 => {
    //   block [0x826E2C38..0x826E2CA8)
	// 826E2C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2C44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2C48: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2C4C: 38AA1C0C  addi r5, r10, 0x1c0c
	ctx.r[5].s64 = ctx.r[10].s64 + 7180;
	// 826E2C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2C54: 390BE510  addi r8, r11, -0x1af0
	ctx.r[8].s64 = ctx.r[11].s64 + -6896;
	// 826E2C58: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E2C5C: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 826E2C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2C64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E2C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2C70: 386A1D2C  addi r3, r10, 0x1d2c
	ctx.r[3].s64 = ctx.r[10].s64 + 7468;
	// 826E2C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2C94: 4BD8418D  bl 0x82466e20
	ctx.lr = 0x826E2C98;
	sub_82466E20(ctx, base);
	// 826E2C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2CA8 size=108
    let mut pc: u32 = 0x826E2CA8;
    'dispatch: loop {
        match pc {
            0x826E2CA8 => {
    //   block [0x826E2CA8..0x826E2D14)
	// 826E2CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2CB4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2CB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E2CBC: 38EBE558  addi r7, r11, -0x1aa8
	ctx.r[7].s64 = ctx.r[11].s64 + -6824;
	// 826E2CC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E2CC4: 388AA984  addi r4, r10, -0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + -22140;
	// 826E2CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2CCC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2CD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E2CD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2CD8: 386A1D5C  addi r3, r10, 0x1d5c
	ctx.r[3].s64 = ctx.r[10].s64 + 7516;
	// 826E2CDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E2CE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2CFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2D00: 4BD84121  bl 0x82466e20
	ctx.lr = 0x826E2D04;
	sub_82466E20(ctx, base);
	// 826E2D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2D18 size=112
    let mut pc: u32 = 0x826E2D18;
    'dispatch: loop {
        match pc {
            0x826E2D18 => {
    //   block [0x826E2D18..0x826E2D88)
	// 826E2D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2D24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2D28: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2D2C: 38AA1C0C  addi r5, r10, 0x1c0c
	ctx.r[5].s64 = ctx.r[10].s64 + 7180;
	// 826E2D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2D34: 390BE588  addi r8, r11, -0x1a78
	ctx.r[8].s64 = ctx.r[11].s64 + -6776;
	// 826E2D38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E2D3C: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 826E2D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2D44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E2D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2D50: 386A1D8C  addi r3, r10, 0x1d8c
	ctx.r[3].s64 = ctx.r[10].s64 + 7564;
	// 826E2D54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E2D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2D64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2D74: 4BD840AD  bl 0x82466e20
	ctx.lr = 0x826E2D78;
	sub_82466E20(ctx, base);
	// 826E2D78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2D88 size=108
    let mut pc: u32 = 0x826E2D88;
    'dispatch: loop {
        match pc {
            0x826E2D88 => {
    //   block [0x826E2D88..0x826E2DF4)
	// 826E2D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2D94: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2D98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2D9C: 38EBE5D8  addi r7, r11, -0x1a28
	ctx.r[7].s64 = ctx.r[11].s64 + -6696;
	// 826E2DA0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E2DA4: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 826E2DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2DAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2DB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E2DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2DB8: 386A1DBC  addi r3, r10, 0x1dbc
	ctx.r[3].s64 = ctx.r[10].s64 + 7612;
	// 826E2DBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E2DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2DDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2DE0: 4BD84041  bl 0x82466e20
	ctx.lr = 0x826E2DE4;
	sub_82466E20(ctx, base);
	// 826E2DE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E2DF8 size=24
    let mut pc: u32 = 0x826E2DF8;
    'dispatch: loop {
        match pc {
            0x826E2DF8 => {
    //   block [0x826E2DF8..0x826E2E10)
	// 826E2DF8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2DFC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E2E00: 394A6078  addi r10, r10, 0x6078
	ctx.r[10].s64 = ctx.r[10].s64 + 24696;
	// 826E2E04: 816BE5D4  lwz r11, -0x1a2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6700 as u32) ) } as u64;
	// 826E2E08: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 826E2E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2E10 size=112
    let mut pc: u32 = 0x826E2E10;
    'dispatch: loop {
        match pc {
            0x826E2E10 => {
    //   block [0x826E2E10..0x826E2E80)
	// 826E2E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2E1C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E2E20: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2E24: 392A7298  addi r9, r10, 0x7298
	ctx.r[9].s64 = ctx.r[10].s64 + 29336;
	// 826E2E28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2E2C: 390B6078  addi r8, r11, 0x6078
	ctx.r[8].s64 = ctx.r[11].s64 + 24696;
	// 826E2E30: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826E2E34: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 826E2E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2E3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2E40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E2E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2E48: 386A1DEC  addi r3, r10, 0x1dec
	ctx.r[3].s64 = ctx.r[10].s64 + 7660;
	// 826E2E4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E2E50: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E2E54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2E64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2E6C: 4BD83FB5  bl 0x82466e20
	ctx.lr = 0x826E2E70;
	sub_82466E20(ctx, base);
	// 826E2E70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2E80 size=108
    let mut pc: u32 = 0x826E2E80;
    'dispatch: loop {
        match pc {
            0x826E2E80 => {
    //   block [0x826E2E80..0x826E2EEC)
	// 826E2E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2E8C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2E94: 38EBE640  addi r7, r11, -0x19c0
	ctx.r[7].s64 = ctx.r[11].s64 + -6592;
	// 826E2E98: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826E2E9C: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 826E2EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2EA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2EA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E2EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2EB0: 386A1E1C  addi r3, r10, 0x1e1c
	ctx.r[3].s64 = ctx.r[10].s64 + 7708;
	// 826E2EB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E2EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2ED4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2ED8: 4BD83F49  bl 0x82466e20
	ctx.lr = 0x826E2EDC;
	sub_82466E20(ctx, base);
	// 826E2EDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2EF0 size=108
    let mut pc: u32 = 0x826E2EF0;
    'dispatch: loop {
        match pc {
            0x826E2EF0 => {
    //   block [0x826E2EF0..0x826E2F5C)
	// 826E2EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2EFC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2F00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E2F04: 38EBE6B8  addi r7, r11, -0x1948
	ctx.r[7].s64 = ctx.r[11].s64 + -6472;
	// 826E2F08: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E2F0C: 388AA9B8  addi r4, r10, -0x5648
	ctx.r[4].s64 = ctx.r[10].s64 + -22088;
	// 826E2F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2F14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2F18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E2F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2F20: 386A1E4C  addi r3, r10, 0x1e4c
	ctx.r[3].s64 = ctx.r[10].s64 + 7756;
	// 826E2F24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E2F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2F2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2F44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2F48: 4BD83ED9  bl 0x82466e20
	ctx.lr = 0x826E2F4C;
	sub_82466E20(ctx, base);
	// 826E2F4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2F60 size=108
    let mut pc: u32 = 0x826E2F60;
    'dispatch: loop {
        match pc {
            0x826E2F60 => {
    //   block [0x826E2F60..0x826E2FCC)
	// 826E2F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2F6C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E2F74: 38EBE718  addi r7, r11, -0x18e8
	ctx.r[7].s64 = ctx.r[11].s64 + -6376;
	// 826E2F78: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826E2F7C: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 826E2F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E2F84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2F88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E2F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E2F90: 386A1E7C  addi r3, r10, 0x1e7c
	ctx.r[3].s64 = ctx.r[10].s64 + 7804;
	// 826E2F94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E2F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E2F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E2FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E2FA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E2FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E2FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E2FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E2FB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E2FB8: 4BD83E69  bl 0x82466e20
	ctx.lr = 0x826E2FBC;
	sub_82466E20(ctx, base);
	// 826E2FBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E2FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E2FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E2FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E2FD0 size=24
    let mut pc: u32 = 0x826E2FD0;
    'dispatch: loop {
        match pc {
            0x826E2FD0 => {
    //   block [0x826E2FD0..0x826E2FE8)
	// 826E2FD0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E2FD4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E2FD8: 394A6180  addi r10, r10, 0x6180
	ctx.r[10].s64 = ctx.r[10].s64 + 24960;
	// 826E2FDC: 816BE5D0  lwz r11, -0x1a30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6704 as u32) ) } as u64;
	// 826E2FE0: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 826E2FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E2FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E2FE8 size=116
    let mut pc: u32 = 0x826E2FE8;
    'dispatch: loop {
        match pc {
            0x826E2FE8 => {
    //   block [0x826E2FE8..0x826E305C)
	// 826E2FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E2FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E2FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E2FF4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E2FF8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E2FFC: 392B71F8  addi r9, r11, 0x71f8
	ctx.r[9].s64 = ctx.r[11].s64 + 29176;
	// 826E3000: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E3004: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E3008: 38E900C8  addi r7, r9, 0xc8
	ctx.r[7].s64 = ctx.r[9].s64 + 200;
	// 826E300C: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 826E3010: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3014: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 826E3018: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E301C: 396B6180  addi r11, r11, 0x6180
	ctx.r[11].s64 = ctx.r[11].s64 + 24960;
	// 826E3020: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826E3024: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3028: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826E302C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3030: 386A1EAC  addi r3, r10, 0x1eac
	ctx.r[3].s64 = ctx.r[10].s64 + 7852;
	// 826E3034: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E3038: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826E303C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3040: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826E3044: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E3048: 4BD83DD9  bl 0x82466e20
	ctx.lr = 0x826E304C;
	sub_82466E20(ctx, base);
	// 826E304C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E3060 size=36
    let mut pc: u32 = 0x826E3060;
    'dispatch: loop {
        match pc {
            0x826E3060 => {
    //   block [0x826E3060..0x826E3084)
	// 826E3060: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3064: 814BE63C  lwz r10, -0x19c4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6596 as u32) ) } as u64;
	// 826E3068: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E306C: 396B6450  addi r11, r11, 0x6450
	ctx.r[11].s64 = ctx.r[11].s64 + 25680;
	// 826E3070: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826E3074: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E3078: 814AE7C0  lwz r10, -0x1840(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-6208 as u32) ) } as u64;
	// 826E307C: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826E3080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3088 size=116
    let mut pc: u32 = 0x826E3088;
    'dispatch: loop {
        match pc {
            0x826E3088 => {
    //   block [0x826E3088..0x826E30FC)
	// 826E3088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E308C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3094: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3098: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E309C: 390B6450  addi r8, r11, 0x6450
	ctx.r[8].s64 = ctx.r[11].s64 + 25680;
	// 826E30A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E30A4: 392A7380  addi r9, r10, 0x7380
	ctx.r[9].s64 = ctx.r[10].s64 + 29568;
	// 826E30A8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E30AC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826E30B0: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E30B4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E30B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E30BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E30C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E30C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E30C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E30CC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E30D0: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 826E30D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E30D8: 386B1EDC  addi r3, r11, 0x1edc
	ctx.r[3].s64 = ctx.r[11].s64 + 7900;
	// 826E30DC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E30E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E30E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E30E8: 4BD83D39  bl 0x82466e20
	ctx.lr = 0x826E30EC;
	sub_82466E20(ctx, base);
	// 826E30EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E30F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E30F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E30F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E3100 size=24
    let mut pc: u32 = 0x826E3100;
    'dispatch: loop {
        match pc {
            0x826E3100 => {
    //   block [0x826E3100..0x826E3118)
	// 826E3100: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3104: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E3108: 394A6480  addi r10, r10, 0x6480
	ctx.r[10].s64 = ctx.r[10].s64 + 25728;
	// 826E310C: 816BE7C8  lwz r11, -0x1838(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6200 as u32) ) } as u64;
	// 826E3110: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826E3114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3118 size=116
    let mut pc: u32 = 0x826E3118;
    'dispatch: loop {
        match pc {
            0x826E3118 => {
    //   block [0x826E3118..0x826E318C)
	// 826E3118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E311C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3124: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3128: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E312C: 390B6480  addi r8, r11, 0x6480
	ctx.r[8].s64 = ctx.r[11].s64 + 25728;
	// 826E3130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3134: 392A73D8  addi r9, r10, 0x73d8
	ctx.r[9].s64 = ctx.r[10].s64 + 29656;
	// 826E3138: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E313C: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 826E3140: 38AA1EDC  addi r5, r10, 0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + 7900;
	// 826E3144: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3148: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E314C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E3150: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E315C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E3160: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 826E3164: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E3168: 386B1F0C  addi r3, r11, 0x1f0c
	ctx.r[3].s64 = ctx.r[11].s64 + 7948;
	// 826E316C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E3170: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3178: 4BD83CA9  bl 0x82466e20
	ctx.lr = 0x826E317C;
	sub_82466E20(ctx, base);
	// 826E317C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3190 size=116
    let mut pc: u32 = 0x826E3190;
    'dispatch: loop {
        match pc {
            0x826E3190 => {
    //   block [0x826E3190..0x826E3204)
	// 826E3190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E319C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E31A0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E31A4: 390BE7D0  addi r8, r11, -0x1830
	ctx.r[8].s64 = ctx.r[11].s64 + -6192;
	// 826E31A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E31AC: 392A7420  addi r9, r10, 0x7420
	ctx.r[9].s64 = ctx.r[10].s64 + 29728;
	// 826E31B0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E31B4: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826E31B8: 38AA1EDC  addi r5, r10, 0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + 7900;
	// 826E31BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E31C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E31C4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E31C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E31CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E31D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E31D4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E31D8: 388AAAE8  addi r4, r10, -0x5518
	ctx.r[4].s64 = ctx.r[10].s64 + -21784;
	// 826E31DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E31E0: 386B1F3C  addi r3, r11, 0x1f3c
	ctx.r[3].s64 = ctx.r[11].s64 + 7996;
	// 826E31E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E31E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E31EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E31F0: 4BD83C31  bl 0x82466e20
	ctx.lr = 0x826E31F4;
	sub_82466E20(ctx, base);
	// 826E31F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E31F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E31FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3208 size=112
    let mut pc: u32 = 0x826E3208;
    'dispatch: loop {
        match pc {
            0x826E3208 => {
    //   block [0x826E3208..0x826E3278)
	// 826E3208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E320C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3214: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E3218: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826E321C: 38EAE8A8  addi r7, r10, -0x1758
	ctx.r[7].s64 = ctx.r[10].s64 + -5976;
	// 826E3220: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E3224: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E3228: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 826E322C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3230: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E3234: 396B7434  addi r11, r11, 0x7434
	ctx.r[11].s64 = ctx.r[11].s64 + 29748;
	// 826E3238: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E323C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3240: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3244: 386A1F6C  addi r3, r10, 0x1f6c
	ctx.r[3].s64 = ctx.r[10].s64 + 8044;
	// 826E3248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E324C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E3250: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3254: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E3258: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E325C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3260: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E3264: 4BD83BBD  bl 0x82466e20
	ctx.lr = 0x826E3268;
	sub_82466E20(ctx, base);
	// 826E3268: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E326C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3278 size=112
    let mut pc: u32 = 0x826E3278;
    'dispatch: loop {
        match pc {
            0x826E3278 => {
    //   block [0x826E3278..0x826E32E8)
	// 826E3278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E327C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3284: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3288: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E328C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E3290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E3294: 390BE938  addi r8, r11, -0x16c8
	ctx.r[8].s64 = ctx.r[11].s64 + -5832;
	// 826E3298: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E329C: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 826E32A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E32A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E32A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E32AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E32B0: 386A1F9C  addi r3, r10, 0x1f9c
	ctx.r[3].s64 = ctx.r[10].s64 + 8092;
	// 826E32B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E32B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E32BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E32C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E32C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E32C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E32CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E32D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E32D4: 4BD83B4D  bl 0x82466e20
	ctx.lr = 0x826E32D8;
	sub_82466E20(ctx, base);
	// 826E32D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E32DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E32E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E32E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E32E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E32E8 size=24
    let mut pc: u32 = 0x826E32E8;
    'dispatch: loop {
        match pc {
            0x826E32E8 => {
    //   block [0x826E32E8..0x826E3300)
	// 826E32E8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E32EC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E32F0: 394A65A0  addi r10, r10, 0x65a0
	ctx.r[10].s64 = ctx.r[10].s64 + 26016;
	// 826E32F4: 816BE950  lwz r11, -0x16b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5808 as u32) ) } as u64;
	// 826E32F8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826E32FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3300 size=112
    let mut pc: u32 = 0x826E3300;
    'dispatch: loop {
        match pc {
            0x826E3300 => {
    //   block [0x826E3300..0x826E3370)
	// 826E3300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E330C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E3310: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3314: 392A7480  addi r9, r10, 0x7480
	ctx.r[9].s64 = ctx.r[10].s64 + 29824;
	// 826E3318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E331C: 390B65A0  addi r8, r11, 0x65a0
	ctx.r[8].s64 = ctx.r[11].s64 + 26016;
	// 826E3320: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826E3324: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 826E3328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E332C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3330: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3338: 386A1FCC  addi r3, r10, 0x1fcc
	ctx.r[3].s64 = ctx.r[10].s64 + 8140;
	// 826E333C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E3340: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E3344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E334C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E3358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E335C: 4BD83AC5  bl 0x82466e20
	ctx.lr = 0x826E3360;
	sub_82466E20(ctx, base);
	// 826E3360: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E336C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3370 size=108
    let mut pc: u32 = 0x826E3370;
    'dispatch: loop {
        match pc {
            0x826E3370 => {
    //   block [0x826E3370..0x826E33DC)
	// 826E3370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E337C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E3384: 38EBE958  addi r7, r11, -0x16a8
	ctx.r[7].s64 = ctx.r[11].s64 + -5800;
	// 826E3388: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E338C: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 826E3390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3394: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3398: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E339C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E33A0: 386A1FFC  addi r3, r10, 0x1ffc
	ctx.r[3].s64 = ctx.r[10].s64 + 8188;
	// 826E33A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E33A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E33AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E33B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E33B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E33B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E33BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E33C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E33C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E33C8: 4BD83A59  bl 0x82466e20
	ctx.lr = 0x826E33CC;
	sub_82466E20(ctx, base);
	// 826E33CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E33D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E33D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E33D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E33E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E33E0 size=108
    let mut pc: u32 = 0x826E33E0;
    'dispatch: loop {
        match pc {
            0x826E33E0 => {
    //   block [0x826E33E0..0x826E344C)
	// 826E33E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E33E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E33E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E33EC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E33F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E33F4: 38EBE9B8  addi r7, r11, -0x1648
	ctx.r[7].s64 = ctx.r[11].s64 + -5704;
	// 826E33F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E33FC: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 826E3400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3404: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3408: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E340C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3410: 386A202C  addi r3, r10, 0x202c
	ctx.r[3].s64 = ctx.r[10].s64 + 8236;
	// 826E3414: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E3418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E341C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E342C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3434: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E3438: 4BD839E9  bl 0x82466e20
	ctx.lr = 0x826E343C;
	sub_82466E20(ctx, base);
	// 826E343C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3450 size=116
    let mut pc: u32 = 0x826E3450;
    'dispatch: loop {
        match pc {
            0x826E3450 => {
    //   block [0x826E3450..0x826E34C4)
	// 826E3450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E345C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3460: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E3464: 390BE9E8  addi r8, r11, -0x1618
	ctx.r[8].s64 = ctx.r[11].s64 + -5656;
	// 826E3468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E346C: 392A74AC  addi r9, r10, 0x74ac
	ctx.r[9].s64 = ctx.r[10].s64 + 29868;
	// 826E3470: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3474: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826E3478: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E347C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3484: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E3488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E348C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3494: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E3498: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 826E349C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E34A0: 386B205C  addi r3, r11, 0x205c
	ctx.r[3].s64 = ctx.r[11].s64 + 8284;
	// 826E34A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E34A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E34AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E34B0: 4BD83971  bl 0x82466e20
	ctx.lr = 0x826E34B4;
	sub_82466E20(ctx, base);
	// 826E34B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E34B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E34BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E34C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E34C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E34C8 size=108
    let mut pc: u32 = 0x826E34C8;
    'dispatch: loop {
        match pc {
            0x826E34C8 => {
    //   block [0x826E34C8..0x826E3534)
	// 826E34C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E34CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E34D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E34D4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E34D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E34DC: 38EBEA00  addi r7, r11, -0x1600
	ctx.r[7].s64 = ctx.r[11].s64 + -5632;
	// 826E34E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E34E4: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 826E34E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E34EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E34F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E34F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E34F8: 386A208C  addi r3, r10, 0x208c
	ctx.r[3].s64 = ctx.r[10].s64 + 8332;
	// 826E34FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E3500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E350C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E351C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E3520: 4BD83901  bl 0x82466e20
	ctx.lr = 0x826E3524;
	sub_82466E20(ctx, base);
	// 826E3524: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E352C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3538 size=112
    let mut pc: u32 = 0x826E3538;
    'dispatch: loop {
        match pc {
            0x826E3538 => {
    //   block [0x826E3538..0x826E35A8)
	// 826E3538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E353C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3544: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3548: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E354C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E3550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E3554: 390BEA18  addi r8, r11, -0x15e8
	ctx.r[8].s64 = ctx.r[11].s64 + -5608;
	// 826E3558: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826E355C: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 826E3560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3564: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3568: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E356C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3570: 386A20BC  addi r3, r10, 0x20bc
	ctx.r[3].s64 = ctx.r[10].s64 + 8380;
	// 826E3574: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E3578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E357C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E358C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3594: 4BD8388D  bl 0x82466e20
	ctx.lr = 0x826E3598;
	sub_82466E20(ctx, base);
	// 826E3598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E359C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E35A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E35A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E35A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E35A8 size=108
    let mut pc: u32 = 0x826E35A8;
    'dispatch: loop {
        match pc {
            0x826E35A8 => {
    //   block [0x826E35A8..0x826E3614)
	// 826E35A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E35AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E35B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E35B4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E35B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E35BC: 38EBEAF0  addi r7, r11, -0x1510
	ctx.r[7].s64 = ctx.r[11].s64 + -5392;
	// 826E35C0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826E35C4: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 826E35C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E35CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E35D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E35D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E35D8: 386A20EC  addi r3, r10, 0x20ec
	ctx.r[3].s64 = ctx.r[10].s64 + 8428;
	// 826E35DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E35E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E35E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E35E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E35EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E35F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E35F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E35F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E35FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E3600: 4BD83821  bl 0x82466e20
	ctx.lr = 0x826E3604;
	sub_82466E20(ctx, base);
	// 826E3604: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E360C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3618 size=108
    let mut pc: u32 = 0x826E3618;
    'dispatch: loop {
        match pc {
            0x826E3618 => {
    //   block [0x826E3618..0x826E3684)
	// 826E3618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E361C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3624: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E362C: 38EBEB68  addi r7, r11, -0x1498
	ctx.r[7].s64 = ctx.r[11].s64 + -5272;
	// 826E3630: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826E3634: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 826E3638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E363C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3640: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E3644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3648: 386A211C  addi r3, r10, 0x211c
	ctx.r[3].s64 = ctx.r[10].s64 + 8476;
	// 826E364C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E3650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E365C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E366C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E3670: 4BD837B1  bl 0x82466e20
	ctx.lr = 0x826E3674;
	sub_82466E20(ctx, base);
	// 826E3674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E367C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3688 size=116
    let mut pc: u32 = 0x826E3688;
    'dispatch: loop {
        match pc {
            0x826E3688 => {
    //   block [0x826E3688..0x826E36FC)
	// 826E3688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E368C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3694: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E3698: 38E00016  li r7, 0x16
	ctx.r[7].s64 = 22;
	// 826E369C: 390AEBB0  addi r8, r10, -0x1450
	ctx.r[8].s64 = ctx.r[10].s64 + -5200;
	// 826E36A0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E36A4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E36A8: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E36AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E36B0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E36B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E36B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E36BC: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 826E36C0: 396B74C0  addi r11, r11, 0x74c0
	ctx.r[11].s64 = ctx.r[11].s64 + 29888;
	// 826E36C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E36C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E36CC: 386A214C  addi r3, r10, 0x214c
	ctx.r[3].s64 = ctx.r[10].s64 + 8524;
	// 826E36D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E36D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E36D8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E36DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E36E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E36E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E36E8: 4BD83739  bl 0x82466e20
	ctx.lr = 0x826E36EC;
	sub_82466E20(ctx, base);
	// 826E36EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E36F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E36F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E36F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3700 size=112
    let mut pc: u32 = 0x826E3700;
    'dispatch: loop {
        match pc {
            0x826E3700 => {
    //   block [0x826E3700..0x826E3770)
	// 826E3700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E370C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3710: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3714: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E3718: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E371C: 390BEDC0  addi r8, r11, -0x1240
	ctx.r[8].s64 = ctx.r[11].s64 + -4672;
	// 826E3720: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E3724: 388AB374  addi r4, r10, -0x4c8c
	ctx.r[4].s64 = ctx.r[10].s64 + -19596;
	// 826E3728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E372C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3730: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3738: 386A217C  addi r3, r10, 0x217c
	ctx.r[3].s64 = ctx.r[10].s64 + 8572;
	// 826E373C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E3740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E374C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E375C: 4BD836C5  bl 0x82466e20
	ctx.lr = 0x826E3760;
	sub_82466E20(ctx, base);
	// 826E3760: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E376C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E3770 size=76
    let mut pc: u32 = 0x826E3770;
    'dispatch: loop {
        match pc {
            0x826E3770 => {
    //   block [0x826E3770..0x826E37BC)
	// 826E3770: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3774: 814BEDD8  lwz r10, -0x1228(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4648 as u32) ) } as u64;
	// 826E3778: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E377C: 396B65D0  addi r11, r11, 0x65d0
	ctx.r[11].s64 = ctx.r[11].s64 + 26064;
	// 826E3780: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 826E3784: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826E3788: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 826E378C: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826E3790: 914B0158  stw r10, 0x158(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 826E3794: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 826E3798: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E379C: 814AEDDC  lwz r10, -0x1224(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4644 as u32) ) } as u64;
	// 826E37A0: 914B0188  stw r10, 0x188(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 826E37A4: 914B01A0  stw r10, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[10].u32 ) };
	// 826E37A8: 914B01B8  stw r10, 0x1b8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 826E37AC: 914B01D0  stw r10, 0x1d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 826E37B0: 914B01E8  stw r10, 0x1e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), ctx.r[10].u32 ) };
	// 826E37B4: 914B0200  stw r10, 0x200(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(512 as u32), ctx.r[10].u32 ) };
	// 826E37B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E37C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E37C0 size=108
    let mut pc: u32 = 0x826E37C0;
    'dispatch: loop {
        match pc {
            0x826E37C0 => {
    //   block [0x826E37C0..0x826E382C)
	// 826E37C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E37C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E37C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E37CC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E37D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E37D4: 38EB65D0  addi r7, r11, 0x65d0
	ctx.r[7].s64 = ctx.r[11].s64 + 26064;
	// 826E37D8: 3900001A  li r8, 0x1a
	ctx.r[8].s64 = 26;
	// 826E37DC: 388AABE8  addi r4, r10, -0x5418
	ctx.r[4].s64 = ctx.r[10].s64 + -21528;
	// 826E37E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E37E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E37E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E37EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E37F0: 386A21AC  addi r3, r10, 0x21ac
	ctx.r[3].s64 = ctx.r[10].s64 + 8620;
	// 826E37F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E37F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E37FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E380C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3814: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E3818: 4BD83609  bl 0x82466e20
	ctx.lr = 0x826E381C;
	sub_82466E20(ctx, base);
	// 826E381C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E3830 size=76
    let mut pc: u32 = 0x826E3830;
    'dispatch: loop {
        match pc {
            0x826E3830 => {
    //   block [0x826E3830..0x826E387C)
	// 826E3830: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3834: 814BEDD8  lwz r10, -0x1228(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4648 as u32) ) } as u64;
	// 826E3838: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E383C: 396B6840  addi r11, r11, 0x6840
	ctx.r[11].s64 = ctx.r[11].s64 + 26688;
	// 826E3840: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 826E3844: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826E3848: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 826E384C: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826E3850: 914B0158  stw r10, 0x158(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 826E3854: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 826E3858: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E385C: 814AEDDC  lwz r10, -0x1224(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4644 as u32) ) } as u64;
	// 826E3860: 914B0188  stw r10, 0x188(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 826E3864: 914B01A0  stw r10, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[10].u32 ) };
	// 826E3868: 914B01B8  stw r10, 0x1b8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 826E386C: 914B01D0  stw r10, 0x1d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 826E3870: 914B01E8  stw r10, 0x1e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), ctx.r[10].u32 ) };
	// 826E3874: 914B0200  stw r10, 0x200(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(512 as u32), ctx.r[10].u32 ) };
	// 826E3878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3880 size=116
    let mut pc: u32 = 0x826E3880;
    'dispatch: loop {
        match pc {
            0x826E3880 => {
    //   block [0x826E3880..0x826E38F4)
	// 826E3880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E388C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3890: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E3894: 390B6840  addi r8, r11, 0x6840
	ctx.r[8].s64 = ctx.r[11].s64 + 26688;
	// 826E3898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E389C: 392A7554  addi r9, r10, 0x7554
	ctx.r[9].s64 = ctx.r[10].s64 + 30036;
	// 826E38A0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E38A4: 38E00028  li r7, 0x28
	ctx.r[7].s64 = 40;
	// 826E38A8: 38AA14BC  addi r5, r10, 0x14bc
	ctx.r[5].s64 = ctx.r[10].s64 + 5308;
	// 826E38AC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E38B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E38B4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E38B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E38BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E38C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E38C4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E38C8: 388AACDC  addi r4, r10, -0x5324
	ctx.r[4].s64 = ctx.r[10].s64 + -21284;
	// 826E38CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E38D0: 386B21DC  addi r3, r11, 0x21dc
	ctx.r[3].s64 = ctx.r[11].s64 + 8668;
	// 826E38D4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E38D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E38DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E38E0: 4BD83541  bl 0x82466e20
	ctx.lr = 0x826E38E4;
	sub_82466E20(ctx, base);
	// 826E38E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E38E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E38EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E38F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E38F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E38F8 size=112
    let mut pc: u32 = 0x826E38F8;
    'dispatch: loop {
        match pc {
            0x826E38F8 => {
    //   block [0x826E38F8..0x826E3968)
	// 826E38F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E38FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3904: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3908: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E390C: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E3910: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3914: 390BEDE0  addi r8, r11, -0x1220
	ctx.r[8].s64 = ctx.r[11].s64 + -4640;
	// 826E3918: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E391C: 388AACF4  addi r4, r10, -0x530c
	ctx.r[4].s64 = ctx.r[10].s64 + -21260;
	// 826E3920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3924: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3928: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E392C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3930: 386A220C  addi r3, r10, 0x220c
	ctx.r[3].s64 = ctx.r[10].s64 + 8716;
	// 826E3934: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E3938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E393C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E394C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3954: 4BD834CD  bl 0x82466e20
	ctx.lr = 0x826E3958;
	sub_82466E20(ctx, base);
	// 826E3958: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E395C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3968 size=108
    let mut pc: u32 = 0x826E3968;
    'dispatch: loop {
        match pc {
            0x826E3968 => {
    //   block [0x826E3968..0x826E39D4)
	// 826E3968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E396C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3974: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3978: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E397C: 38EBEE28  addi r7, r11, -0x11d8
	ctx.r[7].s64 = ctx.r[11].s64 + -4568;
	// 826E3980: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826E3984: 388AAD10  addi r4, r10, -0x52f0
	ctx.r[4].s64 = ctx.r[10].s64 + -21232;
	// 826E3988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E398C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E3994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3998: 386A223C  addi r3, r10, 0x223c
	ctx.r[3].s64 = ctx.r[10].s64 + 8764;
	// 826E399C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E39A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E39A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E39A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E39AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E39B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E39B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E39B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E39BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E39C0: 4BD83461  bl 0x82466e20
	ctx.lr = 0x826E39C4;
	sub_82466E20(ctx, base);
	// 826E39C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E39C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E39CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E39D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E39D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E39D8 size=108
    let mut pc: u32 = 0x826E39D8;
    'dispatch: loop {
        match pc {
            0x826E39D8 => {
    //   block [0x826E39D8..0x826E3A44)
	// 826E39D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E39DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E39E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E39E4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E39E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E39EC: 38EBEE70  addi r7, r11, -0x1190
	ctx.r[7].s64 = ctx.r[11].s64 + -4496;
	// 826E39F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826E39F4: 388AAD38  addi r4, r10, -0x52c8
	ctx.r[4].s64 = ctx.r[10].s64 + -21192;
	// 826E39F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E39FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3A00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E3A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3A08: 386A226C  addi r3, r10, 0x226c
	ctx.r[3].s64 = ctx.r[10].s64 + 8812;
	// 826E3A0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E3A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3A14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3A2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E3A30: 4BD833F1  bl 0x82466e20
	ctx.lr = 0x826E3A34;
	sub_82466E20(ctx, base);
	// 826E3A34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3A48 size=116
    let mut pc: u32 = 0x826E3A48;
    'dispatch: loop {
        match pc {
            0x826E3A48 => {
    //   block [0x826E3A48..0x826E3ABC)
	// 826E3A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3A54: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E3A58: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826E3A5C: 390AEEB8  addi r8, r10, -0x1148
	ctx.r[8].s64 = ctx.r[10].s64 + -4424;
	// 826E3A60: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3A64: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E3A68: 38AA127C  addi r5, r10, 0x127c
	ctx.r[5].s64 = ctx.r[10].s64 + 4732;
	// 826E3A6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3A70: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E3A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3A78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3A7C: 388AAD60  addi r4, r10, -0x52a0
	ctx.r[4].s64 = ctx.r[10].s64 + -21152;
	// 826E3A80: 396B757C  addi r11, r11, 0x757c
	ctx.r[11].s64 = ctx.r[11].s64 + 30076;
	// 826E3A84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3A88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3A8C: 386A229C  addi r3, r10, 0x229c
	ctx.r[3].s64 = ctx.r[10].s64 + 8860;
	// 826E3A90: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E3A94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3A98: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E3A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3AA8: 4BD83379  bl 0x82466e20
	ctx.lr = 0x826E3AAC;
	sub_82466E20(ctx, base);
	// 826E3AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3AC0 size=116
    let mut pc: u32 = 0x826E3AC0;
    'dispatch: loop {
        match pc {
            0x826E3AC0 => {
    //   block [0x826E3AC0..0x826E3B34)
	// 826E3AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3ACC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E3AD0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826E3AD4: 390AEF60  addi r8, r10, -0x10a0
	ctx.r[8].s64 = ctx.r[10].s64 + -4256;
	// 826E3AD8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3ADC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E3AE0: 38AA229C  addi r5, r10, 0x229c
	ctx.r[5].s64 = ctx.r[10].s64 + 8860;
	// 826E3AE4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3AE8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E3AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3AF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3AF4: 388AAD7C  addi r4, r10, -0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + -21124;
	// 826E3AF8: 396B75A0  addi r11, r11, 0x75a0
	ctx.r[11].s64 = ctx.r[11].s64 + 30112;
	// 826E3AFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3B00: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3B04: 386A22CC  addi r3, r10, 0x22cc
	ctx.r[3].s64 = ctx.r[10].s64 + 8908;
	// 826E3B08: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E3B0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3B10: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E3B14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3B1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3B20: 4BD83301  bl 0x82466e20
	ctx.lr = 0x826E3B24;
	sub_82466E20(ctx, base);
	// 826E3B24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E3B38 size=36
    let mut pc: u32 = 0x826E3B38;
    'dispatch: loop {
        match pc {
            0x826E3B38 => {
    //   block [0x826E3B38..0x826E3B5C)
	// 826E3B38: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3B3C: 814BEFA8  lwz r10, -0x1058(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4184 as u32) ) } as u64;
	// 826E3B40: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3B44: 396B6C00  addi r11, r11, 0x6c00
	ctx.r[11].s64 = ctx.r[11].s64 + 27648;
	// 826E3B48: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826E3B4C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E3B50: 814AEFAC  lwz r10, -0x1054(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4180 as u32) ) } as u64;
	// 826E3B54: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826E3B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3B60 size=116
    let mut pc: u32 = 0x826E3B60;
    'dispatch: loop {
        match pc {
            0x826E3B60 => {
    //   block [0x826E3B60..0x826E3BD4)
	// 826E3B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3B6C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3B70: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E3B74: 390B6C00  addi r8, r11, 0x6c00
	ctx.r[8].s64 = ctx.r[11].s64 + 27648;
	// 826E3B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3B7C: 392A75E0  addi r9, r10, 0x75e0
	ctx.r[9].s64 = ctx.r[10].s64 + 30176;
	// 826E3B80: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3B84: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826E3B88: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E3B8C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3B94: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3BA4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E3BA8: 388AAE18  addi r4, r10, -0x51e8
	ctx.r[4].s64 = ctx.r[10].s64 + -20968;
	// 826E3BAC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E3BB0: 386B22FC  addi r3, r11, 0x22fc
	ctx.r[3].s64 = ctx.r[11].s64 + 8956;
	// 826E3BB4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E3BB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3BC0: 4BD83261  bl 0x82466e20
	ctx.lr = 0x826E3BC4;
	sub_82466E20(ctx, base);
	// 826E3BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3BD8 size=112
    let mut pc: u32 = 0x826E3BD8;
    'dispatch: loop {
        match pc {
            0x826E3BD8 => {
    //   block [0x826E3BD8..0x826E3C48)
	// 826E3BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3BE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3BE8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3BEC: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E3BF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3BF4: 390BEFB0  addi r8, r11, -0x1050
	ctx.r[8].s64 = ctx.r[11].s64 + -4176;
	// 826E3BF8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826E3BFC: 388AAE30  addi r4, r10, -0x51d0
	ctx.r[4].s64 = ctx.r[10].s64 + -20944;
	// 826E3C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3C04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3C08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3C10: 386A232C  addi r3, r10, 0x232c
	ctx.r[3].s64 = ctx.r[10].s64 + 9004;
	// 826E3C14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E3C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3C34: 4BD831ED  bl 0x82466e20
	ctx.lr = 0x826E3C38;
	sub_82466E20(ctx, base);
	// 826E3C38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3C48 size=108
    let mut pc: u32 = 0x826E3C48;
    'dispatch: loop {
        match pc {
            0x826E3C48 => {
    //   block [0x826E3C48..0x826E3CB4)
	// 826E3C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3C54: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3C58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3C5C: 38EBF070  addi r7, r11, -0xf90
	ctx.r[7].s64 = ctx.r[11].s64 + -3984;
	// 826E3C60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E3C64: 388AAE50  addi r4, r10, -0x51b0
	ctx.r[4].s64 = ctx.r[10].s64 + -20912;
	// 826E3C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3C6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3C70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E3C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3C78: 386A235C  addi r3, r10, 0x235c
	ctx.r[3].s64 = ctx.r[10].s64 + 9052;
	// 826E3C7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E3C80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3C9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E3CA0: 4BD83181  bl 0x82466e20
	ctx.lr = 0x826E3CA4;
	sub_82466E20(ctx, base);
	// 826E3CA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3CB8 size=112
    let mut pc: u32 = 0x826E3CB8;
    'dispatch: loop {
        match pc {
            0x826E3CB8 => {
    //   block [0x826E3CB8..0x826E3D28)
	// 826E3CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3CC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3CC8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3CCC: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E3CD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3CD4: 390BF0A0  addi r8, r11, -0xf60
	ctx.r[8].s64 = ctx.r[11].s64 + -3936;
	// 826E3CD8: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 826E3CDC: 388AAE6C  addi r4, r10, -0x5194
	ctx.r[4].s64 = ctx.r[10].s64 + -20884;
	// 826E3CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3CE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3CE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3CF0: 386A238C  addi r3, r10, 0x238c
	ctx.r[3].s64 = ctx.r[10].s64 + 9100;
	// 826E3CF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E3CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3D14: 4BD8310D  bl 0x82466e20
	ctx.lr = 0x826E3D18;
	sub_82466E20(ctx, base);
	// 826E3D18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3D28 size=112
    let mut pc: u32 = 0x826E3D28;
    'dispatch: loop {
        match pc {
            0x826E3D28 => {
    //   block [0x826E3D28..0x826E3D98)
	// 826E3D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3D30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3D34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3D38: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3D3C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E3D40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3D44: 390BF1A8  addi r8, r11, -0xe58
	ctx.r[8].s64 = ctx.r[11].s64 + -3672;
	// 826E3D48: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 826E3D4C: 388AAE80  addi r4, r10, -0x5180
	ctx.r[4].s64 = ctx.r[10].s64 + -20864;
	// 826E3D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3D54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3D58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3D60: 386A23BC  addi r3, r10, 0x23bc
	ctx.r[3].s64 = ctx.r[10].s64 + 9148;
	// 826E3D64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E3D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3D84: 4BD8309D  bl 0x82466e20
	ctx.lr = 0x826E3D88;
	sub_82466E20(ctx, base);
	// 826E3D88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3D98 size=116
    let mut pc: u32 = 0x826E3D98;
    'dispatch: loop {
        match pc {
            0x826E3D98 => {
    //   block [0x826E3D98..0x826E3E0C)
	// 826E3D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3DA4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3DA8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E3DAC: 390BF2E8  addi r8, r11, -0xd18
	ctx.r[8].s64 = ctx.r[11].s64 + -3352;
	// 826E3DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3DB4: 392A7618  addi r9, r10, 0x7618
	ctx.r[9].s64 = ctx.r[10].s64 + 30232;
	// 826E3DB8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3DBC: 38E0001D  li r7, 0x1d
	ctx.r[7].s64 = 29;
	// 826E3DC0: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E3DC4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3DCC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3DDC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E3DE0: 388AAE98  addi r4, r10, -0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + -20840;
	// 826E3DE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E3DE8: 386B23EC  addi r3, r11, 0x23ec
	ctx.r[3].s64 = ctx.r[11].s64 + 9196;
	// 826E3DEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E3DF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3DF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3DF8: 4BD83029  bl 0x82466e20
	ctx.lr = 0x826E3DFC;
	sub_82466E20(ctx, base);
	// 826E3DFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3E10 size=112
    let mut pc: u32 = 0x826E3E10;
    'dispatch: loop {
        match pc {
            0x826E3E10 => {
    //   block [0x826E3E10..0x826E3E80)
	// 826E3E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3E1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3E20: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3E24: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E3E28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3E2C: 390BF5A0  addi r8, r11, -0xa60
	ctx.r[8].s64 = ctx.r[11].s64 + -2656;
	// 826E3E30: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E3E34: 388AAEB0  addi r4, r10, -0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + -20816;
	// 826E3E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3E3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3E40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3E48: 386A241C  addi r3, r10, 0x241c
	ctx.r[3].s64 = ctx.r[10].s64 + 9244;
	// 826E3E4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E3E50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3E6C: 4BD82FB5  bl 0x82466e20
	ctx.lr = 0x826E3E70;
	sub_82466E20(ctx, base);
	// 826E3E70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3E80 size=116
    let mut pc: u32 = 0x826E3E80;
    'dispatch: loop {
        match pc {
            0x826E3E80 => {
    //   block [0x826E3E80..0x826E3EF4)
	// 826E3E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3E8C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E3E90: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826E3E94: 390AF5E8  addi r8, r10, -0xa18
	ctx.r[8].s64 = ctx.r[10].s64 + -2584;
	// 826E3E98: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3E9C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E3EA0: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E3EA4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3EA8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E3EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3EB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3EB4: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826E3EB8: 396B762C  addi r11, r11, 0x762c
	ctx.r[11].s64 = ctx.r[11].s64 + 30252;
	// 826E3EBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3EC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3EC4: 386A244C  addi r3, r10, 0x244c
	ctx.r[3].s64 = ctx.r[10].s64 + 9292;
	// 826E3EC8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E3ECC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3ED0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E3ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3EE0: 4BD82F41  bl 0x82466e20
	ctx.lr = 0x826E3EE4;
	sub_82466E20(ctx, base);
	// 826E3EE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3EF8 size=112
    let mut pc: u32 = 0x826E3EF8;
    'dispatch: loop {
        match pc {
            0x826E3EF8 => {
    //   block [0x826E3EF8..0x826E3F68)
	// 826E3EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3F04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3F08: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3F0C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E3F10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3F14: 390BF630  addi r8, r11, -0x9d0
	ctx.r[8].s64 = ctx.r[11].s64 + -2512;
	// 826E3F18: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826E3F1C: 388AAEF4  addi r4, r10, -0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + -20748;
	// 826E3F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3F24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3F28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3F2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3F30: 386A247C  addi r3, r10, 0x247c
	ctx.r[3].s64 = ctx.r[10].s64 + 9340;
	// 826E3F34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E3F38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3F40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3F48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3F50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3F54: 4BD82ECD  bl 0x82466e20
	ctx.lr = 0x826E3F58;
	sub_82466E20(ctx, base);
	// 826E3F58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3F68 size=112
    let mut pc: u32 = 0x826E3F68;
    'dispatch: loop {
        match pc {
            0x826E3F68 => {
    //   block [0x826E3F68..0x826E3FD8)
	// 826E3F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3F74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3F78: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3F7C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E3F80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E3F84: 390BF6C0  addi r8, r11, -0x940
	ctx.r[8].s64 = ctx.r[11].s64 + -2368;
	// 826E3F88: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826E3F8C: 388AAF08  addi r4, r10, -0x50f8
	ctx.r[4].s64 = ctx.r[10].s64 + -20728;
	// 826E3F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E3F94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E3F98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E3F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E3FA0: 386A24AC  addi r3, r10, 0x24ac
	ctx.r[3].s64 = ctx.r[10].s64 + 9388;
	// 826E3FA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E3FA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E3FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E3FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E3FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E3FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E3FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E3FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E3FC4: 4BD82E5D  bl 0x82466e20
	ctx.lr = 0x826E3FC8;
	sub_82466E20(ctx, base);
	// 826E3FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E3FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E3FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E3FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E3FD8 size=24
    let mut pc: u32 = 0x826E3FD8;
    'dispatch: loop {
        match pc {
            0x826E3FD8 => {
    //   block [0x826E3FD8..0x826E3FF0)
	// 826E3FD8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E3FDC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E3FE0: 394A6D08  addi r10, r10, 0x6d08
	ctx.r[10].s64 = ctx.r[10].s64 + 27912;
	// 826E3FE4: 816BF2E4  lwz r11, -0xd1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3356 as u32) ) } as u64;
	// 826E3FE8: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826E3FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E3FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E3FF0 size=116
    let mut pc: u32 = 0x826E3FF0;
    'dispatch: loop {
        match pc {
            0x826E3FF0 => {
    //   block [0x826E3FF0..0x826E4064)
	// 826E3FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E3FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E3FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E3FFC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E4000: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4004: 392B7658  addi r9, r11, 0x7658
	ctx.r[9].s64 = ctx.r[11].s64 + 30296;
	// 826E4008: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E400C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E4010: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826E4014: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826E4018: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E401C: 388AAF24  addi r4, r10, -0x50dc
	ctx.r[4].s64 = ctx.r[10].s64 + -20700;
	// 826E4020: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4024: 396B6D08  addi r11, r11, 0x6d08
	ctx.r[11].s64 = ctx.r[11].s64 + 27912;
	// 826E4028: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826E402C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4030: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826E4034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4038: 386A24DC  addi r3, r10, 0x24dc
	ctx.r[3].s64 = ctx.r[10].s64 + 9436;
	// 826E403C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E4040: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826E4044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4048: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826E404C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E4050: 4BD82DD1  bl 0x82466e20
	ctx.lr = 0x826E4054;
	sub_82466E20(ctx, base);
	// 826E4054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E405C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4068 size=112
    let mut pc: u32 = 0x826E4068;
    'dispatch: loop {
        match pc {
            0x826E4068 => {
    //   block [0x826E4068..0x826E40D8)
	// 826E4068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E406C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E4074: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4078: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E407C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E4080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E4084: 390BF738  addi r8, r11, -0x8c8
	ctx.r[8].s64 = ctx.r[11].s64 + -2248;
	// 826E4088: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826E408C: 388AAF40  addi r4, r10, -0x50c0
	ctx.r[4].s64 = ctx.r[10].s64 + -20672;
	// 826E4090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4094: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4098: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E409C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E40A0: 386A250C  addi r3, r10, 0x250c
	ctx.r[3].s64 = ctx.r[10].s64 + 9484;
	// 826E40A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E40A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E40AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E40B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E40B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E40B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E40BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E40C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E40C4: 4BD82D5D  bl 0x82466e20
	ctx.lr = 0x826E40C8;
	sub_82466E20(ctx, base);
	// 826E40C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E40CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E40D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E40D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E40D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E40D8 size=24
    let mut pc: u32 = 0x826E40D8;
    'dispatch: loop {
        match pc {
            0x826E40D8 => {
    //   block [0x826E40D8..0x826E40F0)
	// 826E40D8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E40DC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E40E0: 394A6DB0  addi r10, r10, 0x6db0
	ctx.r[10].s64 = ctx.r[10].s64 + 28080;
	// 826E40E4: 816BF7B0  lwz r11, -0x850(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2128 as u32) ) } as u64;
	// 826E40E8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826E40EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E40F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E40F0 size=116
    let mut pc: u32 = 0x826E40F0;
    'dispatch: loop {
        match pc {
            0x826E40F0 => {
    //   block [0x826E40F0..0x826E4164)
	// 826E40F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E40F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E40F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E40FC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4100: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E4104: 390B6DB0  addi r8, r11, 0x6db0
	ctx.r[8].s64 = ctx.r[11].s64 + 28080;
	// 826E4108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E410C: 392A76C0  addi r9, r10, 0x76c0
	ctx.r[9].s64 = ctx.r[10].s64 + 30400;
	// 826E4110: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4114: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826E4118: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E411C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E4120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4124: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E4128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E412C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4134: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E4138: 388AAF78  addi r4, r10, -0x5088
	ctx.r[4].s64 = ctx.r[10].s64 + -20616;
	// 826E413C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E4140: 386B253C  addi r3, r11, 0x253c
	ctx.r[3].s64 = ctx.r[11].s64 + 9532;
	// 826E4144: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E4148: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E414C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4150: 4BD82CD1  bl 0x82466e20
	ctx.lr = 0x826E4154;
	sub_82466E20(ctx, base);
	// 826E4154: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E415C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4168 size=112
    let mut pc: u32 = 0x826E4168;
    'dispatch: loop {
        match pc {
            0x826E4168 => {
    //   block [0x826E4168..0x826E41D8)
	// 826E4168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E416C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E4174: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4178: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E417C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E4180: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E4184: 390BF7B8  addi r8, r11, -0x848
	ctx.r[8].s64 = ctx.r[11].s64 + -2120;
	// 826E4188: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826E418C: 388AAF98  addi r4, r10, -0x5068
	ctx.r[4].s64 = ctx.r[10].s64 + -20584;
	// 826E4190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4194: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4198: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E419C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E41A0: 386A256C  addi r3, r10, 0x256c
	ctx.r[3].s64 = ctx.r[10].s64 + 9580;
	// 826E41A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E41A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E41AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E41B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E41B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E41B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E41BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E41C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E41C4: 4BD82C5D  bl 0x82466e20
	ctx.lr = 0x826E41C8;
	sub_82466E20(ctx, base);
	// 826E41C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E41CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E41D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E41D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E41D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E41D8 size=112
    let mut pc: u32 = 0x826E41D8;
    'dispatch: loop {
        match pc {
            0x826E41D8 => {
    //   block [0x826E41D8..0x826E4248)
	// 826E41D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E41DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E41E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E41E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E41E8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E41EC: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E41F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E41F4: 390BF8A8  addi r8, r11, -0x758
	ctx.r[8].s64 = ctx.r[11].s64 + -1880;
	// 826E41F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826E41FC: 388AAFB4  addi r4, r10, -0x504c
	ctx.r[4].s64 = ctx.r[10].s64 + -20556;
	// 826E4200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4204: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4208: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E420C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4210: 386A259C  addi r3, r10, 0x259c
	ctx.r[3].s64 = ctx.r[10].s64 + 9628;
	// 826E4214: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E4218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E421C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E422C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4234: 4BD82BED  bl 0x82466e20
	ctx.lr = 0x826E4238;
	sub_82466E20(ctx, base);
	// 826E4238: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E423C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E4248 size=24
    let mut pc: u32 = 0x826E4248;
    'dispatch: loop {
        match pc {
            0x826E4248 => {
    //   block [0x826E4248..0x826E4260)
	// 826E4248: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E424C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E4250: 394A6E58  addi r10, r10, 0x6e58
	ctx.r[10].s64 = ctx.r[10].s64 + 28248;
	// 826E4254: 816BF7B4  lwz r11, -0x84c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2124 as u32) ) } as u64;
	// 826E4258: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826E425C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4260 size=116
    let mut pc: u32 = 0x826E4260;
    'dispatch: loop {
        match pc {
            0x826E4260 => {
    //   block [0x826E4260..0x826E42D4)
	// 826E4260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E426C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4270: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E4274: 390B6E58  addi r8, r11, 0x6e58
	ctx.r[8].s64 = ctx.r[11].s64 + 28248;
	// 826E4278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E427C: 392A770C  addi r9, r10, 0x770c
	ctx.r[9].s64 = ctx.r[10].s64 + 30476;
	// 826E4280: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4284: 38E00019  li r7, 0x19
	ctx.r[7].s64 = 25;
	// 826E4288: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E428C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E4290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4294: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E4298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E429C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E42A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E42A4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E42A8: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826E42AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E42B0: 386B25CC  addi r3, r11, 0x25cc
	ctx.r[3].s64 = ctx.r[11].s64 + 9676;
	// 826E42B4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E42B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E42BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E42C0: 4BD82B61  bl 0x82466e20
	ctx.lr = 0x826E42C4;
	sub_82466E20(ctx, base);
	// 826E42C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E42C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E42CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E42D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E42D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E42D8 size=116
    let mut pc: u32 = 0x826E42D8;
    'dispatch: loop {
        match pc {
            0x826E42D8 => {
    //   block [0x826E42D8..0x826E434C)
	// 826E42D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E42DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E42E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E42E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E42E8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E42EC: 392B7744  addi r9, r11, 0x7744
	ctx.r[9].s64 = ctx.r[11].s64 + 30532;
	// 826E42F0: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E42F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E42F8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826E42FC: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 826E4300: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4304: 388AB168  addi r4, r10, -0x4e98
	ctx.r[4].s64 = ctx.r[10].s64 + -20120;
	// 826E4308: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E430C: 396BF910  addi r11, r11, -0x6f0
	ctx.r[11].s64 = ctx.r[11].s64 + -1776;
	// 826E4310: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826E4314: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4318: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826E431C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4320: 386A25FC  addi r3, r10, 0x25fc
	ctx.r[3].s64 = ctx.r[10].s64 + 9724;
	// 826E4324: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E4328: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826E432C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4330: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826E4334: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E4338: 4BD82AE9  bl 0x82466e20
	ctx.lr = 0x826E433C;
	sub_82466E20(ctx, base);
	// 826E433C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4350 size=112
    let mut pc: u32 = 0x826E4350;
    'dispatch: loop {
        match pc {
            0x826E4350 => {
    //   block [0x826E4350..0x826E43C0)
	// 826E4350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E435C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4360: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4364: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E4368: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E436C: 390BFAD8  addi r8, r11, -0x528
	ctx.r[8].s64 = ctx.r[11].s64 + -1320;
	// 826E4370: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826E4374: 388AB188  addi r4, r10, -0x4e78
	ctx.r[4].s64 = ctx.r[10].s64 + -20088;
	// 826E4378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E437C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4380: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E4384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4388: 386A262C  addi r3, r10, 0x262c
	ctx.r[3].s64 = ctx.r[10].s64 + 9772;
	// 826E438C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E4390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E4394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E439C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E43A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E43A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E43A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E43AC: 4BD82A75  bl 0x82466e20
	ctx.lr = 0x826E43B0;
	sub_82466E20(ctx, base);
	// 826E43B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E43B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E43B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E43BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E43C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E43C0 size=48
    let mut pc: u32 = 0x826E43C0;
    'dispatch: loop {
        match pc {
            0x826E43C0 => {
    //   block [0x826E43C0..0x826E43F0)
	// 826E43C0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E43C4: 814BFB3C  lwz r10, -0x4c4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1220 as u32) ) } as u64;
	// 826E43C8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E43CC: 396B70B0  addi r11, r11, 0x70b0
	ctx.r[11].s64 = ctx.r[11].s64 + 28848;
	// 826E43D0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826E43D4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E43D8: 814AFB40  lwz r10, -0x4c0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-1216 as u32) ) } as u64;
	// 826E43DC: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 826E43E0: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E43E4: 814AFB38  lwz r10, -0x4c8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-1224 as u32) ) } as u64;
	// 826E43E8: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 826E43EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E43F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E43F0 size=116
    let mut pc: u32 = 0x826E43F0;
    'dispatch: loop {
        match pc {
            0x826E43F0 => {
    //   block [0x826E43F0..0x826E4464)
	// 826E43F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E43F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E43F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E43FC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4400: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E4404: 390B70B0  addi r8, r11, 0x70b0
	ctx.r[8].s64 = ctx.r[11].s64 + 28848;
	// 826E4408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E440C: 392A7800  addi r9, r10, 0x7800
	ctx.r[9].s64 = ctx.r[10].s64 + 30720;
	// 826E4410: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4414: 38E00016  li r7, 0x16
	ctx.r[7].s64 = 22;
	// 826E4418: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E441C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E4420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4424: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E4428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E442C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4434: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E4438: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826E443C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E4440: 386B265C  addi r3, r11, 0x265c
	ctx.r[3].s64 = ctx.r[11].s64 + 9820;
	// 826E4444: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826E4448: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E444C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4450: 4BD829D1  bl 0x82466e20
	ctx.lr = 0x826E4454;
	sub_82466E20(ctx, base);
	// 826E4454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E445C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4468 size=112
    let mut pc: u32 = 0x826E4468;
    'dispatch: loop {
        match pc {
            0x826E4468 => {
    //   block [0x826E4468..0x826E44D8)
	// 826E4468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E446C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E4474: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4478: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E447C: 38AA14EC  addi r5, r10, 0x14ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5356;
	// 826E4480: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E4484: 390BFB48  addi r8, r11, -0x4b8
	ctx.r[8].s64 = ctx.r[11].s64 + -1208;
	// 826E4488: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E448C: 388AB2C4  addi r4, r10, -0x4d3c
	ctx.r[4].s64 = ctx.r[10].s64 + -19772;
	// 826E4490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4494: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E449C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E44A0: 386A268C  addi r3, r10, 0x268c
	ctx.r[3].s64 = ctx.r[10].s64 + 9868;
	// 826E44A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E44A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E44AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E44B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E44B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E44B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E44BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E44C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E44C4: 4BD8295D  bl 0x82466e20
	ctx.lr = 0x826E44C8;
	sub_82466E20(ctx, base);
	// 826E44C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E44CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E44D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E44D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E44D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E44D8 size=112
    let mut pc: u32 = 0x826E44D8;
    'dispatch: loop {
        match pc {
            0x826E44D8 => {
    //   block [0x826E44D8..0x826E4548)
	// 826E44D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E44DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E44E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E44E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E44E8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E44EC: 38AA1AEC  addi r5, r10, 0x1aec
	ctx.r[5].s64 = ctx.r[10].s64 + 6892;
	// 826E44F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E44F4: 390BFB90  addi r8, r11, -0x470
	ctx.r[8].s64 = ctx.r[11].s64 + -1136;
	// 826E44F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826E44FC: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 826E4500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4504: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4508: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E450C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4510: 386A26BC  addi r3, r10, 0x26bc
	ctx.r[3].s64 = ctx.r[10].s64 + 9916;
	// 826E4514: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E4518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E451C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E452C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4534: 4BD828ED  bl 0x82466e20
	ctx.lr = 0x826E4538;
	sub_82466E20(ctx, base);
	// 826E4538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E453C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4548 size=116
    let mut pc: u32 = 0x826E4548;
    'dispatch: loop {
        match pc {
            0x826E4548 => {
    //   block [0x826E4548..0x826E45BC)
	// 826E4548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E454C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E4554: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E4558: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826E455C: 390AFBF0  addi r8, r10, -0x410
	ctx.r[8].s64 = ctx.r[10].s64 + -1040;
	// 826E4560: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4564: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E4568: 38AA1F0C  addi r5, r10, 0x1f0c
	ctx.r[5].s64 = ctx.r[10].s64 + 7948;
	// 826E456C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E4570: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E4574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4578: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E457C: 388AB2F4  addi r4, r10, -0x4d0c
	ctx.r[4].s64 = ctx.r[10].s64 + -19724;
	// 826E4580: 396B783C  addi r11, r11, 0x783c
	ctx.r[11].s64 = ctx.r[11].s64 + 30780;
	// 826E4584: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4588: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E458C: 386A26EC  addi r3, r10, 0x26ec
	ctx.r[3].s64 = ctx.r[10].s64 + 9964;
	// 826E4590: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E4594: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E4598: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E459C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E45A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E45A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E45A8: 4BD82879  bl 0x82466e20
	ctx.lr = 0x826E45AC;
	sub_82466E20(ctx, base);
	// 826E45AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E45B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E45B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E45B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E45C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E45C0 size=108
    let mut pc: u32 = 0x826E45C0;
    'dispatch: loop {
        match pc {
            0x826E45C0 => {
    //   block [0x826E45C0..0x826E462C)
	// 826E45C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E45C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E45C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E45CC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E45D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E45D4: 38EBFC68  addi r7, r11, -0x398
	ctx.r[7].s64 = ctx.r[11].s64 + -920;
	// 826E45D8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826E45DC: 388AB318  addi r4, r10, -0x4ce8
	ctx.r[4].s64 = ctx.r[10].s64 + -19688;
	// 826E45E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E45E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E45E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E45EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E45F0: 386A271C  addi r3, r10, 0x271c
	ctx.r[3].s64 = ctx.r[10].s64 + 10012;
	// 826E45F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E45F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E45FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E460C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4614: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E4618: 4BD82809  bl 0x82466e20
	ctx.lr = 0x826E461C;
	sub_82466E20(ctx, base);
	// 826E461C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4630 size=108
    let mut pc: u32 = 0x826E4630;
    'dispatch: loop {
        match pc {
            0x826E4630 => {
    //   block [0x826E4630..0x826E469C)
	// 826E4630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E463C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4640: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E4644: 38EBFCB0  addi r7, r11, -0x350
	ctx.r[7].s64 = ctx.r[11].s64 + -848;
	// 826E4648: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826E464C: 388AB344  addi r4, r10, -0x4cbc
	ctx.r[4].s64 = ctx.r[10].s64 + -19644;
	// 826E4650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4654: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4658: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E465C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4660: 386A274C  addi r3, r10, 0x274c
	ctx.r[3].s64 = ctx.r[10].s64 + 10060;
	// 826E4664: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E4668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E466C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E467C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4684: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E4688: 4BD82799  bl 0x82466e20
	ctx.lr = 0x826E468C;
	sub_82466E20(ctx, base);
	// 826E468C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E46A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E46A0 size=108
    let mut pc: u32 = 0x826E46A0;
    'dispatch: loop {
        match pc {
            0x826E46A0 => {
    //   block [0x826E46A0..0x826E470C)
	// 826E46A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E46A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E46A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E46AC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E46B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E46B4: 38EBFCF8  addi r7, r11, -0x308
	ctx.r[7].s64 = ctx.r[11].s64 + -776;
	// 826E46B8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826E46BC: 388AB36C  addi r4, r10, -0x4c94
	ctx.r[4].s64 = ctx.r[10].s64 + -19604;
	// 826E46C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E46C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E46C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E46CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E46D0: 386A277C  addi r3, r10, 0x277c
	ctx.r[3].s64 = ctx.r[10].s64 + 10108;
	// 826E46D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E46D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E46DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E46E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E46E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E46E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E46EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E46F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E46F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E46F8: 4BD82729  bl 0x82466e20
	ctx.lr = 0x826E46FC;
	sub_82466E20(ctx, base);
	// 826E46FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4710 size=108
    let mut pc: u32 = 0x826E4710;
    'dispatch: loop {
        match pc {
            0x826E4710 => {
    //   block [0x826E4710..0x826E477C)
	// 826E4710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E471C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4720: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E4724: 38EBFD40  addi r7, r11, -0x2c0
	ctx.r[7].s64 = ctx.r[11].s64 + -704;
	// 826E4728: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826E472C: 388AB398  addi r4, r10, -0x4c68
	ctx.r[4].s64 = ctx.r[10].s64 + -19560;
	// 826E4730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4734: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4738: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E473C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4740: 386A27AC  addi r3, r10, 0x27ac
	ctx.r[3].s64 = ctx.r[10].s64 + 10156;
	// 826E4744: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E4748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E474C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E475C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4764: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E4768: 4BD826B9  bl 0x82466e20
	ctx.lr = 0x826E476C;
	sub_82466E20(ctx, base);
	// 826E476C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4780 size=108
    let mut pc: u32 = 0x826E4780;
    'dispatch: loop {
        match pc {
            0x826E4780 => {
    //   block [0x826E4780..0x826E47EC)
	// 826E4780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E478C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4790: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826E4794: 38EBFDE8  addi r7, r11, -0x218
	ctx.r[7].s64 = ctx.r[11].s64 + -536;
	// 826E4798: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E479C: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 826E47A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E47A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E47A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E47AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E47B0: 386A27DC  addi r3, r10, 0x27dc
	ctx.r[3].s64 = ctx.r[10].s64 + 10204;
	// 826E47B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E47B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E47BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E47C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E47C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E47C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E47CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E47D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E47D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E47D8: 4BD82649  bl 0x82466e20
	ctx.lr = 0x826E47DC;
	sub_82466E20(ctx, base);
	// 826E47DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E47E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E47E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E47E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E47F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E47F0 size=112
    let mut pc: u32 = 0x826E47F0;
    'dispatch: loop {
        match pc {
            0x826E47F0 => {
    //   block [0x826E47F0..0x826E4860)
	// 826E47F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E47F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E47F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E47FC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E4800: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4804: 392A787C  addi r9, r10, 0x787c
	ctx.r[9].s64 = ctx.r[10].s64 + 30844;
	// 826E4808: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E480C: 390BFE20  addi r8, r11, -0x1e0
	ctx.r[8].s64 = ctx.r[11].s64 + -480;
	// 826E4810: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826E4814: 388A8198  addi r4, r10, -0x7e68
	ctx.r[4].s64 = ctx.r[10].s64 + -32360;
	// 826E4818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E481C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4820: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E4824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4828: 386A280C  addi r3, r10, 0x280c
	ctx.r[3].s64 = ctx.r[10].s64 + 10252;
	// 826E482C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E4830: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E4834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E483C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E4844: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E4848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E484C: 4BD825D5  bl 0x82466e20
	ctx.lr = 0x826E4850;
	sub_82466E20(ctx, base);
	// 826E4850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E485C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4860 size=108
    let mut pc: u32 = 0x826E4860;
    'dispatch: loop {
        match pc {
            0x826E4860 => {
    //   block [0x826E4860..0x826E48CC)
	// 826E4860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E486C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E4874: 38EBFE68  addi r7, r11, -0x198
	ctx.r[7].s64 = ctx.r[11].s64 + -408;
	// 826E4878: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826E487C: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 826E4880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4884: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4888: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E488C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4890: 386A283C  addi r3, r10, 0x283c
	ctx.r[3].s64 = ctx.r[10].s64 + 10300;
	// 826E4894: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E4898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E489C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E48A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E48A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E48A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E48AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E48B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E48B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E48B8: 4BD82569  bl 0x82466e20
	ctx.lr = 0x826E48BC;
	sub_82466E20(ctx, base);
	// 826E48BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E48C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E48C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E48C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E48D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E48D0 size=108
    let mut pc: u32 = 0x826E48D0;
    'dispatch: loop {
        match pc {
            0x826E48D0 => {
    //   block [0x826E48D0..0x826E493C)
	// 826E48D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E48D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E48D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E48DC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E48E0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826E48E4: 38EBFEE0  addi r7, r11, -0x120
	ctx.r[7].s64 = ctx.r[11].s64 + -288;
	// 826E48E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E48EC: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 826E48F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E48F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E48F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E48FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4900: 386A286C  addi r3, r10, 0x286c
	ctx.r[3].s64 = ctx.r[10].s64 + 10348;
	// 826E4904: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E4908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E490C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E491C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4924: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E4928: 4BD824F9  bl 0x82466e20
	ctx.lr = 0x826E492C;
	sub_82466E20(ctx, base);
	// 826E492C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4940 size=108
    let mut pc: u32 = 0x826E4940;
    'dispatch: loop {
        match pc {
            0x826E4940 => {
    //   block [0x826E4940..0x826E49AC)
	// 826E4940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E494C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4950: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826E4954: 38EBFF10  addi r7, r11, -0xf0
	ctx.r[7].s64 = ctx.r[11].s64 + -240;
	// 826E4958: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E495C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 826E4960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4964: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4968: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E496C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4970: 386A289C  addi r3, r10, 0x289c
	ctx.r[3].s64 = ctx.r[10].s64 + 10396;
	// 826E4974: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E4978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E497C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E498C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4994: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E4998: 4BD82489  bl 0x82466e20
	ctx.lr = 0x826E499C;
	sub_82466E20(ctx, base);
	// 826E499C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E49A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E49A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E49A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E49B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E49B0 size=24
    let mut pc: u32 = 0x826E49B0;
    'dispatch: loop {
        match pc {
            0x826E49B0 => {
    //   block [0x826E49B0..0x826E49C8)
	// 826E49B0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E49B4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E49B8: 394A72C0  addi r10, r10, 0x72c0
	ctx.r[10].s64 = ctx.r[10].s64 + 29376;
	// 826E49BC: 816BFE1C  lwz r11, -0x1e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-484 as u32) ) } as u64;
	// 826E49C0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826E49C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E49C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E49C8 size=112
    let mut pc: u32 = 0x826E49C8;
    'dispatch: loop {
        match pc {
            0x826E49C8 => {
    //   block [0x826E49C8..0x826E4A38)
	// 826E49C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E49CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E49D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E49D4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E49D8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E49DC: 392A78BC  addi r9, r10, 0x78bc
	ctx.r[9].s64 = ctx.r[10].s64 + 30908;
	// 826E49E0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826E49E4: 390B72C0  addi r8, r11, 0x72c0
	ctx.r[8].s64 = ctx.r[11].s64 + 29376;
	// 826E49E8: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826E49EC: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 826E49F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E49F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E49F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E49FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4A00: 386A28CC  addi r3, r10, 0x28cc
	ctx.r[3].s64 = ctx.r[10].s64 + 10444;
	// 826E4A04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E4A08: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E4A0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4A10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4A18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E4A1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E4A20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4A24: 4BD823FD  bl 0x82466e20
	ctx.lr = 0x826E4A28;
	sub_82466E20(ctx, base);
	// 826E4A28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4A38 size=96
    let mut pc: u32 = 0x826E4A38;
    'dispatch: loop {
        match pc {
            0x826E4A38 => {
    //   block [0x826E4A38..0x826E4A98)
	// 826E4A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E4A44: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826E4A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4A4C: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826E4A50: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4A54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4A58: 386A28FC  addi r3, r10, 0x28fc
	ctx.r[3].s64 = ctx.r[10].s64 + 10492;
	// 826E4A5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E4A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4A64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E4A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E4A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4A78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E4A7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E4A80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E4A84: 4BD8239D  bl 0x82466e20
	ctx.lr = 0x826E4A88;
	sub_82466E20(ctx, base);
	// 826E4A88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4A8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4A90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4A98 size=112
    let mut pc: u32 = 0x826E4A98;
    'dispatch: loop {
        match pc {
            0x826E4A98 => {
    //   block [0x826E4A98..0x826E4B08)
	// 826E4A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E4AA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4AA8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4AAC: 38AA28FC  addi r5, r10, 0x28fc
	ctx.r[5].s64 = ctx.r[10].s64 + 10492;
	// 826E4AB0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826E4AB4: 390BFF28  addi r8, r11, -0xd8
	ctx.r[8].s64 = ctx.r[11].s64 + -216;
	// 826E4AB8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E4ABC: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 826E4AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4AC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4AC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E4ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4AD0: 386A292C  addi r3, r10, 0x292c
	ctx.r[3].s64 = ctx.r[10].s64 + 10540;
	// 826E4AD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E4AD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E4ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E4AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4AF4: 4BD8232D  bl 0x82466e20
	ctx.lr = 0x826E4AF8;
	sub_82466E20(ctx, base);
	// 826E4AF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E4B08 size=24
    let mut pc: u32 = 0x826E4B08;
    'dispatch: loop {
        match pc {
            0x826E4B08 => {
    //   block [0x826E4B08..0x826E4B20)
	// 826E4B08: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4B0C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E4B10: 394A7398  addi r10, r10, 0x7398
	ctx.r[10].s64 = ctx.r[10].s64 + 29592;
	// 826E4B14: 816BFF5C  lwz r11, -0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-164 as u32) ) } as u64;
	// 826E4B18: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 826E4B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4B20 size=112
    let mut pc: u32 = 0x826E4B20;
    'dispatch: loop {
        match pc {
            0x826E4B20 => {
    //   block [0x826E4B20..0x826E4B90)
	// 826E4B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E4B2C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E4B30: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4B34: 392A78E8  addi r9, r10, 0x78e8
	ctx.r[9].s64 = ctx.r[10].s64 + 30952;
	// 826E4B38: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826E4B3C: 390B7398  addi r8, r11, 0x7398
	ctx.r[8].s64 = ctx.r[11].s64 + 29592;
	// 826E4B40: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826E4B44: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 826E4B48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4B4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4B50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E4B54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4B58: 386A295C  addi r3, r10, 0x295c
	ctx.r[3].s64 = ctx.r[10].s64 + 10588;
	// 826E4B5C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E4B60: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E4B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4B68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4B70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E4B74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E4B78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4B7C: 4BD822A5  bl 0x82466e20
	ctx.lr = 0x826E4B80;
	sub_82466E20(ctx, base);
	// 826E4B80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4B8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4B90 size=108
    let mut pc: u32 = 0x826E4B90;
    'dispatch: loop {
        match pc {
            0x826E4B90 => {
    //   block [0x826E4B90..0x826E4BFC)
	// 826E4B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E4B9C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4BA0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826E4BA4: 38EBFF64  addi r7, r11, -0x9c
	ctx.r[7].s64 = ctx.r[11].s64 + -156;
	// 826E4BA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E4BAC: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826E4BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4BB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4BB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E4BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4BC0: 386A298C  addi r3, r10, 0x298c
	ctx.r[3].s64 = ctx.r[10].s64 + 10636;
	// 826E4BC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E4BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E4BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4BD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E4BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4BE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E4BE8: 4BD82239  bl 0x82466e20
	ctx.lr = 0x826E4BEC;
	sub_82466E20(ctx, base);
	// 826E4BEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4BF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4BF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E4C00 size=24
    let mut pc: u32 = 0x826E4C00;
    'dispatch: loop {
        match pc {
            0x826E4C00 => {
    //   block [0x826E4C00..0x826E4C18)
	// 826E4C00: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4C04: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E4C08: 394A7488  addi r10, r10, 0x7488
	ctx.r[10].s64 = ctx.r[10].s64 + 29832;
	// 826E4C0C: 816BFF60  lwz r11, -0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-160 as u32) ) } as u64;
	// 826E4C10: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826E4C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4C18 size=112
    let mut pc: u32 = 0x826E4C18;
    'dispatch: loop {
        match pc {
            0x826E4C18 => {
    //   block [0x826E4C18..0x826E4C88)
	// 826E4C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E4C24: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E4C28: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4C2C: 392A7918  addi r9, r10, 0x7918
	ctx.r[9].s64 = ctx.r[10].s64 + 31000;
	// 826E4C30: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826E4C34: 390B7488  addi r8, r11, 0x7488
	ctx.r[8].s64 = ctx.r[11].s64 + 29832;
	// 826E4C38: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826E4C3C: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 826E4C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4C44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4C48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E4C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4C50: 386A29BC  addi r3, r10, 0x29bc
	ctx.r[3].s64 = ctx.r[10].s64 + 10684;
	// 826E4C54: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E4C58: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E4C5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4C60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4C64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4C68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E4C6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E4C70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4C74: 4BD821AD  bl 0x82466e20
	ctx.lr = 0x826E4C78;
	sub_82466E20(ctx, base);
	// 826E4C78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E4C88 size=40
    let mut pc: u32 = 0x826E4C88;
    'dispatch: loop {
        match pc {
            0x826E4C88 => {
    //   block [0x826E4C88..0x826E4CB0)
	// 826E4C88: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4C8C: 814BFF94  lwz r10, -0x6c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-108 as u32) ) } as u64;
	// 826E4C90: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4C94: 396B74E8  addi r11, r11, 0x74e8
	ctx.r[11].s64 = ctx.r[11].s64 + 29928;
	// 826E4C98: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826E4C9C: 914B0068  stw r10, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 826E4CA0: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E4CA4: 814AFF98  lwz r10, -0x68(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-104 as u32) ) } as u64;
	// 826E4CA8: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 826E4CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4CB0 size=112
    let mut pc: u32 = 0x826E4CB0;
    'dispatch: loop {
        match pc {
            0x826E4CB0 => {
    //   block [0x826E4CB0..0x826E4D20)
	// 826E4CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E4CBC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E4CC0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4CC4: 392A7A90  addi r9, r10, 0x7a90
	ctx.r[9].s64 = ctx.r[10].s64 + 31376;
	// 826E4CC8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826E4CCC: 390B74E8  addi r8, r11, 0x74e8
	ctx.r[8].s64 = ctx.r[11].s64 + 29928;
	// 826E4CD0: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826E4CD4: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826E4CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4CDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4CE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E4CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4CE8: 386A29EC  addi r3, r10, 0x29ec
	ctx.r[3].s64 = ctx.r[10].s64 + 10732;
	// 826E4CEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E4CF0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826E4CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4CF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4D00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E4D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E4D08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4D0C: 4BD82115  bl 0x82466e20
	ctx.lr = 0x826E4D10;
	sub_82466E20(ctx, base);
	// 826E4D10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4D20 size=108
    let mut pc: u32 = 0x826E4D20;
    'dispatch: loop {
        match pc {
            0x826E4D20 => {
    //   block [0x826E4D20..0x826E4D8C)
	// 826E4D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E4D2C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4D30: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826E4D34: 38EBFFA0  addi r7, r11, -0x60
	ctx.r[7].s64 = ctx.r[11].s64 + -96;
	// 826E4D38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E4D3C: 388A6E48  addi r4, r10, 0x6e48
	ctx.r[4].s64 = ctx.r[10].s64 + 28232;
	// 826E4D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4D44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4D48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E4D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4D50: 386A2A1C  addi r3, r10, 0x2a1c
	ctx.r[3].s64 = ctx.r[10].s64 + 10780;
	// 826E4D54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E4D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E4D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E4D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4D74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E4D78: 4BD820A9  bl 0x82466e20
	ctx.lr = 0x826E4D7C;
	sub_82466E20(ctx, base);
	// 826E4D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4D90 size=108
    let mut pc: u32 = 0x826E4D90;
    'dispatch: loop {
        match pc {
            0x826E4D90 => {
    //   block [0x826E4D90..0x826E4DFC)
	// 826E4D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E4D9C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4DA0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826E4DA4: 38EBFFD0  addi r7, r11, -0x30
	ctx.r[7].s64 = ctx.r[11].s64 + -48;
	// 826E4DA8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E4DAC: 388A6E64  addi r4, r10, 0x6e64
	ctx.r[4].s64 = ctx.r[10].s64 + 28260;
	// 826E4DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4DB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4DB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E4DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4DC0: 386A2A4C  addi r3, r10, 0x2a4c
	ctx.r[3].s64 = ctx.r[10].s64 + 10828;
	// 826E4DC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E4DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E4DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E4DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4DE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E4DE8: 4BD82039  bl 0x82466e20
	ctx.lr = 0x826E4DEC;
	sub_82466E20(ctx, base);
	// 826E4DEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4E00 size=108
    let mut pc: u32 = 0x826E4E00;
    'dispatch: loop {
        match pc {
            0x826E4E00 => {
    //   block [0x826E4E00..0x826E4E6C)
	// 826E4E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E4E0C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4E10: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826E4E14: 38EBFFE8  addi r7, r11, -0x18
	ctx.r[7].s64 = ctx.r[11].s64 + -24;
	// 826E4E18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E4E1C: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826E4E20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4E24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4E28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E4E2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4E30: 386A2A7C  addi r3, r10, 0x2a7c
	ctx.r[3].s64 = ctx.r[10].s64 + 10876;
	// 826E4E34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E4E38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E4E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4E40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4E48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E4E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4E50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4E54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E4E58: 4BD81FC9  bl 0x82466e20
	ctx.lr = 0x826E4E5C;
	sub_82466E20(ctx, base);
	// 826E4E5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4E70 size=108
    let mut pc: u32 = 0x826E4E70;
    'dispatch: loop {
        match pc {
            0x826E4E70 => {
    //   block [0x826E4E70..0x826E4EDC)
	// 826E4E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4E78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E4E7C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4E80: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826E4E84: 38EB0018  addi r7, r11, 0x18
	ctx.r[7].s64 = ctx.r[11].s64 + 24;
	// 826E4E88: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826E4E8C: 388A7BB8  addi r4, r10, 0x7bb8
	ctx.r[4].s64 = ctx.r[10].s64 + 31672;
	// 826E4E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4E94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4E98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E4E9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4EA0: 386A2AAC  addi r3, r10, 0x2aac
	ctx.r[3].s64 = ctx.r[10].s64 + 10924;
	// 826E4EA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E4EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E4EAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E4EBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4EC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E4EC8: 4BD81F59  bl 0x82466e20
	ctx.lr = 0x826E4ECC;
	sub_82466E20(ctx, base);
	// 826E4ECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4EE0 size=108
    let mut pc: u32 = 0x826E4EE0;
    'dispatch: loop {
        match pc {
            0x826E4EE0 => {
    //   block [0x826E4EE0..0x826E4F4C)
	// 826E4EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E4EEC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4EF0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826E4EF4: 38EB00A8  addi r7, r11, 0xa8
	ctx.r[7].s64 = ctx.r[11].s64 + 168;
	// 826E4EF8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E4EFC: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 826E4F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4F04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4F08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E4F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4F10: 386A2ADC  addi r3, r10, 0x2adc
	ctx.r[3].s64 = ctx.r[10].s64 + 10972;
	// 826E4F14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E4F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E4F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4F24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E4F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4F34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E4F38: 4BD81EE9  bl 0x82466e20
	ctx.lr = 0x826E4F3C;
	sub_82466E20(ctx, base);
	// 826E4F3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4F50 size=108
    let mut pc: u32 = 0x826E4F50;
    'dispatch: loop {
        match pc {
            0x826E4F50 => {
    //   block [0x826E4F50..0x826E4FBC)
	// 826E4F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4F58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E4F5C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4F60: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826E4F64: 38EB00C0  addi r7, r11, 0xc0
	ctx.r[7].s64 = ctx.r[11].s64 + 192;
	// 826E4F68: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E4F6C: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 826E4F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4F74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4F78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E4F7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E4F80: 386A2B0C  addi r3, r10, 0x2b0c
	ctx.r[3].s64 = ctx.r[10].s64 + 11020;
	// 826E4F84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E4F88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E4F8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E4F90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E4F94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E4F98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E4F9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E4FA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E4FA8: 4BD81E79  bl 0x82466e20
	ctx.lr = 0x826E4FAC;
	sub_82466E20(ctx, base);
	// 826E4FAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E4FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E4FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E4FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E4FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E4FC0 size=112
    let mut pc: u32 = 0x826E4FC0;
    'dispatch: loop {
        match pc {
            0x826E4FC0 => {
    //   block [0x826E4FC0..0x826E5030)
	// 826E4FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E4FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E4FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E4FCC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E4FD0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E4FD4: 392A7AE4  addi r9, r10, 0x7ae4
	ctx.r[9].s64 = ctx.r[10].s64 + 31460;
	// 826E4FD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E4FDC: 390B00F8  addi r8, r11, 0xf8
	ctx.r[8].s64 = ctx.r[11].s64 + 248;
	// 826E4FE0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826E4FE4: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 826E4FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E4FEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E4FF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E4FF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E4FF8: 386A2B3C  addi r3, r10, 0x2b3c
	ctx.r[3].s64 = ctx.r[10].s64 + 11068;
	// 826E4FFC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E5000: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E5004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E500C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E501C: 4BD81E05  bl 0x82466e20
	ctx.lr = 0x826E5020;
	sub_82466E20(ctx, base);
	// 826E5020: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E5028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E502C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5030 size=108
    let mut pc: u32 = 0x826E5030;
    'dispatch: loop {
        match pc {
            0x826E5030 => {
    //   block [0x826E5030..0x826E509C)
	// 826E5030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E5034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E503C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5040: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826E5044: 38EB0170  addi r7, r11, 0x170
	ctx.r[7].s64 = ctx.r[11].s64 + 368;
	// 826E5048: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826E504C: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 826E5050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E5054: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5058: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E505C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5060: 386A2B6C  addi r3, r10, 0x2b6c
	ctx.r[3].s64 = ctx.r[10].s64 + 11116;
	// 826E5064: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E506C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E5074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E507C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5088: 4BD81D99  bl 0x82466e20
	ctx.lr = 0x826E508C;
	sub_82466E20(ctx, base);
	// 826E508C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E5094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E50A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E50A0 size=24
    let mut pc: u32 = 0x826E50A0;
    'dispatch: loop {
        match pc {
            0x826E50A0 => {
    //   block [0x826E50A0..0x826E50B8)
	// 826E50A0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E50A4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E50A8: 394A75C0  addi r10, r10, 0x75c0
	ctx.r[10].s64 = ctx.r[10].s64 + 30144;
	// 826E50AC: 816B00F4  lwz r11, 0xf4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(244 as u32) ) } as u64;
	// 826E50B0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826E50B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E50B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E50B8 size=108
    let mut pc: u32 = 0x826E50B8;
    'dispatch: loop {
        match pc {
            0x826E50B8 => {
    //   block [0x826E50B8..0x826E5124)
	// 826E50B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E50BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E50C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E50C4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E50C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E50CC: 38EB75C0  addi r7, r11, 0x75c0
	ctx.r[7].s64 = ctx.r[11].s64 + 30144;
	// 826E50D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E50D4: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 826E50D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E50DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E50E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E50E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E50E8: 386A2B9C  addi r3, r10, 0x2b9c
	ctx.r[3].s64 = ctx.r[10].s64 + 11164;
	// 826E50EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E50F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E50F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E50F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E50FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E510C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5110: 4BD81D11  bl 0x82466e20
	ctx.lr = 0x826E5114;
	sub_82466E20(ctx, base);
	// 826E5114: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E511C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E5128 size=24
    let mut pc: u32 = 0x826E5128;
    'dispatch: loop {
        match pc {
            0x826E5128 => {
    //   block [0x826E5128..0x826E5140)
	// 826E5128: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E512C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E5130: 394A75F0  addi r10, r10, 0x75f0
	ctx.r[10].s64 = ctx.r[10].s64 + 30192;
	// 826E5134: 816B00F4  lwz r11, 0xf4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(244 as u32) ) } as u64;
	// 826E5138: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826E513C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5140 size=108
    let mut pc: u32 = 0x826E5140;
    'dispatch: loop {
        match pc {
            0x826E5140 => {
    //   block [0x826E5140..0x826E51AC)
	// 826E5140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E5144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E514C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E5154: 38EB75F0  addi r7, r11, 0x75f0
	ctx.r[7].s64 = ctx.r[11].s64 + 30192;
	// 826E5158: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E515C: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 826E5160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E5164: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5168: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E516C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5170: 386A2BCC  addi r3, r10, 0x2bcc
	ctx.r[3].s64 = ctx.r[10].s64 + 11212;
	// 826E5174: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E517C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E5184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E518C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5198: 4BD81C89  bl 0x82466e20
	ctx.lr = 0x826E519C;
	sub_82466E20(ctx, base);
	// 826E519C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E51A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E51A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E51A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E51B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E51B0 size=108
    let mut pc: u32 = 0x826E51B0;
    'dispatch: loop {
        match pc {
            0x826E51B0 => {
    //   block [0x826E51B0..0x826E521C)
	// 826E51B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E51B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E51B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E51BC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E51C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E51C4: 38EB0248  addi r7, r11, 0x248
	ctx.r[7].s64 = ctx.r[11].s64 + 584;
	// 826E51C8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E51CC: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 826E51D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E51D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E51D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E51DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E51E0: 386A2BFC  addi r3, r10, 0x2bfc
	ctx.r[3].s64 = ctx.r[10].s64 + 11260;
	// 826E51E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E51E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E51EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E51F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E51F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E51F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E51FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5208: 4BD81C19  bl 0x82466e20
	ctx.lr = 0x826E520C;
	sub_82466E20(ctx, base);
	// 826E520C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E5214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E5220 size=24
    let mut pc: u32 = 0x826E5220;
    'dispatch: loop {
        match pc {
            0x826E5220 => {
    //   block [0x826E5220..0x826E5238)
	// 826E5220: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5224: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E5228: 394A7620  addi r10, r10, 0x7620
	ctx.r[10].s64 = ctx.r[10].s64 + 30240;
	// 826E522C: 816B00F4  lwz r11, 0xf4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(244 as u32) ) } as u64;
	// 826E5230: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826E5234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5238 size=108
    let mut pc: u32 = 0x826E5238;
    'dispatch: loop {
        match pc {
            0x826E5238 => {
    //   block [0x826E5238..0x826E52A4)
	// 826E5238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E523C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5244: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5248: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E524C: 38EB7620  addi r7, r11, 0x7620
	ctx.r[7].s64 = ctx.r[11].s64 + 30240;
	// 826E5250: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E5254: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 826E5258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E525C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5260: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E5264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5268: 386A2C2C  addi r3, r10, 0x2c2c
	ctx.r[3].s64 = ctx.r[10].s64 + 11308;
	// 826E526C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5270: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E5274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E527C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E528C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5290: 4BD81B91  bl 0x82466e20
	ctx.lr = 0x826E5294;
	sub_82466E20(ctx, base);
	// 826E5294: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E529C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E52A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E52A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E52A8 size=112
    let mut pc: u32 = 0x826E52A8;
    'dispatch: loop {
        match pc {
            0x826E52A8 => {
    //   block [0x826E52A8..0x826E5318)
	// 826E52A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E52AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E52B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E52B4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E52B8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E52BC: 392A7B28  addi r9, r10, 0x7b28
	ctx.r[9].s64 = ctx.r[10].s64 + 31528;
	// 826E52C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E52C4: 390B0260  addi r8, r11, 0x260
	ctx.r[8].s64 = ctx.r[11].s64 + 608;
	// 826E52C8: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826E52CC: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 826E52D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E52D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E52D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E52DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E52E0: 386A2C5C  addi r3, r10, 0x2c5c
	ctx.r[3].s64 = ctx.r[10].s64 + 11356;
	// 826E52E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E52E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E52EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E52F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E52F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E52F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E52FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5304: 4BD81B1D  bl 0x82466e20
	ctx.lr = 0x826E5308;
	sub_82466E20(ctx, base);
	// 826E5308: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E530C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E5310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5318 size=108
    let mut pc: u32 = 0x826E5318;
    'dispatch: loop {
        match pc {
            0x826E5318 => {
    //   block [0x826E5318..0x826E5384)
	// 826E5318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E531C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5324: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E532C: 38EB0290  addi r7, r11, 0x290
	ctx.r[7].s64 = ctx.r[11].s64 + 656;
	// 826E5330: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E5334: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 826E5338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E533C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5340: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E5344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5348: 386A2C8C  addi r3, r10, 0x2c8c
	ctx.r[3].s64 = ctx.r[10].s64 + 11404;
	// 826E534C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E5354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E535C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E536C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5370: 4BD81AB1  bl 0x82466e20
	ctx.lr = 0x826E5374;
	sub_82466E20(ctx, base);
	// 826E5374: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E537C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5388 size=108
    let mut pc: u32 = 0x826E5388;
    'dispatch: loop {
        match pc {
            0x826E5388 => {
    //   block [0x826E5388..0x826E53F4)
	// 826E5388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E538C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5394: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E539C: 38EB02C0  addi r7, r11, 0x2c0
	ctx.r[7].s64 = ctx.r[11].s64 + 704;
	// 826E53A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E53A4: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 826E53A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E53AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E53B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E53B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E53B8: 386A2CBC  addi r3, r10, 0x2cbc
	ctx.r[3].s64 = ctx.r[10].s64 + 11452;
	// 826E53BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E53C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E53C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E53C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E53CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E53D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E53D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E53D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E53DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E53E0: 4BD81A41  bl 0x82466e20
	ctx.lr = 0x826E53E4;
	sub_82466E20(ctx, base);
	// 826E53E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E53E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E53EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E53F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E53F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E53F8 size=108
    let mut pc: u32 = 0x826E53F8;
    'dispatch: loop {
        match pc {
            0x826E53F8 => {
    //   block [0x826E53F8..0x826E5464)
	// 826E53F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E53FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5404: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E540C: 38EB02D8  addi r7, r11, 0x2d8
	ctx.r[7].s64 = ctx.r[11].s64 + 728;
	// 826E5410: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E5414: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826E5418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E541C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E5424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5428: 386A2CEC  addi r3, r10, 0x2cec
	ctx.r[3].s64 = ctx.r[10].s64 + 11500;
	// 826E542C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E5434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E543C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E544C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5450: 4BD819D1  bl 0x82466e20
	ctx.lr = 0x826E5454;
	sub_82466E20(ctx, base);
	// 826E5454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E545C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5468 size=112
    let mut pc: u32 = 0x826E5468;
    'dispatch: loop {
        match pc {
            0x826E5468 => {
    //   block [0x826E5468..0x826E54D8)
	// 826E5468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E546C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5474: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5478: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E547C: 38AA2D4C  addi r5, r10, 0x2d4c
	ctx.r[5].s64 = ctx.r[10].s64 + 11596;
	// 826E5480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E5484: 390B0308  addi r8, r11, 0x308
	ctx.r[8].s64 = ctx.r[11].s64 + 776;
	// 826E5488: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E548C: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 826E5490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E5494: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E549C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E54A0: 386A2D1C  addi r3, r10, 0x2d1c
	ctx.r[3].s64 = ctx.r[10].s64 + 11548;
	// 826E54A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E54A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E54AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E54B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E54B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E54B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E54BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E54C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E54C4: 4BD8195D  bl 0x82466e20
	ctx.lr = 0x826E54C8;
	sub_82466E20(ctx, base);
	// 826E54C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E54CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E54D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E54D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E54D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E54D8 size=108
    let mut pc: u32 = 0x826E54D8;
    'dispatch: loop {
        match pc {
            0x826E54D8 => {
    //   block [0x826E54D8..0x826E5544)
	// 826E54D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E54DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E54E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E54E4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E54E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E54EC: 38EB0320  addi r7, r11, 0x320
	ctx.r[7].s64 = ctx.r[11].s64 + 800;
	// 826E54F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E54F4: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826E54F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E54FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5500: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E5504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5508: 386A2D4C  addi r3, r10, 0x2d4c
	ctx.r[3].s64 = ctx.r[10].s64 + 11596;
	// 826E550C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E5514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E551C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E552C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5530: 4BD818F1  bl 0x82466e20
	ctx.lr = 0x826E5534;
	sub_82466E20(ctx, base);
	// 826E5534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E553C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5548 size=108
    let mut pc: u32 = 0x826E5548;
    'dispatch: loop {
        match pc {
            0x826E5548 => {
    //   block [0x826E5548..0x826E55B4)
	// 826E5548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E554C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5554: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E555C: 38EB0350  addi r7, r11, 0x350
	ctx.r[7].s64 = ctx.r[11].s64 + 848;
	// 826E5560: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E5564: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 826E5568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E556C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E5574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5578: 386A2D7C  addi r3, r10, 0x2d7c
	ctx.r[3].s64 = ctx.r[10].s64 + 11644;
	// 826E557C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E5584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E558C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E559C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E55A0: 4BD81881  bl 0x82466e20
	ctx.lr = 0x826E55A4;
	sub_82466E20(ctx, base);
	// 826E55A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E55A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E55AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E55B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E55B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E55B8 size=108
    let mut pc: u32 = 0x826E55B8;
    'dispatch: loop {
        match pc {
            0x826E55B8 => {
    //   block [0x826E55B8..0x826E5624)
	// 826E55B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E55BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E55C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E55C4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E55C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E55CC: 38EB0368  addi r7, r11, 0x368
	ctx.r[7].s64 = ctx.r[11].s64 + 872;
	// 826E55D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E55D4: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 826E55D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E55DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E55E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E55E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E55E8: 386A2DAC  addi r3, r10, 0x2dac
	ctx.r[3].s64 = ctx.r[10].s64 + 11692;
	// 826E55EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E55F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E55F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E55F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E55FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E560C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5610: 4BD81811  bl 0x82466e20
	ctx.lr = 0x826E5614;
	sub_82466E20(ctx, base);
	// 826E5614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E561C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5628 size=108
    let mut pc: u32 = 0x826E5628;
    'dispatch: loop {
        match pc {
            0x826E5628 => {
    //   block [0x826E5628..0x826E5694)
	// 826E5628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E562C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5634: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E563C: 38EB0398  addi r7, r11, 0x398
	ctx.r[7].s64 = ctx.r[11].s64 + 920;
	// 826E5640: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826E5644: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826E5648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E564C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5650: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E5654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5658: 386A2DDC  addi r3, r10, 0x2ddc
	ctx.r[3].s64 = ctx.r[10].s64 + 11740;
	// 826E565C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E5664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E566C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E567C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5680: 4BD817A1  bl 0x82466e20
	ctx.lr = 0x826E5684;
	sub_82466E20(ctx, base);
	// 826E5684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E568C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5698 size=108
    let mut pc: u32 = 0x826E5698;
    'dispatch: loop {
        match pc {
            0x826E5698 => {
    //   block [0x826E5698..0x826E5704)
	// 826E5698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E569C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E56A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E56A4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E56A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E56AC: 38EB0440  addi r7, r11, 0x440
	ctx.r[7].s64 = ctx.r[11].s64 + 1088;
	// 826E56B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E56B4: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 826E56B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E56BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E56C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E56C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E56C8: 386A2E0C  addi r3, r10, 0x2e0c
	ctx.r[3].s64 = ctx.r[10].s64 + 11788;
	// 826E56CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E56D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E56D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E56D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E56DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E56E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E56E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E56E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E56EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E56F0: 4BD81731  bl 0x82466e20
	ctx.lr = 0x826E56F4;
	sub_82466E20(ctx, base);
	// 826E56F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E56F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E56FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5708 size=108
    let mut pc: u32 = 0x826E5708;
    'dispatch: loop {
        match pc {
            0x826E5708 => {
    //   block [0x826E5708..0x826E5774)
	// 826E5708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E570C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5714: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E571C: 38EB0470  addi r7, r11, 0x470
	ctx.r[7].s64 = ctx.r[11].s64 + 1136;
	// 826E5720: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E5724: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826E5728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E572C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5730: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E5734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5738: 386A2E3C  addi r3, r10, 0x2e3c
	ctx.r[3].s64 = ctx.r[10].s64 + 11836;
	// 826E573C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E5744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E574C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E575C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5760: 4BD816C1  bl 0x82466e20
	ctx.lr = 0x826E5764;
	sub_82466E20(ctx, base);
	// 826E5764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E576C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5778 size=108
    let mut pc: u32 = 0x826E5778;
    'dispatch: loop {
        match pc {
            0x826E5778 => {
    //   block [0x826E5778..0x826E57E4)
	// 826E5778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E577C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5784: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E578C: 38EB0488  addi r7, r11, 0x488
	ctx.r[7].s64 = ctx.r[11].s64 + 1160;
	// 826E5790: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E5794: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 826E5798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E579C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E57A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E57A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E57A8: 386A2E6C  addi r3, r10, 0x2e6c
	ctx.r[3].s64 = ctx.r[10].s64 + 11884;
	// 826E57AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E57B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E57B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E57B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E57BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E57C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E57C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E57C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E57CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E57D0: 4BD81651  bl 0x82466e20
	ctx.lr = 0x826E57D4;
	sub_82466E20(ctx, base);
	// 826E57D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E57D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E57DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E57E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E57E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E57E8 size=112
    let mut pc: u32 = 0x826E57E8;
    'dispatch: loop {
        match pc {
            0x826E57E8 => {
    //   block [0x826E57E8..0x826E5858)
	// 826E57E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E57EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E57F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E57F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E57F8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E57FC: 38AA2CBC  addi r5, r10, 0x2cbc
	ctx.r[5].s64 = ctx.r[10].s64 + 11452;
	// 826E5800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E5804: 390B04B8  addi r8, r11, 0x4b8
	ctx.r[8].s64 = ctx.r[11].s64 + 1208;
	// 826E5808: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826E580C: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826E5810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E5814: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E581C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5820: 386A2E9C  addi r3, r10, 0x2e9c
	ctx.r[3].s64 = ctx.r[10].s64 + 11932;
	// 826E5824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E5828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E582C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E5834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E583C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5844: 4BD815DD  bl 0x82466e20
	ctx.lr = 0x826E5848;
	sub_82466E20(ctx, base);
	// 826E5848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E584C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E5850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E5858 size=24
    let mut pc: u32 = 0x826E5858;
    'dispatch: loop {
        match pc {
            0x826E5858 => {
    //   block [0x826E5858..0x826E5870)
	// 826E5858: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E585C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E5860: 394A7650  addi r10, r10, 0x7650
	ctx.r[10].s64 = ctx.r[10].s64 + 30288;
	// 826E5864: 816B0560  lwz r11, 0x560(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1376 as u32) ) } as u64;
	// 826E5868: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826E586C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5870 size=112
    let mut pc: u32 = 0x826E5870;
    'dispatch: loop {
        match pc {
            0x826E5870 => {
    //   block [0x826E5870..0x826E58E0)
	// 826E5870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E5874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E587C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E5880: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5884: 392A7B54  addi r9, r10, 0x7b54
	ctx.r[9].s64 = ctx.r[10].s64 + 31572;
	// 826E5888: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E588C: 390B7650  addi r8, r11, 0x7650
	ctx.r[8].s64 = ctx.r[11].s64 + 30288;
	// 826E5890: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826E5894: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 826E5898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E589C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E58A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E58A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E58A8: 386A2ECC  addi r3, r10, 0x2ecc
	ctx.r[3].s64 = ctx.r[10].s64 + 11980;
	// 826E58AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E58B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E58B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E58B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E58BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E58C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E58C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E58C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E58CC: 4BD81555  bl 0x82466e20
	ctx.lr = 0x826E58D0;
	sub_82466E20(ctx, base);
	// 826E58D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E58D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E58D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E58DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E58E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E58E0 size=108
    let mut pc: u32 = 0x826E58E0;
    'dispatch: loop {
        match pc {
            0x826E58E0 => {
    //   block [0x826E58E0..0x826E594C)
	// 826E58E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E58E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E58E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E58EC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E58F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E58F4: 38EB0568  addi r7, r11, 0x568
	ctx.r[7].s64 = ctx.r[11].s64 + 1384;
	// 826E58F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E58FC: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 826E5900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E5904: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5908: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E590C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5910: 386A2EFC  addi r3, r10, 0x2efc
	ctx.r[3].s64 = ctx.r[10].s64 + 12028;
	// 826E5914: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5918: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E591C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E5924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E592C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5934: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5938: 4BD814E9  bl 0x82466e20
	ctx.lr = 0x826E593C;
	sub_82466E20(ctx, base);
	// 826E593C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E5944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5950 size=116
    let mut pc: u32 = 0x826E5950;
    'dispatch: loop {
        match pc {
            0x826E5950 => {
    //   block [0x826E5950..0x826E59C4)
	// 826E5950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E5954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E595C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5960: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E5964: 390B0598  addi r8, r11, 0x598
	ctx.r[8].s64 = ctx.r[11].s64 + 1432;
	// 826E5968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E596C: 392A7B98  addi r9, r10, 0x7b98
	ctx.r[9].s64 = ctx.r[10].s64 + 31640;
	// 826E5970: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5974: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826E5978: 38AA2CBC  addi r5, r10, 0x2cbc
	ctx.r[5].s64 = ctx.r[10].s64 + 11452;
	// 826E597C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E5980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E5984: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E5988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E598C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5994: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E5998: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 826E599C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E59A0: 386B2F2C  addi r3, r11, 0x2f2c
	ctx.r[3].s64 = ctx.r[11].s64 + 12076;
	// 826E59A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E59A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E59AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E59B0: 4BD81471  bl 0x82466e20
	ctx.lr = 0x826E59B4;
	sub_82466E20(ctx, base);
	// 826E59B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E59B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E59BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E59C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E59C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E59C8 size=24
    let mut pc: u32 = 0x826E59C8;
    'dispatch: loop {
        match pc {
            0x826E59C8 => {
    //   block [0x826E59C8..0x826E59E0)
	// 826E59C8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E59CC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E59D0: 394A76C8  addi r10, r10, 0x76c8
	ctx.r[10].s64 = ctx.r[10].s64 + 30408;
	// 826E59D4: 816B0658  lwz r11, 0x658(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1624 as u32) ) } as u64;
	// 826E59D8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826E59DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E59E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E59E0 size=112
    let mut pc: u32 = 0x826E59E0;
    'dispatch: loop {
        match pc {
            0x826E59E0 => {
    //   block [0x826E59E0..0x826E5A50)
	// 826E59E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E59E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E59E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E59EC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E59F0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E59F4: 392A7BD4  addi r9, r10, 0x7bd4
	ctx.r[9].s64 = ctx.r[10].s64 + 31700;
	// 826E59F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E59FC: 390B76C8  addi r8, r11, 0x76c8
	ctx.r[8].s64 = ctx.r[11].s64 + 30408;
	// 826E5A00: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826E5A04: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 826E5A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E5A0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5A10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E5A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5A18: 386A2F5C  addi r3, r10, 0x2f5c
	ctx.r[3].s64 = ctx.r[10].s64 + 12124;
	// 826E5A1C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E5A20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E5A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E5A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5A34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5A3C: 4BD813E5  bl 0x82466e20
	ctx.lr = 0x826E5A40;
	sub_82466E20(ctx, base);
	// 826E5A40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E5A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5A50 size=108
    let mut pc: u32 = 0x826E5A50;
    'dispatch: loop {
        match pc {
            0x826E5A50 => {
    //   block [0x826E5A50..0x826E5ABC)
	// 826E5A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E5A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5A5C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E5A64: 38EB065C  addi r7, r11, 0x65c
	ctx.r[7].s64 = ctx.r[11].s64 + 1628;
	// 826E5A68: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E5A6C: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826E5A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E5A74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5A78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E5A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5A80: 386A2F8C  addi r3, r10, 0x2f8c
	ctx.r[3].s64 = ctx.r[10].s64 + 12172;
	// 826E5A84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E5A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E5A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5AA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5AA8: 4BD81379  bl 0x82466e20
	ctx.lr = 0x826E5AAC;
	sub_82466E20(ctx, base);
	// 826E5AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E5AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5AC0 size=108
    let mut pc: u32 = 0x826E5AC0;
    'dispatch: loop {
        match pc {
            0x826E5AC0 => {
    //   block [0x826E5AC0..0x826E5B2C)
	// 826E5AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E5AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5ACC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E5AD4: 38EB0674  addi r7, r11, 0x674
	ctx.r[7].s64 = ctx.r[11].s64 + 1652;
	// 826E5AD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E5ADC: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 826E5AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E5AE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5AE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E5AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5AF0: 386A2FBC  addi r3, r10, 0x2fbc
	ctx.r[3].s64 = ctx.r[10].s64 + 12220;
	// 826E5AF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E5AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E5B04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5B14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5B18: 4BD81309  bl 0x82466e20
	ctx.lr = 0x826E5B1C;
	sub_82466E20(ctx, base);
	// 826E5B1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E5B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E5B30 size=24
    let mut pc: u32 = 0x826E5B30;
    'dispatch: loop {
        match pc {
            0x826E5B30 => {
    //   block [0x826E5B30..0x826E5B48)
	// 826E5B30: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5B34: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E5B38: 394A7710  addi r10, r10, 0x7710
	ctx.r[10].s64 = ctx.r[10].s64 + 30480;
	// 826E5B3C: 816B06A4  lwz r11, 0x6a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1700 as u32) ) } as u64;
	// 826E5B40: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826E5B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5B48 size=112
    let mut pc: u32 = 0x826E5B48;
    'dispatch: loop {
        match pc {
            0x826E5B48 => {
    //   block [0x826E5B48..0x826E5BB8)
	// 826E5B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E5B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5B54: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E5B58: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5B5C: 392A7C10  addi r9, r10, 0x7c10
	ctx.r[9].s64 = ctx.r[10].s64 + 31760;
	// 826E5B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E5B64: 390B7710  addi r8, r11, 0x7710
	ctx.r[8].s64 = ctx.r[11].s64 + 30480;
	// 826E5B68: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826E5B6C: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826E5B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E5B74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5B78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E5B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5B80: 386A2FEC  addi r3, r10, 0x2fec
	ctx.r[3].s64 = ctx.r[10].s64 + 12268;
	// 826E5B84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E5B88: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E5B8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E5B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5B9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5BA4: 4BD8127D  bl 0x82466e20
	ctx.lr = 0x826E5BA8;
	sub_82466E20(ctx, base);
	// 826E5BA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E5BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5BB8 size=112
    let mut pc: u32 = 0x826E5BB8;
    'dispatch: loop {
        match pc {
            0x826E5BB8 => {
    //   block [0x826E5BB8..0x826E5C28)
	// 826E5BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E5BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5BC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5BC8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5BCC: 38AA2CBC  addi r5, r10, 0x2cbc
	ctx.r[5].s64 = ctx.r[10].s64 + 11452;
	// 826E5BD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E5BD4: 390B06A8  addi r8, r11, 0x6a8
	ctx.r[8].s64 = ctx.r[11].s64 + 1704;
	// 826E5BD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E5BDC: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 826E5BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E5BE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5BE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E5BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5BF0: 386A301C  addi r3, r10, 0x301c
	ctx.r[3].s64 = ctx.r[10].s64 + 12316;
	// 826E5BF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E5BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E5BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E5C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5C14: 4BD8120D  bl 0x82466e20
	ctx.lr = 0x826E5C18;
	sub_82466E20(ctx, base);
	// 826E5C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E5C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5C28 size=108
    let mut pc: u32 = 0x826E5C28;
    'dispatch: loop {
        match pc {
            0x826E5C28 => {
    //   block [0x826E5C28..0x826E5C94)
	// 826E5C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E5C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5C34: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5C38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E5C3C: 38EB06D8  addi r7, r11, 0x6d8
	ctx.r[7].s64 = ctx.r[11].s64 + 1752;
	// 826E5C40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E5C44: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826E5C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E5C4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5C50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E5C54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5C58: 386A304C  addi r3, r10, 0x304c
	ctx.r[3].s64 = ctx.r[10].s64 + 12364;
	// 826E5C5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5C60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E5C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5C68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E5C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5C70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5C74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5C78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5C7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5C80: 4BD811A1  bl 0x82466e20
	ctx.lr = 0x826E5C84;
	sub_82466E20(ctx, base);
	// 826E5C84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E5C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5C98 size=108
    let mut pc: u32 = 0x826E5C98;
    'dispatch: loop {
        match pc {
            0x826E5C98 => {
    //   block [0x826E5C98..0x826E5D04)
	// 826E5C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E5C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5CA4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5CA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E5CAC: 38EB0708  addi r7, r11, 0x708
	ctx.r[7].s64 = ctx.r[11].s64 + 1800;
	// 826E5CB0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E5CB4: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826E5CB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E5CBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5CC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E5CC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5CC8: 386A307C  addi r3, r10, 0x307c
	ctx.r[3].s64 = ctx.r[10].s64 + 12412;
	// 826E5CCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5CD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E5CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5CD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E5CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5CE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5CE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5CEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5CF0: 4BD81131  bl 0x82466e20
	ctx.lr = 0x826E5CF4;
	sub_82466E20(ctx, base);
	// 826E5CF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E5CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5D08 size=108
    let mut pc: u32 = 0x826E5D08;
    'dispatch: loop {
        match pc {
            0x826E5D08 => {
    //   block [0x826E5D08..0x826E5D74)
	// 826E5D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E5D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5D14: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5D18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E5D1C: 38EB0768  addi r7, r11, 0x768
	ctx.r[7].s64 = ctx.r[11].s64 + 1896;
	// 826E5D20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E5D24: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826E5D28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E5D2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5D30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E5D34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5D38: 386A30AC  addi r3, r10, 0x30ac
	ctx.r[3].s64 = ctx.r[10].s64 + 12460;
	// 826E5D3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5D40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E5D44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5D48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E5D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5D50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5D54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5D58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5D5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5D60: 4BD810C1  bl 0x82466e20
	ctx.lr = 0x826E5D64;
	sub_82466E20(ctx, base);
	// 826E5D64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E5D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5D78 size=108
    let mut pc: u32 = 0x826E5D78;
    'dispatch: loop {
        match pc {
            0x826E5D78 => {
    //   block [0x826E5D78..0x826E5DE4)
	// 826E5D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E5D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5D80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5D84: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5D88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E5D8C: 38EB0798  addi r7, r11, 0x798
	ctx.r[7].s64 = ctx.r[11].s64 + 1944;
	// 826E5D90: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826E5D94: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 826E5D98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E5D9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5DA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E5DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5DA8: 386A30DC  addi r3, r10, 0x30dc
	ctx.r[3].s64 = ctx.r[10].s64 + 12508;
	// 826E5DAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5DB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E5DB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5DB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E5DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5DC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5DC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5DC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5DCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5DD0: 4BD81051  bl 0x82466e20
	ctx.lr = 0x826E5DD4;
	sub_82466E20(ctx, base);
	// 826E5DD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E5DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5DE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5DE8 size=108
    let mut pc: u32 = 0x826E5DE8;
    'dispatch: loop {
        match pc {
            0x826E5DE8 => {
    //   block [0x826E5DE8..0x826E5E54)
	// 826E5DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E5DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5DF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5DF4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5DF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E5DFC: 38EB08B8  addi r7, r11, 0x8b8
	ctx.r[7].s64 = ctx.r[11].s64 + 2232;
	// 826E5E00: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E5E04: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 826E5E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E5E0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5E10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E5E14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5E18: 386A310C  addi r3, r10, 0x310c
	ctx.r[3].s64 = ctx.r[10].s64 + 12556;
	// 826E5E1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5E20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E5E24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E5E2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5E34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5E3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5E40: 4BD80FE1  bl 0x82466e20
	ctx.lr = 0x826E5E44;
	sub_82466E20(ctx, base);
	// 826E5E44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E5E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5E58 size=108
    let mut pc: u32 = 0x826E5E58;
    'dispatch: loop {
        match pc {
            0x826E5E58 => {
    //   block [0x826E5E58..0x826E5EC4)
	// 826E5E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E5E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5E60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5E64: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5E68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E5E6C: 38EB08D0  addi r7, r11, 0x8d0
	ctx.r[7].s64 = ctx.r[11].s64 + 2256;
	// 826E5E70: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E5E74: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 826E5E78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E5E7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5E80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E5E84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5E88: 386A313C  addi r3, r10, 0x313c
	ctx.r[3].s64 = ctx.r[10].s64 + 12604;
	// 826E5E8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5E90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E5E94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5E98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E5E9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5EA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5EA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5EAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5EB0: 4BD80F71  bl 0x82466e20
	ctx.lr = 0x826E5EB4;
	sub_82466E20(ctx, base);
	// 826E5EB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E5EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5EC8 size=108
    let mut pc: u32 = 0x826E5EC8;
    'dispatch: loop {
        match pc {
            0x826E5EC8 => {
    //   block [0x826E5EC8..0x826E5F34)
	// 826E5EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E5ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5ED4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5ED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E5EDC: 38EB08E8  addi r7, r11, 0x8e8
	ctx.r[7].s64 = ctx.r[11].s64 + 2280;
	// 826E5EE0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E5EE4: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 826E5EE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E5EEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5EF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E5EF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5EF8: 386A316C  addi r3, r10, 0x316c
	ctx.r[3].s64 = ctx.r[10].s64 + 12652;
	// 826E5EFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5F00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E5F04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E5F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5F1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5F20: 4BD80F01  bl 0x82466e20
	ctx.lr = 0x826E5F24;
	sub_82466E20(ctx, base);
	// 826E5F24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E5F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5F38 size=108
    let mut pc: u32 = 0x826E5F38;
    'dispatch: loop {
        match pc {
            0x826E5F38 => {
    //   block [0x826E5F38..0x826E5FA4)
	// 826E5F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E5F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5F40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5F44: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5F48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E5F4C: 38EB0900  addi r7, r11, 0x900
	ctx.r[7].s64 = ctx.r[11].s64 + 2304;
	// 826E5F50: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E5F54: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 826E5F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E5F5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5F60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E5F64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5F68: 386A319C  addi r3, r10, 0x319c
	ctx.r[3].s64 = ctx.r[10].s64 + 12700;
	// 826E5F6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5F70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E5F74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E5F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5F8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E5F90: 4BD80E91  bl 0x82466e20
	ctx.lr = 0x826E5F94;
	sub_82466E20(ctx, base);
	// 826E5F94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E5F98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E5F9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E5FA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E5FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E5FA8 size=108
    let mut pc: u32 = 0x826E5FA8;
    'dispatch: loop {
        match pc {
            0x826E5FA8 => {
    //   block [0x826E5FA8..0x826E6014)
	// 826E5FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E5FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E5FB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E5FB4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E5FB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E5FBC: 38EB0918  addi r7, r11, 0x918
	ctx.r[7].s64 = ctx.r[11].s64 + 2328;
	// 826E5FC0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E5FC4: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 826E5FC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E5FCC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E5FD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E5FD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E5FD8: 386A31CC  addi r3, r10, 0x31cc
	ctx.r[3].s64 = ctx.r[10].s64 + 12748;
	// 826E5FDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E5FE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E5FE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E5FE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E5FEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E5FF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E5FF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E5FF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E5FFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E6000: 4BD80E21  bl 0x82466e20
	ctx.lr = 0x826E6004;
	sub_82466E20(ctx, base);
	// 826E6004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E600C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6018 size=108
    let mut pc: u32 = 0x826E6018;
    'dispatch: loop {
        match pc {
            0x826E6018 => {
    //   block [0x826E6018..0x826E6084)
	// 826E6018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E601C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6024: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6028: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E602C: 38EB0930  addi r7, r11, 0x930
	ctx.r[7].s64 = ctx.r[11].s64 + 2352;
	// 826E6030: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E6034: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 826E6038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E603C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6040: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E6044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6048: 386A31FC  addi r3, r10, 0x31fc
	ctx.r[3].s64 = ctx.r[10].s64 + 12796;
	// 826E604C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E6050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6058: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E605C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6060: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6064: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6068: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E606C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E6070: 4BD80DB1  bl 0x82466e20
	ctx.lr = 0x826E6074;
	sub_82466E20(ctx, base);
	// 826E6074: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E607C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6088 size=108
    let mut pc: u32 = 0x826E6088;
    'dispatch: loop {
        match pc {
            0x826E6088 => {
    //   block [0x826E6088..0x826E60F4)
	// 826E6088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E608C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6094: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6098: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E609C: 38EB0948  addi r7, r11, 0x948
	ctx.r[7].s64 = ctx.r[11].s64 + 2376;
	// 826E60A0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826E60A4: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 826E60A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E60AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E60B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E60B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E60B8: 386A322C  addi r3, r10, 0x322c
	ctx.r[3].s64 = ctx.r[10].s64 + 12844;
	// 826E60BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E60C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E60C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E60C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E60CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E60D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E60D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E60D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E60DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E60E0: 4BD80D41  bl 0x82466e20
	ctx.lr = 0x826E60E4;
	sub_82466E20(ctx, base);
	// 826E60E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E60E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E60EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E60F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E60F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E60F8 size=108
    let mut pc: u32 = 0x826E60F8;
    'dispatch: loop {
        match pc {
            0x826E60F8 => {
    //   block [0x826E60F8..0x826E6164)
	// 826E60F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E60FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6104: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E610C: 38EB09D8  addi r7, r11, 0x9d8
	ctx.r[7].s64 = ctx.r[11].s64 + 2520;
	// 826E6110: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826E6114: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 826E6118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E611C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6120: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E6124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6128: 386A325C  addi r3, r10, 0x325c
	ctx.r[3].s64 = ctx.r[10].s64 + 12892;
	// 826E612C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E6130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E613C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E614C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E6150: 4BD80CD1  bl 0x82466e20
	ctx.lr = 0x826E6154;
	sub_82466E20(ctx, base);
	// 826E6154: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E615C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6168 size=108
    let mut pc: u32 = 0x826E6168;
    'dispatch: loop {
        match pc {
            0x826E6168 => {
    //   block [0x826E6168..0x826E61D4)
	// 826E6168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E616C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6174: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6178: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E617C: 38EB0A98  addi r7, r11, 0xa98
	ctx.r[7].s64 = ctx.r[11].s64 + 2712;
	// 826E6180: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826E6184: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 826E6188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E618C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6190: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E6194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6198: 386A328C  addi r3, r10, 0x328c
	ctx.r[3].s64 = ctx.r[10].s64 + 12940;
	// 826E619C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E61A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E61A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E61A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E61AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E61B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E61B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E61B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E61BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E61C0: 4BD80C61  bl 0x82466e20
	ctx.lr = 0x826E61C4;
	sub_82466E20(ctx, base);
	// 826E61C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E61C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E61CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E61D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E61D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E61D8 size=108
    let mut pc: u32 = 0x826E61D8;
    'dispatch: loop {
        match pc {
            0x826E61D8 => {
    //   block [0x826E61D8..0x826E6244)
	// 826E61D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E61DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E61E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E61E4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E61E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E61EC: 38EB0B70  addi r7, r11, 0xb70
	ctx.r[7].s64 = ctx.r[11].s64 + 2928;
	// 826E61F0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826E61F4: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826E61F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E61FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6200: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E6204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6208: 386A32BC  addi r3, r10, 0x32bc
	ctx.r[3].s64 = ctx.r[10].s64 + 12988;
	// 826E620C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E6210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E621C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E622C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E6230: 4BD80BF1  bl 0x82466e20
	ctx.lr = 0x826E6234;
	sub_82466E20(ctx, base);
	// 826E6234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E623C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6248 size=108
    let mut pc: u32 = 0x826E6248;
    'dispatch: loop {
        match pc {
            0x826E6248 => {
    //   block [0x826E6248..0x826E62B4)
	// 826E6248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E624C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6254: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E625C: 38EB0C30  addi r7, r11, 0xc30
	ctx.r[7].s64 = ctx.r[11].s64 + 3120;
	// 826E6260: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826E6264: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 826E6268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E626C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6270: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E6274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6278: 386A32EC  addi r3, r10, 0x32ec
	ctx.r[3].s64 = ctx.r[10].s64 + 13036;
	// 826E627C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E6280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E628C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E629C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E62A0: 4BD80B81  bl 0x82466e20
	ctx.lr = 0x826E62A4;
	sub_82466E20(ctx, base);
	// 826E62A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E62A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E62AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E62B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E62B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E62B8 size=112
    let mut pc: u32 = 0x826E62B8;
    'dispatch: loop {
        match pc {
            0x826E62B8 => {
    //   block [0x826E62B8..0x826E6328)
	// 826E62B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E62BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E62C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E62C4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E62C8: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826E62CC: 38EA0CD8  addi r7, r10, 0xcd8
	ctx.r[7].s64 = ctx.r[10].s64 + 3288;
	// 826E62D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E62D4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E62D8: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 826E62DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E62E0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E62E4: 396B7C28  addi r11, r11, 0x7c28
	ctx.r[11].s64 = ctx.r[11].s64 + 31784;
	// 826E62E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E62EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E62F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E62F4: 386A331C  addi r3, r10, 0x331c
	ctx.r[3].s64 = ctx.r[10].s64 + 13084;
	// 826E62F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E62FC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E6300: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6304: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E6308: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E630C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E6310: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E6314: 4BD80B0D  bl 0x82466e20
	ctx.lr = 0x826E6318;
	sub_82466E20(ctx, base);
	// 826E6318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E631C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6328 size=112
    let mut pc: u32 = 0x826E6328;
    'dispatch: loop {
        match pc {
            0x826E6328 => {
    //   block [0x826E6328..0x826E6398)
	// 826E6328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E632C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6334: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6338: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E633C: 38AA2CBC  addi r5, r10, 0x2cbc
	ctx.r[5].s64 = ctx.r[10].s64 + 11452;
	// 826E6340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E6344: 390B0E10  addi r8, r11, 0xe10
	ctx.r[8].s64 = ctx.r[11].s64 + 3600;
	// 826E6348: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E634C: 388AA3A4  addi r4, r10, -0x5c5c
	ctx.r[4].s64 = ctx.r[10].s64 + -23644;
	// 826E6350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E6354: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6358: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E635C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6360: 386A334C  addi r3, r10, 0x334c
	ctx.r[3].s64 = ctx.r[10].s64 + 13132;
	// 826E6364: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E6368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E636C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E6374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E637C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E6384: 4BD80A9D  bl 0x82466e20
	ctx.lr = 0x826E6388;
	sub_82466E20(ctx, base);
	// 826E6388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E638C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6398 size=108
    let mut pc: u32 = 0x826E6398;
    'dispatch: loop {
        match pc {
            0x826E6398 => {
    //   block [0x826E6398..0x826E6404)
	// 826E6398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E639C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E63A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E63A4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E63A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E63AC: 38EB0E40  addi r7, r11, 0xe40
	ctx.r[7].s64 = ctx.r[11].s64 + 3648;
	// 826E63B0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E63B4: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 826E63B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E63BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E63C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E63C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E63C8: 386A337C  addi r3, r10, 0x337c
	ctx.r[3].s64 = ctx.r[10].s64 + 13180;
	// 826E63CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E63D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E63D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E63D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E63DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E63E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E63E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E63E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E63EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E63F0: 4BD80A31  bl 0x82466e20
	ctx.lr = 0x826E63F4;
	sub_82466E20(ctx, base);
	// 826E63F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E63F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E63FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6408 size=108
    let mut pc: u32 = 0x826E6408;
    'dispatch: loop {
        match pc {
            0x826E6408 => {
    //   block [0x826E6408..0x826E6474)
	// 826E6408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E640C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6414: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E641C: 38EB0EA0  addi r7, r11, 0xea0
	ctx.r[7].s64 = ctx.r[11].s64 + 3744;
	// 826E6420: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826E6424: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 826E6428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E642C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6430: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E6434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6438: 386A33AC  addi r3, r10, 0x33ac
	ctx.r[3].s64 = ctx.r[10].s64 + 13228;
	// 826E643C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E6440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E644C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E645C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E6460: 4BD809C1  bl 0x82466e20
	ctx.lr = 0x826E6464;
	sub_82466E20(ctx, base);
	// 826E6464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E646C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6478 size=108
    let mut pc: u32 = 0x826E6478;
    'dispatch: loop {
        match pc {
            0x826E6478 => {
    //   block [0x826E6478..0x826E64E4)
	// 826E6478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E647C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6484: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E648C: 38EB0FA8  addi r7, r11, 0xfa8
	ctx.r[7].s64 = ctx.r[11].s64 + 4008;
	// 826E6490: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826E6494: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826E6498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E649C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E64A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E64A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E64A8: 386A33DC  addi r3, r10, 0x33dc
	ctx.r[3].s64 = ctx.r[10].s64 + 13276;
	// 826E64AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E64B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E64B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E64B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E64BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E64C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E64C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E64C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E64CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E64D0: 4BD80951  bl 0x82466e20
	ctx.lr = 0x826E64D4;
	sub_82466E20(ctx, base);
	// 826E64D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E64D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E64DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E64E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E64E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E64E8 size=108
    let mut pc: u32 = 0x826E64E8;
    'dispatch: loop {
        match pc {
            0x826E64E8 => {
    //   block [0x826E64E8..0x826E6554)
	// 826E64E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E64EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E64F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E64F4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E64F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E64FC: 38EB1080  addi r7, r11, 0x1080
	ctx.r[7].s64 = ctx.r[11].s64 + 4224;
	// 826E6500: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826E6504: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 826E6508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E650C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6510: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E6514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6518: 386A340C  addi r3, r10, 0x340c
	ctx.r[3].s64 = ctx.r[10].s64 + 13324;
	// 826E651C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E6520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E652C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E653C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E6540: 4BD808E1  bl 0x82466e20
	ctx.lr = 0x826E6544;
	sub_82466E20(ctx, base);
	// 826E6544: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E654C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6558 size=108
    let mut pc: u32 = 0x826E6558;
    'dispatch: loop {
        match pc {
            0x826E6558 => {
    //   block [0x826E6558..0x826E65C4)
	// 826E6558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E655C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6564: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826E656C: 38EB10C8  addi r7, r11, 0x10c8
	ctx.r[7].s64 = ctx.r[11].s64 + 4296;
	// 826E6570: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E6574: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826E6578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E657C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6580: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E6584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6588: 386A343C  addi r3, r10, 0x343c
	ctx.r[3].s64 = ctx.r[10].s64 + 13372;
	// 826E658C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E6590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6594: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E659C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E65A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E65A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E65A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E65AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E65B0: 4BD80871  bl 0x82466e20
	ctx.lr = 0x826E65B4;
	sub_82466E20(ctx, base);
	// 826E65B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E65B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E65BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E65C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E65C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E65C8 size=108
    let mut pc: u32 = 0x826E65C8;
    'dispatch: loop {
        match pc {
            0x826E65C8 => {
    //   block [0x826E65C8..0x826E6634)
	// 826E65C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E65CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E65D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E65D4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E65D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E65DC: 38EB10E0  addi r7, r11, 0x10e0
	ctx.r[7].s64 = ctx.r[11].s64 + 4320;
	// 826E65E0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E65E4: 388AB3B4  addi r4, r10, -0x4c4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19532;
	// 826E65E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E65EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E65F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E65F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E65F8: 386A346C  addi r3, r10, 0x346c
	ctx.r[3].s64 = ctx.r[10].s64 + 13420;
	// 826E65FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E6600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E660C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E661C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E6620: 4BD80801  bl 0x82466e20
	ctx.lr = 0x826E6624;
	sub_82466E20(ctx, base);
	// 826E6624: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E662C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6638 size=108
    let mut pc: u32 = 0x826E6638;
    'dispatch: loop {
        match pc {
            0x826E6638 => {
    //   block [0x826E6638..0x826E66A4)
	// 826E6638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E663C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6644: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6648: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E664C: 38EB1148  addi r7, r11, 0x1148
	ctx.r[7].s64 = ctx.r[11].s64 + 4424;
	// 826E6650: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826E6654: 388AB3C0  addi r4, r10, -0x4c40
	ctx.r[4].s64 = ctx.r[10].s64 + -19520;
	// 826E6658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E665C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6660: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E6664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6668: 386A349C  addi r3, r10, 0x349c
	ctx.r[3].s64 = ctx.r[10].s64 + 13468;
	// 826E666C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E6670: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6674: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6678: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E667C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6680: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E668C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E6690: 4BD80791  bl 0x82466e20
	ctx.lr = 0x826E6694;
	sub_82466E20(ctx, base);
	// 826E6694: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E669C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E66A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E66A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E66A8 size=116
    let mut pc: u32 = 0x826E66A8;
    'dispatch: loop {
        match pc {
            0x826E66A8 => {
    //   block [0x826E66A8..0x826E671C)
	// 826E66A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E66AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E66B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E66B4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E66B8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E66BC: 390B1208  addi r8, r11, 0x1208
	ctx.r[8].s64 = ctx.r[11].s64 + 4616;
	// 826E66C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E66C4: 392A7CA4  addi r9, r10, 0x7ca4
	ctx.r[9].s64 = ctx.r[10].s64 + 31908;
	// 826E66C8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E66CC: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826E66D0: 38AA346C  addi r5, r10, 0x346c
	ctx.r[5].s64 = ctx.r[10].s64 + 13420;
	// 826E66D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E66D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E66DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E66E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E66E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E66E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E66EC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E66F0: 388AB3E0  addi r4, r10, -0x4c20
	ctx.r[4].s64 = ctx.r[10].s64 + -19488;
	// 826E66F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E66F8: 386B34CC  addi r3, r11, 0x34cc
	ctx.r[3].s64 = ctx.r[11].s64 + 13516;
	// 826E66FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E6700: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6708: 4BD80719  bl 0x82466e20
	ctx.lr = 0x826E670C;
	sub_82466E20(ctx, base);
	// 826E670C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6720 size=112
    let mut pc: u32 = 0x826E6720;
    'dispatch: loop {
        match pc {
            0x826E6720 => {
    //   block [0x826E6720..0x826E6790)
	// 826E6720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E6724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E672C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6730: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6734: 38AA580C  addi r5, r10, 0x580c
	ctx.r[5].s64 = ctx.r[10].s64 + 22540;
	// 826E6738: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E673C: 390B1298  addi r8, r11, 0x1298
	ctx.r[8].s64 = ctx.r[11].s64 + 4760;
	// 826E6740: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826E6744: 388AB3F0  addi r4, r10, -0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + -19472;
	// 826E6748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E674C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6750: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E6754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6758: 386A34FC  addi r3, r10, 0x34fc
	ctx.r[3].s64 = ctx.r[10].s64 + 13564;
	// 826E675C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E6760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E676C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E677C: 4BD806A5  bl 0x82466e20
	ctx.lr = 0x826E6780;
	sub_82466E20(ctx, base);
	// 826E6780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E678C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6790 size=96
    let mut pc: u32 = 0x826E6790;
    'dispatch: loop {
        match pc {
            0x826E6790 => {
    //   block [0x826E6790..0x826E67F0)
	// 826E6790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E6794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E679C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E67A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E67A4: 388AB40C  addi r4, r10, -0x4bf4
	ctx.r[4].s64 = ctx.r[10].s64 + -19444;
	// 826E67A8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E67AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E67B0: 386A352C  addi r3, r10, 0x352c
	ctx.r[3].s64 = ctx.r[10].s64 + 13612;
	// 826E67B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E67B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E67BC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E67C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E67C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E67C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E67CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E67D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E67D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E67D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E67DC: 4BD80645  bl 0x82466e20
	ctx.lr = 0x826E67E0;
	sub_82466E20(ctx, base);
	// 826E67E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E67E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E67E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E67EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E67F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E67F0 size=24
    let mut pc: u32 = 0x826E67F0;
    'dispatch: loop {
        match pc {
            0x826E67F0 => {
    //   block [0x826E67F0..0x826E6808)
	// 826E67F0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E67F4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E67F8: 394A7788  addi r10, r10, 0x7788
	ctx.r[10].s64 = ctx.r[10].s64 + 30600;
	// 826E67FC: 816B1144  lwz r11, 0x1144(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4420 as u32) ) } as u64;
	// 826E6800: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826E6804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6808 size=116
    let mut pc: u32 = 0x826E6808;
    'dispatch: loop {
        match pc {
            0x826E6808 => {
    //   block [0x826E6808..0x826E687C)
	// 826E6808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E680C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6814: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6818: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E681C: 390B7788  addi r8, r11, 0x7788
	ctx.r[8].s64 = ctx.r[11].s64 + 30600;
	// 826E6820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E6824: 392A7CF0  addi r9, r10, 0x7cf0
	ctx.r[9].s64 = ctx.r[10].s64 + 31984;
	// 826E6828: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E682C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826E6830: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E6834: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E6838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E683C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E6840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6844: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E6848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E684C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E6850: 388AB42C  addi r4, r10, -0x4bd4
	ctx.r[4].s64 = ctx.r[10].s64 + -19412;
	// 826E6854: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E6858: 386B355C  addi r3, r11, 0x355c
	ctx.r[3].s64 = ctx.r[11].s64 + 13660;
	// 826E685C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E6860: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6868: 4BD805B9  bl 0x82466e20
	ctx.lr = 0x826E686C;
	sub_82466E20(ctx, base);
	// 826E686C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6880 size=104
    let mut pc: u32 = 0x826E6880;
    'dispatch: loop {
        match pc {
            0x826E6880 => {
    //   block [0x826E6880..0x826E68E8)
	// 826E6880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E6884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E688C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E6890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E6894: 392A7D1C  addi r9, r10, 0x7d1c
	ctx.r[9].s64 = ctx.r[10].s64 + 32028;
	// 826E6898: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E689C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E68A0: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E68A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E68A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E68AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E68B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E68B4: 388AB440  addi r4, r10, -0x4bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -19392;
	// 826E68B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E68BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E68C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E68C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E68C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E68CC: 386A358C  addi r3, r10, 0x358c
	ctx.r[3].s64 = ctx.r[10].s64 + 13708;
	// 826E68D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E68D4: 4BD8054D  bl 0x82466e20
	ctx.lr = 0x826E68D8;
	sub_82466E20(ctx, base);
	// 826E68D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E68DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E68E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E68E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E68E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E68E8 size=96
    let mut pc: u32 = 0x826E68E8;
    'dispatch: loop {
        match pc {
            0x826E68E8 => {
    //   block [0x826E68E8..0x826E6948)
	// 826E68E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E68EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E68F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E68F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E68F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E68FC: 388AB454  addi r4, r10, -0x4bac
	ctx.r[4].s64 = ctx.r[10].s64 + -19372;
	// 826E6900: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6908: 386A35BC  addi r3, r10, 0x35bc
	ctx.r[3].s64 = ctx.r[10].s64 + 13756;
	// 826E690C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E6914: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E6918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E691C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E6924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6928: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E692C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E6930: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E6934: 4BD804ED  bl 0x82466e20
	ctx.lr = 0x826E6938;
	sub_82466E20(ctx, base);
	// 826E6938: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E693C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6948 size=96
    let mut pc: u32 = 0x826E6948;
    'dispatch: loop {
        match pc {
            0x826E6948 => {
    //   block [0x826E6948..0x826E69A8)
	// 826E6948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E694C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6954: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E6958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E695C: 388AB46C  addi r4, r10, -0x4b94
	ctx.r[4].s64 = ctx.r[10].s64 + -19348;
	// 826E6960: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6968: 386A35EC  addi r3, r10, 0x35ec
	ctx.r[3].s64 = ctx.r[10].s64 + 13804;
	// 826E696C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E6974: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E6978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E697C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E6984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6988: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E698C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E6990: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E6994: 4BD8048D  bl 0x82466e20
	ctx.lr = 0x826E6998;
	sub_82466E20(ctx, base);
	// 826E6998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E699C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E69A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E69A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E69A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E69A8 size=100
    let mut pc: u32 = 0x826E69A8;
    'dispatch: loop {
        match pc {
            0x826E69A8 => {
    //   block [0x826E69A8..0x826E6A0C)
	// 826E69A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E69AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E69B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E69B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E69B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E69BC: 38AA358C  addi r5, r10, 0x358c
	ctx.r[5].s64 = ctx.r[10].s64 + 13708;
	// 826E69C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E69C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E69C8: 388AB488  addi r4, r10, -0x4b78
	ctx.r[4].s64 = ctx.r[10].s64 + -19320;
	// 826E69CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E69D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E69D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E69D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E69DC: 386A361C  addi r3, r10, 0x361c
	ctx.r[3].s64 = ctx.r[10].s64 + 13852;
	// 826E69E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E69E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E69E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E69EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E69F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E69F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E69F8: 4BD80429  bl 0x82466e20
	ctx.lr = 0x826E69FC;
	sub_82466E20(ctx, base);
	// 826E69FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6A10 size=112
    let mut pc: u32 = 0x826E6A10;
    'dispatch: loop {
        match pc {
            0x826E6A10 => {
    //   block [0x826E6A10..0x826E6A80)
	// 826E6A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E6A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6A1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6A20: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6A24: 38AA355C  addi r5, r10, 0x355c
	ctx.r[5].s64 = ctx.r[10].s64 + 13660;
	// 826E6A28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E6A2C: 390B1300  addi r8, r11, 0x1300
	ctx.r[8].s64 = ctx.r[11].s64 + 4864;
	// 826E6A30: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E6A34: 388AB4A4  addi r4, r10, -0x4b5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19292;
	// 826E6A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E6A3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6A40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E6A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6A48: 386A364C  addi r3, r10, 0x364c
	ctx.r[3].s64 = ctx.r[10].s64 + 13900;
	// 826E6A4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E6A50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6A54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E6A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E6A6C: 4BD803B5  bl 0x82466e20
	ctx.lr = 0x826E6A70;
	sub_82466E20(ctx, base);
	// 826E6A70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6A80 size=112
    let mut pc: u32 = 0x826E6A80;
    'dispatch: loop {
        match pc {
            0x826E6A80 => {
    //   block [0x826E6A80..0x826E6AF0)
	// 826E6A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E6A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6A8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6A90: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6A94: 38AA355C  addi r5, r10, 0x355c
	ctx.r[5].s64 = ctx.r[10].s64 + 13660;
	// 826E6A98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E6A9C: 390B1348  addi r8, r11, 0x1348
	ctx.r[8].s64 = ctx.r[11].s64 + 4936;
	// 826E6AA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E6AA4: 388AB4B4  addi r4, r10, -0x4b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19276;
	// 826E6AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E6AAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6AB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E6AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6AB8: 386A367C  addi r3, r10, 0x367c
	ctx.r[3].s64 = ctx.r[10].s64 + 13948;
	// 826E6ABC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E6AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6AC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E6ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E6ADC: 4BD80345  bl 0x82466e20
	ctx.lr = 0x826E6AE0;
	sub_82466E20(ctx, base);
	// 826E6AE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


